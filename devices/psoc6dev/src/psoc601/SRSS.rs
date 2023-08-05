#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Multi-Counter Watchdog Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtStruct {
    ptr: *mut u8,
}
unsafe impl Send for McwdtStruct {}
unsafe impl Sync for McwdtStruct {}
impl McwdtStruct {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Multi-Counter Watchdog Sub-counters 0/1"]
    #[inline(always)]
    pub const fn mcwdt_cntlow(self) -> crate::common::Reg<McwdtCntlow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Sub-counter 2"]
    #[inline(always)]
    pub const fn mcwdt_cnthigh(self) -> crate::common::Reg<McwdtCnthigh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Counter Match Register"]
    #[inline(always)]
    pub const fn mcwdt_match(self) -> crate::common::Reg<McwdtMatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Counter Configuration"]
    #[inline(always)]
    pub const fn mcwdt_config(self) -> crate::common::Reg<McwdtConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Counter Control"]
    #[inline(always)]
    pub const fn mcwdt_ctl(self) -> crate::common::Reg<McwdtCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Counter Interrupt Register"]
    #[inline(always)]
    pub const fn mcwdt_intr(self) -> crate::common::Reg<McwdtIntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Counter Interrupt Set Register"]
    #[inline(always)]
    pub const fn mcwdt_intr_set(self) -> crate::common::Reg<McwdtIntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mcwdt_intr_mask(self) -> crate::common::Reg<McwdtIntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Counter Interrupt Masked Register"]
    #[inline(always)]
    pub const fn mcwdt_intr_masked(self) -> crate::common::Reg<McwdtIntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Counter Lock Register"]
    #[inline(always)]
    pub const fn mcwdt_lock_reg(self) -> crate::common::Reg<McwdtLockReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
}
#[doc = "SRSS Core Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srss {
    ptr: *mut u8,
}
unsafe impl Send for Srss {}
unsafe impl Sync for Srss {}
impl Srss {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power Mode Control"]
    #[inline(always)]
    pub const fn pwr_ctl(self) -> crate::common::Reg<PwrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "HIBERNATE Mode Register"]
    #[inline(always)]
    pub const fn pwr_hibernate(self) -> crate::common::Reg<PwrHibernate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Low Voltage Detector (LVD) Configuration Register"]
    #[inline(always)]
    pub const fn pwr_lvd_ctl(self) -> crate::common::Reg<PwrLvdCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Buck Control Register"]
    #[inline(always)]
    pub const fn pwr_buck_ctl(self) -> crate::common::Reg<PwrBuckCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Buck Control Register 2"]
    #[inline(always)]
    pub const fn pwr_buck_ctl2(self) -> crate::common::Reg<PwrBuckCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Low Voltage Detector (LVD) Status Register"]
    #[inline(always)]
    pub const fn pwr_lvd_status(self) -> crate::common::Reg<PwrLvdStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "HIBERNATE Data Register"]
    #[inline(always)]
    pub const fn pwr_hib_data(self, n: usize) -> crate::common::Reg<PwrHibData, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize + n * 4usize) as _) }
    }
    #[doc = "Watchdog Counter Control Register"]
    #[inline(always)]
    pub const fn wdt_ctl(self) -> crate::common::Reg<WdtCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Watchdog Counter Count Register"]
    #[inline(always)]
    pub const fn wdt_cnt(self) -> crate::common::Reg<WdtCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize) as _) }
    }
    #[doc = "Watchdog Counter Match Register"]
    #[inline(always)]
    pub const fn wdt_match(self) -> crate::common::Reg<WdtMatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(392usize) as _) }
    }
    #[doc = "Multi-Counter Watchdog Timer"]
    #[inline(always)]
    pub const fn mcwdt_struct(self, n: usize) -> McwdtStruct {
        assert!(n < 2usize);
        unsafe { McwdtStruct::from_ptr(self.ptr.add(512usize + n * 64usize) as _) }
    }
    #[doc = "Clock DSI Select Register"]
    #[inline(always)]
    pub const fn clk_dsi_select(
        self,
        n: usize,
    ) -> crate::common::Reg<ClkDsiSelect, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize + n * 4usize) as _) }
    }
    #[doc = "Clock Path Select Register"]
    #[inline(always)]
    pub const fn clk_path_select(
        self,
        n: usize,
    ) -> crate::common::Reg<ClkPathSelect, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(832usize + n * 4usize) as _) }
    }
    #[doc = "Clock Root Select Register"]
    #[inline(always)]
    pub const fn clk_root_select(
        self,
        n: usize,
    ) -> crate::common::Reg<ClkRootSelect, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(896usize + n * 4usize) as _) }
    }
    #[doc = "Clock selection register"]
    #[inline(always)]
    pub const fn clk_select(self) -> crate::common::Reg<ClkSelect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1280usize) as _) }
    }
    #[doc = "Timer Clock Control Register"]
    #[inline(always)]
    pub const fn clk_timer_ctl(self) -> crate::common::Reg<ClkTimerCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1284usize) as _) }
    }
    #[doc = "ILO Configuration"]
    #[inline(always)]
    pub const fn clk_ilo_config(self) -> crate::common::Reg<ClkIloConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1292usize) as _) }
    }
    #[doc = "IMO Configuration"]
    #[inline(always)]
    pub const fn clk_imo_config(self) -> crate::common::Reg<ClkImoConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1296usize) as _) }
    }
    #[doc = "Fast Clock Output Select Register"]
    #[inline(always)]
    pub const fn clk_output_fast(self) -> crate::common::Reg<ClkOutputFast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1300usize) as _) }
    }
    #[doc = "Slow Clock Output Select Register"]
    #[inline(always)]
    pub const fn clk_output_slow(self) -> crate::common::Reg<ClkOutputSlow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1304usize) as _) }
    }
    #[doc = "Clock Calibration Counter 1"]
    #[inline(always)]
    pub const fn clk_cal_cnt1(self) -> crate::common::Reg<ClkCalCnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1308usize) as _) }
    }
    #[doc = "Clock Calibration Counter 2"]
    #[inline(always)]
    pub const fn clk_cal_cnt2(self) -> crate::common::Reg<ClkCalCnt2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1312usize) as _) }
    }
    #[doc = "ECO Configuration Register"]
    #[inline(always)]
    pub const fn clk_eco_config(self) -> crate::common::Reg<ClkEcoConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1324usize) as _) }
    }
    #[doc = "ECO Status Register"]
    #[inline(always)]
    pub const fn clk_eco_status(self) -> crate::common::Reg<ClkEcoStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1328usize) as _) }
    }
    #[doc = "Precision ILO Configuration Register"]
    #[inline(always)]
    pub const fn clk_pilo_config(self) -> crate::common::Reg<ClkPiloConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1340usize) as _) }
    }
    #[doc = "FLL Configuration Register"]
    #[inline(always)]
    pub const fn clk_fll_config(self) -> crate::common::Reg<ClkFllConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1408usize) as _) }
    }
    #[doc = "FLL Configuration Register 2"]
    #[inline(always)]
    pub const fn clk_fll_config2(self) -> crate::common::Reg<ClkFllConfig2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1412usize) as _) }
    }
    #[doc = "FLL Configuration Register 3"]
    #[inline(always)]
    pub const fn clk_fll_config3(self) -> crate::common::Reg<ClkFllConfig3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1416usize) as _) }
    }
    #[doc = "FLL Configuration Register 4"]
    #[inline(always)]
    pub const fn clk_fll_config4(self) -> crate::common::Reg<ClkFllConfig4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1420usize) as _) }
    }
    #[doc = "FLL Status Register"]
    #[inline(always)]
    pub const fn clk_fll_status(self) -> crate::common::Reg<ClkFllStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1424usize) as _) }
    }
    #[doc = "PLL Configuration Register"]
    #[inline(always)]
    pub const fn clk_pll_config(
        self,
        n: usize,
    ) -> crate::common::Reg<ClkPllConfig, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1536usize + n * 4usize) as _) }
    }
    #[doc = "PLL Status Register"]
    #[inline(always)]
    pub const fn clk_pll_status(
        self,
        n: usize,
    ) -> crate::common::Reg<ClkPllStatus, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1600usize + n * 4usize) as _) }
    }
    #[doc = "SRSS Interrupt Register"]
    #[inline(always)]
    pub const fn srss_intr(self) -> crate::common::Reg<SrssIntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1792usize) as _) }
    }
    #[doc = "SRSS Interrupt Set Register"]
    #[inline(always)]
    pub const fn srss_intr_set(self) -> crate::common::Reg<SrssIntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1796usize) as _) }
    }
    #[doc = "SRSS Interrupt Mask Register"]
    #[inline(always)]
    pub const fn srss_intr_mask(self) -> crate::common::Reg<SrssIntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1800usize) as _) }
    }
    #[doc = "SRSS Interrupt Masked Register"]
    #[inline(always)]
    pub const fn srss_intr_masked(self) -> crate::common::Reg<SrssIntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1804usize) as _) }
    }
    #[doc = "SRSS Interrupt Configuration Register"]
    #[inline(always)]
    pub const fn srss_intr_cfg(self) -> crate::common::Reg<SrssIntrCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1808usize) as _) }
    }
    #[doc = "Reset Cause Observation Register"]
    #[inline(always)]
    pub const fn res_cause(self) -> crate::common::Reg<ResCause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2048usize) as _) }
    }
    #[doc = "Reset Cause Observation Register 2"]
    #[inline(always)]
    pub const fn res_cause2(self) -> crate::common::Reg<ResCause2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2052usize) as _) }
    }
    #[doc = "Reference Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_ref_ctl(self) -> crate::common::Reg<PwrTrimRefCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32512usize) as _) }
    }
    #[doc = "BOD/OVP Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_bodovp_ctl(
        self,
    ) -> crate::common::Reg<PwrTrimBodovpCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32516usize) as _) }
    }
    #[doc = "CCO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_cco_ctl(self) -> crate::common::Reg<ClkTrimCcoCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32520usize) as _) }
    }
    #[doc = "CCO Trim Register 2"]
    #[inline(always)]
    pub const fn clk_trim_cco_ctl2(self) -> crate::common::Reg<ClkTrimCcoCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32524usize) as _) }
    }
    #[doc = "Wakeup Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_wake_ctl(self) -> crate::common::Reg<PwrTrimWakeCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32560usize) as _) }
    }
    #[doc = "LVD Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_lvd_ctl(self) -> crate::common::Reg<PwrTrimLvdCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65296usize) as _) }
    }
    #[doc = "ILO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_ilo_ctl(self) -> crate::common::Reg<ClkTrimIloCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65304usize) as _) }
    }
    #[doc = "Power System Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_pwrsys_ctl(
        self,
    ) -> crate::common::Reg<PwrTrimPwrsysCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65308usize) as _) }
    }
    #[doc = "ECO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_eco_ctl(self) -> crate::common::Reg<ClkTrimEcoCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65312usize) as _) }
    }
    #[doc = "PILO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_pilo_ctl(self) -> crate::common::Reg<ClkTrimPiloCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65316usize) as _) }
    }
    #[doc = "PILO Trim Register 2"]
    #[inline(always)]
    pub const fn clk_trim_pilo_ctl2(
        self,
    ) -> crate::common::Reg<ClkTrimPiloCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65320usize) as _) }
    }
    #[doc = "PILO Trim Register 3"]
    #[inline(always)]
    pub const fn clk_trim_pilo_ctl3(
        self,
    ) -> crate::common::Reg<ClkTrimPiloCtl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65324usize) as _) }
    }
}
#[doc = "Clock Calibration Counter 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCalCnt1(pub u32);
impl ClkCalCnt1 {
    #[doc = "Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
    #[inline(always)]
    pub const fn cal_counter1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
    #[inline(always)]
    pub fn set_cal_counter1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Status bit indicating that the internal counter #1 is finished counting and CLK_CAL_CNT2.COUNTER stopped counting up"]
    #[inline(always)]
    pub const fn cal_counter_done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit indicating that the internal counter #1 is finished counting and CLK_CAL_CNT2.COUNTER stopped counting up"]
    #[inline(always)]
    pub fn set_cal_counter_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkCalCnt1 {
    #[inline(always)]
    fn default() -> ClkCalCnt1 {
        ClkCalCnt1(0)
    }
}
#[doc = "Clock Calibration Counter 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCalCnt2(pub u32);
impl ClkCalCnt2 {
    #[doc = "Up-counter clocked on fast clock output #1 (see CLK_OUTPUT_FAST). When CLK_CAL_CNT1.CAL_COUNTER_DONE==1, the counter is stopped and can be read by SW. Do not read this value unless CAL_COUNTER_DONE==1. The expected final value is related to the ratio of clock frequencies used for the two counters and the value loaded into counter 1: CLK_CAL_CNT2.COUNTER=(F_cnt2/F_cnt1)*(CLK_CAL_CNT1.COUNTER)"]
    #[inline(always)]
    pub const fn cal_counter2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Up-counter clocked on fast clock output #1 (see CLK_OUTPUT_FAST). When CLK_CAL_CNT1.CAL_COUNTER_DONE==1, the counter is stopped and can be read by SW. Do not read this value unless CAL_COUNTER_DONE==1. The expected final value is related to the ratio of clock frequencies used for the two counters and the value loaded into counter 1: CLK_CAL_CNT2.COUNTER=(F_cnt2/F_cnt1)*(CLK_CAL_CNT1.COUNTER)"]
    #[inline(always)]
    pub fn set_cal_counter2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for ClkCalCnt2 {
    #[inline(always)]
    fn default() -> ClkCalCnt2 {
        ClkCalCnt2(0)
    }
}
#[doc = "Clock DSI Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkDsiSelect(pub u32);
impl ClkDsiSelect {
    #[doc = "Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH<i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
    #[inline(always)]
    pub const fn dsi_mux(&self) -> DsiMux {
        let val = (self.0 >> 0usize) & 0x1f;
        DsiMux::from_bits(val as u8)
    }
    #[doc = "Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH<i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
    #[inline(always)]
    pub fn set_dsi_mux(&mut self, val: DsiMux) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for ClkDsiSelect {
    #[inline(always)]
    fn default() -> ClkDsiSelect {
        ClkDsiSelect(0)
    }
}
#[doc = "ECO Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkEcoConfig(pub u32);
impl ClkEcoConfig {
    #[doc = "Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    pub const fn agc_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    pub fn set_agc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Master enable for ECO oscillator."]
    #[inline(always)]
    pub const fn eco_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for ECO oscillator."]
    #[inline(always)]
    pub fn set_eco_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkEcoConfig {
    #[inline(always)]
    fn default() -> ClkEcoConfig {
        ClkEcoConfig(0)
    }
}
#[doc = "ECO Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkEcoStatus(pub u32);
impl ClkEcoStatus {
    #[doc = "Indicates the ECO internal oscillator circuit has sufficient amplitude. It may not meet the PPM accuracy or duty cycle spec."]
    #[inline(always)]
    pub const fn eco_ok(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the ECO internal oscillator circuit has sufficient amplitude. It may not meet the PPM accuracy or duty cycle spec."]
    #[inline(always)]
    pub fn set_eco_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the ECO internal oscillator circuit has had enough time to fully stabilize. This is the output of a counter since ECO was enabled, and it does not check the ECO output. It is recommended to also confirm ECO_OK==1."]
    #[inline(always)]
    pub const fn eco_ready(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the ECO internal oscillator circuit has had enough time to fully stabilize. This is the output of a counter since ECO was enabled, and it does not check the ECO output. It is recommended to also confirm ECO_OK==1."]
    #[inline(always)]
    pub fn set_eco_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for ClkEcoStatus {
    #[inline(always)]
    fn default() -> ClkEcoStatus {
        ClkEcoStatus(0)
    }
}
#[doc = "FLL Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFllConfig(pub u32);
impl ClkFllConfig {
    #[doc = "Multiplier to determine CCO frequency in multiples of the frequency of the selected reference clock (Fref). Ffll = (FLL_MULT) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV+1)"]
    #[inline(always)]
    pub const fn fll_mult(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Multiplier to determine CCO frequency in multiples of the frequency of the selected reference clock (Fref). Ffll = (FLL_MULT) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV+1)"]
    #[inline(always)]
    pub fn set_fll_mult(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Control bits for Output divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: no division 1: divide by 2"]
    #[inline(always)]
    pub const fn fll_output_div(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Control bits for Output divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: no division 1: divide by 2"]
    #[inline(always)]
    pub fn set_fll_output_div(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Master enable for FLL. The FLL requires firmware sequencing when enabling, disabling, and entering/exiting DEEPSLEEP. To enable the FLL, first enable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=1 and wait until CLK_FLL_STATUS.CCO_READY==1. Next, ensure the reference clock has stabilized and CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF. Next, write FLL_ENABLE=1 and wait until CLK_FLL_STATUS.LOCKED==1. Finally, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. It takes seven reference clock cycles plus four FLL output cycles to switch to the FLL output. Do not disable the FLL before this time completes. To disable the FLL, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF and (optionally) read the same register to ensure the write completes. Then, wait at least seven FLL reference clock cycles before disabling it with FLL_ENABLE=0. Lastly, disable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=0. Before entering DEEPSLEEP, either disable the FLL using above sequence or use the following procedure to deselect/select it before/after DEEPSLEEP. Before entering DEEPSLEEP, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF to change the FLL to use its reference clock. After DEEPSLEEP wakeup, wait until CLK_FLL_STATUS.LOCKED==1 and then write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub const fn fll_enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for FLL. The FLL requires firmware sequencing when enabling, disabling, and entering/exiting DEEPSLEEP. To enable the FLL, first enable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=1 and wait until CLK_FLL_STATUS.CCO_READY==1. Next, ensure the reference clock has stabilized and CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF. Next, write FLL_ENABLE=1 and wait until CLK_FLL_STATUS.LOCKED==1. Finally, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. It takes seven reference clock cycles plus four FLL output cycles to switch to the FLL output. Do not disable the FLL before this time completes. To disable the FLL, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF and (optionally) read the same register to ensure the write completes. Then, wait at least seven FLL reference clock cycles before disabling it with FLL_ENABLE=0. Lastly, disable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=0. Before entering DEEPSLEEP, either disable the FLL using above sequence or use the following procedure to deselect/select it before/after DEEPSLEEP. Before entering DEEPSLEEP, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF to change the FLL to use its reference clock. After DEEPSLEEP wakeup, wait until CLK_FLL_STATUS.LOCKED==1 and then write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn set_fll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkFllConfig {
    #[inline(always)]
    fn default() -> ClkFllConfig {
        ClkFllConfig(0)
    }
}
#[doc = "FLL Configuration Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFllConfig2(pub u32);
impl ClkFllConfig2 {
    #[doc = "Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
    #[inline(always)]
    pub const fn fll_ref_div(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
    #[inline(always)]
    pub fn set_fll_ref_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
    #[inline(always)]
    pub const fn lock_tol(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
    #[inline(always)]
    pub fn set_lock_tol(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
}
impl Default for ClkFllConfig2 {
    #[inline(always)]
    fn default() -> ClkFllConfig2 {
        ClkFllConfig2(0)
    }
}
#[doc = "FLL Configuration Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFllConfig3(pub u32);
impl ClkFllConfig3 {
    #[doc = "FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub const fn fll_lf_igain(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn set_fll_lf_igain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub const fn fll_lf_pgain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn set_fll_lf_pgain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    pub const fn settling_count(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x1fff;
        val as u16
    }
    #[doc = "Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    pub fn set_settling_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 8usize)) | (((val as u32) & 0x1fff) << 8usize);
    }
    #[doc = "Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
    #[inline(always)]
    pub const fn bypass_sel(&self) -> ClkFllConfig3BypassSel {
        let val = (self.0 >> 28usize) & 0x03;
        ClkFllConfig3BypassSel::from_bits(val as u8)
    }
    #[doc = "Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
    #[inline(always)]
    pub fn set_bypass_sel(&mut self, val: ClkFllConfig3BypassSel) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for ClkFllConfig3 {
    #[inline(always)]
    fn default() -> ClkFllConfig3 {
        ClkFllConfig3(0)
    }
}
#[doc = "FLL Configuration Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFllConfig4(pub u32);
impl ClkFllConfig4 {
    #[doc = "Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    pub const fn cco_limit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn set_cco_limit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Frequency range of CCO"]
    #[inline(always)]
    pub const fn cco_range(&self) -> CcoRange {
        let val = (self.0 >> 8usize) & 0x07;
        CcoRange::from_bits(val as u8)
    }
    #[doc = "Frequency range of CCO"]
    #[inline(always)]
    pub fn set_cco_range(&mut self, val: CcoRange) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    pub const fn cco_freq(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    pub fn set_cco_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    pub const fn cco_hw_update_dis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    pub fn set_cco_hw_update_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub const fn cco_enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn set_cco_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkFllConfig4 {
    #[inline(always)]
    fn default() -> ClkFllConfig4 {
        ClkFllConfig4(0)
    }
}
#[doc = "FLL Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFllStatus(pub u32);
impl ClkFllStatus {
    #[doc = "FLL Lock Indicator. LOCKED is high when FLL is within CLK_FLL_CONFIG2.LOCK_TOL. If FLL is outside LOCK_TOL, LOCKED goes low. Note that this can happen during normal operation, if FLL needs to recalculate due to a change in the reference clock, change in voltage, or change in temperature."]
    #[inline(always)]
    pub const fn locked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLL Lock Indicator. LOCKED is high when FLL is within CLK_FLL_CONFIG2.LOCK_TOL. If FLL is outside LOCK_TOL, LOCKED goes low. Note that this can happen during normal operation, if FLL needs to recalculate due to a change in the reference clock, change in voltage, or change in temperature."]
    #[inline(always)]
    pub fn set_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn unlock_occurred(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_unlock_occurred(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This indicates that the CCO is internally settled and ready to use."]
    #[inline(always)]
    pub const fn cco_ready(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This indicates that the CCO is internally settled and ready to use."]
    #[inline(always)]
    pub fn set_cco_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for ClkFllStatus {
    #[inline(always)]
    fn default() -> ClkFllStatus {
        ClkFllStatus(0)
    }
}
#[doc = "ILO Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkIloConfig(pub u32);
impl ClkIloConfig {
    #[doc = "If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    pub const fn ilo_backup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    pub fn set_ilo_backup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkIloConfig {
    #[inline(always)]
    fn default() -> ClkIloConfig {
        ClkIloConfig(0)
    }
}
#[doc = "IMO Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkImoConfig(pub u32);
impl ClkImoConfig {
    #[doc = "Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkImoConfig {
    #[inline(always)]
    fn default() -> ClkImoConfig {
        ClkImoConfig(0)
    }
}
#[doc = "Fast Clock Output Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkOutputFast(pub u32);
impl ClkOutputFast {
    #[doc = "Select signal for fast clock output #0"]
    #[inline(always)]
    pub const fn fast_sel0(&self) -> FastSel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        FastSel0::from_bits(val as u8)
    }
    #[doc = "Select signal for fast clock output #0"]
    #[inline(always)]
    pub fn set_fast_sel0(&mut self, val: FastSel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub const fn path_sel0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn set_path_sel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Selects a HFCLK tree for use in fast clock output #0"]
    #[inline(always)]
    pub const fn hfclk_sel0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects a HFCLK tree for use in fast clock output #0"]
    #[inline(always)]
    pub fn set_hfclk_sel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Select signal for fast clock output #1"]
    #[inline(always)]
    pub const fn fast_sel1(&self) -> FastSel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        FastSel1::from_bits(val as u8)
    }
    #[doc = "Select signal for fast clock output #1"]
    #[inline(always)]
    pub fn set_fast_sel1(&mut self, val: FastSel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub const fn path_sel1(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn set_path_sel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Selects a HFCLK tree for use in fast clock output #1 logic"]
    #[inline(always)]
    pub const fn hfclk_sel1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects a HFCLK tree for use in fast clock output #1 logic"]
    #[inline(always)]
    pub fn set_hfclk_sel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for ClkOutputFast {
    #[inline(always)]
    fn default() -> ClkOutputFast {
        ClkOutputFast(0)
    }
}
#[doc = "Slow Clock Output Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkOutputSlow(pub u32);
impl ClkOutputSlow {
    #[doc = "Select signal for slow clock output #0"]
    #[inline(always)]
    pub const fn slow_sel0(&self) -> SlowSel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        SlowSel0::from_bits(val as u8)
    }
    #[doc = "Select signal for slow clock output #0"]
    #[inline(always)]
    pub fn set_slow_sel0(&mut self, val: SlowSel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select signal for slow clock output #1"]
    #[inline(always)]
    pub const fn slow_sel1(&self) -> SlowSel1 {
        let val = (self.0 >> 4usize) & 0x0f;
        SlowSel1::from_bits(val as u8)
    }
    #[doc = "Select signal for slow clock output #1"]
    #[inline(always)]
    pub fn set_slow_sel1(&mut self, val: SlowSel1) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
}
impl Default for ClkOutputSlow {
    #[inline(always)]
    fn default() -> ClkOutputSlow {
        ClkOutputSlow(0)
    }
}
#[doc = "Clock Path Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkPathSelect(pub u32);
impl ClkPathSelect {
    #[doc = "Selects a source for clock PATH<i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub const fn path_mux(&self) -> PathMux {
        let val = (self.0 >> 0usize) & 0x07;
        PathMux::from_bits(val as u8)
    }
    #[doc = "Selects a source for clock PATH<i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn set_path_mux(&mut self, val: PathMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for ClkPathSelect {
    #[inline(always)]
    fn default() -> ClkPathSelect {
        ClkPathSelect(0)
    }
}
#[doc = "Precision ILO Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkPiloConfig(pub u32);
impl ClkPiloConfig {
    #[doc = "Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub const fn pilo_ffreq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub fn set_pilo_ffreq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub const fn pilo_clk_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn set_pilo_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub const fn pilo_reset_n(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn set_pilo_reset_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub const fn pilo_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub fn set_pilo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkPiloConfig {
    #[inline(always)]
    fn default() -> ClkPiloConfig {
        ClkPiloConfig(0)
    }
}
#[doc = "PLL Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkPllConfig(pub u32);
impl ClkPllConfig {
    #[doc = "Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
    #[inline(always)]
    pub const fn feedback_div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn set_feedback_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
    #[inline(always)]
    pub const fn reference_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn set_reference_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    pub const fn output_div(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn set_output_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\] 1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    pub const fn pll_lf_mode(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\] 1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    pub fn set_pll_lf_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    pub const fn bypass_sel(&self) -> ClkPllConfigBypassSel {
        let val = (self.0 >> 28usize) & 0x03;
        ClkPllConfigBypassSel::from_bits(val as u8)
    }
    #[doc = "Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    pub fn set_bypass_sel(&mut self, val: ClkPllConfigBypassSel) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkPllConfig {
    #[inline(always)]
    fn default() -> ClkPllConfig {
        ClkPllConfig(0)
    }
}
#[doc = "PLL Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkPllStatus(pub u32);
impl ClkPllStatus {
    #[doc = "PLL Lock Indicator"]
    #[inline(always)]
    pub const fn locked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Lock Indicator"]
    #[inline(always)]
    pub fn set_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub const fn unlock_occurred(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn set_unlock_occurred(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for ClkPllStatus {
    #[inline(always)]
    fn default() -> ClkPllStatus {
        ClkPllStatus(0)
    }
}
#[doc = "Clock Root Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRootSelect(pub u32);
impl ClkRootSelect {
    #[doc = "Selects a clock path as the root of HFCLK<k> and for SRSS DSI input <k>. Use CLK_SELECT_PATH\\[i\\] to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\] register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub const fn root_mux(&self) -> RootMux {
        let val = (self.0 >> 0usize) & 0x0f;
        RootMux::from_bits(val as u8)
    }
    #[doc = "Selects a clock path as the root of HFCLK<k> and for SRSS DSI input <k>. Use CLK_SELECT_PATH\\[i\\] to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\] register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn set_root_mux(&mut self, val: RootMux) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects predivider value for this clock root and DSI input."]
    #[inline(always)]
    pub const fn root_div(&self) -> RootDiv {
        let val = (self.0 >> 4usize) & 0x03;
        RootDiv::from_bits(val as u8)
    }
    #[doc = "Selects predivider value for this clock root and DSI input."]
    #[inline(always)]
    pub fn set_root_div(&mut self, val: RootDiv) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkRootSelect {
    #[inline(always)]
    fn default() -> ClkRootSelect {
        ClkRootSelect(0)
    }
}
#[doc = "Clock selection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSelect(pub u32);
impl ClkSelect {
    #[doc = "Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub const fn lfclk_sel(&self) -> LfclkSel {
        let val = (self.0 >> 0usize) & 0x03;
        LfclkSel::from_bits(val as u8)
    }
    #[doc = "Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub fn set_lfclk_sel(&mut self, val: LfclkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects clock PATH<k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    pub const fn pump_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects clock PATH<k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    pub fn set_pump_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    pub const fn pump_div(&self) -> PumpDiv {
        let val = (self.0 >> 12usize) & 0x07;
        PumpDiv::from_bits(val as u8)
    }
    #[doc = "Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    pub fn set_pump_div(&mut self, val: PumpDiv) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    pub const fn pump_enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    pub fn set_pump_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for ClkSelect {
    #[inline(always)]
    fn default() -> ClkSelect {
        ClkSelect(0)
    }
}
#[doc = "Timer Clock Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTimerCtl(pub u32);
impl ClkTimerCtl {
    #[doc = "Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
    #[inline(always)]
    pub const fn timer_sel(&self) -> TimerSel {
        let val = (self.0 >> 0usize) & 0x01;
        TimerSel::from_bits(val as u8)
    }
    #[doc = "Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
    #[inline(always)]
    pub fn set_timer_sel(&mut self, val: TimerSel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
    #[inline(always)]
    pub const fn timer_hf0_div(&self) -> TimerHf0Div {
        let val = (self.0 >> 8usize) & 0x03;
        TimerHf0Div::from_bits(val as u8)
    }
    #[doc = "Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
    #[inline(always)]
    pub fn set_timer_hf0_div(&mut self, val: TimerHf0Div) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
    #[inline(always)]
    pub const fn timer_div(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
    #[inline(always)]
    pub fn set_timer_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkTimerCtl {
    #[inline(always)]
    fn default() -> ClkTimerCtl {
        ClkTimerCtl(0)
    }
}
#[doc = "CCO Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTrimCcoCtl(pub u32);
impl ClkTrimCcoCtl {
    #[doc = "CCO reference current source trim."]
    #[inline(always)]
    pub const fn cco_rcstrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO reference current source trim."]
    #[inline(always)]
    pub fn set_cco_rcstrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub const fn cco_stable_cnt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub fn set_cco_stable_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Enables the automatic stabilization counter."]
    #[inline(always)]
    pub const fn enable_cnt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the automatic stabilization counter."]
    #[inline(always)]
    pub fn set_enable_cnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkTrimCcoCtl {
    #[inline(always)]
    fn default() -> ClkTrimCcoCtl {
        ClkTrimCcoCtl(0)
    }
}
#[doc = "CCO Trim Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTrimCcoCtl2(pub u32);
impl ClkTrimCcoCtl2 {
    #[doc = "CCO frequency 1st range calibration"]
    #[inline(always)]
    pub const fn cco_fctrim1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "CCO frequency 1st range calibration"]
    #[inline(always)]
    pub fn set_cco_fctrim1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "CCO frequency 2nd range calibration"]
    #[inline(always)]
    pub const fn cco_fctrim2(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "CCO frequency 2nd range calibration"]
    #[inline(always)]
    pub fn set_cco_fctrim2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "CCO frequency 3rd range calibration"]
    #[inline(always)]
    pub const fn cco_fctrim3(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "CCO frequency 3rd range calibration"]
    #[inline(always)]
    pub fn set_cco_fctrim3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "CCO frequency 4th range calibration"]
    #[inline(always)]
    pub const fn cco_fctrim4(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "CCO frequency 4th range calibration"]
    #[inline(always)]
    pub fn set_cco_fctrim4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "CCO frequency 5th range calibration"]
    #[inline(always)]
    pub const fn cco_fctrim5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "CCO frequency 5th range calibration"]
    #[inline(always)]
    pub fn set_cco_fctrim5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
}
impl Default for ClkTrimCcoCtl2 {
    #[inline(always)]
    fn default() -> ClkTrimCcoCtl2 {
        ClkTrimCcoCtl2(0)
    }
}
#[doc = "ECO Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTrimEcoCtl(pub u32);
impl ClkTrimEcoCtl {
    #[doc = "Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    pub const fn wdtrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    pub fn set_wdtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    pub const fn atrim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    pub fn set_atrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    pub const fn ftrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    pub fn set_ftrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Feedback resistor Trim"]
    #[inline(always)]
    pub const fn rtrim(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Feedback resistor Trim"]
    #[inline(always)]
    pub fn set_rtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Gain Trim - Startup time"]
    #[inline(always)]
    pub const fn gtrim(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Gain Trim - Startup time"]
    #[inline(always)]
    pub fn set_gtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Current Trim"]
    #[inline(always)]
    pub const fn itrim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Current Trim"]
    #[inline(always)]
    pub fn set_itrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for ClkTrimEcoCtl {
    #[inline(always)]
    fn default() -> ClkTrimEcoCtl {
        ClkTrimEcoCtl(0)
    }
}
#[doc = "ILO Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTrimIloCtl(pub u32);
impl ClkTrimIloCtl {
    #[doc = "ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    pub const fn ilo_ftrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    pub fn set_ilo_ftrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for ClkTrimIloCtl {
    #[inline(always)]
    fn default() -> ClkTrimIloCtl {
        ClkTrimIloCtl(0)
    }
}
#[doc = "PILO Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTrimPiloCtl(pub u32);
impl ClkTrimPiloCtl {
    #[doc = "Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    pub const fn pilo_cfreq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    pub fn set_pilo_cfreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Trim for current in oscillator block."]
    #[inline(always)]
    pub const fn pilo_osc_trim(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Trim for current in oscillator block."]
    #[inline(always)]
    pub fn set_pilo_osc_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trim for comparator bias current."]
    #[inline(always)]
    pub const fn pilo_comp_trim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Trim for comparator bias current."]
    #[inline(always)]
    pub fn set_pilo_comp_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    pub const fn pilo_nbias_trim(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    pub fn set_pilo_nbias_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Trim for beta-multiplier branch current"]
    #[inline(always)]
    pub const fn pilo_res_trim(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for beta-multiplier branch current"]
    #[inline(always)]
    pub fn set_pilo_res_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[doc = "Trim for beta-multiplier current slope"]
    #[inline(always)]
    pub const fn pilo_islope_trim(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Trim for beta-multiplier current slope"]
    #[inline(always)]
    pub fn set_pilo_islope_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    pub const fn pilo_vtdiff_trim(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    pub fn set_pilo_vtdiff_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for ClkTrimPiloCtl {
    #[inline(always)]
    fn default() -> ClkTrimPiloCtl {
        ClkTrimPiloCtl(0)
    }
}
#[doc = "PILO Trim Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTrimPiloCtl2(pub u32);
impl ClkTrimPiloCtl2 {
    #[doc = "Trim for voltage reference"]
    #[inline(always)]
    pub const fn pilo_vref_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim for voltage reference"]
    #[inline(always)]
    pub fn set_pilo_vref_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim for beta-multiplier current reference"]
    #[inline(always)]
    pub const fn pilo_irefbm_trim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for beta-multiplier current reference"]
    #[inline(always)]
    pub fn set_pilo_irefbm_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Trim for current reference"]
    #[inline(always)]
    pub const fn pilo_iref_trim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Trim for current reference"]
    #[inline(always)]
    pub fn set_pilo_iref_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ClkTrimPiloCtl2 {
    #[inline(always)]
    fn default() -> ClkTrimPiloCtl2 {
        ClkTrimPiloCtl2(0)
    }
}
#[doc = "PILO Trim Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTrimPiloCtl3(pub u32);
impl ClkTrimPiloCtl3 {
    #[doc = "Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
    #[inline(always)]
    pub const fn pilo_engopt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
    #[inline(always)]
    pub fn set_pilo_engopt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ClkTrimPiloCtl3 {
    #[inline(always)]
    fn default() -> ClkTrimPiloCtl3 {
        ClkTrimPiloCtl3(0)
    }
}
#[doc = "Multi-Counter Watchdog Sub-counter 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtCnthigh(pub u32);
impl McwdtCnthigh {
    #[doc = "Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub const fn wdt_ctr2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn set_wdt_ctr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for McwdtCnthigh {
    #[inline(always)]
    fn default() -> McwdtCnthigh {
        McwdtCnthigh(0)
    }
}
#[doc = "Multi-Counter Watchdog Sub-counters 0/1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtCntlow(pub u32);
impl McwdtCntlow {
    #[doc = "Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    pub const fn wdt_ctr0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    pub fn set_wdt_ctr0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub const fn wdt_ctr1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn set_wdt_ctr1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for McwdtCntlow {
    #[inline(always)]
    fn default() -> McwdtCntlow {
        McwdtCntlow(0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtConfig(pub u32);
impl McwdtConfig {
    #[doc = "Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
    #[inline(always)]
    pub const fn wdt_mode0(&self) -> WdtMode0 {
        let val = (self.0 >> 0usize) & 0x03;
        WdtMode0::from_bits(val as u8)
    }
    #[doc = "Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR0=WDT_MATCH0)."]
    #[inline(always)]
    pub fn set_wdt_mode0(&mut self, val: WdtMode0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
    #[inline(always)]
    pub const fn wdt_clear0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH0 is 1."]
    #[inline(always)]
    pub fn set_wdt_clear0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
    #[inline(always)]
    pub const fn wdt_cascade0_1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
    #[inline(always)]
    pub fn set_wdt_cascade0_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
    #[inline(always)]
    pub const fn wdt_mode1(&self) -> WdtMode1 {
        let val = (self.0 >> 8usize) & 0x03;
        WdtMode1::from_bits(val as u8)
    }
    #[doc = "Watchdog Counter Action on Match. Action is taken on the next increment after the values match (WDT_CTR1=WDT_MATCH1)."]
    #[inline(always)]
    pub fn set_wdt_mode1(&mut self, val: WdtMode1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
    #[inline(always)]
    pub const fn wdt_clear1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Watchdog Counter when WDT_CTR1==WDT_MATCH1. In other words WDT_CTR1 divides LFCLK by (WDT_MATCH1+1). 0: Free running counter 1: Clear on match. In this mode, the minimum legal setting of WDT_MATCH1 is 1."]
    #[inline(always)]
    pub fn set_wdt_clear1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
    #[inline(always)]
    pub const fn wdt_cascade1_2(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1."]
    #[inline(always)]
    pub fn set_wdt_cascade1_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Watchdog Counter 2 Mode."]
    #[inline(always)]
    pub const fn wdt_mode2(&self) -> WdtMode2 {
        let val = (self.0 >> 16usize) & 0x01;
        WdtMode2::from_bits(val as u8)
    }
    #[doc = "Watchdog Counter 2 Mode."]
    #[inline(always)]
    pub fn set_wdt_mode2(&mut self, val: WdtMode2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
    #[inline(always)]
    pub const fn wdt_bits2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit to observe for WDT_INT2: 0: Assert after bit0 of WDT_CTR2 toggles (one int every tick) ... 31: Assert after bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
    #[inline(always)]
    pub fn set_wdt_bits2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for McwdtConfig {
    #[inline(always)]
    fn default() -> McwdtConfig {
        McwdtConfig(0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtCtl(pub u32);
impl McwdtCtl {
    #[doc = "Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub const fn wdt_enable0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn set_wdt_enable0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates actual state of counter. May lag WDT_ENABLE0 by up to two LFCLK cycles."]
    #[inline(always)]
    pub const fn wdt_enabled0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates actual state of counter. May lag WDT_ENABLE0 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn set_wdt_enabled0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub const fn wdt_reset0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn set_wdt_reset0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub const fn wdt_enable1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn set_wdt_enable1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates actual state of counter. May lag WDT_ENABLE1 by up to two LFCLK cycles."]
    #[inline(always)]
    pub const fn wdt_enabled1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates actual state of counter. May lag WDT_ENABLE1 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn set_wdt_enabled1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub const fn wdt_reset1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn set_wdt_reset1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub const fn wdt_enable2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn set_wdt_enable2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates actual state of counter. May lag WDT_ENABLE2 by up to two LFCLK cycles."]
    #[inline(always)]
    pub const fn wdt_enabled2(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates actual state of counter. May lag WDT_ENABLE2 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn set_wdt_enabled2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub const fn wdt_reset2(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn set_wdt_reset2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for McwdtCtl {
    #[inline(always)]
    fn default() -> McwdtCtl {
        McwdtCtl(0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtIntr(pub u32);
impl McwdtIntr {
    #[doc = "MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
    #[inline(always)]
    pub const fn mcwdt_int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
    #[inline(always)]
    pub fn set_mcwdt_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
    #[inline(always)]
    pub const fn mcwdt_int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
    #[inline(always)]
    pub fn set_mcwdt_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
    #[inline(always)]
    pub const fn mcwdt_int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
    #[inline(always)]
    pub fn set_mcwdt_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for McwdtIntr {
    #[inline(always)]
    fn default() -> McwdtIntr {
        McwdtIntr(0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtIntrMask(pub u32);
impl McwdtIntrMask {
    #[doc = "Mask for sub-counter 0"]
    #[inline(always)]
    pub const fn mcwdt_int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for sub-counter 0"]
    #[inline(always)]
    pub fn set_mcwdt_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask for sub-counter 1"]
    #[inline(always)]
    pub const fn mcwdt_int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for sub-counter 1"]
    #[inline(always)]
    pub fn set_mcwdt_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask for sub-counter 2"]
    #[inline(always)]
    pub const fn mcwdt_int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for sub-counter 2"]
    #[inline(always)]
    pub fn set_mcwdt_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for McwdtIntrMask {
    #[inline(always)]
    fn default() -> McwdtIntrMask {
        McwdtIntrMask(0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Masked Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtIntrMasked(pub u32);
impl McwdtIntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn mcwdt_int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_mcwdt_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn mcwdt_int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_mcwdt_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn mcwdt_int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_mcwdt_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for McwdtIntrMasked {
    #[inline(always)]
    fn default() -> McwdtIntrMasked {
        McwdtIntrMasked(0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtIntrSet(pub u32);
impl McwdtIntrSet {
    #[doc = "Set interrupt for MCWDT_INT0"]
    #[inline(always)]
    pub const fn mcwdt_int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set interrupt for MCWDT_INT0"]
    #[inline(always)]
    pub fn set_mcwdt_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set interrupt for MCWDT_INT1"]
    #[inline(always)]
    pub const fn mcwdt_int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set interrupt for MCWDT_INT1"]
    #[inline(always)]
    pub fn set_mcwdt_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set interrupt for MCWDT_INT2"]
    #[inline(always)]
    pub const fn mcwdt_int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set interrupt for MCWDT_INT2"]
    #[inline(always)]
    pub fn set_mcwdt_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for McwdtIntrSet {
    #[inline(always)]
    fn default() -> McwdtIntrSet {
        McwdtIntrSet(0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Lock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtLockReg(pub u32);
impl McwdtLockReg {
    #[doc = "Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    pub const fn mcwdt_lock(&self) -> McwdtLock {
        let val = (self.0 >> 30usize) & 0x03;
        McwdtLock::from_bits(val as u8)
    }
    #[doc = "Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    pub fn set_mcwdt_lock(&mut self, val: McwdtLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for McwdtLockReg {
    #[inline(always)]
    fn default() -> McwdtLockReg {
        McwdtLockReg(0)
    }
}
#[doc = "Multi-Counter Watchdog Counter Match Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McwdtMatch(pub u32);
impl McwdtMatch {
    #[doc = "Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    pub const fn wdt_match0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    pub fn set_wdt_match0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    pub const fn wdt_match1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    pub fn set_wdt_match1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for McwdtMatch {
    #[inline(always)]
    fn default() -> McwdtMatch {
        McwdtMatch(0)
    }
}
#[doc = "Buck Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrBuckCtl(pub u32);
impl PwrBuckCtl {
    #[doc = "Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    pub const fn buck_out1_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    pub fn set_buck_out1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn buck_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_buck_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
    #[inline(always)]
    pub const fn buck_out1_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
    #[inline(always)]
    pub fn set_buck_out1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PwrBuckCtl {
    #[inline(always)]
    fn default() -> PwrBuckCtl {
        PwrBuckCtl(0)
    }
}
#[doc = "Buck Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrBuckCtl2(pub u32);
impl PwrBuckCtl2 {
    #[doc = "Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
    #[inline(always)]
    pub const fn buck_out2_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
    #[inline(always)]
    pub fn set_buck_out2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
    #[inline(always)]
    pub const fn buck_out2_hw_sel(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
    #[inline(always)]
    pub fn set_buck_out2_hw_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
    #[inline(always)]
    pub const fn buck_out2_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
    #[inline(always)]
    pub fn set_buck_out2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PwrBuckCtl2 {
    #[inline(always)]
    fn default() -> PwrBuckCtl2 {
        PwrBuckCtl2(0)
    }
}
#[doc = "Power Mode Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrCtl(pub u32);
impl PwrCtl {
    #[doc = "Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
    #[inline(always)]
    pub const fn power_mode(&self) -> PowerMode {
        let val = (self.0 >> 0usize) & 0x03;
        PowerMode::from_bits(val as u8)
    }
    #[doc = "Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
    #[inline(always)]
    pub fn set_power_mode(&mut self, val: PowerMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
    #[inline(always)]
    pub const fn debug_session(&self) -> DebugSession {
        let val = (self.0 >> 4usize) & 0x01;
        DebugSession::from_bits(val as u8)
    }
    #[doc = "Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
    #[inline(always)]
    pub fn set_debug_session(&mut self, val: DebugSession) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates whether certain low power functions are ready. The low current circuits take longer to startup after XRES/POR/BOD/HIBERNATE wakeup than the normal mode circuits. HIBERNATE mode may be entered regardless of this bit. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: If a low power circuit operation is requested, it will stay in its normal operating mode until it is ready. If DEEPSLEEP is requested by all processors WFI/WFE, the device will instead enter SLEEP. When low power circuits are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP and low power circuits operate as requested in other registers."]
    #[inline(always)]
    pub const fn lpm_ready(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether certain low power functions are ready. The low current circuits take longer to startup after XRES/POR/BOD/HIBERNATE wakeup than the normal mode circuits. HIBERNATE mode may be entered regardless of this bit. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: If a low power circuit operation is requested, it will stay in its normal operating mode until it is ready. If DEEPSLEEP is requested by all processors WFI/WFE, the device will instead enter SLEEP. When low power circuits are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP and low power circuits operate as requested in other registers."]
    #[inline(always)]
    pub fn set_lpm_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub const fn iref_lpmode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn set_iref_lpmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Indicates that the voltage reference buffer is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting VREFBUF_DIS=1."]
    #[inline(always)]
    pub const fn vrefbuf_ok(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the voltage reference buffer is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting VREFBUF_DIS=1."]
    #[inline(always)]
    pub fn set_vrefbuf_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
    #[inline(always)]
    pub const fn dpslp_reg_dis(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
    #[inline(always)]
    pub fn set_dpslp_reg_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    pub const fn ret_reg_dis(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    pub fn set_ret_reg_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    pub const fn nwell_reg_dis(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    pub fn set_nwell_reg_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
    #[inline(always)]
    pub const fn linreg_dis(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
    #[inline(always)]
    pub fn set_linreg_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    pub const fn linreg_lpmode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    pub fn set_linreg_lpmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub const fn porbod_lpmode(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn set_porbod_lpmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
    #[inline(always)]
    pub const fn bgref_lpmode(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
    #[inline(always)]
    pub fn set_bgref_lpmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    pub const fn pll_ls_bypass(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    pub fn set_pll_ls_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub const fn vrefbuf_lpmode(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn set_vrefbuf_lpmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn vrefbuf_dis(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_vrefbuf_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
    #[inline(always)]
    pub const fn act_ref_dis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
    #[inline(always)]
    pub fn set_act_ref_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Indicates that the normal mode of the Active Reference is ready."]
    #[inline(always)]
    pub const fn act_ref_ok(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the normal mode of the Active Reference is ready."]
    #[inline(always)]
    pub fn set_act_ref_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PwrCtl {
    #[inline(always)]
    fn default() -> PwrCtl {
        PwrCtl(0)
    }
}
#[doc = "HIBERNATE Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrHibData(pub u32);
impl PwrHibData {
    #[doc = "Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub const fn hib_data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn set_hib_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PwrHibData {
    #[inline(always)]
    fn default() -> PwrHibData {
        PwrHibData(0)
    }
}
#[doc = "HIBERNATE Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrHibernate(pub u32);
impl PwrHibernate {
    #[doc = "Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub const fn token(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn set_token(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    pub const fn unlock(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    pub fn set_unlock(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    pub const fn freeze(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    pub fn set_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    pub const fn mask_hibalarm(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    pub fn set_mask_hibalarm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    pub const fn mask_hibwdt(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    pub fn set_mask_hibwdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    pub const fn polarity_hibpin(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    pub fn set_polarity_hibpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    pub const fn mask_hibpin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    pub fn set_mask_hibpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    pub const fn hibernate_disable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    pub fn set_hibernate_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    pub const fn hibernate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    pub fn set_hibernate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PwrHibernate {
    #[inline(always)]
    fn default() -> PwrHibernate {
        PwrHibernate(0)
    }
}
#[doc = "Low Voltage Detector (LVD) Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrLvdCtl(pub u32);
impl PwrLvdCtl {
    #[doc = "Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
    #[inline(always)]
    pub const fn hvlvd1_tripsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
    #[inline(always)]
    pub fn set_hvlvd1_tripsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Source selection for HVLVD1"]
    #[inline(always)]
    pub const fn hvlvd1_srcsel(&self) -> Hvlvd1Srcsel {
        let val = (self.0 >> 4usize) & 0x07;
        Hvlvd1Srcsel::from_bits(val as u8)
    }
    #[doc = "Source selection for HVLVD1"]
    #[inline(always)]
    pub fn set_hvlvd1_srcsel(&mut self, val: Hvlvd1Srcsel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
    #[inline(always)]
    pub const fn hvlvd1_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
    #[inline(always)]
    pub fn set_hvlvd1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PwrLvdCtl {
    #[inline(always)]
    fn default() -> PwrLvdCtl {
        PwrLvdCtl(0)
    }
}
#[doc = "Low Voltage Detector (LVD) Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrLvdStatus(pub u32);
impl PwrLvdStatus {
    #[doc = "HVLVD1 output. 0: below voltage threshold 1: above voltage threshold"]
    #[inline(always)]
    pub const fn hvlvd1_ok(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HVLVD1 output. 0: below voltage threshold 1: above voltage threshold"]
    #[inline(always)]
    pub fn set_hvlvd1_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PwrLvdStatus {
    #[inline(always)]
    fn default() -> PwrLvdStatus {
        PwrLvdStatus(0)
    }
}
#[doc = "BOD/OVP Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrTrimBodovpCtl(pub u32);
impl PwrTrimBodovpCtl {
    #[doc = "HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn hvporbod_tripsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_hvporbod_tripsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn hvporbod_ofstrim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_hvporbod_ofstrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn hvporbod_itrim(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_hvporbod_itrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn lvporbod_tripsel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_lvporbod_tripsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn lvporbod_ofstrim(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_lvporbod_ofstrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[doc = "LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn lvporbod_itrim(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_lvporbod_itrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
}
impl Default for PwrTrimBodovpCtl {
    #[inline(always)]
    fn default() -> PwrTrimBodovpCtl {
        PwrTrimBodovpCtl(0)
    }
}
#[doc = "LVD Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrTrimLvdCtl(pub u32);
impl PwrTrimLvdCtl {
    #[doc = "HVLVD1 offset trim"]
    #[inline(always)]
    pub const fn hvlvd1_ofstrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "HVLVD1 offset trim"]
    #[inline(always)]
    pub fn set_hvlvd1_ofstrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "HVLVD1 current trim"]
    #[inline(always)]
    pub const fn hvlvd1_itrim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "HVLVD1 current trim"]
    #[inline(always)]
    pub fn set_hvlvd1_itrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for PwrTrimLvdCtl {
    #[inline(always)]
    fn default() -> PwrTrimLvdCtl {
        PwrTrimLvdCtl(0)
    }
}
#[doc = "Power System Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrTrimPwrsysCtl(pub u32);
impl PwrTrimPwrsysCtl {
    #[doc = "Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
    #[inline(always)]
    pub const fn act_reg_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/HIBERNATE. Two voltages are supported: 0.9V and 1.1V. The codes for these are stored in SFLASH_LDO_0P9V_TRIM and SFLASH_LDO_1P1V_TRIM, respectively."]
    #[inline(always)]
    pub fn set_act_reg_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn act_reg_boost(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_act_reg_boost(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for PwrTrimPwrsysCtl {
    #[inline(always)]
    fn default() -> PwrTrimPwrsysCtl {
        PwrTrimPwrsysCtl(0)
    }
}
#[doc = "Reference Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrTrimRefCtl(pub u32);
impl PwrTrimRefCtl {
    #[doc = "Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub const fn act_ref_tctrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn set_act_ref_tctrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub const fn act_ref_itrim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn set_act_ref_itrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub const fn act_ref_abstrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn set_act_ref_abstrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub const fn act_ref_iboost(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub fn set_act_ref_iboost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub const fn dpslp_ref_tctrim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn set_dpslp_ref_tctrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn dpslp_ref_abstrim(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_dpslp_ref_abstrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[doc = "DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub const fn dpslp_ref_itrim(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn set_dpslp_ref_itrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for PwrTrimRefCtl {
    #[inline(always)]
    fn default() -> PwrTrimRefCtl {
        PwrTrimRefCtl(0)
    }
}
#[doc = "Wakeup Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrTrimWakeCtl(pub u32);
impl PwrTrimWakeCtl {
    #[doc = "Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    pub const fn wake_delay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    pub fn set_wake_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for PwrTrimWakeCtl {
    #[inline(always)]
    fn default() -> PwrTrimWakeCtl {
        PwrTrimWakeCtl(0)
    }
}
#[doc = "Reset Cause Observation Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResCause(pub u32);
impl ResCause {
    #[doc = "A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    pub const fn reset_wdt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn set_reset_wdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    pub const fn reset_act_fault(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    pub fn set_reset_act_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    pub const fn reset_dpslp_fault(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    pub fn set_reset_dpslp_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    pub const fn reset_csv_wco_loss(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    pub fn set_reset_csv_wco_loss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub const fn reset_soft(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn set_reset_soft(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    pub const fn reset_mcwdt0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    pub fn set_reset_mcwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    pub const fn reset_mcwdt1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    pub fn set_reset_mcwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    pub const fn reset_mcwdt2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    pub fn set_reset_mcwdt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    pub const fn reset_mcwdt3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    pub fn set_reset_mcwdt3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for ResCause {
    #[inline(always)]
    fn default() -> ResCause {
        ResCause(0)
    }
}
#[doc = "Reset Cause Observation Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResCause2(pub u32);
impl ResCause2 {
    #[doc = "Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub const fn reset_csv_hf_loss(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn set_reset_csv_hf_loss(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub const fn reset_csv_hf_freq(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn set_reset_csv_hf_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ResCause2 {
    #[inline(always)]
    fn default() -> ResCause2 {
        ResCause2(0)
    }
}
#[doc = "SRSS Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrssIntr(pub u32);
impl SrssIntr {
    #[doc = "WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    pub const fn wdt_match(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. W1C also feeds the watch dog. Missing 2 interrupts in a row will generate a reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    pub fn set_wdt_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    pub const fn hvlvd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn set_hvlvd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    pub const fn clk_cal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Clock calibration counter is done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    pub fn set_clk_cal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for SrssIntr {
    #[inline(always)]
    fn default() -> SrssIntr {
        SrssIntr(0)
    }
}
#[doc = "SRSS Interrupt Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrssIntrCfg(pub u32);
impl SrssIntrCfg {
    #[doc = "Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    pub const fn hvlvd1_edge_sel(&self) -> Hvlvd1EdgeSel {
        let val = (self.0 >> 0usize) & 0x03;
        Hvlvd1EdgeSel::from_bits(val as u8)
    }
    #[doc = "Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    pub fn set_hvlvd1_edge_sel(&mut self, val: Hvlvd1EdgeSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SrssIntrCfg {
    #[inline(always)]
    fn default() -> SrssIntrCfg {
        SrssIntrCfg(0)
    }
}
#[doc = "SRSS Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrssIntrMask(pub u32);
impl SrssIntrMask {
    #[doc = "Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
    #[inline(always)]
    pub const fn wdt_match(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
    #[inline(always)]
    pub fn set_wdt_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask for low voltage detector HVLVD1"]
    #[inline(always)]
    pub const fn hvlvd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn set_hvlvd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask for clock calibration done"]
    #[inline(always)]
    pub const fn clk_cal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for clock calibration done"]
    #[inline(always)]
    pub fn set_clk_cal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for SrssIntrMask {
    #[inline(always)]
    fn default() -> SrssIntrMask {
        SrssIntrMask(0)
    }
}
#[doc = "SRSS Interrupt Masked Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrssIntrMasked(pub u32);
impl SrssIntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn wdt_match(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_wdt_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn hvlvd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_hvlvd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn clk_cal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_clk_cal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for SrssIntrMasked {
    #[inline(always)]
    fn default() -> SrssIntrMasked {
        SrssIntrMasked(0)
    }
}
#[doc = "SRSS Interrupt Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrssIntrSet(pub u32);
impl SrssIntrSet {
    #[doc = "Set interrupt for low voltage detector WDT_MATCH"]
    #[inline(always)]
    pub const fn wdt_match(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set interrupt for low voltage detector WDT_MATCH"]
    #[inline(always)]
    pub fn set_wdt_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    pub const fn hvlvd1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set interrupt for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn set_hvlvd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    pub const fn clk_cal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set interrupt for clock calibration counter done. This field is reset during DEEPSLEEP mode."]
    #[inline(always)]
    pub fn set_clk_cal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for SrssIntrSet {
    #[inline(always)]
    fn default() -> SrssIntrSet {
        SrssIntrSet(0)
    }
}
#[doc = "Watchdog Counter Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtCnt(pub u32);
impl WdtCnt {
    #[doc = "Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
    #[inline(always)]
    pub const fn counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
    #[inline(always)]
    pub fn set_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for WdtCnt {
    #[inline(always)]
    fn default() -> WdtCnt {
        WdtCnt(0)
    }
}
#[doc = "Watchdog Counter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtCtl(pub u32);
impl WdtCtl {
    #[doc = "Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    pub const fn wdt_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    pub fn set_wdt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    pub const fn wdt_lock(&self) -> WdtLock {
        let val = (self.0 >> 30usize) & 0x03;
        WdtLock::from_bits(val as u8)
    }
    #[doc = "Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    pub fn set_wdt_lock(&mut self, val: WdtLock) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for WdtCtl {
    #[inline(always)]
    fn default() -> WdtCtl {
        WdtCtl(0)
    }
}
#[doc = "Watchdog Counter Match Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtMatch(pub u32);
impl WdtMatch {
    #[doc = "Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub const fn match_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub fn set_match_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    pub const fn ignore_bits(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    pub fn set_ignore_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for WdtMatch {
    #[inline(always)]
    fn default() -> WdtMatch {
        WdtMatch(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CcoRange {
    #[doc = "Target frequency is in range \\[48, 64) MHz"]
    RANGE0 = 0,
    #[doc = "Target frequency is in range \\[64, 85) MHz"]
    RANGE1 = 0x01,
    #[doc = "Target frequency is in range \\[85, 113) MHz"]
    RANGE2 = 0x02,
    #[doc = "Target frequency is in range \\[113, 150) MHz"]
    RANGE3 = 0x03,
    #[doc = "Target frequency is in range \\[150, 200\\] MHz"]
    RANGE4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CcoRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcoRange {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcoRange {
    #[inline(always)]
    fn from(val: u8) -> CcoRange {
        CcoRange::from_bits(val)
    }
}
impl From<CcoRange> for u8 {
    #[inline(always)]
    fn from(val: CcoRange) -> u8 {
        CcoRange::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClkFllConfig3BypassSel {
    #[doc = "N/A"]
    AUTO = 0,
    #[doc = "N/A"]
    AUTO1 = 0x01,
    #[doc = "Select FLL reference input (bypass mode). Ignores lock indicator"]
    FLL_REF = 0x02,
    #[doc = "Select FLL output. Ignores lock indicator."]
    FLL_OUT = 0x03,
}
impl ClkFllConfig3BypassSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkFllConfig3BypassSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkFllConfig3BypassSel {
    #[inline(always)]
    fn from(val: u8) -> ClkFllConfig3BypassSel {
        ClkFllConfig3BypassSel::from_bits(val)
    }
}
impl From<ClkFllConfig3BypassSel> for u8 {
    #[inline(always)]
    fn from(val: ClkFllConfig3BypassSel) -> u8 {
        ClkFllConfig3BypassSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClkPllConfigBypassSel {
    #[doc = "Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output."]
    AUTO = 0,
    #[doc = "Same as AUTO"]
    AUTO1 = 0x01,
    #[doc = "Select PLL reference input (bypass mode). Ignores lock indicator"]
    PLL_REF = 0x02,
    #[doc = "Select PLL output. Ignores lock indicator."]
    PLL_OUT = 0x03,
}
impl ClkPllConfigBypassSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkPllConfigBypassSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkPllConfigBypassSel {
    #[inline(always)]
    fn from(val: u8) -> ClkPllConfigBypassSel {
        ClkPllConfigBypassSel::from_bits(val)
    }
}
impl From<ClkPllConfigBypassSel> for u8 {
    #[inline(always)]
    fn from(val: ClkPllConfigBypassSel) -> u8 {
        ClkPllConfigBypassSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DebugSession {
    #[doc = "No debug session active"]
    NO_SESSION = 0,
    #[doc = "Debug session is active. Power modes behave differently to keep the debug session active."]
    SESSION_ACTIVE = 0x01,
}
impl DebugSession {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugSession {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugSession {
    #[inline(always)]
    fn from(val: u8) -> DebugSession {
        DebugSession::from_bits(val)
    }
}
impl From<DebugSession> for u8 {
    #[inline(always)]
    fn from(val: DebugSession) -> u8 {
        DebugSession::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DsiMux {
    #[doc = "DSI0 - dsi_out\\[0\\]"]
    DSI_OUT0 = 0,
    #[doc = "DSI1 - dsi_out\\[1\\]"]
    DSI_OUT1 = 0x01,
    #[doc = "DSI2 - dsi_out\\[2\\]"]
    DSI_OUT2 = 0x02,
    #[doc = "DSI3 - dsi_out\\[3\\]"]
    DSI_OUT3 = 0x03,
    #[doc = "DSI4 - dsi_out\\[4\\]"]
    DSI_OUT4 = 0x04,
    #[doc = "DSI5 - dsi_out\\[5\\]"]
    DSI_OUT5 = 0x05,
    #[doc = "DSI6 - dsi_out\\[6\\]"]
    DSI_OUT6 = 0x06,
    #[doc = "DSI7 - dsi_out\\[7\\]"]
    DSI_OUT7 = 0x07,
    #[doc = "DSI8 - dsi_out\\[8\\]"]
    DSI_OUT8 = 0x08,
    #[doc = "DSI9 - dsi_out\\[9\\]"]
    DSI_OUT9 = 0x09,
    #[doc = "DSI10 - dsi_out\\[10\\]"]
    DSI_OUT10 = 0x0a,
    #[doc = "DSI11 - dsi_out\\[11\\]"]
    DSI_OUT11 = 0x0b,
    #[doc = "DSI12 - dsi_out\\[12\\]"]
    DSI_OUT12 = 0x0c,
    #[doc = "DSI13 - dsi_out\\[13\\]"]
    DSI_OUT13 = 0x0d,
    #[doc = "DSI14 - dsi_out\\[14\\]"]
    DSI_OUT14 = 0x0e,
    #[doc = "DSI15 - dsi_out\\[15\\]"]
    DSI_OUT15 = 0x0f,
    #[doc = "ILO - Internal Low-speed Oscillator"]
    ILO = 0x10,
    #[doc = "WCO - Watch-Crystal Oscillator"]
    WCO = 0x11,
    #[doc = "ALTLF - Alternate Low-Frequency Clock"]
    ALTLF = 0x12,
    #[doc = "PILO - Precision Internal Low-speed Oscillator"]
    PILO = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl DsiMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DsiMux {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DsiMux {
    #[inline(always)]
    fn from(val: u8) -> DsiMux {
        DsiMux::from_bits(val)
    }
}
impl From<DsiMux> for u8 {
    #[inline(always)]
    fn from(val: DsiMux) -> u8 {
        DsiMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FastSel0 {
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL0 and HFCLK_SEL0."]
    NC = 0,
    #[doc = "External Crystal Oscillator (ECO)"]
    ECO = 0x01,
    #[doc = "External clock input (EXTCLK)"]
    EXTCLK = 0x02,
    #[doc = "Alternate High-Frequency (ALTHF) clock input to SRSS"]
    ALTHF = 0x03,
    #[doc = "Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    TIMERCLK = 0x04,
    #[doc = "Selects the clock path chosen by PATH_SEL0 field"]
    PATH_SEL0 = 0x05,
    #[doc = "Selects the output of the HFCLK_SEL0 mux"]
    HFCLK_SEL0 = 0x06,
    #[doc = "Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL0"]
    SLOW_SEL0 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FastSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FastSel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FastSel0 {
    #[inline(always)]
    fn from(val: u8) -> FastSel0 {
        FastSel0::from_bits(val)
    }
}
impl From<FastSel0> for u8 {
    #[inline(always)]
    fn from(val: FastSel0) -> u8 {
        FastSel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FastSel1 {
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL1 and HFCLK_SEL1."]
    NC = 0,
    #[doc = "External Crystal Oscillator (ECO)"]
    ECO = 0x01,
    #[doc = "External clock input (EXTCLK)"]
    EXTCLK = 0x02,
    #[doc = "Alternate High-Frequency (ALTHF) clock input to SRSS"]
    ALTHF = 0x03,
    #[doc = "Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    TIMERCLK = 0x04,
    #[doc = "Selects the clock path chosen by PATH_SEL1 field"]
    PATH_SEL1 = 0x05,
    #[doc = "Selects the output of the HFCLK_SEL1 mux"]
    HFCLK_SEL1 = 0x06,
    #[doc = "Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL1"]
    SLOW_SEL1 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FastSel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FastSel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FastSel1 {
    #[inline(always)]
    fn from(val: u8) -> FastSel1 {
        FastSel1::from_bits(val)
    }
}
impl From<FastSel1> for u8 {
    #[inline(always)]
    fn from(val: FastSel1) -> u8 {
        FastSel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hvlvd1EdgeSel {
    #[doc = "Disabled"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl Hvlvd1EdgeSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvlvd1EdgeSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvlvd1EdgeSel {
    #[inline(always)]
    fn from(val: u8) -> Hvlvd1EdgeSel {
        Hvlvd1EdgeSel::from_bits(val)
    }
}
impl From<Hvlvd1EdgeSel> for u8 {
    #[inline(always)]
    fn from(val: Hvlvd1EdgeSel) -> u8 {
        Hvlvd1EdgeSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hvlvd1Srcsel {
    #[doc = "Select VDDD"]
    VDDD = 0,
    #[doc = "Select AMUXBUSA (VDDD branch)"]
    AMUXBUSA = 0x01,
    #[doc = "N/A"]
    RSVD = 0x02,
    #[doc = "N/A"]
    VDDIO = 0x03,
    #[doc = "Select AMUXBUSB (VDDD branch)"]
    AMUXBUSB = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Hvlvd1Srcsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvlvd1Srcsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvlvd1Srcsel {
    #[inline(always)]
    fn from(val: u8) -> Hvlvd1Srcsel {
        Hvlvd1Srcsel::from_bits(val)
    }
}
impl From<Hvlvd1Srcsel> for u8 {
    #[inline(always)]
    fn from(val: Hvlvd1Srcsel) -> u8 {
        Hvlvd1Srcsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfclkSel {
    #[doc = "ILO - Internal Low-speed Oscillator"]
    ILO = 0,
    #[doc = "WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    WCO = 0x01,
    #[doc = "ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    ALTLF = 0x02,
    #[doc = "PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    PILO = 0x03,
}
impl LfclkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfclkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfclkSel {
    #[inline(always)]
    fn from(val: u8) -> LfclkSel {
        LfclkSel::from_bits(val)
    }
}
impl From<LfclkSel> for u8 {
    #[inline(always)]
    fn from(val: LfclkSel) -> u8 {
        LfclkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum McwdtLock {
    #[doc = "No effect"]
    NO_CHG = 0,
    #[doc = "Clears bit 0"]
    CLR0 = 0x01,
    #[doc = "Clears bit 1"]
    CLR1 = 0x02,
    #[doc = "Sets both bits 0 and 1"]
    SET01 = 0x03,
}
impl McwdtLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McwdtLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McwdtLock {
    #[inline(always)]
    fn from(val: u8) -> McwdtLock {
        McwdtLock::from_bits(val)
    }
}
impl From<McwdtLock> for u8 {
    #[inline(always)]
    fn from(val: McwdtLock) -> u8 {
        McwdtLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PathMux {
    #[doc = "IMO - Internal R/C Oscillator"]
    IMO = 0,
    #[doc = "EXTCLK - External Clock Pin"]
    EXTCLK = 0x01,
    #[doc = "ECO - External-Crystal Oscillator"]
    ECO = 0x02,
    #[doc = "ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    ALTHF = 0x03,
    #[doc = "DSI_MUX - Output of DSI mux for this path. Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    DSI_MUX = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl PathMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PathMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PathMux {
    #[inline(always)]
    fn from(val: u8) -> PathMux {
        PathMux::from_bits(val)
    }
}
impl From<PathMux> for u8 {
    #[inline(always)]
    fn from(val: PathMux) -> u8 {
        PathMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerMode {
    #[doc = "System is resetting."]
    RESET = 0,
    #[doc = "At least one CPU is running."]
    ACTIVE = 0x01,
    #[doc = "No CPUs are running. Peripherals may be running."]
    SLEEP = 0x02,
    #[doc = "Main high-frequency clock is off; low speed clocks are available. Communication interface clocks may be present."]
    DEEPSLEEP = 0x03,
}
impl PowerMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerMode {
    #[inline(always)]
    fn from(val: u8) -> PowerMode {
        PowerMode::from_bits(val)
    }
}
impl From<PowerMode> for u8 {
    #[inline(always)]
    fn from(val: PowerMode) -> u8 {
        PowerMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PumpDiv {
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    NO_DIV = 0,
    #[doc = "Divide selected clock source by 2"]
    DIV_BY_2 = 0x01,
    #[doc = "Divide selected clock source by 4"]
    DIV_BY_4 = 0x02,
    #[doc = "Divide selected clock source by 8"]
    DIV_BY_8 = 0x03,
    #[doc = "Divide selected clock source by 16"]
    DIV_BY_16 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl PumpDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PumpDiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PumpDiv {
    #[inline(always)]
    fn from(val: u8) -> PumpDiv {
        PumpDiv::from_bits(val)
    }
}
impl From<PumpDiv> for u8 {
    #[inline(always)]
    fn from(val: PumpDiv) -> u8 {
        PumpDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RootDiv {
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    NO_DIV = 0,
    #[doc = "Divide selected clock source by 2"]
    DIV_BY_2 = 0x01,
    #[doc = "Divide selected clock source by 4"]
    DIV_BY_4 = 0x02,
    #[doc = "Divide selected clock source by 8"]
    DIV_BY_8 = 0x03,
}
impl RootDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RootDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RootDiv {
    #[inline(always)]
    fn from(val: u8) -> RootDiv {
        RootDiv::from_bits(val)
    }
}
impl From<RootDiv> for u8 {
    #[inline(always)]
    fn from(val: RootDiv) -> u8 {
        RootDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RootMux {
    #[doc = "Select PATH0 (can be configured for FLL)"]
    PATH0 = 0,
    #[doc = "Select PATH1 (can be configured for PLL0, if available in the product)"]
    PATH1 = 0x01,
    #[doc = "Select PATH2 (can be configured for PLL1, if available in the product)"]
    PATH2 = 0x02,
    #[doc = "Select PATH3 (can be configured for PLL2, if available in the product)"]
    PATH3 = 0x03,
    #[doc = "Select PATH4 (can be configured for PLL3, if available in the product)"]
    PATH4 = 0x04,
    #[doc = "Select PATH5 (can be configured for PLL4, if available in the product)"]
    PATH5 = 0x05,
    #[doc = "Select PATH6 (can be configured for PLL5, if available in the product)"]
    PATH6 = 0x06,
    #[doc = "Select PATH7 (can be configured for PLL6, if available in the product)"]
    PATH7 = 0x07,
    #[doc = "Select PATH8 (can be configured for PLL7, if available in the product)"]
    PATH8 = 0x08,
    #[doc = "Select PATH9 (can be configured for PLL8, if available in the product)"]
    PATH9 = 0x09,
    #[doc = "Select PATH10 (can be configured for PLL9, if available in the product)"]
    PATH10 = 0x0a,
    #[doc = "Select PATH11 (can be configured for PLL10, if available in the product)"]
    PATH11 = 0x0b,
    #[doc = "Select PATH12 (can be configured for PLL11, if available in the product)"]
    PATH12 = 0x0c,
    #[doc = "Select PATH13 (can be configured for PLL12, if available in the product)"]
    PATH13 = 0x0d,
    #[doc = "Select PATH14 (can be configured for PLL13, if available in the product)"]
    PATH14 = 0x0e,
    #[doc = "Select PATH15 (can be configured for PLL14, if available in the product)"]
    PATH15 = 0x0f,
}
impl RootMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RootMux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RootMux {
    #[inline(always)]
    fn from(val: u8) -> RootMux {
        RootMux::from_bits(val)
    }
}
impl From<RootMux> for u8 {
    #[inline(always)]
    fn from(val: RootMux) -> u8 {
        RootMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SlowSel0 {
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    NC = 0,
    #[doc = "Internal Low Speed Oscillator (ILO)"]
    ILO = 0x01,
    #[doc = "Watch-Crystal Oscillator (WCO)"]
    WCO = 0x02,
    #[doc = "Root of the Backup domain clock tree (BAK)"]
    BAK = 0x03,
    #[doc = "Alternate low-frequency clock input to SRSS (ALTLF)"]
    ALTLF = 0x04,
    #[doc = "Root of the low-speed clock tree (LFCLK)"]
    LFCLK = 0x05,
    #[doc = "Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    IMO = 0x06,
    #[doc = "Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    SLPCTRL = 0x07,
    #[doc = "Precision Internal Low Speed Oscillator (PILO)"]
    PILO = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SlowSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowSel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowSel0 {
    #[inline(always)]
    fn from(val: u8) -> SlowSel0 {
        SlowSel0::from_bits(val)
    }
}
impl From<SlowSel0> for u8 {
    #[inline(always)]
    fn from(val: SlowSel0) -> u8 {
        SlowSel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SlowSel1 {
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    NC = 0,
    #[doc = "Internal Low Speed Oscillator (ILO)"]
    ILO = 0x01,
    #[doc = "Watch-Crystal Oscillator (WCO)"]
    WCO = 0x02,
    #[doc = "Root of the Backup domain clock tree (BAK)"]
    BAK = 0x03,
    #[doc = "Alternate low-frequency clock input to SRSS (ALTLF)"]
    ALTLF = 0x04,
    #[doc = "Root of the low-speed clock tree (LFCLK)"]
    LFCLK = 0x05,
    #[doc = "Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    IMO = 0x06,
    #[doc = "Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    SLPCTRL = 0x07,
    #[doc = "Precision Internal Low Speed Oscillator (PILO)"]
    PILO = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SlowSel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowSel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowSel1 {
    #[inline(always)]
    fn from(val: u8) -> SlowSel1 {
        SlowSel1::from_bits(val)
    }
}
impl From<SlowSel1> for u8 {
    #[inline(always)]
    fn from(val: SlowSel1) -> u8 {
        SlowSel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TimerHf0Div {
    #[doc = "Transparent mode, feed through selected clock source w/o dividing or correcting duty cycle."]
    NO_DIV = 0,
    #[doc = "Divide HFCLK0 by 2."]
    DIV_BY_2 = 0x01,
    #[doc = "Divide HFCLK0 by 4."]
    DIV_BY_4 = 0x02,
    #[doc = "Divide HFCLK0 by 8."]
    DIV_BY_8 = 0x03,
}
impl TimerHf0Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimerHf0Div {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimerHf0Div {
    #[inline(always)]
    fn from(val: u8) -> TimerHf0Div {
        TimerHf0Div::from_bits(val)
    }
}
impl From<TimerHf0Div> for u8 {
    #[inline(always)]
    fn from(val: TimerHf0Div) -> u8 {
        TimerHf0Div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TimerSel {
    #[doc = "IMO - Internal Main Oscillator"]
    IMO = 0,
    #[doc = "Select the output of the predivider configured by TIMER_HF0_DIV."]
    HF0_DIV = 0x01,
}
impl TimerSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimerSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimerSel {
    #[inline(always)]
    fn from(val: u8) -> TimerSel {
        TimerSel::from_bits(val)
    }
}
impl From<TimerSel> for u8 {
    #[inline(always)]
    fn from(val: TimerSel) -> u8 {
        TimerSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WdtLock {
    #[doc = "No effect"]
    NO_CHG = 0,
    #[doc = "Clears bit 0"]
    CLR0 = 0x01,
    #[doc = "Clears bit 1"]
    CLR1 = 0x02,
    #[doc = "Sets both bits 0 and 1"]
    SET01 = 0x03,
}
impl WdtLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtLock {
    #[inline(always)]
    fn from(val: u8) -> WdtLock {
        WdtLock::from_bits(val)
    }
}
impl From<WdtLock> for u8 {
    #[inline(always)]
    fn from(val: WdtLock) -> u8 {
        WdtLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WdtMode0 {
    #[doc = "Do nothing"]
    NOTHING = 0,
    #[doc = "Assert WDT_INTx"]
    INT = 0x01,
    #[doc = "Assert WDT Reset"]
    RESET = 0x02,
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    INT_THEN_RESET = 0x03,
}
impl WdtMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtMode0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtMode0 {
    #[inline(always)]
    fn from(val: u8) -> WdtMode0 {
        WdtMode0::from_bits(val)
    }
}
impl From<WdtMode0> for u8 {
    #[inline(always)]
    fn from(val: WdtMode0) -> u8 {
        WdtMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WdtMode1 {
    #[doc = "Do nothing"]
    NOTHING = 0,
    #[doc = "Assert WDT_INTx"]
    INT = 0x01,
    #[doc = "Assert WDT Reset"]
    RESET = 0x02,
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt"]
    INT_THEN_RESET = 0x03,
}
impl WdtMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtMode1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtMode1 {
    #[inline(always)]
    fn from(val: u8) -> WdtMode1 {
        WdtMode1::from_bits(val)
    }
}
impl From<WdtMode1> for u8 {
    #[inline(always)]
    fn from(val: WdtMode1) -> u8 {
        WdtMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WdtMode2 {
    #[doc = "Free running counter with no interrupt requests"]
    NOTHING = 0,
    #[doc = "Free running counter with interrupt request that occurs one LFCLK cycle after the specified bit in CTR2 toggles (see WDT_BITS2)."]
    INT = 0x01,
}
impl WdtMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtMode2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtMode2 {
    #[inline(always)]
    fn from(val: u8) -> WdtMode2 {
        WdtMode2::from_bits(val)
    }
}
impl From<WdtMode2> for u8 {
    #[inline(always)]
    fn from(val: WdtMode2) -> u8 {
        WdtMode2::to_bits(val)
    }
}
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
