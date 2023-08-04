#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Capsense Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csd0 {
    ptr: *mut u8,
}
unsafe impl Send for Csd0 {}
unsafe impl Sync for Csd0 {}
impl Csd0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration and Control"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Spare MMIO"]
    #[inline(always)]
    pub const fn spare(self) -> crate::common::Reg<Spare, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Current Sequencer status"]
    #[inline(always)]
    pub const fn stat_seq(self) -> crate::common::Reg<StatSeq, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Current status counts"]
    #[inline(always)]
    pub const fn stat_cnts(self) -> crate::common::Reg<StatCnts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Current count of the HSCMP counter"]
    #[inline(always)]
    pub const fn stat_hcnt(self) -> crate::common::Reg<StatHcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "Result CSD/CSX accumulation counter value 1"]
    #[inline(always)]
    pub const fn result_val1(self) -> crate::common::Reg<ResultVal1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "Result CSX accumulation counter value 2"]
    #[inline(always)]
    pub const fn result_val2(self) -> crate::common::Reg<ResultVal2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(212usize) as _) }
    }
    #[doc = "ADC measurement"]
    #[inline(always)]
    pub const fn adc_res(self) -> crate::common::Reg<AdcRes, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "CSD Interrupt Request Register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "CSD Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "CSD Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(248usize) as _) }
    }
    #[doc = "CSD Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize) as _) }
    }
    #[doc = "High Speed Comparator configuration"]
    #[inline(always)]
    pub const fn hscmp(self) -> crate::common::Reg<Hscmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Reference Generator configuration"]
    #[inline(always)]
    pub const fn ambuf(self) -> crate::common::Reg<Ambuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize) as _) }
    }
    #[doc = "Reference Generator configuration"]
    #[inline(always)]
    pub const fn refgen(self) -> crate::common::Reg<Refgen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(392usize) as _) }
    }
    #[doc = "CSD Comparator configuration"]
    #[inline(always)]
    pub const fn csdcmp(self) -> crate::common::Reg<Csdcmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(396usize) as _) }
    }
    #[doc = "Switch Resistance configuration"]
    #[inline(always)]
    pub const fn sw_res(self) -> crate::common::Reg<SwRes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(496usize) as _) }
    }
    #[doc = "Sense clock period"]
    #[inline(always)]
    pub const fn sense_period(self) -> crate::common::Reg<SensePeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Sense clock duty cycle"]
    #[inline(always)]
    pub const fn sense_duty(self) -> crate::common::Reg<SenseDuty, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "HSCMP Pos input switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_hs_p_sel(self) -> crate::common::Reg<SwHsPSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(640usize) as _) }
    }
    #[doc = "HSCMP Neg input switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_hs_n_sel(self) -> crate::common::Reg<SwHsNSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(644usize) as _) }
    }
    #[doc = "Shielding switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_shield_sel(self) -> crate::common::Reg<SwShieldSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(648usize) as _) }
    }
    #[doc = "Amuxbuffer switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_amuxbuf_sel(self) -> crate::common::Reg<SwAmuxbufSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(656usize) as _) }
    }
    #[doc = "AMUXBUS bypass switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_byp_sel(self) -> crate::common::Reg<SwBypSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(660usize) as _) }
    }
    #[doc = "CSDCMP Pos Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_cmp_p_sel(self) -> crate::common::Reg<SwCmpPSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(672usize) as _) }
    }
    #[doc = "CSDCMP Neg Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_cmp_n_sel(self) -> crate::common::Reg<SwCmpNSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(676usize) as _) }
    }
    #[doc = "Reference Generator Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_refgen_sel(self) -> crate::common::Reg<SwRefgenSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(680usize) as _) }
    }
    #[doc = "Full Wave Cmod Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_fw_mod_sel(self) -> crate::common::Reg<SwFwModSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(688usize) as _) }
    }
    #[doc = "Full Wave Csh_tank Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_fw_tank_sel(self) -> crate::common::Reg<SwFwTankSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(692usize) as _) }
    }
    #[doc = "DSI output switch control Waveform selection"]
    #[inline(always)]
    pub const fn sw_dsi_sel(self) -> crate::common::Reg<SwDsiSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(704usize) as _) }
    }
    #[doc = "IO output control Waveform selection"]
    #[inline(always)]
    pub const fn io_sel(self) -> crate::common::Reg<IoSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(720usize) as _) }
    }
    #[doc = "Sequencer Timing"]
    #[inline(always)]
    pub const fn seq_time(self) -> crate::common::Reg<SeqTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize) as _) }
    }
    #[doc = "Sequencer Initial conversion and sample counts"]
    #[inline(always)]
    pub const fn seq_init_cnt(self) -> crate::common::Reg<SeqInitCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(784usize) as _) }
    }
    #[doc = "Sequencer Normal conversion and sample counts"]
    #[inline(always)]
    pub const fn seq_norm_cnt(self) -> crate::common::Reg<SeqNormCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(788usize) as _) }
    }
    #[doc = "ADC Control"]
    #[inline(always)]
    pub const fn adc_ctl(self) -> crate::common::Reg<AdcCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(800usize) as _) }
    }
    #[doc = "Sequencer start"]
    #[inline(always)]
    pub const fn seq_start(self) -> crate::common::Reg<SeqStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(832usize) as _) }
    }
    #[doc = "IDACA Configuration"]
    #[inline(always)]
    pub const fn idaca(self) -> crate::common::Reg<Idaca, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "IDACB Configuration"]
    #[inline(always)]
    pub const fn idacb(self) -> crate::common::Reg<Idacb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1280usize) as _) }
    }
}
#[doc = "ADC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCtl(pub u32);
impl AdcCtl {
    #[doc = "ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
    #[inline(always)]
    pub const fn adc_time(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
    #[inline(always)]
    pub fn set_adc_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub const fn adc_mode(&self) -> AdcMode {
        let val = (self.0 >> 16usize) & 0x03;
        AdcMode::from_bits(val as u8)
    }
    #[doc = "Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub fn set_adc_mode(&mut self, val: AdcMode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for AdcCtl {
    #[inline(always)]
    fn default() -> AdcCtl {
        AdcCtl(0)
    }
}
#[doc = "ADC measurement"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcRes(pub u32);
impl AdcRes {
    #[doc = "Count to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub const fn vin_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Count to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn set_vin_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Polarity used for IDACB for this last ADC result, 0= source, 1= sink"]
    #[inline(always)]
    pub const fn hscmp_pol(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity used for IDACB for this last ADC result, 0= source, 1= sink"]
    #[inline(always)]
    pub fn set_hscmp_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This flag is set when the ADC counter overflows. This is an indication to the firmware that the IDACB current level is too low."]
    #[inline(always)]
    pub const fn adc_overflow(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This flag is set when the ADC counter overflows. This is an indication to the firmware that the IDACB current level is too low."]
    #[inline(always)]
    pub fn set_adc_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This flag is set when the ADC sequencer was aborted before tripping HSCMP."]
    #[inline(always)]
    pub const fn adc_abort(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This flag is set when the ADC sequencer was aborted before tripping HSCMP."]
    #[inline(always)]
    pub fn set_adc_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for AdcRes {
    #[inline(always)]
    fn default() -> AdcRes {
        AdcRes(0)
    }
}
#[doc = "Reference Generator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ambuf(pub u32);
impl Ambuf {
    #[doc = "Amux buffer power level"]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> PwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        PwrMode::from_bits(val as u8)
    }
    #[doc = "Amux buffer power level"]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: PwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Ambuf {
    #[inline(always)]
    fn default() -> Ambuf {
        Ambuf(0)
    }
}
#[doc = "Configuration and Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Select Iref supply."]
    #[inline(always)]
    pub const fn iref_sel(&self) -> IrefSel {
        let val = (self.0 >> 0usize) & 0x01;
        IrefSel::from_bits(val as u8)
    }
    #[doc = "Select Iref supply."]
    #[inline(always)]
    pub fn set_iref_sel(&mut self, val: IrefSel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    pub const fn filter_delay(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    pub fn set_filter_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    pub const fn shield_delay(&self) -> ShieldDelay {
        let val = (self.0 >> 10usize) & 0x03;
        ShieldDelay::from_bits(val as u8)
    }
    #[doc = "Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    pub fn set_shield_delay(&mut self, val: ShieldDelay) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    pub const fn sense_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    pub fn set_sense_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables full wave cap sensing mode"]
    #[inline(always)]
    pub const fn full_wave(&self) -> FullWave {
        let val = (self.0 >> 17usize) & 0x01;
        FullWave::from_bits(val as u8)
    }
    #[doc = "Enables full wave cap sensing mode"]
    #[inline(always)]
    pub fn set_full_wave(&mut self, val: FullWave) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables mutual cap sensing mode"]
    #[inline(always)]
    pub const fn mutual_cap(&self) -> MutualCap {
        let val = (self.0 >> 18usize) & 0x01;
        MutualCap::from_bits(val as u8)
    }
    #[doc = "Enables mutual cap sensing mode"]
    #[inline(always)]
    pub fn set_mutual_cap(&mut self, val: MutualCap) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    pub const fn csx_dual_cnt(&self) -> CsxDualCnt {
        let val = (self.0 >> 19usize) & 0x01;
        CsxDualCnt::from_bits(val as u8)
    }
    #[doc = "Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    pub fn set_csx_dual_cnt(&mut self, val: CsxDualCnt) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Select what to output on the dsi_count bus."]
    #[inline(always)]
    pub const fn dsi_count_sel(&self) -> DsiCountSel {
        let val = (self.0 >> 24usize) & 0x01;
        DsiCountSel::from_bits(val as u8)
    }
    #[doc = "Select what to output on the dsi_count bus."]
    #[inline(always)]
    pub fn set_dsi_count_sel(&mut self, val: DsiCountSel) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    pub const fn dsi_sample_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    pub fn set_dsi_sample_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    pub const fn sample_sync(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    pub fn set_sample_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    pub const fn dsi_sense_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    pub fn set_dsi_sense_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    pub const fn lp_mode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    pub fn set_lp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "CSD Comparator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csdcmp(pub u32);
impl Csdcmp {
    #[doc = "CSD Comparator Enable"]
    #[inline(always)]
    pub const fn csdcmp_en(&self) -> CsdcmpEn {
        let val = (self.0 >> 0usize) & 0x01;
        CsdcmpEn::from_bits(val as u8)
    }
    #[doc = "CSD Comparator Enable"]
    #[inline(always)]
    pub fn set_csdcmp_en(&mut self, val: CsdcmpEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Select which IDAC polarity to use to detect CSDCMP triggering"]
    #[inline(always)]
    pub const fn polarity_sel(&self) -> PolaritySel {
        let val = (self.0 >> 4usize) & 0x03;
        PolaritySel::from_bits(val as u8)
    }
    #[doc = "Select which IDAC polarity to use to detect CSDCMP triggering"]
    #[inline(always)]
    pub fn set_polarity_sel(&mut self, val: PolaritySel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
    #[inline(always)]
    pub const fn cmp_phase(&self) -> CmpPhase {
        let val = (self.0 >> 8usize) & 0x03;
        CmpPhase::from_bits(val as u8)
    }
    #[doc = "Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
    #[inline(always)]
    pub fn set_cmp_phase(&mut self, val: CmpPhase) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Select which signal to output on dsi_sample_out."]
    #[inline(always)]
    pub const fn cmp_mode(&self) -> CmpMode {
        let val = (self.0 >> 28usize) & 0x01;
        CmpMode::from_bits(val as u8)
    }
    #[doc = "Select which signal to output on dsi_sample_out."]
    #[inline(always)]
    pub fn set_cmp_mode(&mut self, val: CmpMode) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
    #[inline(always)]
    pub const fn feedback_mode(&self) -> FeedbackMode {
        let val = (self.0 >> 29usize) & 0x01;
        FeedbackMode::from_bits(val as u8)
    }
    #[doc = "This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
    #[inline(always)]
    pub fn set_feedback_mode(&mut self, val: FeedbackMode) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub const fn az_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn set_az_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Csdcmp {
    #[inline(always)]
    fn default() -> Csdcmp {
        Csdcmp(0)
    }
}
#[doc = "High Speed Comparator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hscmp(pub u32);
impl Hscmp {
    #[doc = "High Speed Comparator enable"]
    #[inline(always)]
    pub const fn hscmp_en(&self) -> HscmpEn {
        let val = (self.0 >> 0usize) & 0x01;
        HscmpEn::from_bits(val as u8)
    }
    #[doc = "High Speed Comparator enable"]
    #[inline(always)]
    pub fn set_hscmp_en(&mut self, val: HscmpEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub const fn hscmp_invert(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub fn set_hscmp_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub const fn az_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn set_az_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hscmp {
    #[inline(always)]
    fn default() -> Hscmp {
        Hscmp(0)
    }
}
#[doc = "IDACA Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idaca(pub u32);
impl Idaca {
    #[doc = "Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub const fn pol_dyn(&self) -> IdacaPolDyn {
        let val = (self.0 >> 7usize) & 0x01;
        IdacaPolDyn::from_bits(val as u8)
    }
    #[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub fn set_pol_dyn(&mut self, val: IdacaPolDyn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 & config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub const fn polarity(&self) -> IdacaPolarity {
        let val = (self.0 >> 8usize) & 0x03;
        IdacaPolarity::from_bits(val as u8)
    }
    #[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 & config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub fn set_polarity(&mut self, val: IdacaPolarity) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Balancing mode: only applies to legs configured as CSD."]
    #[inline(always)]
    pub const fn bal_mode(&self) -> IdacaBalMode {
        let val = (self.0 >> 10usize) & 0x03;
        IdacaBalMode::from_bits(val as u8)
    }
    #[doc = "Balancing mode: only applies to legs configured as CSD."]
    #[inline(always)]
    pub fn set_bal_mode(&mut self, val: IdacaBalMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub const fn leg1_mode(&self) -> IdacaLeg1Mode {
        let val = (self.0 >> 16usize) & 0x03;
        IdacaLeg1Mode::from_bits(val as u8)
    }
    #[doc = "Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub fn set_leg1_mode(&mut self, val: IdacaLeg1Mode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Controls the usage mode of LEG2"]
    #[inline(always)]
    pub const fn leg2_mode(&self) -> IdacaLeg2Mode {
        let val = (self.0 >> 18usize) & 0x03;
        IdacaLeg2Mode::from_bits(val as u8)
    }
    #[doc = "Controls the usage mode of LEG2"]
    #[inline(always)]
    pub fn set_leg2_mode(&mut self, val: IdacaLeg2Mode) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
    #[inline(always)]
    pub const fn dsi_ctrl_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
    #[inline(always)]
    pub fn set_dsi_ctrl_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "IDAC multiplier"]
    #[inline(always)]
    pub const fn range(&self) -> IdacaRange {
        let val = (self.0 >> 22usize) & 0x03;
        IdacaRange::from_bits(val as u8)
    }
    #[doc = "IDAC multiplier"]
    #[inline(always)]
    pub fn set_range(&mut self, val: IdacaRange) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "output enable for leg 1 to CSDBUSA"]
    #[inline(always)]
    pub const fn leg1_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg 1 to CSDBUSA"]
    #[inline(always)]
    pub fn set_leg1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "output enable for leg 2 to CSDBUSA"]
    #[inline(always)]
    pub const fn leg2_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg 2 to CSDBUSA"]
    #[inline(always)]
    pub fn set_leg2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Idaca {
    #[inline(always)]
    fn default() -> Idaca {
        Idaca(0)
    }
}
#[doc = "IDACB Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idacb(pub u32);
impl Idacb {
    #[doc = "Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub const fn pol_dyn(&self) -> IdacbPolDyn {
        let val = (self.0 >> 7usize) & 0x01;
        IdacbPolDyn::from_bits(val as u8)
    }
    #[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub fn set_pol_dyn(&mut self, val: IdacbPolDyn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_EN==1 and LEG1_MODE==CSD also mixed with the CSD configuration and operation. In mutual cap mode however (see config.mutual_cap) the polarity of the IDAC is controlled by csd_sense. If LEG3_EN=1 (the other two legs must be off) then the ADC sequencer controls the IDACB polarity, optionally mixed with DSI."]
    #[inline(always)]
    pub const fn polarity(&self) -> IdacbPolarity {
        let val = (self.0 >> 8usize) & 0x03;
        IdacbPolarity::from_bits(val as u8)
    }
    #[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_EN==1 and LEG1_MODE==CSD also mixed with the CSD configuration and operation. In mutual cap mode however (see config.mutual_cap) the polarity of the IDAC is controlled by csd_sense. If LEG3_EN=1 (the other two legs must be off) then the ADC sequencer controls the IDACB polarity, optionally mixed with DSI."]
    #[inline(always)]
    pub fn set_polarity(&mut self, val: IdacbPolarity) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub const fn bal_mode(&self) -> IdacbBalMode {
        let val = (self.0 >> 10usize) & 0x03;
        IdacbBalMode::from_bits(val as u8)
    }
    #[doc = "same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub fn set_bal_mode(&mut self, val: IdacbBalMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub const fn leg1_mode(&self) -> IdacbLeg1Mode {
        let val = (self.0 >> 16usize) & 0x03;
        IdacbLeg1Mode::from_bits(val as u8)
    }
    #[doc = "Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub fn set_leg1_mode(&mut self, val: IdacbLeg1Mode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Controls the usage mode of LEG2"]
    #[inline(always)]
    pub const fn leg2_mode(&self) -> IdacbLeg2Mode {
        let val = (self.0 >> 18usize) & 0x03;
        IdacbLeg2Mode::from_bits(val as u8)
    }
    #[doc = "Controls the usage mode of LEG2"]
    #[inline(always)]
    pub fn set_leg2_mode(&mut self, val: IdacbLeg2Mode) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled) 0: no DSI control IDACB_POLARITY = IDACB.POLARITY IDACB_LEG1_EN = IDACB.LEG1_EN IDACB_LEG2_EN = IDACB.LEG2_EN IDACB_LEG3_EN = IDACB.LEG3_EN 1: Mix MMIO with DSI control IDACB_POLARITY = IDACB.POLARITY EXOR dsi_idacb_pol IDACB_LEG1_EN = IDACB.LEG1_EN AND dsi_idacb_leg1_en IDACB_LEG2_EN = IDACB.LEG2_EN AND dsi_idacb_leg2_en IDACB_LEG3_EN = IDACB.LEG3_EN AND dsi_idacb_leg3_en"]
    #[inline(always)]
    pub const fn dsi_ctrl_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled) 0: no DSI control IDACB_POLARITY = IDACB.POLARITY IDACB_LEG1_EN = IDACB.LEG1_EN IDACB_LEG2_EN = IDACB.LEG2_EN IDACB_LEG3_EN = IDACB.LEG3_EN 1: Mix MMIO with DSI control IDACB_POLARITY = IDACB.POLARITY EXOR dsi_idacb_pol IDACB_LEG1_EN = IDACB.LEG1_EN AND dsi_idacb_leg1_en IDACB_LEG2_EN = IDACB.LEG2_EN AND dsi_idacb_leg2_en IDACB_LEG3_EN = IDACB.LEG3_EN AND dsi_idacb_leg3_en"]
    #[inline(always)]
    pub fn set_dsi_ctrl_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "IDAC multiplier"]
    #[inline(always)]
    pub const fn range(&self) -> IdacbRange {
        let val = (self.0 >> 22usize) & 0x03;
        IdacbRange::from_bits(val as u8)
    }
    #[doc = "IDAC multiplier"]
    #[inline(always)]
    pub fn set_range(&mut self, val: IdacbRange) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "output enable for leg 1 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub const fn leg1_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg 1 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub fn set_leg1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "output enable for leg 2 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub const fn leg2_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg 2 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub fn set_leg2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "output enable for leg3 to CSDBUSC, only allowed when RANGE = IDAC_LO. When this bit is set both other legs should be off. Note that leg3 can only be used for ADC mode, not GP mode. Which means that leg3 can only be on when the ADC Sequencer is in the ADC_measure or Calib_measure state. In those states leg3 is controlled by the ADC configuration and the HSCMP output. In addition this leg3 enable bit can optionally be mixed with DSI (see DSI_CTRL_EN). When LEG3_EN=1 also the IDACB polarity is controlled by the ADC sequencer."]
    #[inline(always)]
    pub const fn leg3_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg3 to CSDBUSC, only allowed when RANGE = IDAC_LO. When this bit is set both other legs should be off. Note that leg3 can only be used for ADC mode, not GP mode. Which means that leg3 can only be on when the ADC Sequencer is in the ADC_measure or Calib_measure state. In those states leg3 is controlled by the ADC configuration and the HSCMP output. In addition this leg3 enable bit can optionally be mixed with DSI (see DSI_CTRL_EN). When LEG3_EN=1 also the IDACB polarity is controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn set_leg3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Idacb {
    #[inline(always)]
    fn default() -> Idacb {
        Idacb(0)
    }
}
#[doc = "CSD Interrupt Request Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "A normal sample is complete"]
    #[inline(always)]
    pub const fn sample(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "A normal sample is complete"]
    #[inline(always)]
    pub fn set_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC Result ready"]
    #[inline(always)]
    pub const fn adc_res(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Result ready"]
    #[inline(always)]
    pub fn set_adc_res(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "CSD Interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn sample(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn adc_res(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_adc_res(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "CSD Interrupt masked register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn sample(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn adc_res(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_adc_res(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "CSD Interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn sample(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn adc_res(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_adc_res(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "IO output control Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IoSel(pub u32);
impl IoSel {
    #[doc = "Select waveform for csd_tx_out output signal"]
    #[inline(always)]
    pub const fn csd_tx_out(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Select waveform for csd_tx_out output signal"]
    #[inline(always)]
    pub fn set_csd_tx_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select waveform for csd_tx_out_en output signal"]
    #[inline(always)]
    pub const fn csd_tx_out_en(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Select waveform for csd_tx_out_en output signal"]
    #[inline(always)]
    pub fn set_csd_tx_out_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Select waveform for csd_tx_amuxb_en output signal"]
    #[inline(always)]
    pub const fn csd_tx_amuxb_en(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Select waveform for csd_tx_amuxb_en output signal"]
    #[inline(always)]
    pub fn set_csd_tx_amuxb_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Select waveform for csd_tx_n_out output signal"]
    #[inline(always)]
    pub const fn csd_tx_n_out(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Select waveform for csd_tx_n_out output signal"]
    #[inline(always)]
    pub fn set_csd_tx_n_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Select waveform for csd_tx_n_out_en output signal"]
    #[inline(always)]
    pub const fn csd_tx_n_out_en(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Select waveform for csd_tx_n_out_en output signal"]
    #[inline(always)]
    pub fn set_csd_tx_n_out_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Select waveform for csd_tx_n_amuxa_en output signal"]
    #[inline(always)]
    pub const fn csd_tx_n_amuxa_en(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Select waveform for csd_tx_n_amuxa_en output signal"]
    #[inline(always)]
    pub fn set_csd_tx_n_amuxa_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for IoSel {
    #[inline(always)]
    fn default() -> IoSel {
        IoSel(0)
    }
}
#[doc = "Reference Generator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Refgen(pub u32);
impl Refgen {
    #[doc = "Reference Generator Enable"]
    #[inline(always)]
    pub const fn refgen_en(&self) -> RefgenEn {
        let val = (self.0 >> 0usize) & 0x01;
        RefgenEn::from_bits(val as u8)
    }
    #[doc = "Reference Generator Enable"]
    #[inline(always)]
    pub fn set_refgen_en(&mut self, val: RefgenEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub const fn vdda_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub fn set_vdda_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub const fn res_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub fn set_res_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub const fn gain(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn set_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub const fn vreflo_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn set_vreflo_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    pub const fn vreflo_int(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    pub fn set_vreflo_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Refgen {
    #[inline(always)]
    fn default() -> Refgen {
        Refgen(0)
    }
}
#[doc = "Result CSD/CSX accumulation counter value 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResultVal1(pub u32);
impl ResultVal1 {
    #[doc = "Accumulated counter value for this result. In case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt) this counter counts when csd_sense is high."]
    #[inline(always)]
    pub const fn value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Accumulated counter value for this result. In case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt) this counter counts when csd_sense is high."]
    #[inline(always)]
    pub fn set_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Number of 'bad' conversion for which the CSD comparator did not trigger within the normal time window, either because Vref was not crossed at all, or if the Vref was already crossed before the window started. This counter is reset when the sequencer is started and will saturate at 255 when more than 255 conversions are bad."]
    #[inline(always)]
    pub const fn bad_convs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of 'bad' conversion for which the CSD comparator did not trigger within the normal time window, either because Vref was not crossed at all, or if the Vref was already crossed before the window started. This counter is reset when the sequencer is started and will saturate at 255 when more than 255 conversions are bad."]
    #[inline(always)]
    pub fn set_bad_convs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ResultVal1 {
    #[inline(always)]
    fn default() -> ResultVal1 {
        ResultVal1(0)
    }
}
#[doc = "Result CSX accumulation counter value 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResultVal2(pub u32);
impl ResultVal2 {
    #[doc = "Only used in case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt), this counter counts when csd_sense is low."]
    #[inline(always)]
    pub const fn value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Only used in case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt), this counter counts when csd_sense is low."]
    #[inline(always)]
    pub fn set_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ResultVal2 {
    #[inline(always)]
    fn default() -> ResultVal2 {
        ResultVal2(0)
    }
}
#[doc = "Sense clock duty cycle"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SenseDuty(pub u32);
impl SenseDuty {
    #[doc = "Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    pub const fn sense_width(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    pub fn set_sense_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub const fn sense_pol(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub fn set_sense_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub const fn overlap_phi1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub fn set_overlap_phi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub const fn overlap_phi2(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub fn set_overlap_phi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for SenseDuty {
    #[inline(always)]
    fn default() -> SenseDuty {
        SenseDuty(0)
    }
}
#[doc = "Sense clock period"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SensePeriod(pub u32);
impl SensePeriod {
    #[doc = "The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
    #[inline(always)]
    pub const fn sense_div(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
    #[inline(always)]
    pub fn set_sense_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
    #[inline(always)]
    pub const fn lfsr_size(&self) -> LfsrSize {
        let val = (self.0 >> 16usize) & 0x07;
        LfsrSize::from_bits(val as u8)
    }
    #[doc = "Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
    #[inline(always)]
    pub fn set_lfsr_size(&mut self, val: LfsrSize) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    pub const fn lfsr_scale(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    pub fn set_lfsr_scale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    pub const fn lfsr_clear(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    pub fn set_lfsr_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
    #[inline(always)]
    pub const fn sel_lfsr_msb(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
    #[inline(always)]
    pub fn set_sel_lfsr_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
    #[inline(always)]
    pub const fn lfsr_bits(&self) -> LfsrBits {
        let val = (self.0 >> 26usize) & 0x03;
        LfsrBits::from_bits(val as u8)
    }
    #[doc = "Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
    #[inline(always)]
    pub fn set_lfsr_bits(&mut self, val: LfsrBits) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
}
impl Default for SensePeriod {
    #[inline(always)]
    fn default() -> SensePeriod {
        SensePeriod(0)
    }
}
#[doc = "Sequencer Initial conversion and sample counts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqInitCnt(pub u32);
impl SeqInitCnt {
    #[doc = "Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    pub const fn conv_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    pub fn set_conv_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SeqInitCnt {
    #[inline(always)]
    fn default() -> SeqInitCnt {
        SeqInitCnt(0)
    }
}
#[doc = "Sequencer Normal conversion and sample counts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqNormCnt(pub u32);
impl SeqNormCnt {
    #[doc = "Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    pub const fn conv_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    pub fn set_conv_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SeqNormCnt {
    #[inline(always)]
    fn default() -> SeqNormCnt {
        SeqNormCnt(0)
    }
}
#[doc = "Sequencer start"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqStart(pub u32);
impl SeqStart {
    #[doc = "Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub const fn seq_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub fn set_seq_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub const fn abort(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub fn set_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub const fn dsi_start_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub fn set_dsi_start_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub const fn az0_skip(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub fn set_az0_skip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub const fn az1_skip(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub fn set_az1_skip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for SeqStart {
    #[inline(always)]
    fn default() -> SeqStart {
        SeqStart(0)
    }
}
#[doc = "Sequencer Timing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqTime(pub u32);
impl SeqTime {
    #[doc = "Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub const fn az_time(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub fn set_az_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SeqTime {
    #[inline(always)]
    fn default() -> SeqTime {
        SeqTime(0)
    }
}
#[doc = "Spare MMIO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spare(pub u32);
impl Spare {
    #[doc = "Spare MMIO"]
    #[inline(always)]
    pub const fn spare(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Spare MMIO"]
    #[inline(always)]
    pub fn set_spare(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Spare {
    #[inline(always)]
    fn default() -> Spare {
        Spare(0)
    }
}
#[doc = "Current status counts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatCnts(pub u32);
impl StatCnts {
    #[doc = "Current number of conversions remaining when in Sample_* states (note that in AutoZero* states the same down counter is reused to count the cycles)"]
    #[inline(always)]
    pub const fn num_conv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current number of conversions remaining when in Sample_* states (note that in AutoZero* states the same down counter is reused to count the cycles)"]
    #[inline(always)]
    pub fn set_num_conv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for StatCnts {
    #[inline(always)]
    fn default() -> StatCnts {
        StatCnts(0)
    }
}
#[doc = "Current count of the HSCMP counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatHcnt(pub u32);
impl StatHcnt {
    #[doc = "Current value of HSCMP counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current value of HSCMP counter"]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for StatHcnt {
    #[inline(always)]
    fn default() -> StatHcnt {
        StatHcnt(0)
    }
}
#[doc = "Current Sequencer status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatSeq(pub u32);
impl StatSeq {
    #[doc = "CSD sequencer state"]
    #[inline(always)]
    pub const fn seq_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "CSD sequencer state"]
    #[inline(always)]
    pub fn set_seq_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "ADC sequencer state (only relevant after SEQ_STATE has reached SAMPLE_NORM and ADC sequencer has started)"]
    #[inline(always)]
    pub const fn adc_state(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "ADC sequencer state (only relevant after SEQ_STATE has reached SAMPLE_NORM and ADC sequencer has started)"]
    #[inline(always)]
    pub fn set_adc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for StatSeq {
    #[inline(always)]
    fn default() -> StatSeq {
        StatSeq(0)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Signal used to drive the Cs switches."]
    #[inline(always)]
    pub const fn csd_sense(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Signal used to drive the Cs switches."]
    #[inline(always)]
    pub fn set_csd_sense(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output of reference buffer comparator used to charge up Cmod and/or Csh_tank (synchronized)"]
    #[inline(always)]
    pub const fn hscmp_out(&self) -> HscmpOut {
        let val = (self.0 >> 2usize) & 0x01;
        HscmpOut::from_bits(val as u8)
    }
    #[doc = "Output of reference buffer comparator used to charge up Cmod and/or Csh_tank (synchronized)"]
    #[inline(always)]
    pub fn set_hscmp_out(&mut self, val: HscmpOut) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Output of main sensing comparator (synchronized)"]
    #[inline(always)]
    pub const fn csdcmp_out(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output of main sensing comparator (synchronized)"]
    #[inline(always)]
    pub fn set_csdcmp_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Amuxbuffer switches Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwAmuxbufSel(pub u32);
impl SwAmuxbufSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irby(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irlb(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irlb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_ica(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_ica(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_icb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_icb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irli(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irli(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irh(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irl(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwAmuxbufSel {
    #[inline(always)]
    fn default() -> SwAmuxbufSel {
        SwAmuxbufSel(0)
    }
}
#[doc = "AMUXBUS bypass switches Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwBypSel(pub u32);
impl SwBypSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_bya(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_bya(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_byb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_byb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub const fn sw_cbcc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn set_sw_cbcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for SwBypSel {
    #[inline(always)]
    fn default() -> SwBypSel {
        SwBypSel(0)
    }
}
#[doc = "CSDCMP Neg Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwCmpNSel(pub u32);
impl SwCmpNSel {
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_scrh(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_scrh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_scrl(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_scrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for SwCmpNSel {
    #[inline(always)]
    fn default() -> SwCmpNSel {
        SwCmpNSel(0)
    }
}
#[doc = "CSDCMP Pos Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwCmpPSel(pub u32);
impl SwCmpPSel {
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfpm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfpm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfpt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfps(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfma(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfmb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfca(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfcb(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for SwCmpPSel {
    #[inline(always)]
    fn default() -> SwCmpPSel {
        SwCmpPSel(0)
    }
}
#[doc = "DSI output switch control Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwDsiSel(pub u32);
impl SwDsiSel {
    #[doc = "Select waveform for dsi_csh_tank output signal 0: static open 1: static closed 2: phi1 3: phi2 4: phi1 & HSCMP 5: phi2 & HSCMP 6: HSCMP // ignores phi1/2 7: !sense // = phi1 but ignores OVERLAP_PHI1 8: phi1_delay // phi1 delayed with shield delay 9: phi2_delay // phi2 delayed with shield delay 10: !phi1 11: !phi2 12: !(phi1 & HSCMP) 13: !(phi2 & HSCMP) 14: !HSCMP // ignores phi1/2 15: sense // = phi2 but ignores OVERLAP_PHI2"]
    #[inline(always)]
    pub const fn dsi_csh_tank(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Select waveform for dsi_csh_tank output signal 0: static open 1: static closed 2: phi1 3: phi2 4: phi1 & HSCMP 5: phi2 & HSCMP 6: HSCMP // ignores phi1/2 7: !sense // = phi1 but ignores OVERLAP_PHI1 8: phi1_delay // phi1 delayed with shield delay 9: phi2_delay // phi2 delayed with shield delay 10: !phi1 11: !phi2 12: !(phi1 & HSCMP) 13: !(phi2 & HSCMP) 14: !HSCMP // ignores phi1/2 15: sense // = phi2 but ignores OVERLAP_PHI2"]
    #[inline(always)]
    pub fn set_dsi_csh_tank(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select waveform for dsi_cmod output signal"]
    #[inline(always)]
    pub const fn dsi_cmod(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Select waveform for dsi_cmod output signal"]
    #[inline(always)]
    pub fn set_dsi_cmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for SwDsiSel {
    #[inline(always)]
    fn default() -> SwDsiSel {
        SwDsiSel(0)
    }
}
#[doc = "Full Wave Cmod Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwFwModSel(pub u32);
impl SwFwModSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_f1pm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f1pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f1ma(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f1ma(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f1ca(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f1ca(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c1cc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c1cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c1cd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c1cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c1f1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c1f1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwFwModSel {
    #[inline(always)]
    fn default() -> SwFwModSel {
        SwFwModSel(0)
    }
}
#[doc = "Full Wave Csh_tank Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwFwTankSel(pub u32);
impl SwFwTankSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_f2pt(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f2pt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f2ma(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f2ma(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f2ca(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f2ca(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f2cb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f2cb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c2cc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c2cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c2cd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c2cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c2f2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c2f2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwFwTankSel {
    #[inline(always)]
    fn default() -> SwFwTankSel {
        SwFwTankSel(0)
    }
}
#[doc = "HSCMP Neg input switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwHsNSel(pub u32);
impl SwHsNSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hccc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hccd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hccd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_hcrh(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hcrh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_hcrl(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hcrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for SwHsNSel {
    #[inline(always)]
    fn default() -> SwHsNSel {
        SwHsNSel(0)
    }
}
#[doc = "HSCMP Pos input switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwHsPSel(pub u32);
impl SwHsPSel {
    #[doc = "Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub const fn sw_hmpm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub fn set_sw_hmpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmpt(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmps(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmma(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmmb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmca(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmcb(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmrh(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmrh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwHsPSel {
    #[inline(always)]
    fn default() -> SwHsPSel {
        SwHsPSel(0)
    }
}
#[doc = "Reference Generator Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwRefgenSel(pub u32);
impl SwRefgenSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_iaib(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_iaib(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_ibcb(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_ibcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sgmb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sgmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sgrp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sgrp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sgre(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sgre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sgr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sgr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwRefgenSel {
    #[inline(always)]
    fn default() -> SwRefgenSel {
        SwRefgenSel(0)
    }
}
#[doc = "Switch Resistance configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwRes(pub u32);
impl SwRes {
    #[doc = "Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub const fn res_hcav(&self) -> ResHcav {
        let val = (self.0 >> 0usize) & 0x03;
        ResHcav::from_bits(val as u8)
    }
    #[doc = "Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub fn set_res_hcav(&mut self, val: ResHcav) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub const fn res_hcag(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_hcag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub const fn res_hcbv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_hcbv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub const fn res_hcbg(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_hcbg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Select resistance for the corresponding switch"]
    #[inline(always)]
    pub const fn res_f1pm(&self) -> ResF1pm {
        let val = (self.0 >> 16usize) & 0x03;
        ResF1pm::from_bits(val as u8)
    }
    #[doc = "Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_f1pm(&mut self, val: ResF1pm) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select resistance for the corresponding switch"]
    #[inline(always)]
    pub const fn res_f2pt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_f2pt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for SwRes {
    #[inline(always)]
    fn default() -> SwRes {
        SwRes(0)
    }
}
#[doc = "Shielding switches Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwShieldSel(pub u32);
impl SwShieldSel {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn sw_hcav(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_sw_hcav(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_hcag(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hcag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn sw_hcbv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_sw_hcbv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub const fn sw_hcbg(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub fn set_sw_hcbg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hccv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hccv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub const fn sw_hccg(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn set_sw_hccg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for SwShieldSel {
    #[inline(always)]
    fn default() -> SwShieldSel {
        SwShieldSel(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AdcMode {
    #[doc = "No ADC measurement"]
    OFF = 0,
    #[doc = "Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    VREF_CNT = 0x01,
    #[doc = "Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    VREF_BY2_CNT = 0x02,
    #[doc = "Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    VIN_CNT = 0x03,
}
impl AdcMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcMode {
    #[inline(always)]
    fn from(val: u8) -> AdcMode {
        AdcMode::from_bits(val)
    }
}
impl From<AdcMode> for u8 {
    #[inline(always)]
    fn from(val: AdcMode) -> u8 {
        AdcMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CmpMode {
    #[doc = "CSD mode: output the filtered sample signal on dsi_sample_out"]
    CSD = 0,
    #[doc = "General Purpose mode: output the unfiltered sample unfiltered comparator output, either asynchronous or flopped."]
    GP = 0x01,
}
impl CmpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpMode {
    #[inline(always)]
    fn from(val: u8) -> CmpMode {
        CmpMode::from_bits(val)
    }
}
impl From<CmpMode> for u8 {
    #[inline(always)]
    fn from(val: CmpMode) -> u8 {
        CmpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CmpPhase {
    #[doc = "Comparator is active from start of Phi2 and kept active into Phi1. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    FULL = 0,
    #[doc = "Comparator is active during Phi1 only. Currently no known use-case."]
    PHI1 = 0x01,
    #[doc = "Comparator is active during Phi2 only. Intended usage: CSD Low EMI."]
    PHI2 = 0x02,
    #[doc = "Comparator is activated at the start of both Phi1 and Phi2 (non-overlap should be enabled). Intended usage: CSX, or Full-Wave."]
    PHI1_2 = 0x03,
}
impl CmpPhase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpPhase {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpPhase {
    #[inline(always)]
    fn from(val: u8) -> CmpPhase {
        CmpPhase::from_bits(val)
    }
}
impl From<CmpPhase> for u8 {
    #[inline(always)]
    fn from(val: CmpPhase) -> u8 {
        CmpPhase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CsdcmpEn {
    #[doc = "Disable comparator, output is zero"]
    OFF = 0,
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 0x01,
}
impl CsdcmpEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsdcmpEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsdcmpEn {
    #[inline(always)]
    fn from(val: u8) -> CsdcmpEn {
        CsdcmpEn::from_bits(val)
    }
}
impl From<CsdcmpEn> for u8 {
    #[inline(always)]
    fn from(val: CsdcmpEn) -> u8 {
        CsdcmpEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CsxDualCnt {
    #[doc = "Use one counter for both phases (source and sink)."]
    ONE = 0,
    #[doc = "Use two counters, separate count for when csd_sense is high and when csd_sense is low."]
    TWO = 0x01,
}
impl CsxDualCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsxDualCnt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsxDualCnt {
    #[inline(always)]
    fn from(val: u8) -> CsxDualCnt {
        CsxDualCnt::from_bits(val)
    }
}
impl From<CsxDualCnt> for u8 {
    #[inline(always)]
    fn from(val: CsxDualCnt) -> u8 {
        CsxDualCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DsiCountSel {
    #[doc = "depending on the dsi_count_val_sel input either output RESULT_VAL1.VALUE (0) or RESULT_VAL2.VALUE (1) on the dsi_count bus. Note that dsi_count_val_sel is not synchronized, i.e. it controls the mux combinatorially."]
    CSD_RESULT = 0,
    #[doc = "output ADC_RES.VIN_CNT on the dsi_count bus"]
    ADC_RESULT = 0x01,
}
impl DsiCountSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DsiCountSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DsiCountSel {
    #[inline(always)]
    fn from(val: u8) -> DsiCountSel {
        DsiCountSel::from_bits(val)
    }
}
impl From<DsiCountSel> for u8 {
    #[inline(always)]
    fn from(val: DsiCountSel) -> u8 {
        DsiCountSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FeedbackMode {
    #[doc = "Use feedback from sampling flip-flop (used in most modes)."]
    FLOP = 0,
    #[doc = "Use feedback from comparator directly (used in single Cmod mutual cap sensing only)"]
    COMP = 0x01,
}
impl FeedbackMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FeedbackMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FeedbackMode {
    #[inline(always)]
    fn from(val: u8) -> FeedbackMode {
        FeedbackMode::from_bits(val)
    }
}
impl From<FeedbackMode> for u8 {
    #[inline(always)]
    fn from(val: FeedbackMode) -> u8 {
        FeedbackMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FullWave {
    #[doc = "Half Wave mode (normal). In this mode the comparator always trips in the same direction (positive or negative edge) and the same Vref, i.e. no polarity change."]
    HALFWAVE = 0,
    #[doc = "Full Wave mode. In this mode the comparator trips in opposite direction and with different Vref in each phase, i.e. the polarity flips."]
    FULLWAVE = 0x01,
}
impl FullWave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FullWave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FullWave {
    #[inline(always)]
    fn from(val: u8) -> FullWave {
        FullWave::from_bits(val)
    }
}
impl From<FullWave> for u8 {
    #[inline(always)]
    fn from(val: FullWave) -> u8 {
        FullWave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HscmpEn {
    #[doc = "Disable comparator, output is zero"]
    OFF = 0,
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 0x01,
}
impl HscmpEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HscmpEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HscmpEn {
    #[inline(always)]
    fn from(val: u8) -> HscmpEn {
        HscmpEn::from_bits(val)
    }
}
impl From<HscmpEn> for u8 {
    #[inline(always)]
    fn from(val: HscmpEn) -> u8 {
        HscmpEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HscmpOut {
    #[doc = "Vin < Vref"]
    C_LT_VREF = 0,
    #[doc = "Vin > Vref"]
    C_GT_VREF = 0x01,
}
impl HscmpOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HscmpOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HscmpOut {
    #[inline(always)]
    fn from(val: u8) -> HscmpOut {
        HscmpOut::from_bits(val)
    }
}
impl From<HscmpOut> for u8 {
    #[inline(always)]
    fn from(val: HscmpOut) -> u8 {
        HscmpOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaBalMode {
    #[doc = "enabled from start of Phi2 until disabled by CSDCMP. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    FULL = 0,
    #[doc = "enabled from start of Phi1 and disabled by CSDCMP or at end of Phi1. Enables dual IDAC CSX or Full-Wave, one for sourcing and the other for sinking."]
    PHI1 = 0x01,
    #[doc = "enabled from start of Phi2 and disabled by CSDCMP or at end of Phi2. Intended usage: CSD Low EMI or dual IDAC CSX or Full-Wave."]
    PHI2 = 0x02,
    #[doc = "enabled from start of both Phi1 and Phi2 and disabled by CSDCMP or at end of Phi1 or Phi2 (if non-overlap enabled). Intended usage: single IDAC CSX, or Full-Wave."]
    PHI1_2 = 0x03,
}
impl IdacaBalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaBalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaBalMode {
    #[inline(always)]
    fn from(val: u8) -> IdacaBalMode {
        IdacaBalMode::from_bits(val)
    }
}
impl From<IdacaBalMode> for u8 {
    #[inline(always)]
    fn from(val: IdacaBalMode) -> u8 {
        IdacaBalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaLeg1Mode {
    #[doc = "General Purpose static mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    GP_STATIC = 0,
    #[doc = "General Purpose dynamic mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    GP = 0x01,
    #[doc = "CSD static mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG1 is controlled by LEG1_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    CSD_STATIC = 0x02,
    #[doc = "CSD dynamic mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In thoses states LEG1 is controlled by LEG1_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    CSD = 0x03,
}
impl IdacaLeg1Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaLeg1Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaLeg1Mode {
    #[inline(always)]
    fn from(val: u8) -> IdacaLeg1Mode {
        IdacaLeg1Mode::from_bits(val)
    }
}
impl From<IdacaLeg1Mode> for u8 {
    #[inline(always)]
    fn from(val: IdacaLeg1Mode) -> u8 {
        IdacaLeg1Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaLeg2Mode {
    #[doc = "General Purpose static mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    GP_STATIC = 0,
    #[doc = "General Purpose dynamic mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    GP = 0x01,
    #[doc = "CSD static mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg2 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    CSD_STATIC = 0x02,
    #[doc = "CSD dynamic mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). In addition leg2 enable can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    CSD = 0x03,
}
impl IdacaLeg2Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaLeg2Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaLeg2Mode {
    #[inline(always)]
    fn from(val: u8) -> IdacaLeg2Mode {
        IdacaLeg2Mode::from_bits(val)
    }
}
impl From<IdacaLeg2Mode> for u8 {
    #[inline(always)]
    fn from(val: IdacaLeg2Mode) -> u8 {
        IdacaLeg2Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaPolDyn {
    #[doc = "Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    STATIC = 0,
    #[doc = "Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    DYNAMIC = 0x01,
}
impl IdacaPolDyn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaPolDyn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaPolDyn {
    #[inline(always)]
    fn from(val: u8) -> IdacaPolDyn {
        IdacaPolDyn::from_bits(val)
    }
}
impl From<IdacaPolDyn> for u8 {
    #[inline(always)]
    fn from(val: IdacaPolDyn) -> u8 {
        IdacaPolDyn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaPolarity {
    #[doc = "Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    VSSA_SRC = 0,
    #[doc = "Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    VDDA_SNK = 0x01,
    #[doc = "The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE = 0x02,
    #[doc = "The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE_INV = 0x03,
}
impl IdacaPolarity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaPolarity {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaPolarity {
    #[inline(always)]
    fn from(val: u8) -> IdacaPolarity {
        IdacaPolarity::from_bits(val)
    }
}
impl From<IdacaPolarity> for u8 {
    #[inline(always)]
    fn from(val: IdacaPolarity) -> u8 {
        IdacaPolarity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaRange {
    #[doc = "1 LSB = 37.5 nA"]
    IDAC_LO = 0,
    #[doc = "1 LSB = 300 nA"]
    IDAC_MED = 0x01,
    #[doc = "1 LSB = 2400 nA"]
    IDAC_HI = 0x02,
    _RESERVED_3 = 0x03,
}
impl IdacaRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaRange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaRange {
    #[inline(always)]
    fn from(val: u8) -> IdacaRange {
        IdacaRange::from_bits(val)
    }
}
impl From<IdacaRange> for u8 {
    #[inline(always)]
    fn from(val: IdacaRange) -> u8 {
        IdacaRange::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbBalMode {
    #[doc = "same as corresponding IDACA Balancing mode"]
    FULL = 0,
    #[doc = "same as corresponding IDACA Balancing mode"]
    PHI1 = 0x01,
    #[doc = "same as corresponding IDACA Balancing mode"]
    PHI2 = 0x02,
    #[doc = "same as corresponding IDACA Balancing mode"]
    PHI1_2 = 0x03,
}
impl IdacbBalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbBalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbBalMode {
    #[inline(always)]
    fn from(val: u8) -> IdacbBalMode {
        IdacbBalMode::from_bits(val)
    }
}
impl From<IdacbBalMode> for u8 {
    #[inline(always)]
    fn from(val: IdacbBalMode) -> u8 {
        IdacbBalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbLeg1Mode {
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    GP_STATIC = 0,
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    GP = 0x01,
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    CSD_STATIC = 0x02,
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    CSD = 0x03,
}
impl IdacbLeg1Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbLeg1Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbLeg1Mode {
    #[inline(always)]
    fn from(val: u8) -> IdacbLeg1Mode {
        IdacbLeg1Mode::from_bits(val)
    }
}
impl From<IdacbLeg1Mode> for u8 {
    #[inline(always)]
    fn from(val: IdacbLeg1Mode) -> u8 {
        IdacbLeg1Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbLeg2Mode {
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    GP_STATIC = 0,
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    GP = 0x01,
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    CSD_STATIC = 0x02,
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    CSD = 0x03,
}
impl IdacbLeg2Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbLeg2Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbLeg2Mode {
    #[inline(always)]
    fn from(val: u8) -> IdacbLeg2Mode {
        IdacbLeg2Mode::from_bits(val)
    }
}
impl From<IdacbLeg2Mode> for u8 {
    #[inline(always)]
    fn from(val: IdacbLeg2Mode) -> u8 {
        IdacbLeg2Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbPolDyn {
    #[doc = "Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    STATIC = 0,
    #[doc = "Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    DYNAMIC = 0x01,
}
impl IdacbPolDyn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbPolDyn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbPolDyn {
    #[inline(always)]
    fn from(val: u8) -> IdacbPolDyn {
        IdacbPolDyn::from_bits(val)
    }
}
impl From<IdacbPolDyn> for u8 {
    #[inline(always)]
    fn from(val: IdacbPolDyn) -> u8 {
        IdacbPolDyn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbPolarity {
    #[doc = "Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    VSSA_SRC = 0,
    #[doc = "Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    VDDA_SNK = 0x01,
    #[doc = "The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE = 0x02,
    #[doc = "The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE_INV = 0x03,
}
impl IdacbPolarity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbPolarity {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbPolarity {
    #[inline(always)]
    fn from(val: u8) -> IdacbPolarity {
        IdacbPolarity::from_bits(val)
    }
}
impl From<IdacbPolarity> for u8 {
    #[inline(always)]
    fn from(val: IdacbPolarity) -> u8 {
        IdacbPolarity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbRange {
    #[doc = "1 LSB = 37.5 nA"]
    IDAC_LO = 0,
    #[doc = "1 LSB = 300 nA"]
    IDAC_MED = 0x01,
    #[doc = "1 LSB = 2400 nA"]
    IDAC_HI = 0x02,
    _RESERVED_3 = 0x03,
}
impl IdacbRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbRange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbRange {
    #[inline(always)]
    fn from(val: u8) -> IdacbRange {
        IdacbRange::from_bits(val)
    }
}
impl From<IdacbRange> for u8 {
    #[inline(always)]
    fn from(val: IdacbRange) -> u8 {
        IdacbRange::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IrefSel {
    #[doc = "select SRSS Iref (default)"]
    IREF_SRSS = 0,
    #[doc = "select PASS.AREF Iref, only available if PASS IP is on the chip."]
    IREF_PASS = 0x01,
}
impl IrefSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrefSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrefSel {
    #[inline(always)]
    fn from(val: u8) -> IrefSel {
        IrefSel::from_bits(val)
    }
}
impl From<IrefSel> for u8 {
    #[inline(always)]
    fn from(val: IrefSel) -> u8 {
        IrefSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfsrBits {
    #[doc = "use 2 bits: range = \\[-2,1\\]"]
    _2B = 0,
    #[doc = "use 3 bits: range = \\[-4,3\\]"]
    _3B = 0x01,
    #[doc = "use 4 bits: range = \\[-8,7\\]"]
    _4B = 0x02,
    #[doc = "use 5 bits: range = \\[-16,15\\] (default)"]
    _5B = 0x03,
}
impl LfsrBits {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfsrBits {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfsrBits {
    #[inline(always)]
    fn from(val: u8) -> LfsrBits {
        LfsrBits::from_bits(val)
    }
}
impl From<LfsrBits> for u8 {
    #[inline(always)]
    fn from(val: LfsrBits) -> u8 {
        LfsrBits::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfsrSize {
    #[doc = "Don't use clock dithering (=spreadspectrum) (LFSR output value is zero)"]
    OFF = 0,
    #[doc = "6-bit LFSR (G(x)=X^6 +X^4+X^3+ X+1, period= 63)"]
    _6B = 0x01,
    #[doc = "7-bit LFSR (G(x)=X^7 +X^4+X^3+X^2+1, period= 127)"]
    _7B = 0x02,
    #[doc = "9-bit LFSR (G(x)=X^9 +X^4+X^3+ X+1, period= 511)"]
    _9B = 0x03,
    #[doc = "10-bit LFSR (G(x)=X^10+X^4+X^3+ X+1, period= 1023)"]
    _10B = 0x04,
    #[doc = "8-bit LFSR (G(x)=X^8+X^4+X^3+X^2+1, period= 255)"]
    _8B = 0x05,
    #[doc = "12-bit LFSR (G(x)=X^12+X^7+X^4+X^3+1, period= 4095)"]
    _12B = 0x06,
    _RESERVED_7 = 0x07,
}
impl LfsrSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfsrSize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfsrSize {
    #[inline(always)]
    fn from(val: u8) -> LfsrSize {
        LfsrSize::from_bits(val)
    }
}
impl From<LfsrSize> for u8 {
    #[inline(always)]
    fn from(val: LfsrSize) -> u8 {
        LfsrSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MutualCap {
    #[doc = "Self-cap mode (configure sense line as CSD_SENSE)"]
    SELFCAP = 0,
    #[doc = "Mutual-cap mode (configure Tx line as CSD_SENSE, inverted Tx line as CSD_SHIELD and Rx Line as AMUXA). In this mode the polarity bit of the IDAC is controlled by csd_sense."]
    MUTUALCAP = 0x01,
}
impl MutualCap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MutualCap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MutualCap {
    #[inline(always)]
    fn from(val: u8) -> MutualCap {
        MutualCap::from_bits(val)
    }
}
impl From<MutualCap> for u8 {
    #[inline(always)]
    fn from(val: MutualCap) -> u8 {
        MutualCap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PolaritySel {
    #[doc = "Use idaca_pol (firmware setting with CSX and optionally DSI mixed in) to determine the direction, this is the most common use-case, used for normal CSD and normal CSX"]
    IDACA_POL = 0,
    #[doc = "Use idacb_pol (firmware setting with optional DSI mixed in) to determine the direction, this is only used for normal CSD if IDACB is used i.s.o. IDACA (not common)"]
    IDACB_POL = 0x01,
    #[doc = "Use the expression (csd_sense ? idaca_pol : idacb_pol) to determine the direction, this is only useful for the CSX with DUAL_IDAC use-case"]
    DUAL_POL = 0x02,
    _RESERVED_3 = 0x03,
}
impl PolaritySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolaritySel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolaritySel {
    #[inline(always)]
    fn from(val: u8) -> PolaritySel {
        PolaritySel::from_bits(val)
    }
}
impl From<PolaritySel> for u8 {
    #[inline(always)]
    fn from(val: PolaritySel) -> u8 {
        PolaritySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PwrMode {
    #[doc = "Disable buffer"]
    OFF = 0,
    #[doc = "On, normal or low power level depending on CONFIG.LP_MODE."]
    NORM = 0x01,
    #[doc = "On, high or low power level depending on CONFIG.LP_MODE."]
    HI = 0x02,
    _RESERVED_3 = 0x03,
}
impl PwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrMode {
    #[inline(always)]
    fn from(val: u8) -> PwrMode {
        PwrMode::from_bits(val)
    }
}
impl From<PwrMode> for u8 {
    #[inline(always)]
    fn from(val: PwrMode) -> u8 {
        PwrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RefgenEn {
    #[doc = "Disable Reference Generator"]
    OFF = 0,
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 0x01,
}
impl RefgenEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefgenEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefgenEn {
    #[inline(always)]
    fn from(val: u8) -> RefgenEn {
        RefgenEn::from_bits(val)
    }
}
impl From<RefgenEn> for u8 {
    #[inline(always)]
    fn from(val: RefgenEn) -> u8 {
        RefgenEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResF1pm {
    #[doc = "Low"]
    LOW = 0,
    #[doc = "Medium"]
    MED = 0x01,
    #[doc = "High"]
    HIGH = 0x02,
    #[doc = "N/A"]
    RSVD = 0x03,
}
impl ResF1pm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ResF1pm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ResF1pm {
    #[inline(always)]
    fn from(val: u8) -> ResF1pm {
        ResF1pm::from_bits(val)
    }
}
impl From<ResF1pm> for u8 {
    #[inline(always)]
    fn from(val: ResF1pm) -> u8 {
        ResF1pm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResHcav {
    #[doc = "Low"]
    LOW = 0,
    #[doc = "Medium"]
    MED = 0x01,
    #[doc = "High"]
    HIGH = 0x02,
    #[doc = "Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    LOWEMI = 0x03,
}
impl ResHcav {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ResHcav {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ResHcav {
    #[inline(always)]
    fn from(val: u8) -> ResHcav {
        ResHcav::from_bits(val)
    }
}
impl From<ResHcav> for u8 {
    #[inline(always)]
    fn from(val: ResHcav) -> u8 {
        ResHcav::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ShieldDelay {
    #[doc = "Delay line is off, csd_shield=csd_sense"]
    OFF = 0,
    #[doc = "Introduces a 5ns delay (typ)"]
    D5NS = 0x01,
    #[doc = "Introduces a 10ns delay (typ)"]
    D10NS = 0x02,
    #[doc = "Introduces a 20ns delay (typ)"]
    D20NS = 0x03,
}
impl ShieldDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShieldDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShieldDelay {
    #[inline(always)]
    fn from(val: u8) -> ShieldDelay {
        ShieldDelay::from_bits(val)
    }
}
impl From<ShieldDelay> for u8 {
    #[inline(always)]
    fn from(val: ShieldDelay) -> u8 {
        ShieldDelay::to_bits(val)
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
