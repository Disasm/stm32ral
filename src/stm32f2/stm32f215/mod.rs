//! stm32ral module for stm32f215

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::crc;
pub use super::instances::dcmi;
pub use super::instances::fsmc;
pub use super::instances::rng;
pub mod dbg;
pub use super::instances::adc;
pub use super::instances::adc_common;
pub use super::instances::can;
pub use super::instances::cryp;
pub use super::instances::dac;
pub use super::instances::dma;
pub use super::instances::ethernet_dma;
pub use super::instances::ethernet_mac;
pub use super::instances::ethernet_mmc;
pub use super::instances::ethernet_ptp;
pub use super::instances::exti;
pub use super::instances::flash;
pub use super::instances::gpio;
pub use super::instances::hash;
pub use super::instances::i2c;
pub use super::instances::iwdg;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::otg_fs_device;
pub use super::instances::otg_fs_global;
pub use super::instances::otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub use super::instances::otg_hs_device;
pub use super::instances::otg_hs_global;
pub use super::instances::otg_hs_host;
pub use super::instances::otg_hs_pwrclk;
pub use super::instances::pwr;
pub use super::instances::rcc;
pub use super::instances::rtc;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::sdio;
pub use super::instances::spi;
pub use super::instances::stk;
pub use super::instances::syscfg;
pub use super::instances::tim1;
pub use super::instances::tim10;
pub use super::instances::tim11;
pub use super::instances::tim12;
pub use super::instances::tim13;
pub use super::instances::tim14;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub use super::instances::tim8;
pub use super::instances::tim9;
pub use super::instances::uart4;
pub use super::instances::uart5;
pub use super::instances::usart;
pub use super::instances::wwdg;