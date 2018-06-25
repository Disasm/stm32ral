// Copyright 2018 Adam Greig
// See LICENSE-APACHE and LICENSE-MIT for license details.

//! This project provides a register abstraction layer (RAL) for all STM32 microcontrollers.

#![no_std]
#![warn(missing_docs)]

#[macro_use]
mod register;

pub use register::{RORegister, RWRegister};
