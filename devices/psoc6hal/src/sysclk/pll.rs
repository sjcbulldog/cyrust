use super::super::srss::hw::get_srss ;
use super::super::HalStatus ;

/* PLL OUTPUT_DIV bitfield allowable range */
const PLL_MIN_OUTPUT_DIV:u32 = 2 ;
const PLL_MAX_OUTPUT_DIV:u32 = 16 ;

/* PLL REFERENCE_DIV bitfield allowable range */
const PLL_MIN_REF_DIV:u32 = 1 ;
const PLL_MAX_REF_DIV:u32 = 18 ;

/* PLL FEEDBACK_DIV bitfield allowable ranges, LF and normal modes */
const PLL_MIN_FB_DIV_LF:u32 = 19 ;
const PLL_MAX_FB_DIV_LF:u32 = 56 ;
const PLL_MIN_FB_DIV_NORM:u32 = 22 ;
const PLL_MAX_FB_DIV_NORM:u32 = 112 ;

/* PLL FEEDBACK_DIV bitfield allowable range selection */
//#define PLL_MIN_FB_DIV       ((config->low_frequency_mode) ? PLL_MIN_FB_DIV_LF : PLL_MIN_FB_DIV_NORM)
//#define PLL_MAX_FB_DIV       ((config->low_frequency_mode) ? PLL_MAX_FB_DIV_LF : PLL_MAX_FB_DIV_NORM)

/* PLL Fvco range allowable ranges, LF and normal modes */
const PLL_MIN_FVCO_LF:u32 = 170000000 ;
const PLL_MAX_FVCO_LF:u32 = 200000000 ;
const PLL_MIN_FVCO_NORM:u32 = 200000000 ;
const PLL_MAX_FVCO_NORM:u32 = 400000000 ;

/* PLL input and output frequency limits */
const PLL_MIN_IN_FREQ:u32 = 4000000 ;
const PLL_MAX_IN_FREQ:u32 = 64000000 ;
const PLL_MAX_OUT_FREQ:u32 = 150000000 ;

pub struct PLLManualConfig
{
    pub feedback_divider: u8 ,
    pub reference_divider: u8 ,
    pub output_divider: u8 ,
    pub low_frequency_mode : bool ,
    pub output_mode: super::FllPllOutputMode,
}

fn pll_min_fvco(lfmode: bool) -> u32 {
    return if lfmode 
    {
        PLL_MIN_FVCO_LF
    }
    else
    {
        PLL_MIN_FVCO_NORM
    }
}

fn pll_max_fvco(lfmode: bool) -> u32 {
    return if lfmode 
    {
        PLL_MAX_FVCO_LF
    }
    else
    {
        PLL_MAX_FVCO_NORM
    }
}

fn pll_min_fb_div(lfmode: bool) -> u32 {
    return if lfmode 
    {
        PLL_MIN_FB_DIV_LF
    }
    else
    {
        PLL_MIN_FB_DIV_NORM
    }
}

fn pll_max_fb_div(lfmode: bool) -> u32 {
    return if lfmode 
    {
        PLL_MAX_FB_DIV_LF
    }
    else
    {
        PLL_MAX_FB_DIV_NORM
    }
}

pub fn configure(path: u8, infreq: u32, outfreq: u32, lfmode: bool, outmode: super::FllPllOutputMode) -> HalStatus {
    let ret:HalStatus ;

    if infreq < PLL_MIN_IN_FREQ || infreq > PLL_MAX_IN_FREQ
    {
        return Err(super::BAD_PARAM) ;
    }

    if outfreq < pll_min_fvco(lfmode) || outfreq > PLL_MAX_OUT_FREQ
    {
        return Err(super::BAD_PARAM) ;
    }

    let mut config = PLLManualConfig {
        feedback_divider: 0,
        reference_divider: 0,
        output_divider: 0,
        low_frequency_mode : false,
        output_mode: super::FllPllOutputMode::FLLPLL_OUTPUT_AUTO
    } ;

    if outmode != super::FllPllOutputMode::FLLPLL_OUTPUT_INPUT
    {
        let mut p: u32  ;
        let mut q: u32 = pll_min_fb_div(lfmode) ;
        let mut out: u32 ;
        let mut fout_best : u32 = 0 ;
        let mut fvco:u32 ;

        while q < PLL_MAX_REF_DIV && fout_best != outfreq
        {
            p = PLL_MIN_REF_DIV ;
            while p < pll_max_fb_div(lfmode) && fout_best != outfreq
            {
                fvco = ((infreq as u64) * (p as u64) / (q as u64)) as u32 ;
                if fvco >= pll_min_fvco(lfmode) && fvco <= pll_max_fvco(lfmode)
                {
                    out = PLL_MIN_OUTPUT_DIV ;
                    while out < PLL_MAX_OUTPUT_DIV && fout_best != outfreq
                    {
                        let fout:u32 = ((p * infreq) / q) / out ;
                        let curdiff:i32 = ((fout as i32) - (outfreq as i32)).abs() ;
                        let bestdiff:i32 = ((fout_best as i32) - (outfreq as i32)).abs() ;

                        if curdiff < bestdiff 
                        {
                            fout_best = fout ;
                            config.feedback_divider = p as u8 ;
                            config.reference_divider = q as u8 ;
                            config.output_divider = out as u8 ;
                        }
                        out += 1 ;
                    }
                }
                p += 1 ;
            }
            q += 1 ;
        }
    }
    else
    {
        ret = get_configuration(path, &mut config) ;
        if ret != Ok(0)
        {
            return ret ;
        }
    }

    config.output_mode = outmode ;
    return manual_configure(path, config) ;
}

pub fn manual_configure(path: u8, config: PLLManualConfig) -> HalStatus {
    let srss = get_srss() ;

    if path > super::number_plls() as u8
    {
        return Err(super::BAD_PARAM) ;
    }

    if enabled(path as u8)
    {
        return Err(super::INVALID_STATE) ;
    }

    if config.output_mode != super::FllPllOutputMode::FLLPLL_OUTPUT_INPUT
    {
        srss.clk_pll_config[(path -1) as usize].write(|w| unsafe {
            w.feedback_div().bits(config.feedback_divider).reference_div().bits(config.reference_divider).
                output_div().bits(config.output_divider).pll_lf_mode().bit(config.low_frequency_mode)
        }) ;
    }

    srss.clk_pll_config[(path - 1) as usize].modify(|_, w|  { w.bypass_sel().bits(config.output_mode as u8)}) ;
    return Ok(0) ;
}

pub fn get_configuration(path: u8, config: &mut PLLManualConfig) -> HalStatus {
    let srss = get_srss() ;
    let d = srss.clk_pll_config[path as usize].read() ;

    config.feedback_divider = d.feedback_div().bits() ;
    config.reference_divider = d.reference_div().bits() ;
    config.output_divider = d.output_div().bits() ;
    config.low_frequency_mode = d.pll_lf_mode().bit() ;
    config.output_mode = match d.bypass_sel().bits() {
        0 => super::FllPllOutputMode::FLLPLL_OUTPUT_AUTO,
        1 => super::FllPllOutputMode::FLLPLL_OUTPUT_AUTO1,
        2 => super::FllPllOutputMode::FLLPLL_OUTPUT_INPUT,
        3 => super::FllPllOutputMode::FLLPLL_OUTPUT_OUTPUT,
        _ => unreachable!()
    } ;

    return Ok(0) ;
}

pub fn enable(path: u8, timeout: u32) -> HalStatus {
    let srss = get_srss() ;
    let mut to = timeout ;

    if path > super::number_plls() as u8
    {
        return Err(super::BAD_PARAM) ;
    }

    srss.clk_pll_config[(path - 1) as usize].modify(|_, w|  { w.enable().set_bit()}) ;

    while !locked(path) && to > 0
    {
        super::delay_us(1) ;
        to -= 1 ;
    }

    if to == 0
    {
        return Err(super::TIMEOUT) ;
    }

    return Ok(0) ;
}

pub fn enabled(path: u8) -> bool {
    let srss = get_srss() ;
    return srss.clk_pll_config[(path - 1) as usize].read().enable().bits() ;
}

pub fn locked(path: u8) -> bool {
    let srss = get_srss() ;
    return srss.clk_pll_status[(path - 1) as usize].read().locked().bits() ;
}

pub fn lost_lock(path: u8) -> bool {
    let srss = get_srss() ;
    return srss.clk_pll_status[(path - 1) as usize].read().unlock_occurred().bits() ;
}

pub fn disable(path: u8) -> HalStatus {
    let srss = get_srss() ;

    let v: u8 = super::FllPllOutputMode::FLLPLL_OUTPUT_INPUT as u8 ;
    srss.clk_pll_config[(path - 1) as usize].modify(|_, w|  { w.bypass_sel().bits(v)}) ;
    super::delay_us(1) ;
    srss.clk_pll_config[(path - 1) as usize].modify(|_, w|  { w.enable().clear_bit()}) ;
    return Ok(0) ;
}

pub fn get_feedback_div(path: u8) -> u32 {
    let srss = get_srss() ;
    return srss.clk_pll_config[path as usize].read().feedback_div().bits() as u32 ;
}

pub fn get_reference_div(path: u8) -> u32  {
    let srss = get_srss() ;
    return srss.clk_pll_config[path as usize].read().reference_div().bits() as u32 ;
}

pub fn get_output_div(path: u8) -> u32  {
    let srss = get_srss() ;
    return srss.clk_pll_config[path as usize].read().output_div().bits() as u32 ;
}