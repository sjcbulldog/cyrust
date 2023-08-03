use super::super::srss::hw::get_srss ;
use super::super::HalStatus ;
use super::super::syslib::div_round ;
use super::super::syslib::div_round64 ;

pub const FLL_MIN_CCO_OUTPUT_FREQ:u32 = 48000000 ;
pub const FLL_MIN_OUTPUT_FREQ:u32 = FLL_MIN_CCO_OUTPUT_FREQ / 2 ;
pub const FLL_MAX_OUTPUT_FREQ:u32 = 100000000 ;

const FLL_INT_COEF: u32 = 327680000 ;
const FLL_GAIN_IDX: u32 = 11 ;
const FLL_GAIN_VAL: u32 = 8 * FLL_INT_COEF ;
const TRIM_STEPS_SCALE: u64 = 100000000 ;
const MARGIN_SCALE:u64 = 100000 ;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[derive(Copy, Clone)] 
pub enum OutputRange
{
    FLL_CCO_RANGE0 = 0, /*  Target frequency is in range  48 -  64 MHz */
    FLL_CCO_RANGE1 = 1, /*  Target frequency is in range  64 -  85 MHz */
    FLL_CCO_RANGE2 = 2, /*  Target frequency is in range  85 - 113 MHz */
    FLL_CCO_RANGE3 = 3, /*  Target frequency is in range 113 - 150 MHz */
    FLL_CCO_RANGE4 = 4  /*  Target frequency is in range 150 - 200 MHz */
}

pub struct FllManualConfig
{
    pub fll_mult: u32,
    pub ref_divider: u16,
    pub cco_range: OutputRange,
    pub enable_output_divider: bool,
    pub lock_tolerance: u16,
    pub igain: u8,
    pub pgain: u8,
    pub settling_count : u16,
    pub output_mode: super::FllPllOutputMode,
    pub cco_frequency : u16
}

const TRIMSTEPS: [u32; 5] = [110340, 110200, 110000, 110000, 117062] ;       /* Scaled by 10^8 */
const MARGIN: [u32 ; 5] = [436, 581, 772, 1030, 1320]; /* Scaled by 10^5 */

pub fn configure_manual(config: &FllManualConfig) -> HalStatus {
    let srss = get_srss() ;

    if enabled()
    {
        return Err(super::INVALID_STATE) ;
    }

    srss.clk_fll_config.write(|w| unsafe { w.fll_mult().bits(config.fll_mult).fll_output_div().bit(config.enable_output_divider) }) ;
    srss.clk_fll_config2.write(|w| unsafe { w.fll_ref_div().bits(config.ref_divider).lock_tol().bits(config.lock_tolerance)}) ;
    srss.clk_fll_config3.write(|w| unsafe { 
        w.fll_lf_igain().bits(config.igain).fll_lf_pgain().bits(config.pgain)
            .settling_count().bits(config.settling_count).bypass_sel().bits(config.output_mode as u8)
    }) ;
    srss.clk_fll_config4.write(|w| unsafe { w.cco_range().bits(config.cco_range as u8).cco_freq().bits(config.cco_frequency) }) ;

    return Ok(0) ;
}

pub fn configure(infreq: u32, outfreq: u32, mode: super::FllPllOutputMode) -> HalStatus {
    let srss = get_srss() ;

    if outfreq < FLL_MIN_OUTPUT_FREQ || outfreq > FLL_MAX_OUTPUT_FREQ || (outfreq * 5) / infreq < 1
    {
        return Err(super::BAD_PARAM) ;
    }

    let mut ret:HalStatus = Ok(0) ;

    if mode != super::FllPllOutputMode::FLLPLL_OUTPUT_INPUT
    {
        let wco = super::get_path_source(0) == super::ClockPathSource::CLKPATH_IN_WCO ;
        
        /* Step 1: Set output division by 2 */
        let en_output_div = true ;

        /* Step 2: Compute the target cco frequency */
        let mut ccofreq = outfreq * 2 ;

        /* Step 3: Compute the CCO range */
        let ccorange = 
            if ccofreq >= 150339200
            {
                OutputRange::FLL_CCO_RANGE4
            }
            else if ccofreq >= 113009380
            {
                OutputRange::FLL_CCO_RANGE3
            }
            else if ccofreq >=  84948700
            {
                OutputRange::FLL_CCO_RANGE2
            }
            else if ccofreq >=  63855600
            {
                OutputRange::FLL_CCO_RANGE1
            }
            else
            {
                OutputRange::FLL_CCO_RANGE0
            } ;

        /* Step 4: Compute the FLL reference divider */
        let refdiv = 
        if wco 
        { 
            19 
        }
        else 
        { 
            div_round(infreq * 250, outfreq)
        } ;

        /* Step 5: Compute the FLL multiplier */
        let mult = div_round64((ccofreq * refdiv) as u64, infreq as u64) as u32 ;

        /* Step 6: compute the lock tolerance */
        let tol = div_round(mult * 18939, 1000000) as u16 ;

        /* Step 7: Compute the CCO igain and pgain */
        let kcco = TRIMSTEPS[ccorange as usize] * MARGIN[ccorange as usize] ;
        let kip = div_round64((850 as u64) * (FLL_INT_COEF as u64) * (infreq as u64), (kcco as u64) * (refdiv as u64)) as u32 ;
        let mut locigain = FLL_GAIN_VAL ;
        let mut locpgain = FLL_GAIN_VAL ;
        let mut igain:u32 = FLL_GAIN_IDX ;
        let mut pgain:u32 = FLL_GAIN_IDX ;

        while locigain > kip && igain != 0
        {
            locigain >>= 1 ;
            igain -= 1 ;
        }

        if wco && igain > 0
        {
            igain -= 1  ;
            locigain >>= 1 ;
        }

        while locpgain > (kip - locigain) && pgain != 0
        {
            locpgain >>= 1 ;
            pgain -= 1 ;
        }

        if wco &&  pgain > 0
        {
            pgain -= 1 ;
        }

        /* Step 8: Compute the CCO FREQ bits in CLK FLL CONFIG4 register */
        let cmp:u64 = div_round64(TRIM_STEPS_SCALE / MARGIN_SCALE * (ccofreq as  u64), MARGIN[ccorange as usize] as u64) ;
        let index = ccorange as usize ;
        let mlt = TRIM_STEPS_SCALE + (TRIMSTEPS[index] as u64) ;
        let mut res = mlt ;

        while res < cmp 
        {
            res *= mlt ;
            res /= TRIM_STEPS_SCALE ;
            ccofreq += 1 ;
        }

        /* Step 9: Compute the settling count, using a 1 ms settling time */
        let fref:u64 = div_round64(6000 * (infreq as u64), refdiv as u64) ;
        let divval:u32 = div_round(infreq, 1000000) ;
        let altval:u32 = (div_round64((divval as u64) * fref, 6000000)) as u32 + 1 ;

        let settling_count:u32 =
            if wco
            {
                200
            }
            else if outfreq < fref as u32
            {
                divval
            }
            else
            {
                altval
            } ;

        let config = FllManualConfig { 
            fll_mult: mult, 
            ref_divider: refdiv as u16,
            cco_range: ccorange,
            enable_output_divider: en_output_div,
            lock_tolerance: tol,
            igain: igain as u8,
            pgain: pgain as u8,
            settling_count: settling_count as u16,
            output_mode: mode,
            cco_frequency: ccofreq as u16
        } ;

        ret = configure_manual(&config) ;
    }
    else
    {
        srss.clk_fll_config3.modify(|_, w|  { w.bypass_sel().bits(super::FllPllOutputMode::FLLPLL_OUTPUT_INPUT as u8)})
    }

    return ret ;
}

pub fn enable(timeout: u32) -> HalStatus {
    let srss = get_srss() ;
    let mut to = timeout ;

    srss.clk_fll_config4.modify(|_, w|  { w.cco_enable().set_bit()}) ;

    while !is_ready() && to > 0
    {
        super::delay_us(1) ;
        to -= 1 ;
    }

    srss.clk_fll_config3.write(|w|  { w.bypass_sel().bits(super::FllPllOutputMode::FLLPLL_OUTPUT_INPUT as u8) } ) ;

    if timeout == 0 || to != 0
    {
        srss.clk_fll_config.modify(|_, w|  { w.fll_enable().bit(true)}) ;
    }

    while !is_locked() && to > 0
    {
        super::delay_us(1) ;
        to -= 1 ;
    }

    if timeout == 0 || to != 0
    {
        srss.clk_fll_config3.modify(|_, w|  { w.bypass_sel().bits(super::FllPllOutputMode::FLLPLL_OUTPUT_OUTPUT as u8)}) ;
    }
    else
    {
        let _ = disable() ;
    }

    if timeout != 0 && to == 0
    {
        return Err(super::TIMEOUT) ;
    }
    
    return Ok(0) ;
}

pub fn is_ready() -> bool {
    let srss = get_srss() ;
    return srss.clk_fll_status.read().cco_ready().bit() ;
}

pub fn is_locked() -> bool {
    let srss = get_srss() ;
    return srss.clk_fll_status.read().locked().bit() ;
}

pub fn has_unlock_occured() -> bool {
    let srss = get_srss() ;
    return srss.clk_fll_status.read().unlock_occurred().bit() ;
}

pub fn enabled() -> bool {
    let srss = get_srss() ;
    return srss.clk_fll_config.read().fll_enable().bits() ;
}

pub fn locked() -> bool {
    let srss = get_srss() ;
    return srss.clk_fll_status.read().locked().bits() ;
}

pub fn disable() -> HalStatus {
    let srss = get_srss() ;
    srss.clk_fll_config3.write(|w|  { w.bypass_sel().bits(2)}) ;
    srss.clk_fll_config.modify(|_, w|  { w.fll_enable().clear_bit()}) ;
    srss.clk_fll_config4.modify(|_, w|  { w.cco_enable().clear_bit()}) ;

    return Ok(0) ;
}

pub fn get_multiplier() -> u32 {
    let srss = get_srss() ;
    return srss.clk_fll_config.read().fll_mult().bits() as u32 ;
}

pub fn get_reference_div() -> u32 {
    let srss = get_srss() ;
    return srss.clk_fll_config2.read().fll_ref_div().bits() as u32 ;
}

pub fn output_div_enabled() -> bool {
    let srss = get_srss() ;
    return srss.clk_fll_config.read().fll_output_div().bits() ;
}