#![no_main]
#![no_std]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac as pac;
use panic_halt as _;

fn wait() {
    for _ in 0..4_000_000 {
        nop();
    }
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = peripherals.P0;

    p0.pin_cnf[21].write(|w| {
        w.dir().output()
    });
    p0.pin_cnf[28].write(|w| {
        w.dir().output()
    });
    p0.outclr.write(|w| {
        w.pin28().clear()
    });

    loop {
        p0.outset.write(|w| {
            w.pin21().set()
        });
        wait();
        p0.outclr.write(|w| {
            w.pin21().clear()
        });
        wait();
    }
}
