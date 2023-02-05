#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4;

#[cortex_m_rt::entry]
fn main() -> ! {
    let _peris = stm32f4::Peripherals::take().unwrap();
    loop {
    }
}
