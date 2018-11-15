#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal serial bus full-speed device interface
//!
//! Used by: stm32f302, stm32f303

#[cfg(not(feature = "nosync"))]
pub use stm32f3::peripherals::usb::Instance;
pub use stm32f3::peripherals::usb::{RegisterBlock, ResetValues};
pub use stm32f3::peripherals::usb::{
    BTABLE, DADDR, FNR, ISTR, USB_CNTR, USB_EP0R, USB_EP1R, USB_EP2R, USB_EP3R, USB_EP4R, USB_EP5R,
    USB_EP6R, USB_EP7R,
};

/// Access functions for the USB_FS peripheral instance
pub mod USB_FS {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40005c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USB_FS
    pub const reset: ResetValues = ResetValues {
        USB_EP0R: 0x00000000,
        USB_EP1R: 0x00000000,
        USB_EP2R: 0x00000000,
        USB_EP3R: 0x00000000,
        USB_EP4R: 0x00000000,
        USB_EP5R: 0x00000000,
        USB_EP6R: 0x00000000,
        USB_EP7R: 0x00000000,
        USB_CNTR: 0x00000003,
        ISTR: 0x00000000,
        FNR: 0x00000000,
        DADDR: 0x00000000,
        BTABLE: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
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
pub const USB_FS: *const RegisterBlock = 0x40005c00 as *const _;
