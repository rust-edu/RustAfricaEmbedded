#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

/// Memory location of GPIO registers.
const GPIO_BASE: usize = 0x50000000;

/// Memory offset for P0 registers.
const GPIO_P0: usize = GPIO_BASE + 0;

// Memory offset for P1 registers.
// const GPIO_P1: usize = GPIO_BASE + 0x300;

/// Memory offet for block of pin config registers.
const GPIO_PIN_CNF_BASE: usize = 0x700;

/// ROW1: P0.21
const ROW1_PORT: usize = GPIO_P0;
const ROW1_PIN: usize = 21;

/// COL1: P0.28
const COL1_PORT: usize = GPIO_P0;
const COL1_PIN: usize = 28;

#[entry]
fn main() -> ! {
    // Pointers to configuration registers.
    let row1_cfg = (ROW1_PORT + GPIO_PIN_CNF_BASE + ROW1_PIN * 4) as *mut u32;
    let col1_cfg = (COL1_PORT + GPIO_PIN_CNF_BASE + COL1_PIN * 4) as *mut u32;

    unsafe {
        // Set both pins to input mode.
        *row1_cfg = 0b10;
        *col1_cfg = 0b10;

        loop {
            asm::nop();
        }
    }
}
