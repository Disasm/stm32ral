//! stm32ral module for stm32f469

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::cryp;
pub use super::instances::dcmi;
pub use super::instances::hash;
pub use super::instances::rng;
pub mod fmc;
pub use super::instances::dbg_f427_f429_f469 as dbg;
pub use super::instances::dma;
pub mod rcc;
pub use super::instances::adc;
pub use super::instances::adc_common_f405_f407_f427_f429_f446_f469 as adc_common;
pub use super::instances::dac;
pub use super::instances::gpio_f429_f469 as gpio;
pub use super::instances::iwdg;
pub use super::instances::pwr_f429_f469 as pwr;
pub use super::instances::rtc_f405_f407_f427_f429_f446_f469 as rtc;
pub use super::instances::sdio;
pub use super::instances::spi_f405_f407_f427_f429_f469 as spi;
pub use super::instances::syscfg_f429_f446_f469 as syscfg;
pub use super::instances::tim1;
pub use super::instances::tim2;
pub use super::instances::tim3;
pub use super::instances::tim4;
pub use super::instances::tim5;
pub use super::instances::tim8;
pub use super::instances::uart_f427_f429_f446_f469 as uart;
pub use super::instances::usart_f427_f429_f469 as usart;
pub use super::instances::wwdg;
pub mod tim12;
pub mod tim9;
pub use super::instances::crc;
pub use super::instances::ethernet_dma;
pub use super::instances::ethernet_mac;
pub use super::instances::ethernet_mmc;
pub use super::instances::ethernet_ptp;
pub use super::instances::tim10_f401_f411_f412_f413_f427_f429_f446_f469 as tim10;
pub use super::instances::tim11_f401_f410_f411_f412_f413_f427_f429_f446_f469 as tim11;
pub use super::instances::tim13_f427_f429_f446_f469 as tim13;
pub use super::instances::tim14_f427_f429_f446_f469 as tim14;
pub use super::instances::tim6;
pub use super::instances::tim7;
pub mod otg_fs_global;
pub use super::instances::can;
pub use super::instances::nvic_f427_f429_f446_f469 as nvic;
pub use super::instances::otg_fs_device;
pub use super::instances::otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub mod flash;
pub use super::instances::exti;
pub use super::instances::ltdc;
pub use super::instances::otg_hs_device;
pub use super::instances::otg_hs_global;
pub use super::instances::otg_hs_host;
pub use super::instances::otg_hs_pwrclk;
pub mod sai;
pub use super::instances::dma2d;
pub use super::instances::i2c_f427_f429_f446_f469 as i2c;
pub mod dsihost;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::mpu;
pub use super::instances::nvic_stir;
pub use super::instances::quadspi;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::stk;
