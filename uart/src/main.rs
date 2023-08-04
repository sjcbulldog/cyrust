#![no_std]
#![no_main]

#[entry]
fn main() -> ! {
    loop {
        asm::nop() ;
    }
}
