#![no_std]

use cortex_m_rt::pre_init ;

pub mod gpio ;
pub mod hsiom ;
pub mod sysclk ;
pub mod syslib ;
pub mod cpuss ;
pub mod scb ;
pub mod uart ;
pub mod peri ;
pub mod srss ;
pub mod wdt ;
pub mod backup ;
pub mod systick;

type HalStatus = Result<u32, u32> ;

#[macro_export]
macro_rules! MAKE_STATUS {
    ($mod:expr, $err:expr) => {
        (($mod as u32) << 24 | $err)
    }
}

fn system_init() {
    let srss = srss::hw::get_srss() ;

    //
    // Reset the clocking since the ROM leave us in a less than ideal state
    //
    srss.clk_fll_config.modify(|_, w| { w.fll_enable().clear_bit() });
    srss.clk_root_select[0].modify(|_, w| { w.root_div().bits(0)}) ;

    srss.clk_fll_config.write(|w| unsafe { w.bits(sysclk::CY_FB_CLK_FLL_CONFIG_VALUE)}) ;
    srss.clk_fll_config2.write(|w| unsafe { w.bits(sysclk::CY_FB_CLK_FLL_CONFIG2_VALUE)}) ;
    srss.clk_fll_config3.write(|w| unsafe { w.bits(sysclk::CY_FB_CLK_FLL_CONFIG3_VALUE)}) ;
    srss.clk_fll_config4.write(|w| unsafe { w.bits(sysclk::CY_FB_CLK_FLL_CONFIG4_VALUE)}) ;

    wdt::unlock() ;
    wdt::disable() ;
    let _ = sysclk::core_clock_update() ;
}

#[pre_init]
unsafe fn setup_vtor() {
    let vaddr = 0x1000_4000 ;
    const VTOR: *mut u32 = 0xE000_ED08 as *mut u32;
    core::ptr::write_volatile(VTOR, vaddr) ;

    //
    // Initialize the clocks
    //
    system_init() ;
}
