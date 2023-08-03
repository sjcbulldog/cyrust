use super::srss::hw::* ;

pub const DEFAULT_MATCH_VALUE:u16 = 4096 ;
pub const DEFAULT_IGNORE_BITS:u8 = 0 ;

pub fn init() {
    disable() ;
    set_match(DEFAULT_MATCH_VALUE) ;
    set_ignore_bits(DEFAULT_IGNORE_BITS) ;
    clear_interrupt() ;
}

pub fn disable() {
    let srss = get_srss() ;
    srss.wdt_ctl.modify(|_, w| { w.wdt_en().clear_bit()}) ;
}

pub fn enable() {
    let srss = get_srss() ;
    srss.wdt_ctl.modify(|_, w| { w.wdt_en().set_bit()}) ;
    clear_interrupt() ;
}

pub fn is_enabled() -> bool {
    let srss = get_srss() ;    
    return srss.wdt_ctl.read().wdt_en().bits() ;
}

pub fn get_match() -> u16 {
    let srss = get_srss()  ;
    return srss.wdt_match.read().match_().bits() ;
}

pub fn get_count() -> u16 {
    let srss = get_srss()  ;

    return srss.wdt_cnt.read().counter().bits() ;
}

pub fn get_ignore_bits() -> u8 {
    let srss = get_srss() ;
    return srss.wdt_match.read().ignore_bits().bits() ;
}

pub fn lock() {
    let srss = get_srss() ;
    srss.wdt_ctl.modify(|_, w| { w.wdt_lock().bits(3)}) ;
}

pub fn locked() -> bool {
    let srss = get_srss() ;
    return srss.wdt_ctl.read().wdt_lock().bits() != 0 ;
}

pub fn unlock() {
    let srss = get_srss() ;

    srss.wdt_ctl.modify(|_, w| { w.wdt_lock().bits(1)}) ;
    srss.wdt_ctl.modify(|_, w| { w.wdt_lock().bits(2)}) ;
}

pub fn set_match(value: u16) {
    if !locked()
    {
        let srss = get_srss() ;
        srss.wdt_match.modify(|_, w| unsafe { w.match_().bits(value)}) ;
    }
}

pub fn set_ignore_bits(value: u8) {
    if !locked()
    {
        let srss = get_srss() ;
        srss.wdt_match.modify(|_, w| unsafe { w.ignore_bits().bits(value)}) ;
    }
}

pub fn clear_interrupt() {
    let srss = get_srss() ;
    srss.srss_intr.write(|w| { w.wdt_match().set_bit()}) ;
    srss.srss_intr.read() ;
}

pub fn clear_watchdog() {
    clear_interrupt() ;
}

pub fn mask_interrupt() {
    let srss = get_srss() ;
    srss.srss_intr_mask.modify(|_, w| { w.wdt_match().clear_bit()}) ;
}

pub fn unmask_interrupt() {
    let srss = get_srss() ;
    srss.srss_intr_mask.modify(|_, w| { w.wdt_match().set_bit()}) ;
}