use psoc6_02::* ;

pub fn get_hsiom(port: u8) -> &'static hsiom::PRT {
    let hsiom = unsafe { &(*HSIOM::ptr())} ;

    let hsiom = match port {
        0 => &hsiom.prt0,
        1 => &hsiom.prt1,
        2 => &hsiom.prt2,
        3 => &hsiom.prt3,
        4 => &hsiom.prt4,
        5 => &hsiom.prt5,
        6 => &hsiom.prt6,
        7 => &hsiom.prt7,
        8 => &hsiom.prt8,
        9 => &hsiom.prt9,
        10 => &hsiom.prt10,
        11 => &hsiom.prt11,
        12 => &hsiom.prt12,
        13 => &hsiom.prt13,
        14 => &hsiom.prt14,
        _ => unreachable!(),
    } ;

    return hsiom ;
}
