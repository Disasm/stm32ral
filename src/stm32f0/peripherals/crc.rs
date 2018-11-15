#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! cyclic redundancy check calculation unit
//!
//! Used by: stm32f0x0, stm32f0x1, stm32f0x2, stm32f0x8

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use RWRegister;

/// Data register
pub mod DR {

    /// Data register bits
    pub mod DR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Independent data register
pub mod IDR {

    /// General-purpose 8-bit data register bits
    pub mod IDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Control register
pub mod CR {

    /// reset bit
    pub mod RESET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Polynomial size
    pub mod POLYSIZE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 32-bit polynomial
            pub const Polysize32: u32 = 0b00;

            /// 0b01: 16-bit polynomial
            pub const Polysize16: u32 = 0b01;

            /// 0b10: 8-bit polynomial
            pub const Polysize8: u32 = 0b10;

            /// 0b11: 7-bit polynomial
            pub const Polysize7: u32 = 0b11;
        }
    }

    /// Reverse input data
    pub mod REV_IN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Bit order not affected
            pub const Normal: u32 = 0b00;

            /// 0b01: Bit reversal done by byte
            pub const Byte: u32 = 0b01;

            /// 0b10: Bit reversal done by half-word
            pub const HalfWord: u32 = 0b10;

            /// 0b11: Bit reversal done by word
            pub const Word: u32 = 0b11;
        }
    }

    /// Reverse output data
    pub mod REV_OUT {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Bit order not affected
            pub const Normal: u32 = 0b0;

            /// 0b1: Bit reversed output
            pub const Reversed: u32 = 0b1;
        }
    }
}

/// Initial CRC value
pub mod INIT {

    /// Programmable initial CRC value
    pub mod INIT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// Data register
    pub DR: RWRegister<u32>,

    /// Independent data register
    pub IDR: RWRegister<u32>,

    /// Control register
    pub CR: RWRegister<u32>,

    /// Initial CRC value
    pub INIT: RWRegister<u32>,
}
pub struct ResetValues {
    pub DR: u32,
    pub IDR: u32,
    pub CR: u32,
    pub INIT: u32,
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