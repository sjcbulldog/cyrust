use super::super::srss::hw::get_srss ;
use super::super::HalStatus ;

pub fn enable() -> HalStatus {
    let srss = get_srss() ;

    srss.clk_ilo_config.modify(|_, w| { w.enable().set_bit()}) ;
    return Ok(0) ;
}

pub fn enabled() -> bool {
    let srss = get_srss() ;
    return srss.clk_ilo_config.read().enable().bits() ;
}

pub fn disable() -> HalStatus {
    if super::super::wdt::is_enabled()
    {
        return Err(super::ERROR_WDT_ENABLED) ;
    }

    let srss = get_srss() ;
    srss.clk_ilo_config.modify(|_, w| { w.enable().clear_bit()}) ;
    return Ok(0) ;
}

pub fn hibernate_on(on: bool) -> HalStatus {
    let srss = get_srss() ;
    srss.clk_ilo_config.modify(|_, w| { w.ilo_backup().bit(on)}) ;
    return Ok(0) ;
}

