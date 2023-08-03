pub mod pilo ;
pub mod ilo ;
pub mod eco ;
pub mod pll ;
pub mod fll ;
pub mod wco ;
pub mod ext ;
pub mod altlf ;
pub mod althf ;

use psoc6_02::*  ;
use super::syslib::div_round ;
use super::HalStatus ;
use super::MAKE_STATUS;
use super::cpuss::hw::get_cpuss ;
use super::srss::hw::get_srss ;

pub const MODULE_ID:u8 = 1 ;
pub const ERROR_WDT_ENABLED: u32 = MAKE_STATUS!(MODULE_ID, 1) ;
pub const TIMEOUT: u32 = MAKE_STATUS!(MODULE_ID, 2) ;
pub const INVALID_EXT_FREQ: u32 = MAKE_STATUS!(MODULE_ID, 3) ;
pub const INVALID_STATE: u32 = MAKE_STATUS!(MODULE_ID, 4) ;
pub const BAD_PARAM: u32 = MAKE_STATUS!(MODULE_ID, 5) ;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[derive(Copy, Clone)] 
pub enum FllPllOutputMode {
    FLLPLL_OUTPUT_AUTO   = 0, /*  Output FLL/PLL input source when not locked, and FLL/PLL output when locked */
    FLLPLL_OUTPUT_AUTO1  = 1, /*  Same as AUTO */
    FLLPLL_OUTPUT_INPUT  = 2, /*  Output FLL/PLL input source regardless of lock status */
    FLLPLL_OUTPUT_OUTPUT = 3
}

#[derive(PartialEq)]
#[derive(Copy, Clone)] 
pub enum DividerType {
    EightBitDivider = 0,
    SixteenBitDivider = 1,
    SixteenBitFractionalDivider = 2,
    TwentyFourBitFractionalDivider = 3
}

pub const IMO_FREQ:u32 = 8000000 ;
pub const ILO_FREQ:u32 = 32768 ;
pub const WCO_FREQ:u32 = 32768 ;
pub const PILO_FREQ:u32 = 32768 ;
pub const CY_FB_CLK_FLL_CONFIG_VALUE:u32 = 0x01000000 ;
pub const CY_FB_CLK_FLL_CONFIG2_VALUE:u32 = 0x00020001 ;
pub const CY_FB_CLK_FLL_CONFIG3_VALUE:u32 = 0x00002800 ;
pub const CY_FB_CLK_FLL_CONFIG4_VALUE:u32 = 0x000000FF ;


#[allow(non_camel_case_types)]
#[derive(Copy, Clone)] 
pub enum ClockType
{
    SCB0_CLOCK                 = 0x0000,  /* scb[0].clock */
    SCB1_CLOCK                 = 0x0001,  /* scb[1].clock */
    SCB2_CLOCK                 = 0x0002,  /* scb[2].clock */
    SCB3_CLOCK                 = 0x0003,  /* scb[3].clock */
    SCB4_CLOCK                 = 0x0004,  /* scb[4].clock */
    SCB5_CLOCK                 = 0x0005,  /* scb[5].clock */
    SCB6_CLOCK                 = 0x0006,  /* scb[6].clock */
    SCB7_CLOCK                 = 0x0007,  /* scb[7].clock */
    SCB8_CLOCK                 = 0x0008,  /* scb[8].clock */
    SCB9_CLOCK                 = 0x0009,  /* scb[9].clock */
    SCB10_CLOCK                = 0x000A,  /* scb[10].clock */
    SCB11_CLOCK                = 0x000B,  /* scb[11].clock */
    SCB12_CLOCK                = 0x000C,  /* scb[12].clock */
    SMARTIO8_CLOCK             = 0x000D,  /* smartio[8].clock */
    SMARTIO9_CLOCK             = 0x000E,  /* smartio[9].clock */
    TCPWM0_CLOCKS0             = 0x000F,  /* tcpwm[0].clocks[0] */
    TCPWM0_CLOCKS1             = 0x0010,  /* tcpwm[0].clocks[1] */
    TCPWM0_CLOCKS2             = 0x0011,  /* tcpwm[0].clocks[2] */
    TCPWM0_CLOCKS3             = 0x0012,  /* tcpwm[0].clocks[3] */
    TCPWM0_CLOCKS4             = 0x0013,  /* tcpwm[0].clocks[4] */
    TCPWM0_CLOCKS5             = 0x0014,  /* tcpwm[0].clocks[5] */
    TCPWM0_CLOCKS6             = 0x0015,  /* tcpwm[0].clocks[6] */
    TCPWM0_CLOCKS7             = 0x0016,  /* tcpwm[0].clocks[7] */
    TCPWM1_CLOCKS0             = 0x0017,  /* tcpwm[1].clocks[0] */
    TCPWM1_CLOCKS1             = 0x0018,  /* tcpwm[1].clocks[1] */
    TCPWM1_CLOCKS2             = 0x0019,  /* tcpwm[1].clocks[2] */
    TCPWM1_CLOCKS3             = 0x001A,  /* tcpwm[1].clocks[3] */
    TCPWM1_CLOCKS4             = 0x001B,  /* tcpwm[1].clocks[4] */
    TCPWM1_CLOCKS5             = 0x001C,  /* tcpwm[1].clocks[5] */
    TCPWM1_CLOCKS6             = 0x001D,  /* tcpwm[1].clocks[6] */
    TCPWM1_CLOCKS7             = 0x001E,  /* tcpwm[1].clocks[7] */
    TCPWM1_CLOCKS8             = 0x001F,  /* tcpwm[1].clocks[8] */
    TCPWM1_CLOCKS9             = 0x0020,  /* tcpwm[1].clocks[9] */
    TCPWM1_CLOCKS10            = 0x0021,  /* tcpwm[1].clocks[10] */
    TCPWM1_CLOCKS11            = 0x0022,  /* tcpwm[1].clocks[11] */
    TCPWM1_CLOCKS12            = 0x0023,  /* tcpwm[1].clocks[12] */
    TCPWM1_CLOCKS13            = 0x0024,  /* tcpwm[1].clocks[13] */
    TCPWM1_CLOCKS14            = 0x0025,  /* tcpwm[1].clocks[14] */
    TCPWM1_CLOCKS15            = 0x0026,  /* tcpwm[1].clocks[15] */
    TCPWM1_CLOCKS16            = 0x0027,  /* tcpwm[1].clocks[16] */
    TCPWM1_CLOCKS17            = 0x0028,  /* tcpwm[1].clocks[17] */
    TCPWM1_CLOCKS18            = 0x0029,  /* tcpwm[1].clocks[18] */
    TCPWM1_CLOCKS19            = 0x002A,  /* tcpwm[1].clocks[19] */
    TCPWM1_CLOCKS20            = 0x002B,  /* tcpwm[1].clocks[20] */
    TCPWM1_CLOCKS21            = 0x002C,  /* tcpwm[1].clocks[21] */
    TCPWM1_CLOCKS22            = 0x002D,  /* tcpwm[1].clocks[22] */
    TCPWM1_CLOCKS23            = 0x002E,  /* tcpwm[1].clocks[23] */
    CSD_CLOCK                  = 0x002F,  /* csd.clock */
    LCD_CLOCK                  = 0x0030,  /* lcd.clock */
    PROFILE_CLOCK_PROFILE      = 0x0031,  /* profile.clock_profile */
    CPUSS_CLOCK_TRACE_IN       = 0x0032,  /* cpuss.clock_trace_in */
    PASS_CLOCK_PUMP_PERI       = 0x0033,  /* pass.clock_pump_peri */
    PASS_CLOCK_SAR             = 0x0034,  /* pass.clock_sar */
    USB_CLOCK_DEV_BRS          = 0x0035   /* usb.clock_dev_brs */
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[derive(Copy, Clone)] 
pub enum ClockPathSource
{
    CLKPATH_IN_IMO    =     0,  /* Select the IMO as the output of the path mux */
    CLKPATH_IN_EXT    =     1,  /* Select the EXT as the output of the path mux */
    CLKPATH_IN_ECO    =     2,  /* Select the ECO as the output of the path mux */
    CLKPATH_IN_ALTHF  =     3,  /* Select the ALTHF as the output of the path mux.
                                           *   Make sure the ALTHF clock source is available on used device.
                                           */
    CLKPATH_IN_DSIMUX =     4,  /* Select the DSI MUX output as the output of the path mux */
    CLKPATH_IN_DSI    = 0x100,  /* Select a DSI signal (0 - 15) as the output of the DSI mux and path mux.
                                           *   Make sure the DSI clock sources are available on used device.
                                           */
    CLKPATH_IN_ILO    = 0x110,  /* Select the ILO (16) as the output of the DSI mux and path mux */
    CLKPATH_IN_WCO    = 0x111,  /* Select the WCO (17) as the output of the DSI mux and path mux */
    CLKPATH_IN_ALTLF  = 0x112,  /* Select the ALTLF (18) as the output of the DSI mux and path mux.
                                           *   Make sure the ALTLF clock sources in available on used device.
                                           */
    CLKPATH_IN_PILO   = 0x113   /* Select the PILO (19) as the output of the DSI mux and path mux.
                                           *   Make sure the PILO clock sources in available on used device.
                                           */
}

static mut PRIV_HFCLK0_FREQ_HZ:u32 = 0 ;
static mut PRIV_PERI_CLK_FREQ_HZ:u32 = 0 ;
static mut PRIV_SYSTEM_CORE_CLOCK:u32 = 0 ;

pub fn number_hfroots() -> u32 {
    return match PSOC_FAMILY {
        1 => 5,
        2 => 6,
        3 => 5,
        4 => 4,
        _ => panic!()
    }
}

pub fn number_plls() -> u32 {
    return match PSOC_FAMILY {
        1 => 1,
        2 => 2,
        3 => 1,
        4 => 1,
        _ => panic!()
    }
}

pub fn number_clkpaths() -> u32 {
    return match PSOC_FAMILY {
        1 => 5,
        2 => 6,
        3 => 5,
        4 => 5,
        _ => panic!()
    }
}

pub fn core_clock_update() -> HalStatus {
    let loc_hf0_clock = get_hf_frequency(0) ;

    if loc_hf0_clock != 0
    {
        unsafe {
            PRIV_HFCLK0_FREQ_HZ = loc_hf0_clock ;
            PRIV_PERI_CLK_FREQ_HZ = loc_hf0_clock / (1 + (get_peri_divider() as u32)) ;
            PRIV_SYSTEM_CORE_CLOCK = loc_hf0_clock / (1 + get_fast_divider()) ;
        }
    }

    return Ok(0) ;
}

pub fn delay_us(_length: u32) {
}

pub fn sqrt(x: u32) -> u32 {
    let mut res: u32 = 0;
    let mut add:u32 = 0x8000;

    for _ in 0 .. 15
    {
        let tmp: u32  = res | add;

        if x >= (tmp * tmp)
        {
            res = tmp;
        }

        add >>= 1;
    }

    return res;
}

pub fn set_path_source(path: u32, src: ClockPathSource) -> HalStatus {
    let srss = get_srss() ;

    if path >= number_clkpaths()
    {
        return Err(BAD_PARAM) ;
    }

    if src as u32 >= ClockPathSource::CLKPATH_IN_DSI as u32
    {
        srss.clk_dsi_select[path as usize].write(|w| unsafe { w.dsi_mux().bits(src as u8)}) ;
        srss.clk_path_select[path as usize].write(|w| unsafe { w.path_mux().bits(ClockPathSource::CLKPATH_IN_DSIMUX as u8)}) ;
    }
    else
    {
        srss.clk_path_select[path as usize].write(|w| unsafe { w.path_mux().bits(src as u8)}) ;
    }

    return Ok(0) ;
}

pub fn get_path_source(path: u8) -> ClockPathSource {
    let ret : ClockPathSource ;
    let srss = get_srss() ;

    let mut src = srss.clk_path_select[path as usize].read().path_mux().bits() ;
    if src == ClockPathSource::CLKPATH_IN_DSIMUX as u8
    {
        src = srss.clk_dsi_select[path as usize].read().dsi_mux().bits() ;
        ret = match src {
            0x10 => ClockPathSource::CLKPATH_IN_ILO,
            0x11 => ClockPathSource::CLKPATH_IN_WCO,
            0x12 => ClockPathSource::CLKPATH_IN_ALTLF,
            0x13 => ClockPathSource::CLKPATH_IN_PILO,
            _ => unreachable!()
        }
    }
    else
    {
        ret = match src {
            0 => ClockPathSource::CLKPATH_IN_IMO,
            1 => ClockPathSource::CLKPATH_IN_EXT,
            2 => ClockPathSource::CLKPATH_IN_ECO,
            3 => ClockPathSource::CLKPATH_IN_ALTHF,
            _ => unreachable!()
        } ;
    }

    return ret ;
}

pub fn get_path_mux_frequency(path: u8) -> u32 {
    let src = get_path_source(path) ;

    let ret = match src {
        ClockPathSource::CLKPATH_IN_IMO => IMO_FREQ,
        ClockPathSource::CLKPATH_IN_EXT => ext::get_frequency(),
        ClockPathSource::CLKPATH_IN_ECO => eco::get_frequency(),
        ClockPathSource::CLKPATH_IN_ALTHF => althf::get_frequency(),
        ClockPathSource::CLKPATH_IN_ILO => if ilo::enabled() { ILO_FREQ } else { 0 },
        ClockPathSource::CLKPATH_IN_WCO => if wco::ok() { WCO_FREQ } else { 0 },
        ClockPathSource::CLKPATH_IN_PILO => if pilo::enabled() { PILO_FREQ } else { 0 },
        ClockPathSource::CLKPATH_IN_ALTLF => altlf::get_frequency(),
        _ => panic!()
    } ;

    return ret ;
}

pub fn get_path_frequency(path: u8) -> u32 {
    let mut freq = get_path_mux_frequency(path) ;
    let mut fdiv:u32 = 1 ;
    let mut rdiv:u32 = 1 ;
    let mut odiv:u32 = 1 ;
    let mut enabled:bool = false ;

    if path == 0 {
        fdiv = fll::get_multiplier() ;
        rdiv = fll::get_reference_div() ;
        odiv = if fll::output_div_enabled() { 2 } else { 1 } ;
        enabled = fll::enabled() ;
    }
    else if path < number_plls() as u8
    {
        fdiv = pll::get_feedback_div(path) ;
        rdiv = pll::get_reference_div(path) ;
        odiv = pll::get_output_div(path) ;
        enabled = pll::enabled(path) ;
    }

    if enabled && rdiv != 0 && odiv != 0
    {
        freq = div_round(freq * fdiv, rdiv * odiv) ;
    }

    return freq ;
}

pub fn get_fast_divider() -> u32 {
    let cpu = get_cpuss() ;
    return cpu.cm4_clock_ctl.read().fast_int_div().bits() as u32 ;
}

pub fn get_hf_divider(path: u8) -> u32 {
    let srss = get_srss() ;
    return srss.clk_root_select[path as usize].read().root_div().bits() as u32 ;
}

pub fn get_hf_source(path: u8) -> u8 {
    let srss = get_srss() ;
    return srss.clk_root_select[path as usize].read().root_mux().bits() ;
}

pub fn get_hf_frequency(path: u8) -> u32 {
    let div = 1 << get_hf_divider(path) ;
    let path = get_hf_source(path) ;
    let freq = get_path_frequency(path) ;

    return div_round(freq, div) ;
}

pub fn get_peri_divider() -> u8 {
    let cpu = get_cpuss() ;
    return cpu.cm0_clock_ctl.read().peri_int_div().bits() ;
}

pub fn set_peri_divider(div: u8) {
    let cpu = get_cpuss() ;
    cpu.cm0_clock_ctl.modify(|_, w| unsafe { w.peri_int_div().bits(div)}) ;
}

pub fn get_peri_frequency() -> u32 {
    let freq = get_hf_frequency(0) ;
    let div = get_peri_divider()  + 1 ;
    return div_round(freq, div as u32) ;
}
