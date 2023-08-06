#![no_std]
#![no_main]
#[allow(unused_imports)]

use panic_halt as _;
use cortex_m::asm;
use cortex_m_rt::* ;
use psoc6dev::psoc602::CPUSS::Cpuss ;

#[entry]
fn main() -> ! {
    unsafe {
        let p = 0x0900 as *mut ();
        let _c = Cpuss::from_ptr(p);

        loop {
            asm::nop() ;
        }
    }
}
