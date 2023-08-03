use psoc6_02::* ;

pub fn get_hw(port: u8) -> &'static gpio::PRT {
    let gpio = unsafe { &(*GPIO::ptr()) } ;

    let prt = match port {
        0 => &gpio.prt0,
        1 => &gpio.prt1,
        2 => &gpio.prt2,
        3 => &gpio.prt3,
        4 => &gpio.prt4,
        5 => &gpio.prt5,
        6 => &gpio.prt6,
        7 => &gpio.prt7,
        8 => &gpio.prt8,
        9 => &gpio.prt9,
        10 => &gpio.prt10,
        11 => &gpio.prt11,
        12 => &gpio.prt12,
        13 => &gpio.prt13,
        14 => &gpio.prt14,          
        _ => unreachable!(),
    } ;
    
    return prt ;
}