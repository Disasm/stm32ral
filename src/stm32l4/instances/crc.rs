#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cyclic redundancy check calculation unit
//!
//! Used by: stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

#[cfg(not(feature = "nosync"))]
pub use stm32l4::peripherals::crc::Instance;
pub use stm32l4::peripherals::crc::{RegisterBlock, ResetValues};
pub use stm32l4::peripherals::crc::{CR, DR, IDR, INIT, POL};

/// Access functions for the CRC peripheral instance
pub mod CRC {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40023000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRC
    pub const reset: ResetValues = ResetValues {
        DR: 0xFFFFFFFF,
        IDR: 0x00000000,
        CR: 0x00000000,
        INIT: 0xFFFFFFFF,
        POL: 0x04C11DB7,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut CRC_TAKEN: bool = false;

    /// Safe access to CRC
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
            if CRC_TAKEN {
                None
            } else {
                CRC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRC_TAKEN && inst.addr == INSTANCE.addr {
                CRC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to CRC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRC: *const RegisterBlock = 0x40023000 as *const _;
