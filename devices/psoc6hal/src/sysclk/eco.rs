use super::super::srss::hw::get_srss ;
use super::super::HalStatus ;
use super::super::syslib::div_round ;

static mut PRIV_ECO_CLK_FREQ_HZ:u32 = 0 ;

const ECO_FREQ_MIN:u32 = 16000000; /* 16 MHz */
const ECO_FREQ_MAX:u32 = 35000000; /* 35 MHz */

const ECO_CSM_MAX:u32 = 100;      /* 100 pF */
const ECO_ESR_MAX:u32 = 1000;     /* 1000 Ohm */
const ECO_DRV_MAX:u32 = 2000;     /* 2 mW */

pub fn get_frequency() -> u32 {
    let mut ret:u32 = 0 ;
    if is_stable()
    {
        unsafe {
            ret = PRIV_ECO_CLK_FREQ_HZ ;
        }
    }

    return ret ;
}

pub fn configure(freq: u32, csum: u32, esr: u32, drive: u32) -> HalStatus {
    let srss = get_srss() ;

    if srss.clk_eco_config.read().eco_en().bit()
    {
        return Err(super::INVALID_STATE);
    }

    if freq < ECO_FREQ_MIN || freq > ECO_FREQ_MAX
    {
        return Err(super::BAD_PARAM) ;
    }

    if csum > ECO_CSM_MAX
    {
        return Err(super::BAD_PARAM) ;
    }

    if esr > ECO_ESR_MAX
    {
        return Err(super::BAD_PARAM) ;
    }

    if drive > ECO_DRV_MAX
    {
        return Err(super::BAD_PARAM) ;
    }

    let freq: u32 = div_round(freq, 1000) ;
    let maxamp: u32 = div_round(159155 * super::sqrt(div_round(2000000 * drive, esr)), freq * csum) ;
    let ampsect: u32 = (div_round(csum * csum * div_round(freq * freq, 126651), 100) * esr) / 900000 ;

    if maxamp < 650 || ampsect > 3
    {
        return Err(super::BAD_PARAM) ;
    }

    let gtrim: u8  ;

    if ampsect > 1
    {
        gtrim = ampsect as u8 ;
    }
    else
    {
        gtrim = 1 - (ampsect as u8) ;
    }

    srss.clk_trim_eco_ctl.modify(|_, w| unsafe {
        w.wdtrim().bits(7).atrim().bits(15).ftrim().bits(3).rtrim().bits(0).gtrim().bits(gtrim)
    }) ;

    srss.clk_eco_config.modify(|_, w| { w.agc_en().set_bit()}) ;

    unsafe {
        PRIV_ECO_CLK_FREQ_HZ = freq ;
    }

    return Ok(0) ;
}

pub fn disable() -> HalStatus {
    let srss = get_srss() ;
    srss.clk_eco_config.modify(|_, w| { w.eco_en().clear_bit()}) ;

    return Ok(0) ;
}

pub fn is_stable() -> bool {
    let srss = get_srss() ;

    return srss.clk_eco_status.read().eco_ready().bit() ;
}

 pub fn is_ok() -> bool {
    let srss = get_srss() ;

    return srss.clk_eco_status.read().eco_ok().bit() ;
 }

pub fn enable(timeout: u32) -> HalStatus {
    let srss = get_srss() ;

    if srss.clk_eco_config.read().eco_en().bit()
    {
        return Err(super::INVALID_STATE) ;
    }

    srss.clk_eco_config.modify(|_, w| { w.eco_en().set_bit()}) ;

    let mut to = timeout ;
    while !is_stable() && to > 0
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
    let srss = get_srss() ;
    return srss.clk_eco_status.read().eco_ok().bits() ;
}

pub fn ready() -> bool {
    let srss = get_srss() ;
    return srss.clk_eco_status.read().eco_ready().bits() ;
}
