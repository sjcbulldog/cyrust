use psoc6_02::* ;

pub fn get_hw(port: u8) -> &'static scb0::RegisterBlock {
    let ret = match port {
        0 => unsafe { &(*SCB0::ptr()) },
        1 => unsafe { &(*SCB1::ptr()) },
        2 => unsafe { &(*SCB2::ptr()) },
        3 => unsafe { &(*SCB3::ptr()) },
        4 => unsafe { &(*SCB4::ptr()) },
        5 => unsafe { &(*SCB5::ptr()) },
        6 => unsafe { &(*SCB6::ptr()) },
        7 => unsafe { &(*SCB7::ptr()) },
        8 => unsafe { &(*SCB8::ptr()) },
        9 => unsafe { &(*SCB9::ptr()) },
        10 => unsafe { &(*SCB10::ptr()) },
        11 => unsafe { &(*SCB11::ptr()) },
        12 => unsafe { &(*SCB11::ptr()) },
        _ => unreachable!(),
    } ;

    return ret ;
}
