#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]
// list of all NIC registers and some structs
// copied and changed from the ixy C driver and DPDK

/*******************************************************************************

Copyright (c) 2001-2020, Intel Corporation
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

 1. Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.

 2. Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimer in the
    documentation and/or other materials provided with the distribution.

 3. Neither the name of the Intel Corporation nor the names of its
    contributors may be used to endorse or promote products derived from
    this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.

***************************************************************************/

/* Vendor ID */
pub const IGB_INTEL_VENDOR_ID: u32 = 0x8086;

/* Device IDs */
pub const IGB_DEV_ID_82576: u32 = 0x10C9;

// unused/unsupported by ixy
pub fn IXGBE_BY_MAC(_hw: u32, _r: u32) -> u32 {
    0
}

/* General Registers */
pub const IGB_CTRL: u32 = 0x00000;
pub const IGB_STATUS: u32 = 0x00008;
pub const IGB_CTRL_EXT: u32 = 0x00018;
pub const IGB_MDIC: u32 = 0x00020;

pub const IGB_RCTL: u32 = 0x00000100;
pub const IGB_TCTL: u32 = 0x00000400;

pub const IGB_TXDCTL_WTHRESH: u32 = 0x10000;
pub const IGB_TXDCTL_EN: u32 = 0x2000000;
pub const IGB_TCTL_EN: u32 = 0x2;


/* Interrupt Registers */
pub const IGB_EICR: u32 = 0x01580;
pub const IGB_EICS: u32 = 0x01520;
pub const IGB_EIMS: u32 = 0x01524;
pub const IGB_EIMC: u32 = 0x01528;
pub const IGB_EIAC: u32 = 0x0152C;
pub const IGB_EIAM: u32 = 0x01530;

pub const IGB_IRQ_CLEAR_MASK: u32 = 0xFFFFFFFF;

/* Receive DMA Registers */
pub fn IGB_RDBAL(i: u32) -> u32 {
    if i == 0 {
        0x0C000
    } else if i <= 3{
        0x0C040 + ((i - 1) * 0x40)
    } else {
        0xC100 + (i - 4) * 0x40
    }
}
pub fn IGB_RDBAH(i: u32) -> u32 {
    0x0C004 + i * 0x40
}
pub fn IGB_RDLEN(i: u32) -> u32 {
        0x0C008 + i * 0x40
}
pub fn IGB_RDH(i: u32) -> u32 {
        0x0C010 + i * 0x40
}
pub fn IGB_RDT(i: u32) -> u32 {
        0x0C018 + (i * 0x40)
}
pub fn IGB_RXDCTL(i: u32) -> u32 {
        0x0C028 + i * 0x40
}

/*
 * Split and Replication Receive Control Registers
 */
pub fn IGB_SRRCTL(i: u32) -> u32 {
    if i == 0 {
        0x0C00C
    } else if i <= 3 {
        0x0C04C + (i - 1) * 0x40
    } else {
        0xC10C + ((i - 4) * 0x40)
    }
}
/*
 * Rx DCA Control Register:
 * 00-15 : 0x02200 + n*4;
 * 16-64 : 0x0100C + n*0x40;
 * 64-127: 0x0D00C + (n-64)*0x40;
 */
pub fn IGB_DCA_RXCTRL(i: u32) -> u32 {
    0x0C014 + i * 0x40
}
/* Transmit DMA registers */
pub fn IGB_TDBAL(i: u32) -> u32 {
    0x0E000 + i * 0x40
} /* 32 of them (0-31)*/
pub fn IGB_TDBAH(i: u32) -> u32 {
    0x0E004 + i * 0x40
}

pub fn IGB_TDLEN(i: u32) -> u32 {
    0x0E008 + i * 0x40
}

pub fn IGB_TDH(i: u32) -> u32 {
    0x0E010 + i * 0x40
}

pub fn IGB_TDT(i: u32) -> u32 {
    0x0E018 + i * 0x40
}

pub fn IGB_TXDCTL(i: u32) -> u32 {
    0x0E028 + i * 0x40
}

pub fn IGB_TDWBAL(i: u32) -> u32 {
    0x0E038 + i * 0x40
}

pub fn IGB_TDWBAH(i: u32) -> u32 {
    0x0E03C + i * 0x40
}

pub const IGB_DMATXCTL: u32 = 0x03590;

pub const IGB_TXPBSIZE: u32 = 0x3404;

/*statistic register*/
pub const IGB_GPRC: u32 = 0x04074;
pub const IGB_BPRC: u32 = 0x04078;
pub const IGB_MPRC: u32 = 0x0407C;
pub const IGB_GPTC: u32 = 0x04080;
pub const IGB_GORCL: u32 = 0x04088;
pub const IGB_GORCH: u32 = 0x0408C;
pub const IGB_GOTCL: u32 = 0x04090;
pub const IGB_GOTCH: u32 = 0x04094;

/* CTRL Bit Masks */
pub const IGB_CTRL_FD: u32 = 0x00000001; /* Full-Duplex */
pub const IGB_CTRL_LNK_RST: u32 = 0x00000008; /* Link Reset. Resets everything. */
pub const IGB_CTRL_SLU: u32 = 0x00000040; /* set link up. */
pub const IGB_CTRL_SPEED: u32 = 0x00000300; /* SPEED. */
pub const IGB_CTRL_RST: u32 = 0x04000000; /* Reset (SW) */
pub const IGB_CTRL_RFCE: u32 = 0x08000000; /* Receive Flow Control Enable. */
pub const IGB_CTRL_TFCE: u32 = 0x10000000; /* Transmit Flow Control Enable. */
pub const IGB_CTRL_PHY_RST: u32 = 0x80000000; /* PHY Reset. */
pub const IGB_CTRL_RST_MASK: u32 = IGB_CTRL_LNK_RST | IGB_CTRL_RST;

/* STATUS register bit mask*/
pub const IGB_STATUS_FD: u32 = 0x00000001;
pub const IGB_STATUS_LU: u32 = 0x00000002;
pub const IGB_STATUS_TXOFF: u32 = 0x00000010;

/*RX control bit mask */
pub const IGB_RCTL_EN: u32 = 0x00000002;

pub const IGB_CTRL_EXT_NS_DIS: u32 = 0x00010000; /* No Snoop disable */

pub const IGB_TXPBSIZE_40KB: u32 = 0x28; /* 40KB Packet Buffer */

/* Packet buffer allocation strategies */

pub const PBA_STRATEGY_EQUAL: u32 = 0; /* Distribute PB space equally */
pub const PBA_STRATEGY_WEIGHTED: u32 = 1; /* Weight front half of TCs */


pub const IGB_SRRCTL_DESCTYPE_ADV_ONEBUF: u32 = 0x02000000;
pub const IGB_SRRCTL_DESCTYPE_MASK: u32 = 0x0E000000;

pub const IGB_LINKS_UP: u32 = 0x2;
pub const IGB_LINKS_SPEED_82576: u32 = 0xC0;
pub const IGB_LINKS_SPEED_10_82576: u32 = 0x0;
pub const IGB_LINKS_SPEED_100_82576: u32 = 0x1;
pub const IGB_LINKS_SPEED_1000_82576: u32 = 0x2;

pub fn IGB_RAL(i: u32) -> u32 {
    if i <= 15 {
        0x05400 + i * 8
    } else {
        0x054E0 + (i - 16) * 8
    }
}

pub fn IGB_RAH(i: u32) -> u32 {
    if i <= 15 {
        0x05404 + i * 8
    } else {
        0x054E4 + (i - 16) * 8
    }
}

pub const IGB_ADVTXD_STAT_DD: u32 = 0x1;
pub const IGB_SRRCTL_DROP_EN: u32 = 0x80000000;
pub const IGB_RXDCTL_ENABLE: u32 = 0x02000000; /* Ena specific Rx Queue */

pub const IGB_PHY_CTRL: u32 = 0;
pub const IGB_PHY_STATUS: u32 = 0;

pub const IGB_PHY_AUTONE: u32 = 0x1000;
pub const IGB_PHY_RESTART: u32 = 0x0200;