<<<<<<< HEAD
// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! Peripheral implementations for the LPC55S69 MCU.
//!
//! LPC55S69 Exception Table: <https://developer.arm.com/documentation/100235/0003/the-cortex-m33-processor/exception-model/exception-types>

#![no_std]

pub mod nvic;

// Peripherals

pub mod gpio;

use cortexm33::{initialize_ram_jump_to_main, unhandled_interrupt, CortexM33, CortexMVariant};

extern "C" {
    // _estack is not really a function, but it makes the types work
    // You should never actually invoke it!!
    fn _estack();
}

#[cfg_attr(
    all(target_arch = "arm", target_os = "none"),
    link_section = ".vectors"
)]
// used Ensures that the symbol is kept until the final binary
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static BASE_VECTORS: [unsafe extern "C" fn(); 16] = [
    _estack,
    initialize_ram_jump_to_main,
    unhandled_interrupt,           // NMI
    CortexM33::HARD_FAULT_HANDLER, // Hard Fault
    unhandled_interrupt,           // MemManage
    unhandled_interrupt,           // BusFault
    unhandled_interrupt,           // UsageFault
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    CortexM33::SVC_HANDLER, // SVC
    unhandled_interrupt,    // DebugMon
    unhandled_interrupt,
    unhandled_interrupt,        // PendSV
    CortexM33::SYSTICK_HANDLER, // SysTick
];

// LPC55S69 has total of 60 interrupts
// Extracted from `MCUXpresso /devices/lpc55s69_cm33_core0.h` file
#[cfg_attr(all(target_arch = "arm", target_os = "none"), link_section = ".irqs")]
// used Ensures that the symbol is kept until the final binary
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static IRQS: [unsafe extern "C" fn(); 60] = [
    CortexM33::GENERIC_ISR, // WDT_BOD (0)
    CortexM33::GENERIC_ISR, // DMA0 (1)
    CortexM33::GENERIC_ISR, // GINT0 (2)
    CortexM33::GENERIC_ISR, // GINT1 (3)
    CortexM33::GENERIC_ISR, // PIN_INT0 (4)
    CortexM33::GENERIC_ISR, // PIN_INT1 (5)
    CortexM33::GENERIC_ISR, // PIN_INT2 (6)
    CortexM33::GENERIC_ISR, // PIN_INT3 (7)
    CortexM33::GENERIC_ISR, // UTICK0 (8)
    CortexM33::GENERIC_ISR, // MRT0 (9)
    CortexM33::GENERIC_ISR, // CTIMER0 (10)
    CortexM33::GENERIC_ISR, // CTIMER1 (11)
    CortexM33::GENERIC_ISR, // SCT0 (12)
    CortexM33::GENERIC_ISR, // CTIMER3 (13)
    CortexM33::GENERIC_ISR, // FLEXCOMM0 (14)
    CortexM33::GENERIC_ISR, // FLEXCOMM1 (15)
    CortexM33::GENERIC_ISR, // FLEXCOMM2 (16)
    CortexM33::GENERIC_ISR, // FLEXCOMM3 (17)
    CortexM33::GENERIC_ISR, // FLEXCOMM4 (18)
    CortexM33::GENERIC_ISR, // FLEXCOMM5 (19)
    CortexM33::GENERIC_ISR, // FLEXCOMM6 (20)
    CortexM33::GENERIC_ISR, // FLEXCOMM7 (21)
    CortexM33::GENERIC_ISR, // ADC0 (22)
    unhandled_interrupt,    // (23)
    CortexM33::GENERIC_ISR, // ACMP (24)
    unhandled_interrupt,    // (25)
    unhandled_interrupt,    // (26)
    CortexM33::GENERIC_ISR, // USB0_NEEDCLK (27)
    CortexM33::GENERIC_ISR, // USB0 (28)
    CortexM33::GENERIC_ISR, // RTC (29)
    unhandled_interrupt,    // (30)
    CortexM33::GENERIC_ISR, // MAILBOX (31)
    CortexM33::GENERIC_ISR, // PIN_INT4 (32)
    CortexM33::GENERIC_ISR, // PIN_INT5 (33)
    CortexM33::GENERIC_ISR, // PIN_INT6 (34)
    CortexM33::GENERIC_ISR, // PIN_INT7 (35)
    CortexM33::GENERIC_ISR, // CTIMER2 (36)
    CortexM33::GENERIC_ISR, // CTIMER4 (37)
    CortexM33::GENERIC_ISR, // OS_EVENT (38)
    unhandled_interrupt,    // (39)
    unhandled_interrupt,    // (40)
    unhandled_interrupt,    // (41)
    CortexM33::GENERIC_ISR, // SDIO (42)
    unhandled_interrupt,    // (43)
    unhandled_interrupt,    // (44)
    unhandled_interrupt,    // (45)
    CortexM33::GENERIC_ISR, // USB1_PHY (46)
    CortexM33::GENERIC_ISR, // USB1 (47)
    CortexM33::GENERIC_ISR, // USB1_NEEDCLK (48)
    CortexM33::GENERIC_ISR, // SEC_HYPERVISOR_CALL (49)
    CortexM33::GENERIC_ISR, // SEC_GPIO_INT0_IRQ0 (50)
    CortexM33::GENERIC_ISR, // SEC_GPIO_INT0_IRQ1 (51)
    CortexM33::GENERIC_ISR, // PLU (52)
    CortexM33::GENERIC_ISR, // SEC_VIO (53)
    CortexM33::GENERIC_ISR, // HASHCRYPT (54)
    CortexM33::GENERIC_ISR, // CASER (55)
    CortexM33::GENERIC_ISR, // PUF (56)
    CortexM33::GENERIC_ISR, // PQ (57)
    CortexM33::GENERIC_ISR, // DMA1 (58)
    CortexM33::GENERIC_ISR, // FLEXCOMM8 (59)
];

pub unsafe fn init() {
    cortexm33::nvic::disable_all();
    cortexm33::nvic::clear_all_pending();
    cortexm33::nvic::enable_all();
}
=======

>>>>>>> 53694e6ec (Updated arch Cortex-M33 and added lpc55s69)
