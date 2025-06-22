// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! General Purpose Input/Output driver.

<<<<<<< HEAD
=======
#![no_main]
#![no_std]
>>>>>>> 53694e6ec (Updated arch Cortex-M33 and added lpc55s69)
#![allow(unused_imports)]
#![allow(dead_code)]

use core::cell::Cell;
use kernel::hil;
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::{
<<<<<<< HEAD
    self, register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly
};
use kernel::utilities::StaticRef;
use enum_primitive::enum_from_primitive;
=======
    register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly,
};
use kernel::utilities::StaticRef;
use panic_halt as _;
>>>>>>> 53694e6ec (Updated arch Cortex-M33 and added lpc55s69)

const GPIO_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x4008C000 as *const GpioRegisters) };

<<<<<<< HEAD
const GPIO_PER_PORT: usize = 32;

=======
>>>>>>> 53694e6ec (Updated arch Cortex-M33 and added lpc55s69)
register_structs! {
    /// General Purpose I/O (GPIO)
    GpioRegisters {
        /// Byte pin registers for all port GPIO pins
        (0x000 => b_0_b_s: ReadWrite<u8>),
        (0x001 => _reserved0),
        /// Byte pin registers for all port GPIO pins
        (0x020 => b_1_b_s: ReadWrite<u8>),
        (0x021 => _reserved1),
        /// Word pin registers for all port GPIO pins
        (0x1000 => w_0_w_s: ReadWrite<u32>),
        (0x1004 => _reserved2),
        /// Word pin registers for all port GPIO pins
        (0x1080 => w_1_w_s: ReadWrite<u32>),
        (0x1084 => _reserved3),
        /// Direction registers for all port GPIO pins
        (0x2000 => dir_0: ReadWrite<u32, Direction::Register>),
        /// Direction registers for all port GPIO pins
        (0x2004 => dir_1: ReadWrite<u32, Direction::Register>),
        (0x2008 => _reserved4),
        /// Mask register for all port GPIO pins
        (0x2080 => mask_0: ReadWrite<u32, Control::Register>),
        /// Mask register for all port GPIO pins
        (0x2084 => mask_1: ReadWrite<u32, Control::Register>),
        (0x2088 => _reserved5),
        /// Port pin register for all port GPIO pins
        (0x2100 => pin_0: ReadWrite<u32, Read::Register>),
        /// Port pin register for all port GPIO pins
        (0x2104 => pin_1: ReadWrite<u32, Read::Register>),
        (0x2108 => _reserved6),
        /// Masked port register for all port GPIO pins
        (0x2180 => mpin_0: ReadWrite<u32, Mask::Register>),
        /// Masked port register for all port GPIO pins
        (0x2184 => mpin_1: ReadWrite<u32, Mask::Register>),
        (0x2188 => _reserved7),
        /// Write: Set register for port. Read: output bits for port
        (0x2200 => set_0: ReadWrite<u32, Set::Register>),
        /// Write: Set register for port. Read: output bits for port
        (0x2204 => set_1: ReadWrite<u32, Set::Register>),
        (0x2208 => _reserved8),
        /// Clear port for all port GPIO pins
        (0x2280 => clr_0: WriteOnly<u32, Clear::Register>),
        /// Clear port for all port GPIO pins
        (0x2284 => clr_1: WriteOnly<u32, Clear::Register>),
        (0x2288 => _reserved9),
        /// Toggle port for all port GPIO pins
        (0x2300 => not_0: WriteOnly<u32, Toggle::Register>),
        /// Toggle port for all port GPIO pins
        (0x2304 => not_1: WriteOnly<u32, Toggle::Register>),
        (0x2308 => _reserved10),
        /// Set pin direction bits for port
        (0x2380 => dirset_0: WriteOnly<u32, Dirset::Register>),
        /// Set pin direction bits for port
        (0x2384 => dirset_1: WriteOnly<u32, Dirset::Register>),
        (0x2388 => _reserved11),
        /// Clear pin direction bits for port
        (0x2400 => dirclr_0: WriteOnly<u32, Dirclr::Register>),
        /// Clear pin direction bits for port
        (0x2404 => dirclr_1: WriteOnly<u32, Dirclr::Register>),
        (0x2408 => _reserved12),
        /// Toggle pin direction bits for port
        (0x2480 => dirnot_0: WriteOnly<u32, Dirnot::Register>),
        /// Toggle pin direction bits for port
        (0x2484 => dirnot_1: WriteOnly<u32, Dirnot::Register>),
        (0x2488 => @END),
    }
}

register_bitfields![u32,
    Direction [
        /// Pin direction control
        DIR OFFSET(0) NUMBITS(32)
    ],
    Control [
        /// Mask control for active bits
        MASK OFFSET(0) NUMBITS(32)
    ],
    Read [
        /// Read pin states
        PORT OFFSET(0) NUMBITS(32)
    ],
    Mask [
        /// Masked port control
        MPORT OFFSET(0) NUMBITS(32)
    ],
    Set [
        /// Set output bits
        SET OFFSET(0) NUMBITS(32)
    ],
    Clear [
        /// Clear output bits
        CLR OFFSET(0) NUMBITS(32)
    ],
    Toggle [
        /// Toggle output bits
        NOT OFFSET(0) NUMBITS(32)
    ],
    Dirset [
        /// Set direction bits
        DIRSET OFFSET(0) NUMBITS(32)
    ],
    Dirclr [
        /// Clear direction bits
        DIRCLR OFFSET(0) NUMBITS(32)
    ],
    Dirnot [
        /// Toggle direction bits
        DIRNOT OFFSET(0) NUMBITS(32)
    ]
];

<<<<<<< HEAD
enum_from_primitive! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[rustfmt::skip]
    pub enum Pin {
        P0_00, P0_01, P0_02, P0_03, P0_04, P0_05, P0_06, P0_07, P0_08, P0_09, P0_10, P0_11, P0_12, P0_13, P0_14, P0_15,
        

    }
}

pub struct GPIOPin<'a> {
    pin: u8,
    port: u8,
    gpio_registers: StaticRef<GpioRegisters>,
    client: OptionalCell<&'a dyn hil::gpio::Client>,
}

impl<'a> GPIOPin<'a> {
    pub fn new(pin: Pin) -> GPIOPin<'a> {
        GPIOPin {
        pin : ((pin as usize) % GPIO_PER_PORT) as u8,
        port : ((pin as usize) / GPIO_PER_PORT) as u8,
        gpio_registers : GPIO_BASE,
        client : OptionalCell::empty(),
        }
    }
}
=======
pub struct IntPin<'a> {
    pin: u8,
    port: u8,
    registers: StaticRef<GpioRegisters>,
    client: OptionalCell<&'a dyn hil::gpio::Client>,
}
>>>>>>> 53694e6ec (Updated arch Cortex-M33 and added lpc55s69)
