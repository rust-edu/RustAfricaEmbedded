#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit as _;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("hello world");
    loop {
        asm::nop();
    }
}
