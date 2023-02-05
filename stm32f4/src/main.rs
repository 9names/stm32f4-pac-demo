#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::Peripherals;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    p
}
