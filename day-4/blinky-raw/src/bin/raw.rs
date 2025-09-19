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

/// Memory offet for output set register.
const GPIO_OUTSET_OFFSET: usize = 0x508;
/// Memory offet for output clear register.
const GPIO_OUTCLR_OFFSET: usize = 0x50c;

/// ROW1: P0.21
const ROW1_PORT: usize = GPIO_P0;
const ROW1_PIN: usize = 21;

/// COL1: P0.28
const COL1_PORT: usize = GPIO_P0;
const COL1_PIN: usize = 28;

/// Spin delay for about ½ second.
fn delay() {
    for _ in 0..4_000_000 {
        asm::nop();
    }
}

#[entry]
fn main() -> ! {
    // Pointers to configuration registers.
    let row1_cfg = (ROW1_PORT + GPIO_PIN_CNF_BASE + ROW1_PIN * 4) as *mut u32;
    let col1_cfg = (COL1_PORT + GPIO_PIN_CNF_BASE + COL1_PIN * 4) as *mut u32;

    // Pointers to bit set/clear registers.
    let row1_outset = (ROW1_PORT + GPIO_OUTSET_OFFSET) as *mut u32;
    let row1_outclr = (ROW1_PORT + GPIO_OUTCLR_OFFSET) as *mut u32;
    let col1_outclr = (COL1_PORT + GPIO_OUTCLR_OFFSET) as *mut u32;

    unsafe {
        // Set both pins to output mode.
        *row1_cfg = 1;
        *col1_cfg = 1;

        // Set column pin to low (ground) and leave it that way.
        *col1_outclr = 1 << COL1_PIN;

        loop {
            // Set row pin to high (3.3V).
            *row1_outset = 1 << ROW1_PIN;
            delay();
            // Set row pin to low (ground).
            *row1_outclr = 1 << ROW1_PIN;
            delay();
        }
    }
}
