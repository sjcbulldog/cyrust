#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Flash controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashc {
    ptr: *mut u8,
}
unsafe impl Send for Flashc {}
unsafe impl Sync for Flashc {}
impl Flashc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn flash_ctl(self) -> crate::common::Reg<FlashCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Flash power control"]
    #[inline(always)]
    pub const fn flash_pwr_ctl(self) -> crate::common::Reg<FlashPwrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Command"]
    #[inline(always)]
    pub const fn flash_cmd(self) -> crate::common::Reg<FlashCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(self) -> crate::common::Reg<EccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(672usize) as _) }
    }
    #[doc = "eCT Flash SRAM ECC control 0"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl0(self) -> crate::common::Reg<FmSramEccCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(688usize) as _) }
    }
    #[doc = "eCT Flash SRAM ECC control 1"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl1(self) -> crate::common::Reg<FmSramEccCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(692usize) as _) }
    }
    #[doc = "eCT Flash SRAM ECC control 2"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl2(self) -> crate::common::Reg<FmSramEccCtl2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(696usize) as _) }
    }
    #[doc = "eCT Flash SRAM ECC control 3"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl3(self) -> crate::common::Reg<FmSramEccCtl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(700usize) as _) }
    }
    #[doc = "CM0+ cache control"]
    #[inline(always)]
    pub const fn cm0_ca_ctl0(self) -> crate::common::Reg<Cm0CaCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "CM0+ cache control"]
    #[inline(always)]
    pub const fn cm0_ca_ctl1(self) -> crate::common::Reg<Cm0CaCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1028usize) as _) }
    }
    #[doc = "CM0+ cache control"]
    #[inline(always)]
    pub const fn cm0_ca_ctl2(self) -> crate::common::Reg<Cm0CaCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1032usize) as _) }
    }
    #[doc = "CM0+ cache status 0"]
    #[inline(always)]
    pub const fn cm0_ca_status0(self) -> crate::common::Reg<Cm0CaStatus0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1088usize) as _) }
    }
    #[doc = "CM0+ cache status 1"]
    #[inline(always)]
    pub const fn cm0_ca_status1(self) -> crate::common::Reg<Cm0CaStatus1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1092usize) as _) }
    }
    #[doc = "CM0+ cache status 2"]
    #[inline(always)]
    pub const fn cm0_ca_status2(self) -> crate::common::Reg<Cm0CaStatus2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1096usize) as _) }
    }
    #[doc = "CM0+ interface status"]
    #[inline(always)]
    pub const fn cm0_status(self) -> crate::common::Reg<Cm0Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1120usize) as _) }
    }
    #[doc = "CM4 cache control"]
    #[inline(always)]
    pub const fn cm4_ca_ctl0(self) -> crate::common::Reg<Cm4CaCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1152usize) as _) }
    }
    #[doc = "CM4 cache control"]
    #[inline(always)]
    pub const fn cm4_ca_ctl1(self) -> crate::common::Reg<Cm4CaCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1156usize) as _) }
    }
    #[doc = "CM4 cache control"]
    #[inline(always)]
    pub const fn cm4_ca_ctl2(self) -> crate::common::Reg<Cm4CaCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1160usize) as _) }
    }
    #[doc = "CM4 cache status 0"]
    #[inline(always)]
    pub const fn cm4_ca_status0(self) -> crate::common::Reg<Cm4CaStatus0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1216usize) as _) }
    }
    #[doc = "CM4 cache status 1"]
    #[inline(always)]
    pub const fn cm4_ca_status1(self) -> crate::common::Reg<Cm4CaStatus1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1220usize) as _) }
    }
    #[doc = "CM4 cache status 2"]
    #[inline(always)]
    pub const fn cm4_ca_status2(self) -> crate::common::Reg<Cm4CaStatus2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1224usize) as _) }
    }
    #[doc = "CM4 interface status"]
    #[inline(always)]
    pub const fn cm4_status(self) -> crate::common::Reg<Cm4Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1248usize) as _) }
    }
    #[doc = "Cryptography buffer control"]
    #[inline(always)]
    pub const fn crypto_buff_ctl(self) -> crate::common::Reg<CryptoBuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1280usize) as _) }
    }
    #[doc = "Datawire 0 buffer control"]
    #[inline(always)]
    pub const fn dw0_buff_ctl(self) -> crate::common::Reg<Dw0BuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1408usize) as _) }
    }
    #[doc = "Datawire 1 buffer control"]
    #[inline(always)]
    pub const fn dw1_buff_ctl(self) -> crate::common::Reg<Dw1BuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1536usize) as _) }
    }
    #[doc = "DMA controller buffer control"]
    #[inline(always)]
    pub const fn dmac_buff_ctl(self) -> crate::common::Reg<DmacBuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1664usize) as _) }
    }
    #[doc = "External master 0 buffer control"]
    #[inline(always)]
    pub const fn ext_ms0_buff_ctl(self) -> crate::common::Reg<ExtMs0BuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1792usize) as _) }
    }
    #[doc = "External master 1 buffer control"]
    #[inline(always)]
    pub const fn ext_ms1_buff_ctl(self) -> crate::common::Reg<ExtMs1BuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1920usize) as _) }
    }
    #[doc = "Flash Macro Registers"]
    #[inline(always)]
    pub const fn fm_ctl(self) -> FmCtl {
        unsafe { FmCtl::from_ptr(self.ptr.add(61440usize) as _) }
    }
}
#[doc = "Flash Macro Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmCtl {
    ptr: *mut u8,
}
unsafe impl Send for FmCtl {}
unsafe impl Sync for FmCtl {}
impl FmCtl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash macro control"]
    #[inline(always)]
    pub const fn fm_ctl_reg(self) -> crate::common::Reg<FmCtlReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Flash macro address"]
    #[inline(always)]
    pub const fn fm_addr(self) -> crate::common::Reg<FmAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Bookmark register - keeps the current FW HV seq"]
    #[inline(always)]
    pub const fn bookmark(self) -> crate::common::Reg<Bookmark, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Regular flash geometry"]
    #[inline(always)]
    pub const fn geometry(self) -> crate::common::Reg<Geometry, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Supervisory flash geometry"]
    #[inline(always)]
    pub const fn geometry_supervisory(
        self,
    ) -> crate::common::Reg<GeometrySupervisory, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Analog control 0"]
    #[inline(always)]
    pub const fn ana_ctl0(self) -> crate::common::Reg<AnaCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Analog control 1"]
    #[inline(always)]
    pub const fn ana_ctl1(self) -> crate::common::Reg<AnaCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Wait State control"]
    #[inline(always)]
    pub const fn wait_ctl(self) -> crate::common::Reg<WaitCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Timer prescaler (clk_t to timer clock frequency divider)"]
    #[inline(always)]
    pub const fn timer_clk_ctl(self) -> crate::common::Reg<TimerClkCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Timer control"]
    #[inline(always)]
    pub const fn timer_ctl(self) -> crate::common::Reg<TimerCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "MPCON clock"]
    #[inline(always)]
    pub const fn aclk_ctl(self) -> crate::common::Reg<AclkCtl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Cal control BG LO trim bits"]
    #[inline(always)]
    pub const fn cal_ctl0(self) -> crate::common::Reg<CalCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Cal control BG HI trim bits"]
    #[inline(always)]
    pub const fn cal_ctl1(self) -> crate::common::Reg<CalCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Cal control BG LO&HI trim bits"]
    #[inline(always)]
    pub const fn cal_ctl2(self) -> crate::common::Reg<CalCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Cal control osc trim bits, idac, sdac, itim"]
    #[inline(always)]
    pub const fn cal_ctl3(self) -> crate::common::Reg<CalCtl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "Cal Control Vlim, SA, fdiv, reg_act"]
    #[inline(always)]
    pub const fn cal_ctl4(self) -> crate::common::Reg<CalCtl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Cal control"]
    #[inline(always)]
    pub const fn cal_ctl5(self) -> crate::common::Reg<CalCtl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "SA trim LP/ULP"]
    #[inline(always)]
    pub const fn cal_ctl6(self) -> crate::common::Reg<CalCtl6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "Cal control"]
    #[inline(always)]
    pub const fn cal_ctl7(self) -> crate::common::Reg<CalCtl7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "Redundancy Control normal sectors 0,1"]
    #[inline(always)]
    pub const fn red_ctl01(self) -> crate::common::Reg<RedCtl01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Redundancy Control normal sectors 2,3"]
    #[inline(always)]
    pub const fn red_ctl23(self) -> crate::common::Reg<RedCtl23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Redundancy Control normal sectors 4,5"]
    #[inline(always)]
    pub const fn red_ctl45(self) -> crate::common::Reg<RedCtl45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Redundancy Control normal sectors 6,7"]
    #[inline(always)]
    pub const fn red_ctl67(self) -> crate::common::Reg<RedCtl67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "Redundancy Control special sectors 0,1"]
    #[inline(always)]
    pub const fn red_ctl_sm01(self) -> crate::common::Reg<RedCtlSm01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "R-grant delay for program"]
    #[inline(always)]
    pub const fn rgrant_delay_prg(self) -> crate::common::Reg<RgrantDelayPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
    #[doc = "HV Pulse Delay for seq 1&2 pre"]
    #[inline(always)]
    pub const fn pw_seq12(self) -> crate::common::Reg<PwSeq12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "HV Pulse Delay for seq2 post & seq3"]
    #[inline(always)]
    pub const fn pw_seq23(self) -> crate::common::Reg<PwSeq23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "R-grant delay scale for erase"]
    #[inline(always)]
    pub const fn rgrant_scale_ers(self) -> crate::common::Reg<RgrantScaleErs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "R-grant delay for erase"]
    #[inline(always)]
    pub const fn rgrant_delay_ers(self) -> crate::common::Reg<RgrantDelayErs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(172usize) as _) }
    }
    #[doc = "Flash macro write page latches all"]
    #[inline(always)]
    pub const fn fm_pl_wrdata_all(self) -> crate::common::Reg<FmPlWrdataAll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2044usize) as _) }
    }
    #[doc = "Flash macro Page Latches data"]
    #[inline(always)]
    pub const fn fm_pl_data(self, n: usize) -> crate::common::Reg<FmPlData, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2048usize + n * 4usize) as _) }
    }
    #[doc = "Flash macro memory sense amplifier and column decoder data"]
    #[inline(always)]
    pub const fn fm_mem_data(self, n: usize) -> crate::common::Reg<FmMemData, crate::common::R> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3072usize + n * 4usize) as _) }
    }
}
#[doc = "MPCON clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AclkCtl(pub u32);
impl AclkCtl {
    #[doc = "A write to this register generates the clock pulse for HV control registers (mpcon outputs)"]
    #[inline(always)]
    pub const fn aclk_gen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "A write to this register generates the clock pulse for HV control registers (mpcon outputs)"]
    #[inline(always)]
    pub fn set_aclk_gen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for AclkCtl {
    #[inline(always)]
    fn default() -> AclkCtl {
        AclkCtl(0)
    }
}
#[doc = "Analog control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnaCtl0(pub u32);
impl AnaCtl0 {
    #[doc = "Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub const fn mdac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn set_mdac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trimming of common source line DAC."]
    #[inline(always)]
    pub const fn csldac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Trimming of common source line DAC."]
    #[inline(always)]
    pub fn set_csldac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
    #[inline(always)]
    pub const fn flip_amuxbus_ab(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn set_flip_amuxbus_ab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "NDAC staircase min value"]
    #[inline(always)]
    pub const fn ndac_min(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "NDAC staircase min value"]
    #[inline(always)]
    pub fn set_ndac_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "PDAC staircase min value"]
    #[inline(always)]
    pub const fn pdac_min(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "PDAC staircase min value"]
    #[inline(always)]
    pub fn set_pdac_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_prg_seq01(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_prg_seq01(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_prg_seq12(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_prg_seq12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_prg_seq23(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_prg_seq23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "PROG&PRE_PROG& ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_seq30(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "PROG&PRE_PROG& ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_seq30(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_prg_peon(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_prg_peon(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_prg_peoff(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_prg_peoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for AnaCtl0 {
    #[inline(always)]
    fn default() -> AnaCtl0 {
        AnaCtl0(0)
    }
}
#[doc = "Analog control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnaCtl1(pub u32);
impl AnaCtl1 {
    #[doc = "Ndac Max Value.Trimming of negative pump output Voltage."]
    #[inline(always)]
    pub const fn ndac_max(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Ndac Max Value.Trimming of negative pump output Voltage."]
    #[inline(always)]
    pub fn set_ndac_max(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Ndac step increment"]
    #[inline(always)]
    pub const fn ndac_step(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Ndac step increment"]
    #[inline(always)]
    pub fn set_ndac_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Pdac Max Value.Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub const fn pdac_max(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Pdac Max Value.Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn set_pdac_max(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Pdac step increment"]
    #[inline(always)]
    pub const fn pdac_step(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Pdac step increment"]
    #[inline(always)]
    pub fn set_pdac_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
    #[inline(always)]
    pub const fn npdac_step_time(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
    #[inline(always)]
    pub fn set_npdac_step_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
    #[inline(always)]
    pub const fn npdac_zero_time(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
    #[inline(always)]
    pub fn set_npdac_zero_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for AnaCtl1 {
    #[inline(always)]
    fn default() -> AnaCtl1 {
        AnaCtl1(0)
    }
}
#[doc = "Bookmark register - keeps the current FW HV seq"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bookmark(pub u32);
impl Bookmark {
    #[doc = "Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    pub const fn bookmark(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    pub fn set_bookmark(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bookmark {
    #[inline(always)]
    fn default() -> Bookmark {
        Bookmark(0)
    }
}
#[doc = "Cal control BG LO trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCtl0(pub u32);
impl CalCtl0 {
    #[doc = "LO Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub const fn vct_trim_lo_hv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "LO Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn set_vct_trim_lo_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub const fn cdac_lo_hv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub fn set_cdac_lo_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "LO Bandgap Voltage trim control."]
    #[inline(always)]
    pub const fn vbg_trim_lo_hv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "LO Bandgap Voltage trim control."]
    #[inline(always)]
    pub fn set_vbg_trim_lo_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "LO Bandgap Voltage Temperature Compensation trim control"]
    #[inline(always)]
    pub const fn vbg_tc_trim_lo_hv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "LO Bandgap Voltage Temperature Compensation trim control"]
    #[inline(always)]
    pub fn set_vbg_tc_trim_lo_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub const fn icref_tc_trim_lo_hv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn set_icref_tc_trim_lo_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Adds 100-150nA boost on IPREF_LO"]
    #[inline(always)]
    pub const fn ipref_trima_lo_hv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Adds 100-150nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn set_ipref_trima_lo_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CalCtl0 {
    #[inline(always)]
    fn default() -> CalCtl0 {
        CalCtl0(0)
    }
}
#[doc = "Cal control BG HI trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCtl1(pub u32);
impl CalCtl1 {
    #[doc = "HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub const fn vct_trim_hi_hv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn set_vct_trim_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub const fn cdac_hi_hv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub fn set_cdac_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "HI Bandgap Voltage trim control."]
    #[inline(always)]
    pub const fn vbg_trim_hi_hv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "HI Bandgap Voltage trim control."]
    #[inline(always)]
    pub fn set_vbg_trim_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub const fn vbg_tc_trim_hi_hv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn set_vbg_tc_trim_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    pub const fn icref_tc_trim_hi_hv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    pub fn set_icref_tc_trim_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Adds 100-150nA boost on IPREF_HI"]
    #[inline(always)]
    pub const fn ipref_trima_hi_hv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Adds 100-150nA boost on IPREF_HI"]
    #[inline(always)]
    pub fn set_ipref_trima_hi_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CalCtl1 {
    #[inline(always)]
    fn default() -> CalCtl1 {
        CalCtl1(0)
    }
}
#[doc = "Cal control BG LO&HI trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCtl2(pub u32);
impl CalCtl2 {
    #[doc = "LO Bandgap Current trim control."]
    #[inline(always)]
    pub const fn icref_trim_lo_hv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn set_icref_trim_lo_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "HI Bandgap Current trim control."]
    #[inline(always)]
    pub const fn icref_trim_hi_hv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn set_icref_trim_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "LO Bandgap IPTAT trim control."]
    #[inline(always)]
    pub const fn ipref_trim_lo_hv(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "LO Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn set_ipref_trim_lo_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub const fn ipref_trim_hi_hv(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn set_ipref_trim_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
}
impl Default for CalCtl2 {
    #[inline(always)]
    fn default() -> CalCtl2 {
        CalCtl2(0)
    }
}
#[doc = "Cal control osc trim bits, idac, sdac, itim"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCtl3(pub u32);
impl CalCtl3 {
    #[doc = "Flash macro pump clock trim control."]
    #[inline(always)]
    pub const fn osc_trim_hv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Flash macro pump clock trim control."]
    #[inline(always)]
    pub fn set_osc_trim_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub const fn osc_range_trim_hv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub fn set_osc_range_trim_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub const fn vprot_act_hv(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub fn set_vprot_act_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
    #[inline(always)]
    pub const fn ipref_tc_hv(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0: Increases the IPREF Tempco by subtracting ICREF from IPREF - IPREF internal will be 0.5uA 1: Reduces the IPREF Tempco without subtracting ICREF from IPREF - IPREF internal will be 1uA"]
    #[inline(always)]
    pub fn set_ipref_tc_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
    #[inline(always)]
    pub const fn vref_sel_hv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage reference: 0: internal bandgap reference 1: external voltage reference"]
    #[inline(always)]
    pub fn set_vref_sel_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Current reference: 0: internal current reference 1: external current reference"]
    #[inline(always)]
    pub const fn iref_sel_hv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Current reference: 0: internal current reference 1: external current reference"]
    #[inline(always)]
    pub fn set_iref_sel_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub const fn reg_act_hv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0: VBST regulator will operate in active/standby mode based on control signal. 1: Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn set_reg_act_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
    #[inline(always)]
    pub const fn fdiv_trim_hv(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "FDIV_TRIM_HV\\[1:0\\]: Assuming oscillator frequency of 8MHz in standby. Following are the clock frequencies seen by doubler 00: F = 1MHz 01: F = 0.5MHz 10: F = 2MHz 11: F = 4MHz"]
    #[inline(always)]
    pub fn set_fdiv_trim_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "0: vdd < 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
    #[inline(always)]
    pub const fn vddhi_hv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0: vdd < 2.3V 1: vdd >= 2.3V '0' setting can used for vdd > 2.3V also, but with a current penalty."]
    #[inline(always)]
    pub fn set_vddhi_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
    #[inline(always)]
    pub const fn turbo_pulsew_hv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Turbo pulse width trim (Typical) 00: 40 us 01: 20 us 10: 15 us 11: 8 us"]
    #[inline(always)]
    pub fn set_turbo_pulsew_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
    #[inline(always)]
    pub const fn bglo_en_hv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0: Normal (Automatic change over from HI to LO) 1: Force enable LO Bandgap"]
    #[inline(always)]
    pub fn set_bglo_en_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
    #[inline(always)]
    pub const fn bghi_en_hv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0: Normal (Automatic change over from HI to LO) 1: Force enable HI Bandgap When both BGLO_EN_HV and BGHI_EN_HV are HIGH, only BGHI output is used and turbo_hv_n pulse is active"]
    #[inline(always)]
    pub fn set_bghi_en_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0: The internal logic controls the CL isolation 1: Forces CL bypass"]
    #[inline(always)]
    pub const fn cl_iso_dis_hv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0: The internal logic controls the CL isolation 1: Forces CL bypass"]
    #[inline(always)]
    pub fn set_cl_iso_dis_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
    #[inline(always)]
    pub const fn r_grant_en_hv(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0: r_grant handshake disabled, r_grant always 1. 1: r_grand handshake enabled"]
    #[inline(always)]
    pub fn set_r_grant_en_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LP<-->ULP switch for trim signals: 0: LP 1: ULP"]
    #[inline(always)]
    pub const fn lp_ulp_sw_hv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LP<-->ULP switch for trim signals: 0: LP 1: ULP"]
    #[inline(always)]
    pub fn set_lp_ulp_sw_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CalCtl3 {
    #[inline(always)]
    fn default() -> CalCtl3 {
        CalCtl3(0)
    }
}
#[doc = "Cal Control Vlim, SA, fdiv, reg_act"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCtl4(pub u32);
impl CalCtl4 {
    #[doc = "VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub const fn vlim_trim_ulp_hv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub fn set_vlim_trim_ulp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub const fn idac_ulp_hv(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub fn set_idac_ulp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub const fn sdac_ulp_hv(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub fn set_sdac_ulp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Trimming of timing current"]
    #[inline(always)]
    pub const fn itim_ulp_hv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Trimming of timing current"]
    #[inline(always)]
    pub fn set_itim_ulp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub const fn fm_ready_del_ulp_hv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub fn set_fm_ready_del_ulp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn spare451_ulp_hv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_spare451_ulp_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
    #[inline(always)]
    pub const fn ready_restart_n_hv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
    #[inline(always)]
    pub fn set_ready_restart_n_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
    #[inline(always)]
    pub const fn vbst_s_dis_hv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
    #[inline(always)]
    pub fn set_vbst_s_dis_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
    #[inline(always)]
    pub const fn auto_hvpulse_hv(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
    #[inline(always)]
    pub fn set_auto_hvpulse_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "UGB enable in TM control"]
    #[inline(always)]
    pub const fn ugb_en_hv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "UGB enable in TM control"]
    #[inline(always)]
    pub fn set_ugb_en_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CalCtl4 {
    #[inline(always)]
    fn default() -> CalCtl4 {
        CalCtl4(0)
    }
}
#[doc = "Cal control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCtl5(pub u32);
impl CalCtl5 {
    #[doc = "VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub const fn vlim_trim_lp_hv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub fn set_vlim_trim_lp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub const fn idac_lp_hv(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub fn set_idac_lp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub const fn sdac_lp_hv(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub fn set_sdac_lp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Trimming of timing current"]
    #[inline(always)]
    pub const fn itim_lp_hv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Trimming of timing current"]
    #[inline(always)]
    pub fn set_itim_lp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "00: Delayed by 1us 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub const fn fm_ready_del_lp_hv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "00: Delayed by 1us 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub fn set_fm_ready_del_lp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn spare451_lp_hv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_spare451_lp_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn spare52_hv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_spare52_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Amux Select in AMUX_UGB 00: Bypass UGB for both amuxbusa and amuxbusb 01: Bypass UGB for amuxbusb while passing amuxbusa through UGB. 10: Bypass UGB for amuxbusa while passing amuxbusb through UGB. 11: UGB Calibrate mode"]
    #[inline(always)]
    pub const fn amux_sel_hv(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Amux Select in AMUX_UGB 00: Bypass UGB for both amuxbusa and amuxbusb 01: Bypass UGB for amuxbusb while passing amuxbusa through UGB. 10: Bypass UGB for amuxbusa while passing amuxbusb through UGB. 11: UGB Calibrate mode"]
    #[inline(always)]
    pub fn set_amux_sel_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for CalCtl5 {
    #[inline(always)]
    fn default() -> CalCtl5 {
        CalCtl5(0)
    }
}
#[doc = "SA trim LP/ULP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCtl6(pub u32);
impl CalCtl6 {
    #[doc = "clk_trk delay"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t1_ulp_hv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "clk_trk delay"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t1_ulp_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SA_CTL_TRIM_T4_ULP_HV<2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV<1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t4_ulp_hv(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "SA_CTL_TRIM_T4_ULP_HV<2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV<1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t4_ulp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "SA_CTL_TRIM_T5_ULP_HV<2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV<1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t5_ulp_hv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "SA_CTL_TRIM_T5_ULP_HV<2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV<1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t5_ulp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "SA_CTL_TRIM_T6_ULP_HV<1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV<0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t6_ulp_hv(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "SA_CTL_TRIM_T6_ULP_HV<1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV<0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t6_ulp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t8_ulp_hv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t8_ulp_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "clk_trk delay"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t1_lp_hv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "clk_trk delay"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t1_lp_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SA_CTL_TRIM_T4_LP_HV<2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV<1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t4_lp_hv(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "SA_CTL_TRIM_T4_LP_HV<2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV<1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t4_lp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[doc = "SA_CTL_TRIM_T5_LP_HV<2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV<1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t5_lp_hv(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "SA_CTL_TRIM_T5_LP_HV<2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV<1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t5_lp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[doc = "SA_CTL_TRIM_T6_LP_HV<1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV<0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t6_lp_hv(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "SA_CTL_TRIM_T6_LP_HV<1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV<0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t6_lp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub const fn sa_ctl_trim_t8_lp_hv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn set_sa_ctl_trim_t8_lp_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CalCtl6 {
    #[inline(always)]
    fn default() -> CalCtl6 {
        CalCtl6(0)
    }
}
#[doc = "Cal control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCtl7(pub u32);
impl CalCtl7 {
    #[doc = "Clock frequency into the ersx8 shift register block 00: Oscillator clock 01: Oscillator clock / 2 10: Oscillator clock / 4 11: Oscillator clock"]
    #[inline(always)]
    pub const fn ersx8_clk_sel_hv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Clock frequency into the ersx8 shift register block 00: Oscillator clock 01: Oscillator clock / 2 10: Oscillator clock / 4 11: Oscillator clock"]
    #[inline(always)]
    pub fn set_ersx8_clk_sel_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "0: Normal operation 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub const fn fm_active_hv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0: Normal operation 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub fn set_fm_active_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0: Normal operation 1: Uses external turbo pulse"]
    #[inline(always)]
    pub const fn turbo_ext_hv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0: Normal operation 1: Uses external turbo pulse"]
    #[inline(always)]
    pub fn set_turbo_ext_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0': ndac, pdac staircase hardware controlled 1: ndac, pdac staircase disabled. Enables FW control."]
    #[inline(always)]
    pub const fn npdac_hwctl_dis_hv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0': ndac, pdac staircase hardware controlled 1: ndac, pdac staircase disabled. Enables FW control."]
    #[inline(always)]
    pub fn set_npdac_hwctl_dis_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0': fm ready is enabled 1: fm ready is disabled (fm_ready is always '1')"]
    #[inline(always)]
    pub const fn fm_ready_dis_hv(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0': fm ready is enabled 1: fm ready is disabled (fm_ready is always '1')"]
    #[inline(always)]
    pub fn set_fm_ready_dis_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0': Staggered turn on/off of GWL 1: GWL are turned on/off at the same time (old FM legacy)"]
    #[inline(always)]
    pub const fn ersx8_en_all_hv(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0': Staggered turn on/off of GWL 1: GWL are turned on/off at the same time (old FM legacy)"]
    #[inline(always)]
    pub fn set_ersx8_en_all_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0: Load common HV params during API HV operations depends on the HV_PARAMS_LOADED bit in RGRANT_DELAY_PRG register. 1: All HV params are loaded during every API HV operation irrespective of HV_PARAMS_LOADED bit in the RGRANT_DELAY_PRG register."]
    #[inline(always)]
    pub const fn disable_load_once_hv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0: Load common HV params during API HV operations depends on the HV_PARAMS_LOADED bit in RGRANT_DELAY_PRG register. 1: All HV params are loaded during every API HV operation irrespective of HV_PARAMS_LOADED bit in the RGRANT_DELAY_PRG register."]
    #[inline(always)]
    pub fn set_disable_load_once_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn spare7_hv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_spare7_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn spare7_ulp_hv(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_spare7_ulp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn spare7_lp_hv(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_spare7_lp_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
}
impl Default for CalCtl7 {
    #[inline(always)]
    fn default() -> CalCtl7 {
        CalCtl7(0)
    }
}
#[doc = "CM0+ cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0CaCtl0(pub u32);
impl Cm0CaCtl0 {
    #[doc = "Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    pub const fn ram_ecc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    pub fn set_ram_ecc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\] word address."]
    #[inline(always)]
    pub const fn ram_ecc_inj_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\] word address."]
    #[inline(always)]
    pub fn set_ram_ecc_inj_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub const fn way(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_way(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub const fn set_addr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
    #[inline(always)]
    pub const fn ca_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
    #[inline(always)]
    pub fn set_ca_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cm0CaCtl0 {
    #[inline(always)]
    fn default() -> Cm0CaCtl0 {
        Cm0CaCtl0(0)
    }
}
#[doc = "CM0+ cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0CaCtl1(pub u32);
impl Cm0CaCtl1 {
    #[doc = "Specifies power mode for CM0 cache. The following sequnece should be followed for truning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache."]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> Cm0CaCtl1PwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        Cm0CaCtl1PwrMode::from_bits(val as u8)
    }
    #[doc = "Specifies power mode for CM0 cache. The following sequnece should be followed for truning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache."]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: Cm0CaCtl1PwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
    #[inline(always)]
    pub fn set_vectkeystat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cm0CaCtl1 {
    #[inline(always)]
    fn default() -> Cm0CaCtl1 {
        Cm0CaCtl1(0)
    }
}
#[doc = "CM0+ cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0CaCtl2(pub u32);
impl Cm0CaCtl2 {
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub const fn pwrup_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn set_pwrup_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Cm0CaCtl2 {
    #[inline(always)]
    fn default() -> Cm0CaCtl2 {
        Cm0CaCtl2(0)
    }
}
#[doc = "CM0+ cache status 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0CaStatus0(pub u32);
impl Cm0CaStatus0 {
    #[doc = "Sixteen valid bits of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub const fn valid32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Sixteen valid bits of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn set_valid32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm0CaStatus0 {
    #[inline(always)]
    fn default() -> Cm0CaStatus0 {
        Cm0CaStatus0(0)
    }
}
#[doc = "CM0+ cache status 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0CaStatus1(pub u32);
impl Cm0CaStatus1 {
    #[doc = "Cache line address of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub const fn tag(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cache line address of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn set_tag(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm0CaStatus1 {
    #[inline(always)]
    fn default() -> Cm0CaStatus1 {
        Cm0CaStatus1(0)
    }
}
#[doc = "CM0+ cache status 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0CaStatus2(pub u32);
impl Cm0CaStatus2 {
    #[doc = "Six bit LRU representation of the cache set specified by CM0_CA_CTL.SET_ADDR. The encoding of the field is as follows ('X_LRU_Y' indicates that way X is Less Recently Used than way Y): Bit 5: 0_LRU_1: way 0 less recently used than way 1. Bit 4: 0_LRU_2. Bit 3: 0_LRU_3. Bit 2: 1_LRU_2. Bit 1: 1_LRU_3. Bit 0: 2_LRU_3."]
    #[inline(always)]
    pub const fn lru(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Six bit LRU representation of the cache set specified by CM0_CA_CTL.SET_ADDR. The encoding of the field is as follows ('X_LRU_Y' indicates that way X is Less Recently Used than way Y): Bit 5: 0_LRU_1: way 0 less recently used than way 1. Bit 4: 0_LRU_2. Bit 3: 0_LRU_3. Bit 2: 1_LRU_2. Bit 1: 1_LRU_3. Bit 0: 2_LRU_3."]
    #[inline(always)]
    pub fn set_lru(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Cm0CaStatus2 {
    #[inline(always)]
    fn default() -> Cm0CaStatus2 {
        Cm0CaStatus2(0)
    }
}
#[doc = "CM0+ interface status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0Status(pub u32);
impl Cm0Status {
    #[doc = "Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access. SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
    #[inline(always)]
    pub const fn main_internal_err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access. SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
    #[inline(always)]
    pub fn set_main_internal_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub const fn work_internal_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn set_work_internal_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Cm0Status {
    #[inline(always)]
    fn default() -> Cm0Status {
        Cm0Status(0)
    }
}
#[doc = "CM4 cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4CaCtl0(pub u32);
impl Cm4CaCtl0 {
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub const fn ram_ecc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_ram_ecc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub const fn ram_ecc_inj_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_ram_ecc_inj_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub const fn way(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_way(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub const fn set_addr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub const fn ca_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_ca_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cm4CaCtl0 {
    #[inline(always)]
    fn default() -> Cm4CaCtl0 {
        Cm4CaCtl0(0)
    }
}
#[doc = "CM4 cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4CaCtl1(pub u32);
impl Cm4CaCtl1 {
    #[doc = "Specifies power mode for CM4 cache. Refer CM0_CA_CTL1 for more details."]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> Cm4CaCtl1PwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        Cm4CaCtl1PwrMode::from_bits(val as u8)
    }
    #[doc = "Specifies power mode for CM4 cache. Refer CM0_CA_CTL1 for more details."]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: Cm4CaCtl1PwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
    #[inline(always)]
    pub fn set_vectkeystat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cm4CaCtl1 {
    #[inline(always)]
    fn default() -> Cm4CaCtl1 {
        Cm4CaCtl1(0)
    }
}
#[doc = "CM4 cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4CaCtl2(pub u32);
impl Cm4CaCtl2 {
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub const fn pwrup_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn set_pwrup_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Cm4CaCtl2 {
    #[inline(always)]
    fn default() -> Cm4CaCtl2 {
        Cm4CaCtl2(0)
    }
}
#[doc = "CM4 cache status 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4CaStatus0(pub u32);
impl Cm4CaStatus0 {
    #[doc = "See CM0_CA_STATUS0."]
    #[inline(always)]
    pub const fn valid32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "See CM0_CA_STATUS0."]
    #[inline(always)]
    pub fn set_valid32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm4CaStatus0 {
    #[inline(always)]
    fn default() -> Cm4CaStatus0 {
        Cm4CaStatus0(0)
    }
}
#[doc = "CM4 cache status 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4CaStatus1(pub u32);
impl Cm4CaStatus1 {
    #[doc = "See CM0_CA_STATUS1."]
    #[inline(always)]
    pub const fn tag(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "See CM0_CA_STATUS1."]
    #[inline(always)]
    pub fn set_tag(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm4CaStatus1 {
    #[inline(always)]
    fn default() -> Cm4CaStatus1 {
        Cm4CaStatus1(0)
    }
}
#[doc = "CM4 cache status 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4CaStatus2(pub u32);
impl Cm4CaStatus2 {
    #[doc = "See CM0_CA_STATUS2."]
    #[inline(always)]
    pub const fn lru(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "See CM0_CA_STATUS2."]
    #[inline(always)]
    pub fn set_lru(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Cm4CaStatus2 {
    #[inline(always)]
    fn default() -> Cm4CaStatus2 {
        Cm4CaStatus2(0)
    }
}
#[doc = "CM4 interface status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4Status(pub u32);
impl Cm4Status {
    #[doc = "See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub const fn main_internal_err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn set_main_internal_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub const fn work_internal_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn set_work_internal_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Cm4Status {
    #[inline(always)]
    fn default() -> Cm4Status {
        Cm4Status(0)
    }
}
#[doc = "Cryptography buffer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoBuffCtl(pub u32);
impl CryptoBuffCtl {
    #[doc = "Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CryptoBuffCtl {
    #[inline(always)]
    fn default() -> CryptoBuffCtl {
        CryptoBuffCtl(0)
    }
}
#[doc = "DMA controller buffer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmacBuffCtl(pub u32);
impl DmacBuffCtl {
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for DmacBuffCtl {
    #[inline(always)]
    fn default() -> DmacBuffCtl {
        DmacBuffCtl(0)
    }
}
#[doc = "Datawire 0 buffer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dw0BuffCtl(pub u32);
impl Dw0BuffCtl {
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Dw0BuffCtl {
    #[inline(always)]
    fn default() -> Dw0BuffCtl {
        Dw0BuffCtl(0)
    }
}
#[doc = "Datawire 1 buffer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dw1BuffCtl(pub u32);
impl Dw1BuffCtl {
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Dw1BuffCtl {
    #[inline(always)]
    fn default() -> Dw1BuffCtl {
        Dw1BuffCtl(0)
    }
}
#[doc = "ECC control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccCtl(pub u32);
impl EccCtl {
    #[doc = "Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\] is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\] is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\] is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
    #[inline(always)]
    pub const fn word_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\] is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\] is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\] is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
    #[inline(always)]
    pub fn set_word_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\] is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\] is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\] is for a 32-bit word."]
    #[inline(always)]
    pub const fn parity(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\] is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\] is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\] is for a 32-bit word."]
    #[inline(always)]
    pub fn set_parity(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for EccCtl {
    #[inline(always)]
    fn default() -> EccCtl {
        EccCtl(0)
    }
}
#[doc = "External master 0 buffer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtMs0BuffCtl(pub u32);
impl ExtMs0BuffCtl {
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for ExtMs0BuffCtl {
    #[inline(always)]
    fn default() -> ExtMs0BuffCtl {
        ExtMs0BuffCtl(0)
    }
}
#[doc = "External master 1 buffer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtMs1BuffCtl(pub u32);
impl ExtMs1BuffCtl {
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for ExtMs1BuffCtl {
    #[inline(always)]
    fn default() -> ExtMs1BuffCtl {
        ExtMs1BuffCtl(0)
    }
}
#[doc = "Command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashCmd(pub u32);
impl FlashCmd {
    #[doc = "Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
    #[inline(always)]
    pub const fn buff_inv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
    #[inline(always)]
    pub fn set_buff_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for FlashCmd {
    #[inline(always)]
    fn default() -> FlashCmd {
        FlashCmd(0)
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashCtl(pub u32);
impl FlashCtl {
    #[doc = "FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    pub const fn main_ws(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    pub fn set_main_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub const fn main_map(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn set_main_map(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub const fn work_map(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn set_work_map(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub const fn main_bank_mode(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn set_main_bank_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub const fn work_bank_mode(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn set_work_bank_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub const fn main_ecc_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn set_main_ecc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\] word address."]
    #[inline(always)]
    pub const fn main_ecc_inj_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\] word address."]
    #[inline(always)]
    pub fn set_main_ecc_inj_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
    #[inline(always)]
    pub const fn main_err_silent(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
    #[inline(always)]
    pub fn set_main_err_silent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub const fn work_ecc_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn set_work_ecc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\] word address."]
    #[inline(always)]
    pub const fn work_ecc_inj_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\] word address."]
    #[inline(always)]
    pub fn set_work_ecc_inj_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
    #[inline(always)]
    pub const fn work_err_silent(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
    #[inline(always)]
    pub fn set_work_err_silent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for FlashCtl {
    #[inline(always)]
    fn default() -> FlashCtl {
        FlashCtl(0)
    }
}
#[doc = "Flash power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashPwrCtl(pub u32);
impl FlashPwrCtl {
    #[doc = "Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    pub const fn enable_hv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    pub fn set_enable_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for FlashPwrCtl {
    #[inline(always)]
    fn default() -> FlashPwrCtl {
        FlashPwrCtl(0)
    }
}
#[doc = "Flash macro address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmAddr(pub u32);
impl FmAddr {
    #[doc = "Row address."]
    #[inline(always)]
    pub const fn ra(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Row address."]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Bank address."]
    #[inline(always)]
    pub const fn ba(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bank address."]
    #[inline(always)]
    pub fn set_ba(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
    #[inline(always)]
    pub const fn axa(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Auxiliary address field: 0: regular flash memory. 1: supervisory flash memory."]
    #[inline(always)]
    pub fn set_axa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for FmAddr {
    #[inline(always)]
    fn default() -> FmAddr {
        FmAddr(0)
    }
}
#[doc = "Flash macro control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmCtlReg(pub u32);
impl FmCtlReg {
    #[doc = "Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
    #[inline(always)]
    pub const fn fm_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
    #[inline(always)]
    pub fn set_fm_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
    #[inline(always)]
    pub const fn fm_seq(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
    #[inline(always)]
    pub fn set_fm_seq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Direct memory cell access address."]
    #[inline(always)]
    pub const fn daa_mux_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Direct memory cell access address."]
    #[inline(always)]
    pub fn set_daa_mux_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub const fn if_sel(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub fn set_if_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub const fn wr_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub fn set_wr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for FmCtlReg {
    #[inline(always)]
    fn default() -> FmCtlReg {
        FmCtlReg(0)
    }
}
#[doc = "Flash macro memory sense amplifier and column decoder data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmMemData(pub u32);
impl FmMemData {
    #[doc = "Sense amplifier and column multiplexer structure Bytes. The read data is dependent on FM_CTL.IF_SEL: - IF_SEL is 0: data as specified by the R interface address - IF_SEL is 1: data as specified by FM_MEM_ADDR and the offset of the accessed FM_MEM_DATA register."]
    #[inline(always)]
    pub const fn data32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Sense amplifier and column multiplexer structure Bytes. The read data is dependent on FM_CTL.IF_SEL: - IF_SEL is 0: data as specified by the R interface address - IF_SEL is 1: data as specified by FM_MEM_ADDR and the offset of the accessed FM_MEM_DATA register."]
    #[inline(always)]
    pub fn set_data32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FmMemData {
    #[inline(always)]
    fn default() -> FmMemData {
        FmMemData(0)
    }
}
#[doc = "Flash macro Page Latches data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmPlData(pub u32);
impl FmPlData {
    #[doc = "Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub const fn data32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub fn set_data32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FmPlData {
    #[inline(always)]
    fn default() -> FmPlData {
        FmPlData(0)
    }
}
#[doc = "Flash macro write page latches all"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmPlWrdataAll(pub u32);
impl FmPlWrdataAll {
    #[doc = "Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
    #[inline(always)]
    pub const fn data32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
    #[inline(always)]
    pub fn set_data32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FmPlWrdataAll {
    #[inline(always)]
    fn default() -> FmPlWrdataAll {
        FmPlWrdataAll(0)
    }
}
#[doc = "eCT Flash SRAM ECC control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmSramEccCtl0(pub u32);
impl FmSramEccCtl0 {
    #[doc = "32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub const fn ecc_inj_data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn set_ecc_inj_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FmSramEccCtl0 {
    #[inline(always)]
    fn default() -> FmSramEccCtl0 {
        FmSramEccCtl0(0)
    }
}
#[doc = "eCT Flash SRAM ECC control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmSramEccCtl1(pub u32);
impl FmSramEccCtl1 {
    #[doc = "7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub const fn ecc_inj_parity(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn set_ecc_inj_parity(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for FmSramEccCtl1 {
    #[inline(always)]
    fn default() -> FmSramEccCtl1 {
        FmSramEccCtl1(0)
    }
}
#[doc = "eCT Flash SRAM ECC control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmSramEccCtl2(pub u32);
impl FmSramEccCtl2 {
    #[doc = "32-bit corrected data output of the ECC syndrome logic."]
    #[inline(always)]
    pub const fn corrected_data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32-bit corrected data output of the ECC syndrome logic."]
    #[inline(always)]
    pub fn set_corrected_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FmSramEccCtl2 {
    #[inline(always)]
    fn default() -> FmSramEccCtl2 {
        FmSramEccCtl2(0)
    }
}
#[doc = "eCT Flash SRAM ECC control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmSramEccCtl3(pub u32);
impl FmSramEccCtl3 {
    #[doc = "ECC generation/check enable for eCT Flash SRAM memory."]
    #[inline(always)]
    pub const fn ecc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ECC generation/check enable for eCT Flash SRAM memory."]
    #[inline(always)]
    pub fn set_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "eCT Flash SRAM ECC error injection test enable. Follow the steps below for ECC logic test: 1. Write corrupted or uncorrupted 39-bit data to FM_SRAM_ECC_CTL0/1 registers. 2. Set the ECC_INJ_EN bit to '1'. 3. Confirm that the bit ECC_TEST_FAIL is '0'. If this is not the case, start over at item 1 because the eCT Flash was not idle. 4. Check the corrected data in FM_SRAM_ECC_CTL2. 5. Confirm that fault was reported to fault structure, and check syndrome (only applicable if corrupted data was written in step 1). 6. If not finished, start over at 1 with different data."]
    #[inline(always)]
    pub const fn ecc_inj_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "eCT Flash SRAM ECC error injection test enable. Follow the steps below for ECC logic test: 1. Write corrupted or uncorrupted 39-bit data to FM_SRAM_ECC_CTL0/1 registers. 2. Set the ECC_INJ_EN bit to '1'. 3. Confirm that the bit ECC_TEST_FAIL is '0'. If this is not the case, start over at item 1 because the eCT Flash was not idle. 4. Check the corrected data in FM_SRAM_ECC_CTL2. 5. Confirm that fault was reported to fault structure, and check syndrome (only applicable if corrupted data was written in step 1). 6. If not finished, start over at 1 with different data."]
    #[inline(always)]
    pub fn set_ecc_inj_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status of ECC test. 1 : ECC test failed because eCT Flash macro is busy and using the SRAM. 0: ECC was performed."]
    #[inline(always)]
    pub const fn ecc_test_fail(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Status of ECC test. 1 : ECC test failed because eCT Flash macro is busy and using the SRAM. 0: ECC was performed."]
    #[inline(always)]
    pub fn set_ecc_test_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for FmSramEccCtl3 {
    #[inline(always)]
    fn default() -> FmSramEccCtl3 {
        FmSramEccCtl3(0)
    }
}
#[doc = "Regular flash geometry"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Geometry(pub u32);
impl Geometry {
    #[doc = "Number of rows (minus 1): 0: 1 row 1: 2 rows 2: 3 rows ... '65535': 65536 rows"]
    #[inline(always)]
    pub const fn row_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of rows (minus 1): 0: 1 row 1: 2 rows 2: 3 rows ... '65535': 65536 rows"]
    #[inline(always)]
    pub fn set_row_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Number of banks (minus 1): 0: 1 bank 1: 2 banks ... '255': 256 banks"]
    #[inline(always)]
    pub const fn bank_count(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of banks (minus 1): 0: 1 bank 1: 2 banks ... '255': 256 banks"]
    #[inline(always)]
    pub fn set_bank_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Number of Bytes per word (log 2). A word is defined as the data that is read from the flash macro over the R interface with a single read access: 0: 1 Byte 1: 2 Bytes 2: 4 Bytes ... 3: 128 Bytes The currently planned flash macros have a word size of either 32-bit, 64-bit or 128-bit, resulting in WORD_SIZE_LOG2 settings of 2, 3 and 4 respectively."]
    #[inline(always)]
    pub const fn word_size_log2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Bytes per word (log 2). A word is defined as the data that is read from the flash macro over the R interface with a single read access: 0: 1 Byte 1: 2 Bytes 2: 4 Bytes ... 3: 128 Bytes The currently planned flash macros have a word size of either 32-bit, 64-bit or 128-bit, resulting in WORD_SIZE_LOG2 settings of 2, 3 and 4 respectively."]
    #[inline(always)]
    pub fn set_word_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Number of Bytes per page (log 2): 0: 1 Byte 1: 2 Bytes 2: 4 Bytes ... 15: 32768 Bytes The currently planned flash macros have a page size of either 256 Byte or 512 Byte, resulting in PAGE_SIZE_LOG2 settings of 8 and 9 respectively."]
    #[inline(always)]
    pub const fn page_size_log2(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Bytes per page (log 2): 0: 1 Byte 1: 2 Bytes 2: 4 Bytes ... 15: 32768 Bytes The currently planned flash macros have a page size of either 256 Byte or 512 Byte, resulting in PAGE_SIZE_LOG2 settings of 8 and 9 respectively."]
    #[inline(always)]
    pub fn set_page_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Geometry {
    #[inline(always)]
    fn default() -> Geometry {
        Geometry(0)
    }
}
#[doc = "Supervisory flash geometry"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GeometrySupervisory(pub u32);
impl GeometrySupervisory {
    #[doc = "Number of rows (minus 1). ROW_COUNT is typically less than GEOMETRY.ROW_COUNT"]
    #[inline(always)]
    pub const fn row_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of rows (minus 1). ROW_COUNT is typically less than GEOMETRY.ROW_COUNT"]
    #[inline(always)]
    pub fn set_row_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Number of banks (minus 1). BANK_COUNT is less or equal to GEOMETRY.BANK_COUNT."]
    #[inline(always)]
    pub const fn bank_count(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of banks (minus 1). BANK_COUNT is less or equal to GEOMETRY.BANK_COUNT."]
    #[inline(always)]
    pub fn set_bank_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Number of Bytes per word (log 2). See GEOMETRY.WORD_SIZE_LOG2. Typically, WORD_SIZE_LOG2 equals GEOMETRY.WORD_SIZE_LOG2."]
    #[inline(always)]
    pub const fn word_size_log2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Bytes per word (log 2). See GEOMETRY.WORD_SIZE_LOG2. Typically, WORD_SIZE_LOG2 equals GEOMETRY.WORD_SIZE_LOG2."]
    #[inline(always)]
    pub fn set_word_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Number of Bytes per page (log 2). See GEOMETRY.PAGE_SIZE_LOG2. Typically, PAGE_SIZE_LOG2 equals GEOMETRY.PAGE_SIZE_LOG2."]
    #[inline(always)]
    pub const fn page_size_log2(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Bytes per page (log 2). See GEOMETRY.PAGE_SIZE_LOG2. Typically, PAGE_SIZE_LOG2 equals GEOMETRY.PAGE_SIZE_LOG2."]
    #[inline(always)]
    pub fn set_page_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GeometrySupervisory {
    #[inline(always)]
    fn default() -> GeometrySupervisory {
        GeometrySupervisory(0)
    }
}
#[doc = "Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Set to '1', when event is detected. Write INTR field with '1', to clear bit. Write INTR_SET field with '1', to set bit."]
    #[inline(always)]
    pub const fn timer_expired(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set to '1', when event is detected. Write INTR field with '1', to clear bit. Write INTR_SET field with '1', to set bit."]
    #[inline(always)]
    pub fn set_timer_expired(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub const fn timer_expired(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn set_timer_expired(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Interrupt masked"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask fields."]
    #[inline(always)]
    pub const fn timer_expired(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask fields."]
    #[inline(always)]
    pub fn set_timer_expired(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Interrupt set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub const fn timer_expired(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn set_timer_expired(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "HV Pulse Delay for seq 1&2 pre"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwSeq12(pub u32);
impl PwSeq12 {
    #[doc = "Seq1 delay"]
    #[inline(always)]
    pub const fn pw_seq1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Seq1 delay"]
    #[inline(always)]
    pub fn set_pw_seq1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Seq2 pre delay"]
    #[inline(always)]
    pub const fn pw_seq2_pre(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Seq2 pre delay"]
    #[inline(always)]
    pub fn set_pw_seq2_pre(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PwSeq12 {
    #[inline(always)]
    fn default() -> PwSeq12 {
        PwSeq12(0)
    }
}
#[doc = "HV Pulse Delay for seq2 post & seq3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwSeq23(pub u32);
impl PwSeq23 {
    #[doc = "Seq2 post delay"]
    #[inline(always)]
    pub const fn pw_seq2_post(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Seq2 post delay"]
    #[inline(always)]
    pub fn set_pw_seq2_post(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Seq3 delay"]
    #[inline(always)]
    pub const fn pw_seq3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Seq3 delay"]
    #[inline(always)]
    pub fn set_pw_seq3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PwSeq23 {
    #[inline(always)]
    fn default() -> PwSeq23 {
        PwSeq23(0)
    }
}
#[doc = "Redundancy Control normal sectors 0,1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RedCtl01(pub u32);
impl RedCtl01 {
    #[doc = "Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    pub const fn red_addr_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    pub fn set_red_addr_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "1: Redundancy Enable for Sector 0"]
    #[inline(always)]
    pub const fn red_en_0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1: Redundancy Enable for Sector 0"]
    #[inline(always)]
    pub fn set_red_en_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    pub const fn red_addr_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    pub fn set_red_addr_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "1: Redundancy Enable for Sector 1"]
    #[inline(always)]
    pub const fn red_en_1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "1: Redundancy Enable for Sector 1"]
    #[inline(always)]
    pub fn set_red_en_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for RedCtl01 {
    #[inline(always)]
    fn default() -> RedCtl01 {
        RedCtl01(0)
    }
}
#[doc = "Redundancy Control normal sectors 2,3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RedCtl23(pub u32);
impl RedCtl23 {
    #[doc = "Bad Row Pair Address for Sector 2"]
    #[inline(always)]
    pub const fn red_addr_2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Sector 2"]
    #[inline(always)]
    pub fn set_red_addr_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "1: Redundancy Enable for Sector 2"]
    #[inline(always)]
    pub const fn red_en_2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1: Redundancy Enable for Sector 2"]
    #[inline(always)]
    pub fn set_red_en_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bad Row Pair Address for Sector 3"]
    #[inline(always)]
    pub const fn red_addr_3(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Sector 3"]
    #[inline(always)]
    pub fn set_red_addr_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "1: Redundancy Enable for Sector 3"]
    #[inline(always)]
    pub const fn red_en_3(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "1: Redundancy Enable for Sector 3"]
    #[inline(always)]
    pub fn set_red_en_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for RedCtl23 {
    #[inline(always)]
    fn default() -> RedCtl23 {
        RedCtl23(0)
    }
}
#[doc = "Redundancy Control normal sectors 4,5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RedCtl45(pub u32);
impl RedCtl45 {
    #[doc = "Bad Row Pair Address for Sector 4"]
    #[inline(always)]
    pub const fn red_addr_4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Sector 4"]
    #[inline(always)]
    pub fn set_red_addr_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "1: Redundancy Enable for Sector 4"]
    #[inline(always)]
    pub const fn red_en_4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1: Redundancy Enable for Sector 4"]
    #[inline(always)]
    pub fn set_red_en_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bad Row Pair Address for Sector 5"]
    #[inline(always)]
    pub const fn red_addr_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Sector 5"]
    #[inline(always)]
    pub fn set_red_addr_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "1: Redundancy Enable for Sector 5"]
    #[inline(always)]
    pub const fn red_en_5(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "1: Redundancy Enable for Sector 5"]
    #[inline(always)]
    pub fn set_red_en_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for RedCtl45 {
    #[inline(always)]
    fn default() -> RedCtl45 {
        RedCtl45(0)
    }
}
#[doc = "Redundancy Control normal sectors 6,7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RedCtl67(pub u32);
impl RedCtl67 {
    #[doc = "Bad Row Pair Address for Sector 6"]
    #[inline(always)]
    pub const fn red_addr_6(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Sector 6"]
    #[inline(always)]
    pub fn set_red_addr_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "1: Redundancy Enable for Sector 6"]
    #[inline(always)]
    pub const fn red_en_6(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1: Redundancy Enable for Sector 6"]
    #[inline(always)]
    pub fn set_red_en_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bad Row Pair Address for Sector 7"]
    #[inline(always)]
    pub const fn red_addr_7(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Sector 7"]
    #[inline(always)]
    pub fn set_red_addr_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "1: Redundancy Enable for Sector 7"]
    #[inline(always)]
    pub const fn red_en_7(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "1: Redundancy Enable for Sector 7"]
    #[inline(always)]
    pub fn set_red_en_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for RedCtl67 {
    #[inline(always)]
    fn default() -> RedCtl67 {
        RedCtl67(0)
    }
}
#[doc = "Redundancy Control special sectors 0,1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RedCtlSm01(pub u32);
impl RedCtlSm01 {
    #[doc = "Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    pub const fn red_addr_sm0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    pub fn set_red_addr_sm0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    pub const fn red_en_sm0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    pub fn set_red_en_sm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    pub const fn red_addr_sm1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    pub fn set_red_addr_sm1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    pub const fn red_en_sm1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    pub fn set_red_en_sm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for RedCtlSm01 {
    #[inline(always)]
    fn default() -> RedCtlSm01 {
        RedCtlSm01(0)
    }
}
#[doc = "R-grant delay for erase"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RgrantDelayErs(pub u32);
impl RgrantDelayErs {
    #[doc = "ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_ers_seq01(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_ers_seq01(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_ers_seq12(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_ers_seq12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_ers_seq23(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_ers_seq23(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for RgrantDelayErs {
    #[inline(always)]
    fn default() -> RgrantDelayErs {
        RgrantDelayErs(0)
    }
}
#[doc = "R-grant delay for program"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RgrantDelayPrg(pub u32);
impl RgrantDelayPrg {
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_prg_seq12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_prg_seq12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_prg_seq23(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_prg_seq23(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PROG&PRE_PROG & ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_seq30(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PROG&PRE_PROG & ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_seq30(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
    #[inline(always)]
    pub const fn rgrant_delay_clk(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
    #[inline(always)]
    pub fn set_rgrant_delay_clk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
    #[inline(always)]
    pub const fn hv_params_loaded(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
    #[inline(always)]
    pub fn set_hv_params_loaded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RgrantDelayPrg {
    #[inline(always)]
    fn default() -> RgrantDelayPrg {
        RgrantDelayPrg(0)
    }
}
#[doc = "R-grant delay scale for erase"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RgrantScaleErs(pub u32);
impl RgrantScaleErs {
    #[doc = "ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_ers_seq01(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_ers_seq01(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_ers_seq12(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_ers_seq12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_ers_seq23(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_ers_seq23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_ers_peon(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_ers_peon(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub const fn scale_ers_peoff(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn set_scale_ers_peoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_ers_peon(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_ers_peon(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_ers_peoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_ers_peoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for RgrantScaleErs {
    #[inline(always)]
    fn default() -> RgrantScaleErs {
        RgrantScaleErs(0)
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "This is the timer_en bit set by writing a '1' in the TIMER_CTL bit 31. It is reset by HW when the timer expires 0: timer not running 1: Timer is enabled and not expired yet"]
    #[inline(always)]
    pub const fn timer_enabled(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This is the timer_en bit set by writing a '1' in the TIMER_CTL bit 31. It is reset by HW when the timer expires 0: timer not running 1: Timer is enabled and not expired yet"]
    #[inline(always)]
    pub fn set_timer_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the isolation status at HV trim and redundancy registers inputs 0: Not isolated, writing permitted 1: isolated writing disabled"]
    #[inline(always)]
    pub const fn hv_regs_isolated(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the isolation status at HV trim and redundancy registers inputs 0: Not isolated, writing permitted 1: isolated writing disabled"]
    #[inline(always)]
    pub fn set_hv_regs_isolated(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates a bulk, sector erase, program has been requested when axa=1 0: no error 1: illegal HV operation error"]
    #[inline(always)]
    pub const fn illegal_hvop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a bulk, sector erase, program has been requested when axa=1 0: no error 1: illegal HV operation error"]
    #[inline(always)]
    pub fn set_illegal_hvop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. 0: turbo mode 1: normal mode"]
    #[inline(always)]
    pub const fn turbo_n(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. 0: turbo mode 1: normal mode"]
    #[inline(always)]
    pub fn set_turbo_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FM_CTL.WR_EN bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub const fn wr_en_mon(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FM_CTL.WR_EN bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn set_wr_en_mon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FM_CTL.IF_SEL bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub const fn if_sel_mon(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FM_CTL.IF_SEL bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn set_if_sel_mon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The actual timer state sync-ed in clk_c domain: 0: timer is not running: 1: timer is running;"]
    #[inline(always)]
    pub const fn timer_status(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The actual timer state sync-ed in clk_c domain: 0: timer is not running: 1: timer is running;"]
    #[inline(always)]
    pub fn set_timer_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0: R_GRANT_DELAY timer is not running 1: R_GRANT_DELAY timer is running"]
    #[inline(always)]
    pub const fn r_grant_delay_status(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0: R_GRANT_DELAY timer is not running 1: R_GRANT_DELAY timer is running"]
    #[inline(always)]
    pub fn set_r_grant_delay_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0': FM not busy 1: FM BUSY : R_GRANT is 0 as result of a busy request from FM ready, or from HV operations."]
    #[inline(always)]
    pub const fn fm_busy(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0': FM not busy 1: FM BUSY : R_GRANT is 0 as result of a busy request from FM ready, or from HV operations."]
    #[inline(always)]
    pub fn set_fm_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0: FM not ready 1: FM ready"]
    #[inline(always)]
    pub const fn fm_ready(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0: FM not ready 1: FM ready"]
    #[inline(always)]
    pub fn set_fm_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "POS pump VLO"]
    #[inline(always)]
    pub const fn pos_pump_vlo(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "POS pump VLO"]
    #[inline(always)]
    pub fn set_pos_pump_vlo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "NEG pump VHI"]
    #[inline(always)]
    pub const fn neg_pump_vhi(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "NEG pump VHI"]
    #[inline(always)]
    pub fn set_neg_pump_vhi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FM Type (Read While Write or Not Read While Write): 0: Non RWW FM Type 1: RWW FM Type"]
    #[inline(always)]
    pub const fn rww(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FM Type (Read While Write or Not Read While Write): 0: Non RWW FM Type 1: RWW FM Type"]
    #[inline(always)]
    pub fn set_rww(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Internal memory core max data out size (number of data out bits per column): 0: x128 bits 1: x256 bits"]
    #[inline(always)]
    pub const fn max_dout_width(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Internal memory core max data out size (number of data out bits per column): 0: x128 bits 1: x256 bits"]
    #[inline(always)]
    pub fn set_max_dout_width(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0: Sector 0 does not contain special rows. The special rows are located in separate special sectors. 1: Sector 0 contains special rows"]
    #[inline(always)]
    pub const fn sector0_sr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0: Sector 0 does not contain special rows. The special rows are located in separate special sectors. 1: Sector 0 contains special rows"]
    #[inline(always)]
    pub fn set_sector0_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Test_only, internal node: mpcon reset_mm"]
    #[inline(always)]
    pub const fn reset_mm(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test_only, internal node: mpcon reset_mm"]
    #[inline(always)]
    pub fn set_reset_mm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Test_only, internal node: mpcon row_odd"]
    #[inline(always)]
    pub const fn row_odd(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Test_only, internal node: mpcon row_odd"]
    #[inline(always)]
    pub fn set_row_odd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Test_only, internal node: mpcon row_even"]
    #[inline(always)]
    pub const fn row_even(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Test_only, internal node: mpcon row_even"]
    #[inline(always)]
    pub fn set_row_even(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Test_only, internal node: mpcon bk_subb"]
    #[inline(always)]
    pub const fn hvop_sub_sector_n(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Test_only, internal node: mpcon bk_subb"]
    #[inline(always)]
    pub fn set_hvop_sub_sector_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Test_only, internal node: mpcon bk_sec"]
    #[inline(always)]
    pub const fn hvop_sector(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Test_only, internal node: mpcon bk_sec"]
    #[inline(always)]
    pub fn set_hvop_sector(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Test_only, internal node: mpcon bk_all"]
    #[inline(always)]
    pub const fn hvop_bulk_all(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Test_only, internal node: mpcon bk_all"]
    #[inline(always)]
    pub fn set_hvop_bulk_all(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Test_only, internal node: mpcon ra match"]
    #[inline(always)]
    pub const fn cbus_ra_match(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Test_only, internal node: mpcon ra match"]
    #[inline(always)]
    pub fn set_cbus_ra_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Test_only, internal node: mpcon red_row_en"]
    #[inline(always)]
    pub const fn cbus_red_row_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Test_only, internal node: mpcon red_row_en"]
    #[inline(always)]
    pub fn set_cbus_red_row_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Test_only, internal node: rq_error sync-de in clk_c domain"]
    #[inline(always)]
    pub const fn rq_error(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Test_only, internal node: rq_error sync-de in clk_c domain"]
    #[inline(always)]
    pub fn set_rq_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Test_only, internal node: regif pdac outputs to pos pump"]
    #[inline(always)]
    pub const fn pump_pdac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Test_only, internal node: regif pdac outputs to pos pump"]
    #[inline(always)]
    pub fn set_pump_pdac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Test_only, internal node: regif ndac outputs to pos pump"]
    #[inline(always)]
    pub const fn pump_ndac(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Test_only, internal node: regif ndac outputs to pos pump"]
    #[inline(always)]
    pub fn set_pump_ndac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Timer prescaler (clk_t to timer clock frequency divider)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerClkCtl(pub u32);
impl TimerClkCtl {
    #[doc = "Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
    #[inline(always)]
    pub const fn timer_clock_freq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
    #[inline(always)]
    pub fn set_timer_clock_freq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_prg_peon(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_prg_peon(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_prg_peoff(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_prg_peoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub const fn rgrant_delay_prg_seq01(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PROG&PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn set_rgrant_delay_prg_seq01(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TimerClkCtl {
    #[inline(always)]
    fn default() -> TimerClkCtl {
        TimerClkCtl(0)
    }
}
#[doc = "Timer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerCtl(pub u32);
impl TimerCtl {
    #[doc = "Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub const fn period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn set_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
    #[inline(always)]
    pub const fn scale(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
    #[inline(always)]
    pub fn set_scale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "1': Starts1 the HV automatic sequencing Cleared by HW"]
    #[inline(always)]
    pub const fn auto_sequence(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "1': Starts1 the HV automatic sequencing Cleared by HW"]
    #[inline(always)]
    pub fn set_auto_sequence(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "1 during pre-program operation"]
    #[inline(always)]
    pub const fn pre_prog(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "1 during pre-program operation"]
    #[inline(always)]
    pub fn set_pre_prog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub const fn pre_prog_csl(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn set_pre_prog_csl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub const fn pump_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn set_pump_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub const fn aclk_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn set_aclk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub const fn timer_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub fn set_timer_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TimerCtl {
    #[inline(always)]
    fn default() -> TimerCtl {
        TimerCtl(0)
    }
}
#[doc = "Wait State control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaitCtl(pub u32);
impl WaitCtl {
    #[doc = "Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub const fn wait_fm_mem_rd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub fn set_wait_fm_mem_rd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub const fn wait_fm_hv_rd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub fn set_wait_fm_hv_rd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub const fn wait_fm_hv_wr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub fn set_wait_fm_hv_wr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
    #[inline(always)]
    pub const fn fm_rww_mode(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
    #[inline(always)]
    pub fn set_fm_rww_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Spare register"]
    #[inline(always)]
    pub const fn lv_spare_1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Spare register"]
    #[inline(always)]
    pub fn set_lv_spare_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
    #[inline(always)]
    pub const fn drmm(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
    #[inline(always)]
    pub fn set_drmm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
    #[inline(always)]
    pub const fn mba(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
    #[inline(always)]
    pub fn set_mba(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
    #[inline(always)]
    pub const fn pl_soft_set_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
    #[inline(always)]
    pub fn set_pl_soft_set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for WaitCtl {
    #[inline(always)]
    fn default() -> WaitCtl {
        WaitCtl(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cm0CaCtl1PwrMode {
    #[doc = "Power OFF the CM0 cache SRAM."]
    OFF = 0,
    #[doc = "Undefined"]
    RSVD = 0x01,
    #[doc = "Put CM0 cache SRAM in retained mode."]
    RETAINED = 0x02,
    #[doc = "Enable/Turn ON the CM0 cache SRAM."]
    ENABLED = 0x03,
}
impl Cm0CaCtl1PwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm0CaCtl1PwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm0CaCtl1PwrMode {
    #[inline(always)]
    fn from(val: u8) -> Cm0CaCtl1PwrMode {
        Cm0CaCtl1PwrMode::from_bits(val)
    }
}
impl From<Cm0CaCtl1PwrMode> for u8 {
    #[inline(always)]
    fn from(val: Cm0CaCtl1PwrMode) -> u8 {
        Cm0CaCtl1PwrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cm4CaCtl1PwrMode {
    #[doc = "See CM0_CA_CTL1"]
    OFF = 0,
    #[doc = "Undefined"]
    RSVD = 0x01,
    #[doc = "See CM0_CA_CTL1"]
    RETAINED = 0x02,
    #[doc = "See CM0_CA_CTL1"]
    ENABLED = 0x03,
}
impl Cm4CaCtl1PwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm4CaCtl1PwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm4CaCtl1PwrMode {
    #[inline(always)]
    fn from(val: u8) -> Cm4CaCtl1PwrMode {
        Cm4CaCtl1PwrMode::from_bits(val)
    }
}
impl From<Cm4CaCtl1PwrMode> for u8 {
    #[inline(always)]
    fn from(val: Cm4CaCtl1PwrMode) -> u8 {
        Cm4CaCtl1PwrMode::to_bits(val)
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
