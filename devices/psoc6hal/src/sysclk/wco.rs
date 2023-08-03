use super::super::backup::hw::get_backup ;
use super::super::HalStatus ;

pub fn enable(timeout: u32) -> HalStatus {
    let back = get_backup() ;
    let mut to = timeout ;

    back.ctl.modify(|_, w| { w.wco_en().set_bit()}) ;

    while !ok() && to != 0
    {
        to = to - 1 ;
        super::delay_us(1) ;
    }

    if to == 0 
    {
        return Err(super::TIMEOUT) ;
    }

    return Ok(0) ;
}

pub fn ok() -> bool {
    let back = get_backup() ;
    return back.status.read().wco_ok().bits() ;
}

pub fn disable() -> HalStatus {
    let back = get_backup() ;
    back.ctl.modify(|_, w| { w.wco_en().clear_bit()}) ;

    return Ok(0) ;
}

pub fn bypass(by: bool) -> HalStatus {
    let back = get_backup() ;
    back.ctl.modify(|_, w| { w.wco_bypass().bit(by)}) ;

    return Ok(0) ;
}
