//! Common traits and types for network device (NIC) drivers.

// #![no_std]
// #![feature(const_mut_refs)]
// #![feature(const_slice_from_raw_parts_mut)]

use core::convert::From;
use core::{mem::ManuallyDrop, ptr::NonNull};
use alloc::{collections::VecDeque, sync::Arc};
use axdriver_base::{BaseDriverOps, DevError, DevResult, DeviceType};
use crate::{IgbDevice, IgbError, IgbNetBuf, MemPool, NicDevice};
// pub use crate::{IgbHal, PhysAddr, INTEL_82576, INTEL_VEND};
pub use crate::IgbHal;


// pub use crate::net_buf::{NetBuf, NetBufBox, NetBufPool};

/// The ethernet address of the NIC (MAC address).
pub struct EthernetAddress(pub [u8; 6]);

/// Operations that require a network device (NIC) driver to implement.
pub trait NetDriverOps: BaseDriverOps {
    /// The ethernet address of the NIC.
    fn mac_address(&self) -> EthernetAddress;

    /// Whether can transmit packets.
    fn can_transmit(&self) -> bool;

    /// Whether can receive packets.
    fn can_receive(&self) -> bool;

    /// Size of the receive queue.
    fn rx_queue_size(&self) -> usize;

    /// Size of the transmit queue.
    fn tx_queue_size(&self) -> usize;

    /// Gives back the `rx_buf` to the receive queue for later receiving.
    ///
    /// `rx_buf` should be the same as the one returned by
    /// [`NetDriverOps::receive`].
    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult;

    /// Poll the transmit queue and gives back the buffers for previous transmiting.
    /// returns [`DevResult`].
    fn recycle_tx_buffers(&mut self) -> DevResult;

    /// Transmits a packet in the buffer to the network, without blocking,
    /// returns [`DevResult`].
    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult;

    /// Receives a packet from the network and store it in the [`NetBuf`],
    /// returns the buffer.
    ///
    /// Before receiving, the driver should have already populated some buffers
    /// in the receive queue by [`NetDriverOps::recycle_rx_buffer`].
    ///
    /// If currently no incomming packets, returns an error with type
    /// [`DevError::Again`].
    fn receive(&mut self) -> DevResult<NetBufPtr>;

    /// Allocate a memory buffer of a specified size for network transmission,
    /// returns [`DevResult`]
    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr>;
}

/// A raw buffer struct for network device.
pub struct NetBufPtr {
    // The raw pointer of the original object.
    raw_ptr: NonNull<u8>,
    // The pointer to the net buffer.
    buf_ptr: NonNull<u8>,
    len: usize,
}

impl NetBufPtr {
    /// Create a new [`NetBufPtr`].
    pub fn new(raw_ptr: NonNull<u8>, buf_ptr: NonNull<u8>, len: usize) -> Self {
        Self {
            raw_ptr,
            buf_ptr,
            len,
        }
    }

    /// Return raw pointer of the original object.
    pub fn raw_ptr<T>(&self) -> *mut T {
        self.raw_ptr.as_ptr() as *mut T
    }

    /// Return [`NetBufPtr`] buffer len.
    pub fn packet_len(&self) -> usize {
        self.len
    }

    /// Return [`NetBufPtr`] buffer as &[u8].
    pub fn packet(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buf_ptr.as_ptr() as *const u8, self.len) }
    }

    /// Return [`NetBufPtr`] buffer as &mut [u8].
    pub fn packet_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.buf_ptr.as_ptr(), self.len) }
    }
}


const RECV_BATCH_SIZE: usize = 64;
const RX_BUFFER_SIZE: usize = 1024;
const MEM_POOL: usize = 4096;
const MEM_POOL_ENTRY_SIZE: usize = 2048;

/// The ixgbe NIC device driver.
///
/// `QS` is the ixgbe queue size, `QN` is the ixgbe queue num.
pub struct IgbNic<H: IgbHal, const QS: usize, const QN: u16> {
    inner: IgbDevice<H, QS>,
    mem_pool: Arc<MemPool>,
    rx_buffer_queue: VecDeque<NetBufPtr>,
}

unsafe impl<H: IgbHal, const QS: usize, const QN: u16> Sync for IgbNic<H, QS, QN> {}
unsafe impl<H: IgbHal, const QS: usize, const QN: u16> Send for IgbNic<H, QS, QN> {}

impl<H: IgbHal, const QS: usize, const QN: u16> IgbNic<H, QS, QN> {
    /// Creates a net ixgbe NIC instance and initialize, or returns a error if
    /// any step fails.
    pub fn init(base: usize, len: usize) -> DevResult<Self> {
        let mem_pool = MemPool::allocate::<H>(MEM_POOL, MEM_POOL_ENTRY_SIZE)
            .map_err(|_| DevError::NoMemory)?;
        let inner = IgbDevice::<H, QS>::init(base, len, QN, QN, &mem_pool).map_err(|err| {
            log::error!("Failed to initialize ixgbe device: {:?}", err);
            DevError::BadState
        })?;

        let rx_buffer_queue = VecDeque::with_capacity(RX_BUFFER_SIZE);
        Ok(Self {
            inner,
            mem_pool,
            rx_buffer_queue,
        })
    }
}

impl<H: IgbHal, const QS: usize, const QN: u16> BaseDriverOps for IgbNic<H, QS, QN> {
    fn device_name(&self) -> &str {
        self.inner.get_driver_name()
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Net
    }
}

impl<H: IgbHal, const QS: usize, const QN: u16> NetDriverOps for IgbNic<H, QS, QN> {
    fn mac_address(&self) -> EthernetAddress {
        EthernetAddress(self.inner.get_mac_addr())
    }

    fn rx_queue_size(&self) -> usize {
        QS
    }

    fn tx_queue_size(&self) -> usize {
        QS
    }

    fn can_receive(&self) -> bool {
        !self.rx_buffer_queue.is_empty() || self.inner.can_receive(0).unwrap()
    }

    fn can_transmit(&self) -> bool {
        // Default implementation is return true forever.
        self.inner.can_send(0).unwrap()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        let rx_buf = igb_ptr_to_buf(rx_buf, &self.mem_pool)?;
        drop(rx_buf);
        Ok(())
    }

    fn recycle_tx_buffers(&mut self) -> DevResult {
        self.inner
            .recycle_tx_buffers(0)
            .map_err(|_| DevError::BadState)?;
        Ok(())
    }

    fn receive(&mut self) -> DevResult<NetBufPtr> {
        if !self.can_receive() {
            return Err(DevError::Again);
        }
        if !self.rx_buffer_queue.is_empty() {
            // RX buffer have received packets.
            Ok(self.rx_buffer_queue.pop_front().unwrap())
        } else {
            let f = |rx_buf| {
                let rx_buf = NetBufPtr::from(rx_buf);
                self.rx_buffer_queue.push_back(rx_buf);
            };

            // RX queue is empty, receive from ixgbe NIC.
            match self.inner.receive_packets(0, RECV_BATCH_SIZE, f) {
                Ok(recv_nums) => {
                    if recv_nums == 0 {
                        // No packet is received, it is impossible things.
                        panic!("Error: No receive packets.")
                    } else {
                        Ok(self.rx_buffer_queue.pop_front().unwrap())
                    }
                }
                Err(e) => match e {
                    IgbError::NotReady => Err(DevError::Again),
                    _ => Err(DevError::BadState),
                },
            }
        }
    }

    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        let tx_buf = igb_ptr_to_buf(tx_buf, &self.mem_pool)?;
        match self.inner.send(0, tx_buf) {
            Ok(_) => Ok(()),
            Err(err) => match err {
                IgbError::QueueFull => Err(DevError::Again),
                _ => panic!("Unexpected err: {:?}", err),
            },
        }
    }

    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        let tx_buf = IgbNetBuf::alloc(&self.mem_pool, size).map_err(|_| DevError::NoMemory)?;
        Ok(NetBufPtr::from(tx_buf))
    }
}

impl From<IgbNetBuf> for NetBufPtr {
    fn from(buf: IgbNetBuf) -> Self {
        // Use `ManuallyDrop` to avoid drop `tx_buf`.
        let mut buf = ManuallyDrop::new(buf);
        // In ixgbe, `raw_ptr` is the pool entry, `buf_ptr` is the packet ptr, `len` is packet len
        // to avoid too many dynamic memory allocation.
        let buf_ptr = buf.packet_mut().as_mut_ptr();
        Self::new(
            NonNull::new(buf.pool_entry() as *mut u8).unwrap(),
            NonNull::new(buf_ptr).unwrap(),
            buf.packet_len(),
        )
    }
}

// Converts a `NetBufPtr` to `IxgbeNetBuf`.
fn igb_ptr_to_buf(ptr: NetBufPtr, pool: &Arc<MemPool>) -> DevResult<IgbNetBuf> {
    IgbNetBuf::construct(ptr.raw_ptr.as_ptr() as usize, pool, ptr.len)
        .map_err(|_| DevError::BadState)
}
