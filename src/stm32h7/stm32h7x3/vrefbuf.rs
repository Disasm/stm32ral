#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! VREFBUF

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use RWRegister;

/// VREFBUF control and status register
pub mod VREFBUF_CSR {

    /// Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.
    pub mod ENVR {
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

    /// High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration.
    pub mod HIZ {
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

    /// Voltage reference buffer ready
    pub mod VRR {
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

    /// Voltage reference scale These bits select the value generated by the voltage reference buffer. Other: Reserved
    pub mod VRS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// VREFBUF calibration control register
pub mod VREFBUF_CCR {

    /// Trimming code These bits are automatically initialized after reset with the trimming value stored in the Flash memory during the production test. Writing into these bits allows to tune the internal reference buffer voltage.
    pub mod TRIM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// VREFBUF control and status register
    pub VREFBUF_CSR: RWRegister<u32>,

    /// VREFBUF calibration control register
    pub VREFBUF_CCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub VREFBUF_CSR: u32,
    pub VREFBUF_CCR: u32,
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

/// Access functions for the VREFBUF peripheral instance
pub mod VREFBUF {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58003c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in VREFBUF
    pub const reset: ResetValues = ResetValues {
        VREFBUF_CSR: 0x00000002,
        VREFBUF_CCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut VREFBUF_TAKEN: bool = false;

    /// Safe access to VREFBUF
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if VREFBUF_TAKEN {
                None
            } else {
                VREFBUF_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to VREFBUF
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if VREFBUF_TAKEN && inst.addr == INSTANCE.addr {
                VREFBUF_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to VREFBUF
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const VREFBUF: *const RegisterBlock = 0x58003c00 as *const _;
