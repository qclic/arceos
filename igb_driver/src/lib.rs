//! Intel IGB NIC Driver Implementation.

#![no_std]
#![deny(warnings)]
#![deny(missing_docs)]
#![allow(dead_code)]


mod constants;
mod descriptor;
mod hal;
mod interrupts;
mod igb;
mod memory;

// mod net_buf;
pub mod net_igb;

extern crate alloc;
#[macro_use]
extern crate log;

pub use hal::IgbHal;
pub use igb::{IgbDevice, IgbNetBuf};

pub use memory::{alloc_pkt, MemPool, PhysAddr};

/// Vendor ID for Intel....
pub const INTEL_VEND: u16 = 0x8086;

/// Device ID for the 82576, used to identify the device from the PCI space.
pub const INTEL_82576: u16 = 0x10C9;

#[derive(Debug)]
/// Error type for Ixgbe functions.
pub enum IgbError {
    /// Queue size is not aligned.
    QueueNotAligned,
    /// Threr are not enough descriptors available in the queue, try again later.
    QueueFull,
    /// No memory
    NoMemory,
    /// Allocated page not aligned.
    PageNotAligned,
    /// The device is not ready.
    NotReady,
    /// Invalid `queue_id`.
    InvalidQueue,
}

/// Result type for Ixgbe functions.
pub type IgbResult<T = ()> = Result<T, IgbError>;

/// Used for implementing an ixy device driver like ixgbe or virtio.
pub trait NicDevice<H: IgbHal> {
    /// Returns the driver's name.
    fn get_driver_name(&self) -> &str;

    /// Returns the layer 2 address of this device.
    fn get_mac_addr(&self) -> [u8; 6];

    /// Resets the network card's stats registers.
    fn reset_stats(&mut self);

    /// Returns the network card's link speed.
    fn get_link_speed(&self) -> u16;

    /// Pool the transmit queue for sent packets and free their buffers.
    fn recycle_tx_buffers(&mut self, queue_id: u16) -> IgbResult;

    /// Receives `packet_nums` [`RxBuffer`] from network. If currently no data, returns an error
    /// with type [`IxgbeError::NotReady`], else returns the number of received packets. clourse `f` will
    /// be called for avoiding too many dynamic memory allocations.
    fn receive_packets<F>(&mut self, queue_id: u16, packet_nums: usize, f: F) -> IgbResult<usize>
    where
        F: FnMut(IgbNetBuf);

    /// Sends a [`TxBuffer`] to network. If currently queue is full, returns an
    /// error with type [`IxgbeError::QueueFull`].
    fn send(&mut self, queue_id: u16, tx_buf: IgbNetBuf) -> IgbResult;

    /// Whether can receive packet.
    fn can_receive(&self, queue_id: u16) -> IgbResult<bool>;

    /// Whether can send packet.
    fn can_send(&self, queue_id: u16) -> IgbResult<bool>;
}

/// Holds network card stats about sent and received packets.
#[allow(missing_docs)]
#[derive(Default, Copy, Clone)]
pub struct DeviceStats {
    pub rx_pkts: u64,
    pub tx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

impl core::fmt::Display for DeviceStats {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "rx_pkts: {}, tx_pkts: {}, rx_bytes: {}, tx_bytes: {}",
            self.rx_pkts, self.tx_pkts, self.rx_bytes, self.tx_bytes
        )
    }
}
