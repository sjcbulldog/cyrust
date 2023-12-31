#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

pub mod cm4 ;

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    cm4::enable_cm4(0x10004000) ;
    loop {
        asm::wfi();
    }
}
