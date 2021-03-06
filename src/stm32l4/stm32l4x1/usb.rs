#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal serial bus full-speed device interface

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l4::peripherals::usb_v1::Instance;
pub use crate::stm32l4::peripherals::usb_v1::{RegisterBlock, ResetValues};
pub use crate::stm32l4::peripherals::usb_v1::{
    BCDR, BTABLE, CNTR, DADDR, EP0R, EP1R, EP2R, EP3R, EP4R, EP5R, EP6R, EP7R, FNR, ISTR, LPMCSR,
};

/// Access functions for the USB_FS peripheral instance
pub mod USB_FS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40006800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USB_FS
    pub const reset: ResetValues = ResetValues {
        EP0R: 0x00000000,
        EP1R: 0x00000000,
        EP2R: 0x00000000,
        EP3R: 0x00000000,
        EP4R: 0x00000000,
        EP5R: 0x00000000,
        EP6R: 0x00000000,
        EP7R: 0x00000000,
        CNTR: 0x00000003,
        ISTR: 0x00000000,
        FNR: 0x00000000,
        DADDR: 0x00000000,
        BTABLE: 0x00000000,
        LPMCSR: 0x00000000,
        BCDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USB_FS_TAKEN: bool = false;

    /// Safe access to USB_FS
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
            if USB_FS_TAKEN {
                None
            } else {
                USB_FS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USB_FS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USB_FS_TAKEN && inst.addr == INSTANCE.addr {
                USB_FS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USB_FS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USB_FS_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USB_FS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USB_FS: *const RegisterBlock = 0x40006800 as *const _;

/// Access functions for the USB_SRAM peripheral instance
pub mod USB_SRAM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40006c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USB_SRAM
    pub const reset: ResetValues = ResetValues {
        EP0R: 0x00000000,
        EP1R: 0x00000000,
        EP2R: 0x00000000,
        EP3R: 0x00000000,
        EP4R: 0x00000000,
        EP5R: 0x00000000,
        EP6R: 0x00000000,
        EP7R: 0x00000000,
        CNTR: 0x00000003,
        ISTR: 0x00000000,
        FNR: 0x00000000,
        DADDR: 0x00000000,
        BTABLE: 0x00000000,
        LPMCSR: 0x00000000,
        BCDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USB_SRAM_TAKEN: bool = false;

    /// Safe access to USB_SRAM
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
            if USB_SRAM_TAKEN {
                None
            } else {
                USB_SRAM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USB_SRAM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USB_SRAM_TAKEN && inst.addr == INSTANCE.addr {
                USB_SRAM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USB_SRAM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USB_SRAM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USB_SRAM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USB_SRAM: *const RegisterBlock = 0x40006c00 as *const _;
