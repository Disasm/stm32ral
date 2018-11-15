#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Floting point unit
//!
//! Used by: stm32f401, stm32f405, stm32f407, stm32f410, stm32f411, stm32f412, stm32f413, stm32f427, stm32f429, stm32f446, stm32f469

#[cfg(not(feature = "nosync"))]
pub use stm32f4::peripherals::fpu::Instance;
pub use stm32f4::peripherals::fpu::{RegisterBlock, ResetValues};
pub use stm32f4::peripherals::fpu::{FPCAR, FPCCR, FPSCR};

/// Access functions for the FPU peripheral instance
pub mod FPU {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000ef34,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FPU
    pub const reset: ResetValues = ResetValues {
        FPCCR: 0x00000000,
        FPCAR: 0x00000000,
        FPSCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut FPU_TAKEN: bool = false;

    /// Safe access to FPU
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
            if FPU_TAKEN {
                None
            } else {
                FPU_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FPU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FPU_TAKEN && inst.addr == INSTANCE.addr {
                FPU_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to FPU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FPU: *const RegisterBlock = 0xe000ef34 as *const _;