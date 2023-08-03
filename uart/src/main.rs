#![no_std]
#![no_main]

//
// This is here to force the INTERRUPT table from the psoc6_02 create
#[allow(unused_imports)]
use psoc6_02::interrupt ;
use cy8c624abzi_d44::hsiom::* ;
use psoc6hal::gpio::* ;
use psoc6hal::* ;
use psoc6hal::uart::*;

use panic_halt as _;
use cortex_m::asm;
use cortex_m_rt::* ;

#[interrupt]
fn IOSS_INTERRUPTS_GPIO_0() {
    asm::nop() ;
}

#[entry]
fn main() -> ! {
    //
    // Initialization parameters
    //
    let oversample:u8 = 12 ;                                    // 12 x oversample
    let baud = 115200 ;                                         // 115200 baud rate

    let freq = sysclk::get_peri_frequency() ;                   // The system frequency
    let div = freq / ((oversample as u32) * baud) ;             // The divider needed given the oversample and baud rate

    //
    // Configure the TX pin
    //
    let mut tx = Gpio { port: 5, pin: 1} ;
    tx.set_drive_mode(DriveMode::StrongInOff) ;
    tx.set_hsioim(P5_1_SCB5_UART_TX) ;

    //
    // Configure the RX pin
    //
    let mut rx = Gpio { port: 5, pin: 0} ;
    rx.set_drive_mode(DriveMode::HighZ) ;
    rx.set_hsioim(P5_0_SCB5_UART_RX) ;

    //
    // Configure the clocks to teh SCB block, using eight bit divider 0
    //
    peri::periph_assign_divider(sysclk::ClockType::SCB5_CLOCK, sysclk::DividerType::EightBitDivider, 0) ;
    peri::set_divider(sysclk::DividerType::EightBitDivider, 0, div) ;
    peri::enable_divider(sysclk::DividerType::EightBitDivider, 0) ;

    //
    // Configure the UART
    //
    let mut uart = Uart { port: 5 } ;
    uart.init(8, 1, oversample) ;
    uart.enable() ;
    uart.put_string("\r\n\r\n\r\nHello World From Rust - ha ha ha\r\n") ;

    loop {
        asm::nop() ;
    }
}
