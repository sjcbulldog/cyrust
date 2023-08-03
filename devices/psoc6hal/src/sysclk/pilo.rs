use super::super::srss::hw::get_srss ;
use super::super::HalStatus ;

pub fn enable() -> HalStatus {
    let srss = get_srss() ;
    srss.clk_pilo_config.modify(|_,w| { w.pilo_en().set_bit()}) ;    
    srss.clk_pilo_config.modify(|_,w| { w.pilo_reset_n().set_bit().pilo_clk_en().set_bit()}) ;
    return  Ok(0) ;
}

pub fn enabled() -> bool {
    let srss = get_srss() ;
    return srss.clk_pilo_config.read().pilo_en().bits() ;
}

pub fn disable() -> HalStatus {
    let srss = get_srss() ;
    srss.clk_pilo_config.modify(|_,w| { 
        w.pilo_reset_n().clear_bit().pilo_clk_en().clear_bit().pilo_en().clear_bit()
    }) ;

    return Ok(0) ;
}

pub fn set_trim(trimval: u16) -> HalStatus {
    let srss = get_srss() ;
    srss.clk_pilo_config.modify(|_,w| unsafe { w.pilo_ffreq().bits(trimval)}) ;   

    return Ok(0) ;
}

pub fn get_trim() -> u16 {
    let srss = get_srss() ;
    return srss.clk_pilo_config.read().pilo_ffreq().bits() ;
}
