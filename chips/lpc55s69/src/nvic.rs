// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! Named constants for NVIC ids

#![allow(non_upper_case_globals)]

pub const WDT_BOD: u32 = 0;
pub const DMA0: u32 = 1;
pub const GINT0: u32 = 2;
pub const GINT1: u32 = 3;
pub const PIN_INT0: u32 = 4;
pub const PIN_INT1: u32 = 5;
pub const PIN_INT2: u32 = 6;
pub const PIN_INT3: u32 = 7;
pub const UTICK0: u32 = 8;
pub const MRT0: u32 = 9;
pub const CTIMER0: u32 = 10;
pub const CTIMER1: u32 = 11;
pub const SCT0: u32 = 12;
pub const CTIMER3: u32 = 13;
pub const FLEXCOMM0: u32 = 14;
pub const FLEXCOMM1: u32 = 15;
pub const FLEXCOMM2: u32 = 16;
pub const FLEXCOMM3: u32 = 17;
pub const FLEXCOMM4: u32 = 18;
pub const FLEXCOMM5: u32 = 19;
pub const FLEXCOMM6: u32 = 20;
pub const FLEXCOMM7: u32 = 21;
pub const ADC0: u32 = 22;

pub const ACMP: u32 = 24;

pub const USB0_NEEDCLK: u32 = 27;
pub const USB0: u32 = 28;
pub const RTC: u32 = 29;

pub const MAILBOX: u32 = 31;
pub const PIN_INT4: u32 = 32;
pub const PIN_INT5: u32 = 33;
pub const PIN_INT6: u32 = 34;
pub const PIN_INT7: u32 = 35;
pub const CTIMER2: u32 = 36;
pub const CTRTIMER4: u32 = 37;
pub const OS_EVENT: u32 = 38;

pub const SDIO: u32 = 42;

pub const USB1_PHY: u32 = 46;
pub const USB1: u32 = 47;
pub const USB1_NEEDCLK: u32 = 48;
pub const SEC_HYPERVISOR_CALL: u32 = 49;
pub const SEC_GPIO_INT0_IRQ0: u32 = 50;
pub const SEC_GPIO_INT0_IRQ1: u32 = 51;
pub const PLU: u32 = 52;
pub const SEC_VIO: u32 = 53;
pub const HASHCRYPT: u32 = 54;
pub const CASER: u32 = 55;
pub const PUF: u32 = 56;
pub const PQ: u32 = 57;
pub const DMA1: u32 = 58;
pub const FLEXCOMM8: u32 = 59;
