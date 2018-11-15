#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flash
//!
//! Used by: stm32l0x1, stm32l0x2, stm32l0x3

#[cfg(not(feature = "nosync"))]
pub use stm32l0::peripherals::flash::Instance;
pub use stm32l0::peripherals::flash::{RegisterBlock, ResetValues};
pub use stm32l0::peripherals::flash::{ACR, OBR, OPTKEYR, PDKEYR, PECR, PEKEYR, PRGKEYR, SR, WRPR};

/// Access functions for the Flash peripheral instance
pub mod Flash {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40022000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Flash
    pub const reset: ResetValues = ResetValues {
        ACR: 0x00000000,
        PECR: 0x00000007,
        PDKEYR: 0x00000000,
        PEKEYR: 0x00000000,
        PRGKEYR: 0x00000000,
        OPTKEYR: 0x00000000,
        SR: 0x00000004,
        OBR: 0x00F80000,
        WRPR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut Flash_TAKEN: bool = false;

    /// Safe access to Flash
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
            if Flash_TAKEN {
                None
            } else {
                Flash_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Flash
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Flash_TAKEN && inst.addr == INSTANCE.addr {
                Flash_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to Flash
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Flash: *const RegisterBlock = 0x40022000 as *const _;