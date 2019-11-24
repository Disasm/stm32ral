#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod wwdg;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod iwdg;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod i2c;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod flash;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod dbgmcu;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod rcc;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod pwr;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod rng;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g483", feature="stm32g484"))]
pub mod aes;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod gpio;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod tim15;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g431", feature="stm32g441", feature="stm32g441", feature="stm32g471", feature="stm32g471", feature="stm32g473", feature="stm32g473", feature="stm32g474", feature="stm32g474", feature="stm32g483", feature="stm32g483", feature="stm32g484", feature="stm32g484"))]
pub mod tim16;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g431", feature="stm32g441", feature="stm32g441", feature="stm32g471", feature="stm32g471", feature="stm32g473", feature="stm32g473", feature="stm32g473", feature="stm32g474", feature="stm32g474", feature="stm32g474", feature="stm32g483", feature="stm32g483", feature="stm32g483", feature="stm32g484", feature="stm32g484", feature="stm32g484"))]
pub mod tim1;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g431", feature="stm32g431", feature="stm32g441", feature="stm32g441", feature="stm32g441", feature="stm32g471", feature="stm32g471", feature="stm32g471", feature="stm32g471", feature="stm32g473", feature="stm32g473", feature="stm32g473", feature="stm32g473", feature="stm32g474", feature="stm32g474", feature="stm32g474", feature="stm32g474", feature="stm32g483", feature="stm32g483", feature="stm32g483", feature="stm32g483", feature="stm32g484", feature="stm32g484", feature="stm32g484", feature="stm32g484"))]
pub mod tim2;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g431", feature="stm32g441", feature="stm32g441", feature="stm32g471", feature="stm32g471", feature="stm32g473", feature="stm32g473", feature="stm32g474", feature="stm32g474", feature="stm32g483", feature="stm32g483", feature="stm32g484", feature="stm32g484"))]
pub mod tim6;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod lptimer1;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod usart;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod lpuart1;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod spi;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod exti;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod rtc;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod dma;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod dmamux;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod syscfg;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod vrefbuf;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471"))]
pub mod comp_v1;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471"))]
pub mod opamp_v1;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod dac;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod adc;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod adc12_common;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod fmac;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod cordic;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod sai;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod tamp;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod fpu;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod mpu;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod stk;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod scb;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod nvic;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod fpu_cpacr;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod scb_actrl;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod fdcan;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod ucpd1;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod usb_fs_device;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod crs;

#[cfg(any(feature="doc", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod fmc;

#[cfg(any(feature="doc", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod comp_v2;

#[cfg(any(feature="doc", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod opamp_v2;

#[cfg(any(feature="doc", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod quadspi;

#[cfg(any(feature="doc", feature="stm32g474", feature="stm32g484"))]
pub mod hrtim_master;

#[cfg(any(feature="doc", feature="stm32g474", feature="stm32g484"))]
pub mod hrtim_tima;

#[cfg(any(feature="doc", feature="stm32g474", feature="stm32g484"))]
pub mod hrtim_timb;

#[cfg(any(feature="doc", feature="stm32g474", feature="stm32g484"))]
pub mod hrtim_timc;

#[cfg(any(feature="doc", feature="stm32g474", feature="stm32g484"))]
pub mod hrtim_timd;

#[cfg(any(feature="doc", feature="stm32g474", feature="stm32g484"))]
pub mod hrtim_time;

#[cfg(any(feature="doc", feature="stm32g474", feature="stm32g484"))]
pub mod hrtim_timf;

#[cfg(any(feature="doc", feature="stm32g474", feature="stm32g484"))]
pub mod hrtim_common;

