#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Power control
//!
//! Used by: stm32f401, stm32f411, stm32f412, stm32f413

#[cfg(not(feature = "nosync"))]
pub use stm32f4::peripherals::pwr_v1::Instance;
pub use stm32f4::peripherals::pwr_v1::{RegisterBlock, ResetValues};
pub use stm32f4::peripherals::pwr_v1::{CR, CSR};

/// Access functions for the PWR peripheral instance
pub mod PWR {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40007000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PWR
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        CSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut PWR_TAKEN: bool = false;

    /// Safe access to PWR
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
            if PWR_TAKEN {
                None
            } else {
                PWR_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PWR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PWR_TAKEN && inst.addr == INSTANCE.addr {
                PWR_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to PWR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWR: *const RegisterBlock = 0x40007000 as *const _;
