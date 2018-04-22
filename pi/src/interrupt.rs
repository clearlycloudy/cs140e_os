use common::IO_BASE;
use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

const INT_BASE: usize = IO_BASE + 0xB000 + 0x200;

#[derive(Copy, Clone, PartialEq)]
pub enum Interrupt {
    Timer1 = 1,
    Timer3 = 3,
    Usb = 9,
    Gpio0 = 49,
    Gpio1 = 50,
    Gpio2 = 51,
    Gpio3 = 52,
    Uart = 57,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    pub IRQ_Basic_Pending: u32,
    pub IRQ_Pending_1: u32,
    pub IRQ_Pending_2: u32,
    pub FIQ_Control: u32,
    pub Enable_IRQ_1: u32,
    pub Enable_IRQ_2: u32,
    pub Enable_Basic_IRQs: u32,
    pub Disable_IRQ_1: u32,
    pub Disable_IRQ_2: u32,
    pub Disable_Basic_IRQs: u32,
}

/// An interrupt controller. Used to enable and disable interrupts as well as to
/// check if an interrupt is pending.
/// see BCM2837 ARM peripherals manual section on regular IRQs
pub struct Controller {
    registers: &'static mut Registers
}

impl Controller {
    /// Returns a new handle to the interrupt controller.
    pub fn new() -> Controller {
        Controller {
            registers: unsafe { &mut *(INT_BASE as *mut Registers) },
        }
    }

    /// Enables the interrupt `int`.
    pub fn enable(&mut self, int: Interrupt) {
        if int as u32 >= 32 {
            let adjust = int as u32 - 32;
            self.registers.Enable_IRQ_2 |= 1 << adjust;
        } else {
            self.registers.Enable_IRQ_1 |= 1 << int as u32;
        }
    }

    /// Disables the interrupt `int`.
    pub fn disable(&mut self, int: Interrupt) {
        if int as u32 >= 32 {
            let adjust = int as u32 - 32;
            self.registers.Disable_IRQ_2 |= 1 << adjust;
        } else {
            self.registers.Disable_IRQ_1 |= 1 << int as u32;
        }        
    }

    /// Returns `true` if `int` is pending. Otherwise, returns `false`.
    pub fn is_pending(&self, int: Interrupt) -> bool {
        if int as u32 >= 32 {
            let adjust = int as u32 - 32;
            self.registers.IRQ_Pending_2 & ( 1 << adjust ) != 0
        } else {
            self.registers.IRQ_Pending_1 & ( 1 << int as u32 ) != 0
        }
    }
}
