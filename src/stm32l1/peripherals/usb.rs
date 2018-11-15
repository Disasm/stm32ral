#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal serial bus full-speed device interface
//!
//! Used by: stm32l100, stm32l151, stm32l162

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister};

/// endpoint 0 register
pub mod USB_EP0R {

    /// Endpoint address
    pub mod EA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status bits, for transmission transfers
    pub mod STAT_TX {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Toggle, for transmission transfers
    pub mod DTOG_TX {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Correct Transfer for transmission
    pub mod CTR_TX {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Endpoint kind
    pub mod EP_KIND {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Endpoint type
    pub mod EP_TYPE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Setup transaction completed
    pub mod SETUP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status bits, for reception transfers
    pub mod STAT_RX {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Toggle, for reception transfers
    pub mod DTOG_RX {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Correct transfer for reception
    pub mod CTR_RX {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// endpoint 1 register
pub mod USB_EP1R {
    pub use super::USB_EP0R::CTR_RX;
    pub use super::USB_EP0R::CTR_TX;
    pub use super::USB_EP0R::DTOG_RX;
    pub use super::USB_EP0R::DTOG_TX;
    pub use super::USB_EP0R::EA;
    pub use super::USB_EP0R::EP_KIND;
    pub use super::USB_EP0R::EP_TYPE;
    pub use super::USB_EP0R::SETUP;
    pub use super::USB_EP0R::STAT_RX;
    pub use super::USB_EP0R::STAT_TX;
}

/// endpoint 2 register
pub mod USB_EP2R {
    pub use super::USB_EP0R::CTR_RX;
    pub use super::USB_EP0R::CTR_TX;
    pub use super::USB_EP0R::DTOG_RX;
    pub use super::USB_EP0R::DTOG_TX;
    pub use super::USB_EP0R::EA;
    pub use super::USB_EP0R::EP_KIND;
    pub use super::USB_EP0R::EP_TYPE;
    pub use super::USB_EP0R::SETUP;
    pub use super::USB_EP0R::STAT_RX;
    pub use super::USB_EP0R::STAT_TX;
}

/// endpoint 3 register
pub mod USB_EP3R {
    pub use super::USB_EP0R::CTR_RX;
    pub use super::USB_EP0R::CTR_TX;
    pub use super::USB_EP0R::DTOG_RX;
    pub use super::USB_EP0R::DTOG_TX;
    pub use super::USB_EP0R::EA;
    pub use super::USB_EP0R::EP_KIND;
    pub use super::USB_EP0R::EP_TYPE;
    pub use super::USB_EP0R::SETUP;
    pub use super::USB_EP0R::STAT_RX;
    pub use super::USB_EP0R::STAT_TX;
}

/// endpoint 4 register
pub mod USB_EP4R {
    pub use super::USB_EP0R::CTR_RX;
    pub use super::USB_EP0R::CTR_TX;
    pub use super::USB_EP0R::DTOG_RX;
    pub use super::USB_EP0R::DTOG_TX;
    pub use super::USB_EP0R::EA;
    pub use super::USB_EP0R::EP_KIND;
    pub use super::USB_EP0R::EP_TYPE;
    pub use super::USB_EP0R::SETUP;
    pub use super::USB_EP0R::STAT_RX;
    pub use super::USB_EP0R::STAT_TX;
}

/// endpoint 5 register
pub mod USB_EP5R {
    pub use super::USB_EP0R::CTR_RX;
    pub use super::USB_EP0R::CTR_TX;
    pub use super::USB_EP0R::DTOG_RX;
    pub use super::USB_EP0R::DTOG_TX;
    pub use super::USB_EP0R::EA;
    pub use super::USB_EP0R::EP_KIND;
    pub use super::USB_EP0R::EP_TYPE;
    pub use super::USB_EP0R::SETUP;
    pub use super::USB_EP0R::STAT_RX;
    pub use super::USB_EP0R::STAT_TX;
}

/// endpoint 6 register
pub mod USB_EP6R {
    pub use super::USB_EP0R::CTR_RX;
    pub use super::USB_EP0R::CTR_TX;
    pub use super::USB_EP0R::DTOG_RX;
    pub use super::USB_EP0R::DTOG_TX;
    pub use super::USB_EP0R::EA;
    pub use super::USB_EP0R::EP_KIND;
    pub use super::USB_EP0R::EP_TYPE;
    pub use super::USB_EP0R::SETUP;
    pub use super::USB_EP0R::STAT_RX;
    pub use super::USB_EP0R::STAT_TX;
}

/// endpoint 7 register
pub mod USB_EP7R {
    pub use super::USB_EP0R::CTR_RX;
    pub use super::USB_EP0R::CTR_TX;
    pub use super::USB_EP0R::DTOG_RX;
    pub use super::USB_EP0R::DTOG_TX;
    pub use super::USB_EP0R::EA;
    pub use super::USB_EP0R::EP_KIND;
    pub use super::USB_EP0R::EP_TYPE;
    pub use super::USB_EP0R::SETUP;
    pub use super::USB_EP0R::STAT_RX;
    pub use super::USB_EP0R::STAT_TX;
}

/// control register
pub mod USB_CNTR {

    /// Force USB Reset
    pub mod FRES {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power down
    pub mod PDWN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low-power mode
    pub mod LPMODE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Force suspend
    pub mod FSUSP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Resume request
    pub mod RESUME {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Expected start of frame interrupt mask
    pub mod ESOFM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Start of frame interrupt mask
    pub mod SOFM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB reset interrupt mask
    pub mod RESETM {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Suspend mode interrupt mask
    pub mod SUSPM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup interrupt mask
    pub mod WKUPM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Error interrupt mask
    pub mod ERRM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Packet memory area over / underrun interrupt mask
    pub mod PMAOVRM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Correct transfer interrupt mask
    pub mod CTRM {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt status register
pub mod ISTR {

    /// Endpoint Identifier
    pub mod EP_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Direction of transaction
    pub mod DIR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Expected start frame
    pub mod ESOF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// start of frame
    pub mod SOF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// reset request
    pub mod RESET {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Suspend mode request
    pub mod SUSP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup
    pub mod WKUP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Error
    pub mod ERR {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Packet memory area over / underrun
    pub mod PMAOVR {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Correct transfer
    pub mod CTR {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// frame number register
pub mod FNR {

    /// Frame number
    pub mod FN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lost SOF
    pub mod LSOF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Locked
    pub mod LCK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive data - line status
    pub mod RXDM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive data + line status
    pub mod RXDP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// device address
pub mod DADDR {

    /// Device address
    pub mod ADD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable function
    pub mod EF {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Buffer table address
pub mod BTABLE {

    /// Buffer table
    pub mod BTABLE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (13 bits: 0x1fff << 3)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// endpoint 0 register
    pub USB_EP0R: RWRegister<u32>,

    /// endpoint 1 register
    pub USB_EP1R: RWRegister<u32>,

    /// endpoint 2 register
    pub USB_EP2R: RWRegister<u32>,

    /// endpoint 3 register
    pub USB_EP3R: RWRegister<u32>,

    /// endpoint 4 register
    pub USB_EP4R: RWRegister<u32>,

    /// endpoint 5 register
    pub USB_EP5R: RWRegister<u32>,

    /// endpoint 6 register
    pub USB_EP6R: RWRegister<u32>,

    /// endpoint 7 register
    pub USB_EP7R: RWRegister<u32>,

    _reserved1: [u32; 8],

    /// control register
    pub USB_CNTR: RWRegister<u32>,

    /// interrupt status register
    pub ISTR: RWRegister<u32>,

    /// frame number register
    pub FNR: RORegister<u32>,

    /// device address
    pub DADDR: RWRegister<u32>,

    /// Buffer table address
    pub BTABLE: RWRegister<u32>,
}
pub struct ResetValues {
    pub USB_EP0R: u32,
    pub USB_EP1R: u32,
    pub USB_EP2R: u32,
    pub USB_EP3R: u32,
    pub USB_EP4R: u32,
    pub USB_EP5R: u32,
    pub USB_EP6R: u32,
    pub USB_EP7R: u32,
    pub USB_CNTR: u32,
    pub ISTR: u32,
    pub FNR: u32,
    pub DADDR: u32,
    pub BTABLE: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
