//! stm32ral module for stm32f7x9

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::cryp_f7x5_f7x6_f7x7_f7x9 as cryp;
pub use super::instances::dcmi;
pub use super::instances::dma_f7x5_f7x9 as dma;
pub use super::instances::fmc_f7x5_f7x6_f7x7_f7x9 as fmc;
pub use super::instances::gpio_f7x5_f7x6_f7x7_f7x9 as gpio;
pub use super::instances::hash;
pub use super::instances::rcc_f7x5_f7x9 as rcc;
pub use super::instances::rng;
pub use super::instances::syscfg_f7x5_f7x6_f7x7_f7x9 as syscfg;
pub mod spi;
pub use super::instances::adc_f7x5_f7x6_f7x9 as adc;
pub use super::instances::dac;
pub use super::instances::iwdg_f7x5_f7x6_f7x7_f7x9 as iwdg;
pub use super::instances::pwr_f7x5_f7x6_f7x7_f7x9 as pwr;
pub use super::instances::wwdg;
pub mod tim1;
pub mod tim8;
pub use super::instances::ac;
pub use super::instances::adc_common_f7x7_f7x9 as adc_common;
pub use super::instances::can_f7x7_f7x9 as can;
pub use super::instances::cec;
pub use super::instances::crc_f7x5_f7x6_f7x7_f7x9 as crc;
pub use super::instances::dfsdm;
pub use super::instances::dma2d;
pub use super::instances::dsi;
pub use super::instances::ethernet_dma;
pub use super::instances::ethernet_mac;
pub use super::instances::ethernet_mmc;
pub use super::instances::ethernet_ptp;
pub use super::instances::exti_f7x5_f7x6_f7x7_f7x9 as exti;
pub use super::instances::flash_f7x7_f7x9 as flash;
pub use super::instances::fpu;
pub use super::instances::fpu_cpacr;
pub use super::instances::i2c_f7x5_f7x6_f7x7_f7x9 as i2c;
pub use super::instances::jpeg;
pub use super::instances::lptim1;
pub use super::instances::ltdc;
pub use super::instances::mdios;
pub use super::instances::mpu;
pub use super::instances::nvic;
pub use super::instances::nvic_stir;
pub use super::instances::otg_fs_device_f7x5_f7x6_f7x7_f7x9 as otg_fs_device;
pub use super::instances::otg_fs_global_f7x5_f7x6_f7x7_f7x9 as otg_fs_global;
pub use super::instances::otg_fs_host_f7x5_f7x6_f7x7_f7x9 as otg_fs_host;
pub use super::instances::otg_fs_pwrclk;
pub use super::instances::otg_hs_device_f7x5_f7x6_f7x7_f7x9 as otg_hs_device;
pub use super::instances::otg_hs_global_f7x5_f7x6_f7x7_f7x9 as otg_hs_global;
pub use super::instances::otg_hs_host_f7x5_f7x6_f7x7_f7x9 as otg_hs_host;
pub use super::instances::otg_hs_pwrclk;
pub use super::instances::pf;
pub use super::instances::quadspi;
pub use super::instances::rtc_f7x5_f7x6_f7x7_f7x9 as rtc;
pub use super::instances::sai_f7x5_f7x6_f7x7_f7x9 as sai;
pub use super::instances::scb;
pub use super::instances::scb_actrl;
pub use super::instances::sdmmc_f7x7_f7x9 as sdmmc;
pub use super::instances::spdifrx;
pub use super::instances::stk;
pub use super::instances::tim10_f7x5_f7x6_f7x7_f7x9 as tim10;
pub use super::instances::tim11_f7x5_f7x6_f7x7_f7x9 as tim11;
pub use super::instances::tim12_f7x5_f7x6_f7x7_f7x9 as tim12;
pub use super::instances::tim13_f7x5_f7x6_f7x7_f7x9 as tim13;
pub use super::instances::tim14_f7x5_f7x6_f7x7_f7x9 as tim14;
pub use super::instances::tim2_f7x7_f7x9 as tim2;
pub use super::instances::tim3_f7x7_f7x9 as tim3;
pub use super::instances::tim4_f7x7_f7x9 as tim4;
pub use super::instances::tim5_f7x7_f7x9 as tim5;
pub use super::instances::tim6_f7x5_f7x6_f7x7_f7x9 as tim6;
pub use super::instances::tim7_f7x5_f7x6_f7x7_f7x9 as tim7;
pub use super::instances::tim9_f7x5_f7x6_f7x7_f7x9 as tim9;
pub use super::instances::usart_f7x5_f7x6_f7x7_f7x9 as usart;
