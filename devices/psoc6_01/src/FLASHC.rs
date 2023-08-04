#![no_std]
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
    #[doc = "BIST control"]
    #[inline(always)]
    pub const fn bist_ctl(self) -> crate::common::Reg<BistCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "BIST command"]
    #[inline(always)]
    pub const fn bist_cmd(self) -> crate::common::Reg<BistCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "BIST address start register"]
    #[inline(always)]
    pub const fn bist_addr_start(self) -> crate::common::Reg<BistAddrStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "BIST data register(s)"]
    #[inline(always)]
    pub const fn bist_data(self, n: usize) -> crate::common::Reg<BistData, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(268usize + n * 4usize) as _) }
    }
    #[doc = "BIST data actual register(s)"]
    #[inline(always)]
    pub const fn bist_data_act(
        self,
        n: usize,
    ) -> crate::common::Reg<BistDataAct, crate::common::R> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(300usize + n * 4usize) as _) }
    }
    #[doc = "BIST data expected register(s)"]
    #[inline(always)]
    pub const fn bist_data_exp(
        self,
        n: usize,
    ) -> crate::common::Reg<BistDataExp, crate::common::R> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(332usize + n * 4usize) as _) }
    }
    #[doc = "BIST address register"]
    #[inline(always)]
    pub const fn bist_addr(self) -> crate::common::Reg<BistAddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(364usize) as _) }
    }
    #[doc = "BIST status register"]
    #[inline(always)]
    pub const fn bist_status(self) -> crate::common::Reg<BistStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
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
    #[doc = "CM0+ cache command"]
    #[inline(always)]
    pub const fn cm0_ca_cmd(self) -> crate::common::Reg<Cm0CaCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1036usize) as _) }
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
    #[doc = "CM4 cache command"]
    #[inline(always)]
    pub const fn cm4_ca_cmd(self) -> crate::common::Reg<Cm4CaCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1164usize) as _) }
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
    #[doc = "Cryptography buffer control"]
    #[inline(always)]
    pub const fn crypto_buff_ctl(self) -> crate::common::Reg<CryptoBuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1280usize) as _) }
    }
    #[doc = "Cryptography buffer command"]
    #[inline(always)]
    pub const fn crypto_buff_cmd(self) -> crate::common::Reg<CryptoBuffCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1288usize) as _) }
    }
    #[doc = "Datawire 0 buffer control"]
    #[inline(always)]
    pub const fn dw0_buff_ctl(self) -> crate::common::Reg<Dw0BuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1408usize) as _) }
    }
    #[doc = "Datawire 0 buffer command"]
    #[inline(always)]
    pub const fn dw0_buff_cmd(self) -> crate::common::Reg<Dw0BuffCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1416usize) as _) }
    }
    #[doc = "Datawire 1 buffer control"]
    #[inline(always)]
    pub const fn dw1_buff_ctl(self) -> crate::common::Reg<Dw1BuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1536usize) as _) }
    }
    #[doc = "Datawire 1 buffer command"]
    #[inline(always)]
    pub const fn dw1_buff_cmd(self) -> crate::common::Reg<Dw1BuffCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1544usize) as _) }
    }
    #[doc = "Debug access port buffer control"]
    #[inline(always)]
    pub const fn dap_buff_ctl(self) -> crate::common::Reg<DapBuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1664usize) as _) }
    }
    #[doc = "Debug access port buffer command"]
    #[inline(always)]
    pub const fn dap_buff_cmd(self) -> crate::common::Reg<DapBuffCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1672usize) as _) }
    }
    #[doc = "External master 0 buffer control"]
    #[inline(always)]
    pub const fn ext_ms0_buff_ctl(self) -> crate::common::Reg<ExtMs0BuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1792usize) as _) }
    }
    #[doc = "External master 0 buffer command"]
    #[inline(always)]
    pub const fn ext_ms0_buff_cmd(self) -> crate::common::Reg<ExtMs0BuffCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1800usize) as _) }
    }
    #[doc = "External master 1 buffer control"]
    #[inline(always)]
    pub const fn ext_ms1_buff_ctl(self) -> crate::common::Reg<ExtMs1BuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1920usize) as _) }
    }
    #[doc = "External master 1 buffer command"]
    #[inline(always)]
    pub const fn ext_ms1_buff_cmd(self) -> crate::common::Reg<ExtMs1BuffCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1928usize) as _) }
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
    pub const fn fm_ctl(self) -> crate::common::Reg<FmCtl, crate::common::RW> {
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
    #[doc = "Regular flash geometry"]
    #[inline(always)]
    pub const fn geometry(self) -> crate::common::Reg<Geometry, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Supervisory flash geometry"]
    #[inline(always)]
    pub const fn geometry_supervisory(
        self,
    ) -> crate::common::Reg<GeometrySupervisory, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Timer control"]
    #[inline(always)]
    pub const fn timer_ctl(self) -> crate::common::Reg<TimerCtl, crate::common::RW> {
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
    #[doc = "N/A, DNU"]
    #[inline(always)]
    pub const fn geometry_gen(self) -> crate::common::Reg<GeometryGen, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Test mode control"]
    #[inline(always)]
    pub const fn test_ctl(self) -> crate::common::Reg<TestCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Wiat State control"]
    #[inline(always)]
    pub const fn wait_ctl(self) -> crate::common::Reg<WaitCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Monitor Status"]
    #[inline(always)]
    pub const fn monitor_status(self) -> crate::common::Reg<MonitorStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Scratch Control"]
    #[inline(always)]
    pub const fn scratch_ctl(self) -> crate::common::Reg<ScratchCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "High voltage control"]
    #[inline(always)]
    pub const fn hv_ctl(self) -> crate::common::Reg<HvCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Aclk control"]
    #[inline(always)]
    pub const fn aclk_ctl(self) -> crate::common::Reg<AclkCtl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Flash macro high Voltage page latches data (for all page latches)"]
    #[inline(always)]
    pub const fn fm_hv_data_all(self) -> crate::common::Reg<FmHvDataAll, crate::common::W> {
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
    #[doc = "Cal control BG LO&HI ipref trim, ref sel, fm_active, turbo_ext"]
    #[inline(always)]
    pub const fn cal_ctl2(self) -> crate::common::Reg<CalCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Cal control osc trim bits, idac, sdac, itim, bdac."]
    #[inline(always)]
    pub const fn cal_ctl3(self) -> crate::common::Reg<CalCtl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "Bookmark register - keeps the current FW HV seq"]
    #[inline(always)]
    pub const fn bookmark(self) -> crate::common::Reg<Bookmark, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Redundancy Control normal sectors 0,1"]
    #[inline(always)]
    pub const fn red_ctl01(self) -> crate::common::Reg<RedCtl01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Redundancy Controll normal sectors 2,3"]
    #[inline(always)]
    pub const fn red_ctl23(self) -> crate::common::Reg<RedCtl23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Redundancy Controll normal sectors 4,5"]
    #[inline(always)]
    pub const fn red_ctl45(self) -> crate::common::Reg<RedCtl45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Redundancy Controll normal sectors 6,7"]
    #[inline(always)]
    pub const fn red_ctl67(self) -> crate::common::Reg<RedCtl67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "Redundancy Controll special sectors 0,1"]
    #[inline(always)]
    pub const fn red_ctl_sm01(self) -> crate::common::Reg<RedCtlSm01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "Do Not Use"]
    #[inline(always)]
    pub const fn tm_cmpr(self, n: usize) -> crate::common::Reg<TmCmpr, crate::common::R> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
    #[doc = "Flash macro high Voltage page latches data"]
    #[inline(always)]
    pub const fn fm_hv_data(self, n: usize) -> crate::common::Reg<FmHvData, crate::common::RW> {
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
#[doc = "Aclk control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AclkCtl(pub u32);
impl AclkCtl {
    #[doc = "A write to this register generates a ACLK pulse for the flash macro (also requires FM_CTL.IF_SEL to be '1')."]
    #[inline(always)]
    pub const fn aclk_gen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "A write to this register generates a ACLK pulse for the flash macro (also requires FM_CTL.IF_SEL to be '1')."]
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
    #[doc = "Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
    #[inline(always)]
    pub const fn vcc_sel(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
    #[inline(always)]
    pub fn set_vcc_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
    #[inline(always)]
    pub const fn flip_amuxbus_ab(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn set_flip_amuxbus_ab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
    #[doc = "Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub const fn pdac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn set_pdac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trimming of negative pump output Voltage:"]
    #[inline(always)]
    pub const fn ndac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Trimming of negative pump output Voltage:"]
    #[inline(always)]
    pub fn set_ndac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "'0': vprot = BG.vprot. '1': vprot = vcc"]
    #[inline(always)]
    pub const fn vprot_override(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "'0': vprot = BG.vprot. '1': vprot = vcc"]
    #[inline(always)]
    pub fn set_vprot_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
    #[inline(always)]
    pub const fn r_grant_ctl(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
    #[inline(always)]
    pub fn set_r_grant_ctl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "'1': Page Latches Soft Reset"]
    #[inline(always)]
    pub const fn rst_sft_hvpl(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "'1': Page Latches Soft Reset"]
    #[inline(always)]
    pub fn set_rst_sft_hvpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for AnaCtl1 {
    #[inline(always)]
    fn default() -> AnaCtl1 {
        AnaCtl1(0)
    }
}
#[doc = "BIST address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistAddr(pub u32);
impl BistAddr {
    #[doc = "Current column address."]
    #[inline(always)]
    pub const fn col_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current column address."]
    #[inline(always)]
    pub fn set_col_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Current row address."]
    #[inline(always)]
    pub const fn row_addr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Current row address."]
    #[inline(always)]
    pub fn set_row_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for BistAddr {
    #[inline(always)]
    fn default() -> BistAddr {
        BistAddr(0)
    }
}
#[doc = "BIST address start register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistAddrStart(pub u32);
impl BistAddrStart {
    #[doc = "Column start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of columns of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with n columns, the legal range is \\[0, n-1\\]."]
    #[inline(always)]
    pub const fn col_addr_start(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Column start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of columns of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with n columns, the legal range is \\[0, n-1\\]."]
    #[inline(always)]
    pub fn set_col_addr_start(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Row start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of rows of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with m columns, the legal range is \\[0, m-1\\]."]
    #[inline(always)]
    pub const fn row_addr_start(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Row start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of rows of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with m columns, the legal range is \\[0, m-1\\]."]
    #[inline(always)]
    pub fn set_row_addr_start(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for BistAddrStart {
    #[inline(always)]
    fn default() -> BistAddrStart {
        BistAddrStart(0)
    }
}
#[doc = "BIST command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistCmd(pub u32);
impl BistCmd {
    #[doc = "1: Start FLASH BIST. Hardware set this field to '0' when BIST is completed."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: Start FLASH BIST. Hardware set this field to '0' when BIST is completed."]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for BistCmd {
    #[inline(always)]
    fn default() -> BistCmd {
        BistCmd(0)
    }
}
#[doc = "BIST control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistCtl(pub u32);
impl BistCtl {
    #[doc = "This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\] and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
    #[inline(always)]
    pub const fn opcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\] and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
    #[inline(always)]
    pub fn set_opcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
    #[inline(always)]
    pub fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub const fn row_first(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub fn set_row_first(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
    #[inline(always)]
    pub const fn addr_start_enabled(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
    #[inline(always)]
    pub fn set_addr_start_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
    #[inline(always)]
    pub const fn addr_compliment_enabled(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
    #[inline(always)]
    pub fn set_addr_compliment_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
    #[inline(always)]
    pub const fn incr_decr_both(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
    #[inline(always)]
    pub fn set_incr_decr_both(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
    #[inline(always)]
    pub const fn stop_on_error(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
    #[inline(always)]
    pub fn set_stop_on_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for BistCtl {
    #[inline(always)]
    fn default() -> BistCtl {
        BistCtl(0)
    }
}
#[doc = "BIST data register(s)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistData(pub u32);
impl BistData {
    #[doc = "BIST data register to store the expected value for data comparison. For a 128-bit Flash memory, there will be 4 BIST_DATA registers to store 128-bit value."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BIST data register to store the expected value for data comparison. For a 128-bit Flash memory, there will be 4 BIST_DATA registers to store 128-bit value."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BistData {
    #[inline(always)]
    fn default() -> BistData {
        BistData(0)
    }
}
#[doc = "BIST data actual register(s)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistDataAct(pub u32);
impl BistDataAct {
    #[doc = "This field specified the actual Flash data output that caused the BIST failure."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field specified the actual Flash data output that caused the BIST failure."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BistDataAct {
    #[inline(always)]
    fn default() -> BistDataAct {
        BistDataAct(0)
    }
}
#[doc = "BIST data expected register(s)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistDataExp(pub u32);
impl BistDataExp {
    #[doc = "This field specified the expected Flash data output."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field specified the expected Flash data output."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BistDataExp {
    #[inline(always)]
    fn default() -> BistDataExp {
        BistDataExp(0)
    }
}
#[doc = "BIST status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistStatus(pub u32);
impl BistStatus {
    #[doc = "0: BIST passed. 1: BIST failed."]
    #[inline(always)]
    pub const fn fail(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: BIST passed. 1: BIST failed."]
    #[inline(always)]
    pub fn set_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for BistStatus {
    #[inline(always)]
    fn default() -> BistStatus {
        BistStatus(0)
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
    #[doc = "LO Bandgap IPTAT trim control."]
    #[inline(always)]
    pub const fn ipref_trim_lo_hv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "LO Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn set_ipref_trim_lo_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
    #[doc = "HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub const fn ipref_trim_hi_hv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn set_ipref_trim_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for CalCtl1 {
    #[inline(always)]
    fn default() -> CalCtl1 {
        CalCtl1(0)
    }
}
#[doc = "Cal control BG LO&HI ipref trim, ref sel, fm_active, turbo_ext"]
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
    #[doc = "LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub const fn icref_tc_trim_lo_hv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn set_icref_tc_trim_lo_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "HI Bandgap Current trim control."]
    #[inline(always)]
    pub const fn icref_trim_hi_hv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn set_icref_trim_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    pub const fn icref_tc_trim_hi_hv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    pub fn set_icref_tc_trim_hi_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Voltage reference: '0': internal bandgap reference '1': external voltage reference"]
    #[inline(always)]
    pub const fn vref_sel_hv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage reference: '0': internal bandgap reference '1': external voltage reference"]
    #[inline(always)]
    pub fn set_vref_sel_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Current reference: '0': internal current reference '1': external current reference"]
    #[inline(always)]
    pub const fn iref_sel_hv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Current reference: '0': internal current reference '1': external current reference"]
    #[inline(always)]
    pub fn set_iref_sel_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0: No Action 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub const fn fm_active_hv(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0: No Action 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub fn set_fm_active_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0: turbo signal generated internally 1: turbo cleared by clk_pump_ext HI"]
    #[inline(always)]
    pub const fn turbo_ext_hv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0: turbo signal generated internally 1: turbo cleared by clk_pump_ext HI"]
    #[inline(always)]
    pub fn set_turbo_ext_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CalCtl2 {
    #[inline(always)]
    fn default() -> CalCtl2 {
        CalCtl2(0)
    }
}
#[doc = "Cal control osc trim bits, idac, sdac, itim, bdac."]
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
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn idac_hv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_idac_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn sdac_hv(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_sdac_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Trimming of timing current"]
    #[inline(always)]
    pub const fn itim_hv(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Trimming of timing current"]
    #[inline(always)]
    pub fn set_itim_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "0': vdd<2.3V '1': vdd>=2.3V"]
    #[inline(always)]
    pub const fn vddhi_hv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0': vdd<2.3V '1': vdd>=2.3V"]
    #[inline(always)]
    pub fn set_vddhi_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Turbo pulse width trim"]
    #[inline(always)]
    pub const fn turbo_pulsew_hv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Turbo pulse width trim"]
    #[inline(always)]
    pub fn set_turbo_pulsew_hv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "LO Bandgap Enable"]
    #[inline(always)]
    pub const fn bglo_en_hv(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LO Bandgap Enable"]
    #[inline(always)]
    pub fn set_bglo_en_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "HI Bandgap Enable"]
    #[inline(always)]
    pub const fn bghi_en_hv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "HI Bandgap Enable"]
    #[inline(always)]
    pub fn set_bghi_en_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CalCtl3 {
    #[inline(always)]
    fn default() -> CalCtl3 {
        CalCtl3(0)
    }
}
#[doc = "CM0+ cache command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0CaCmd(pub u32);
impl Cm0CaCmd {
    #[doc = "FLASH cache invalidation. SW writes a '1' to clear the cache. W sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The cache's LRU structure is also reset to its default state."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLASH cache invalidation. SW writes a '1' to clear the cache. W sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The cache's LRU structure is also reset to its default state."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Cm0CaCmd {
    #[inline(always)]
    fn default() -> Cm0CaCmd {
        Cm0CaCmd(0)
    }
}
#[doc = "CM0+ cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0CaCtl0(pub u32);
impl Cm0CaCtl0 {
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
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
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
    #[doc = "Set Power mode for CM0 cache"]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> Cm0CaCtl1PwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        Cm0CaCtl1PwrMode::from_bits(val as u8)
    }
    #[doc = "Set Power mode for CM0 cache"]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: Cm0CaCtl1PwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
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
    pub const fn valid16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sixteen valid bits of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn set_valid16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
#[doc = "CM4 cache command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4CaCmd(pub u32);
impl Cm4CaCmd {
    #[doc = "See CM0_CA_CMD."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See CM0_CA_CMD."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Cm4CaCmd {
    #[inline(always)]
    fn default() -> Cm4CaCmd {
        Cm4CaCmd(0)
    }
}
#[doc = "CM4 cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4CaCtl0(pub u32);
impl Cm4CaCtl0 {
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
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
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
    #[doc = "Set Power mode for CM4 cache"]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> Cm4CaCtl1PwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        Cm4CaCtl1PwrMode::from_bits(val as u8)
    }
    #[doc = "Set Power mode for CM4 cache"]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: Cm4CaCtl1PwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
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
    pub const fn valid16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "See CM0_CA_STATUS0."]
    #[inline(always)]
    pub fn set_valid16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
#[doc = "Cryptography buffer command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoBuffCmd(pub u32);
impl CryptoBuffCmd {
    #[doc = "FLASH buffer invalidation. SW writes a '1' to clear the buffer. HW sets this field to '0' when the operation is completed."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLASH buffer invalidation. SW writes a '1' to clear the buffer. HW sets this field to '0' when the operation is completed."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CryptoBuffCmd {
    #[inline(always)]
    fn default() -> CryptoBuffCmd {
        CryptoBuffCmd(0)
    }
}
#[doc = "Cryptography buffer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoBuffCtl(pub u32);
impl CryptoBuffCtl {
    #[doc = "Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the buffer to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the buffer to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Cache enable: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Cache enable: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CryptoBuffCtl {
    #[inline(always)]
    fn default() -> CryptoBuffCtl {
        CryptoBuffCtl(0)
    }
}
#[doc = "Debug access port buffer command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DapBuffCmd(pub u32);
impl DapBuffCmd {
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for DapBuffCmd {
    #[inline(always)]
    fn default() -> DapBuffCmd {
        DapBuffCmd(0)
    }
}
#[doc = "Debug access port buffer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DapBuffCtl(pub u32);
impl DapBuffCtl {
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
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DapBuffCtl {
    #[inline(always)]
    fn default() -> DapBuffCtl {
        DapBuffCtl(0)
    }
}
#[doc = "Datawire 0 buffer command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dw0BuffCmd(pub u32);
impl Dw0BuffCmd {
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Dw0BuffCmd {
    #[inline(always)]
    fn default() -> Dw0BuffCmd {
        Dw0BuffCmd(0)
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
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dw0BuffCtl {
    #[inline(always)]
    fn default() -> Dw0BuffCtl {
        Dw0BuffCtl(0)
    }
}
#[doc = "Datawire 1 buffer command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dw1BuffCmd(pub u32);
impl Dw1BuffCmd {
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Dw1BuffCmd {
    #[inline(always)]
    fn default() -> Dw1BuffCmd {
        Dw1BuffCmd(0)
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
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dw1BuffCtl {
    #[inline(always)]
    fn default() -> Dw1BuffCtl {
        Dw1BuffCtl(0)
    }
}
#[doc = "External master 0 buffer command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtMs0BuffCmd(pub u32);
impl ExtMs0BuffCmd {
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ExtMs0BuffCmd {
    #[inline(always)]
    fn default() -> ExtMs0BuffCmd {
        ExtMs0BuffCmd(0)
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
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ExtMs0BuffCtl {
    #[inline(always)]
    fn default() -> ExtMs0BuffCtl {
        ExtMs0BuffCtl(0)
    }
}
#[doc = "External master 1 buffer command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtMs1BuffCmd(pub u32);
impl ExtMs1BuffCmd {
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ExtMs1BuffCmd {
    #[inline(always)]
    fn default() -> ExtMs1BuffCmd {
        ExtMs1BuffCmd(0)
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
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
    #[doc = "FLASH cache and buffer invalidation for ALL cache and buffers. SW writes a '1' to clear the cache and buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLASH cache and buffer invalidation for ALL cache and buffers. SW writes a '1' to clear the cache and buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
    #[doc = "FLASH macro main interface wait states: 0: 0 wait states. ... 15: 15 wait states"]
    #[inline(always)]
    pub const fn main_ws(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FLASH macro main interface wait states: 0: 0 wait states. ... 15: 15 wait states"]
    #[inline(always)]
    pub fn set_main_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Specifies remapping of FLASH macro main region. 0: No remapping. 1: Remapping. The highest address bit of the FLASH main region is inverted. This effectively remaps the location of FLASH main region physical sectors in the logical address space. In other words, the higher half physical sectors are swapped with the lower half physical sectors. Note: remapping only affects reading of the FLASH main region (over the R interface). It does NOT affect programming/erasing of the FLASH memory region (over the C interface). E.g., for a 512 KB / 4 Mb main region, the logical address space ranges from \\[0x1000:0000, 0x1007:ffff\\] (the highest bit if the FLASH main region is bit 18). The memory has four physical sectors: sectors 0, 1, 2 and 3. If REMAP is '0', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 1: \\[0x1002:0000, 0x1003:ffff\\]. - The physical region 2: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 3: \\[0x1006:0000, 0x1007:ffff\\]. If REMAP is '1', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 1: \\[0x1006:0000, 0x1007:ffff\\]. - The physical region 2: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 3: \\[0x1002:0000, 0x1003:ffff\\]. Note: when the REMAP is changed, SW should invalidate the caches and buffers."]
    #[inline(always)]
    pub const fn remap(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies remapping of FLASH macro main region. 0: No remapping. 1: Remapping. The highest address bit of the FLASH main region is inverted. This effectively remaps the location of FLASH main region physical sectors in the logical address space. In other words, the higher half physical sectors are swapped with the lower half physical sectors. Note: remapping only affects reading of the FLASH main region (over the R interface). It does NOT affect programming/erasing of the FLASH memory region (over the C interface). E.g., for a 512 KB / 4 Mb main region, the logical address space ranges from \\[0x1000:0000, 0x1007:ffff\\] (the highest bit if the FLASH main region is bit 18). The memory has four physical sectors: sectors 0, 1, 2 and 3. If REMAP is '0', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 1: \\[0x1002:0000, 0x1003:ffff\\]. - The physical region 2: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 3: \\[0x1006:0000, 0x1007:ffff\\]. If REMAP is '1', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 1: \\[0x1006:0000, 0x1007:ffff\\]. - The physical region 2: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 3: \\[0x1002:0000, 0x1003:ffff\\]. Note: when the REMAP is changed, SW should invalidate the caches and buffers."]
    #[inline(always)]
    pub fn set_remap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
    #[doc = "Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
    #[inline(always)]
    pub const fn axa(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
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
pub struct FmCtl(pub u32);
impl FmCtl {
    #[doc = "Flash macro mode selection: '0': Normal functional mode. '1': Sets 'pre-program control bit' for soft pre-program operation of all selected SONOS cells. the control bit is cleared by the HW after any program operation. '2': Sets ... '15': TBD"]
    #[inline(always)]
    pub const fn fm_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Flash macro mode selection: '0': Normal functional mode. '1': Sets 'pre-program control bit' for soft pre-program operation of all selected SONOS cells. the control bit is cleared by the HW after any program operation. '2': Sets ... '15': TBD"]
    #[inline(always)]
    pub fn set_fm_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Flash macro sequence select: '0': TBD '1': TBD '2': TBD '3': TBD"]
    #[inline(always)]
    pub const fn fm_seq(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Flash macro sequence select: '0': TBD '1': TBD '2': TBD '3': TBD"]
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
    #[doc = "Interface selection. Specifies the interface that is used for flash memory read operations: '0': R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. '1': C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure."]
    #[inline(always)]
    pub const fn if_sel(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interface selection. Specifies the interface that is used for flash memory read operations: '0': R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. '1': C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure."]
    #[inline(always)]
    pub fn set_if_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "'0': normal mode '1': Fm Write Enable"]
    #[inline(always)]
    pub const fn wr_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "'0': normal mode '1': Fm Write Enable"]
    #[inline(always)]
    pub fn set_wr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for FmCtl {
    #[inline(always)]
    fn default() -> FmCtl {
        FmCtl(0)
    }
}
#[doc = "Flash macro high Voltage page latches data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmHvData(pub u32);
impl FmHvData {
    #[doc = "Four page latch Bytes (when writing to the page latches, it also requires FM_CTL.IF_SEL to be '1'). Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub const fn data32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four page latch Bytes (when writing to the page latches, it also requires FM_CTL.IF_SEL to be '1'). Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub fn set_data32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FmHvData {
    #[inline(always)]
    fn default() -> FmHvData {
        FmHvData(0)
    }
}
#[doc = "Flash macro high Voltage page latches data (for all page latches)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmHvDataAll(pub u32);
impl FmHvDataAll {
    #[doc = "Write all high Voltage page latches with the same 32-bit data in a single write cycle"]
    #[inline(always)]
    pub const fn data32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write all high Voltage page latches with the same 32-bit data in a single write cycle"]
    #[inline(always)]
    pub fn set_data32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FmHvDataAll {
    #[inline(always)]
    fn default() -> FmHvDataAll {
        FmHvDataAll(0)
    }
}
#[doc = "Flash macro memory sense amplifier and column decoder data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmMemData(pub u32);
impl FmMemData {
    #[doc = "Sense amplifier and column multiplexer structure Bytes. The read data is dependent on FM_CTL.IF_SEL: - IF_SEL is '0': data as specified by the R interface address - IF_SEL is '1': data as specified by FM_MEM_ADDR and the offset of the accessed FM_MEM_DATA register."]
    #[inline(always)]
    pub const fn data32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Sense amplifier and column multiplexer structure Bytes. The read data is dependent on FM_CTL.IF_SEL: - IF_SEL is '0': data as specified by the R interface address - IF_SEL is '1': data as specified by FM_MEM_ADDR and the offset of the accessed FM_MEM_DATA register."]
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
#[doc = "Regular flash geometry"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Geometry(pub u32);
impl Geometry {
    #[doc = "Number of Bytes per word (log 2). A word is defined as the data that is read from the flash macro over the R interface with a single read access: '0': 1 Byte '1': 2 Bytes '2': 4 Bytes ... '7': 128 Bytes The currently planned flash macros have a word size of either 32-bit, 64-bit or 128-bit, resulting in WORD_SIZE_LOG2 settings of 2, 3 and 4 respectively."]
    #[inline(always)]
    pub const fn word_size_log2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Bytes per word (log 2). A word is defined as the data that is read from the flash macro over the R interface with a single read access: '0': 1 Byte '1': 2 Bytes '2': 4 Bytes ... '7': 128 Bytes The currently planned flash macros have a word size of either 32-bit, 64-bit or 128-bit, resulting in WORD_SIZE_LOG2 settings of 2, 3 and 4 respectively."]
    #[inline(always)]
    pub fn set_word_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of Bytes per page (log 2): '0': 1 Byte '1': 2 Bytes '2': 4 Bytes ... '15': 32768 Bytes The currently planned flash macros have a page size of either 256 Byte or 512 Byte, resulting in PAGE_SIZE_LOG2 settings of 8 and 9 respectively."]
    #[inline(always)]
    pub const fn page_size_log2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Bytes per page (log 2): '0': 1 Byte '1': 2 Bytes '2': 4 Bytes ... '15': 32768 Bytes The currently planned flash macros have a page size of either 256 Byte or 512 Byte, resulting in PAGE_SIZE_LOG2 settings of 8 and 9 respectively."]
    #[inline(always)]
    pub fn set_page_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Number of rows (minus 1): '0': 1 row '1': 2 rows '2': 3 rows ... '65535': 65536 rows"]
    #[inline(always)]
    pub const fn row_count(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of rows (minus 1): '0': 1 row '1': 2 rows '2': 3 rows ... '65535': 65536 rows"]
    #[inline(always)]
    pub fn set_row_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
    }
    #[doc = "Number of banks (minus 1): '0': 1 bank '1': 2 banks ... '255': 256 banks"]
    #[inline(always)]
    pub const fn bank_count(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Number of banks (minus 1): '0': 1 bank '1': 2 banks ... '255': 256 banks"]
    #[inline(always)]
    pub fn set_bank_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Geometry {
    #[inline(always)]
    fn default() -> Geometry {
        Geometry(0)
    }
}
#[doc = "N/A, DNU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GeometryGen(pub u32);
impl GeometryGen {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dnu_0x20_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dnu_0x20_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dnu_0x20_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dnu_0x20_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dnu_0x20_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dnu_0x20_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for GeometryGen {
    #[inline(always)]
    fn default() -> GeometryGen {
        GeometryGen(0)
    }
}
#[doc = "Supervisory flash geometry"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GeometrySupervisory(pub u32);
impl GeometrySupervisory {
    #[doc = "Number of Bytes per word (log 2). See GEOMETRY.WORD_SIZE_LOG2. Typically, WORD_SIZE_LOG2 equals GEOMETRY.WORD_SIZE_LOG2."]
    #[inline(always)]
    pub const fn word_size_log2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Bytes per word (log 2). See GEOMETRY.WORD_SIZE_LOG2. Typically, WORD_SIZE_LOG2 equals GEOMETRY.WORD_SIZE_LOG2."]
    #[inline(always)]
    pub fn set_word_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of Bytes per page (log 2). See GEOMETRY.PAGE_SIZE_LOG2. Typically, PAGE_SIZE_LOG2 equals GEOMETRY.PAGE_SIZE_LOG2."]
    #[inline(always)]
    pub const fn page_size_log2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Bytes per page (log 2). See GEOMETRY.PAGE_SIZE_LOG2. Typically, PAGE_SIZE_LOG2 equals GEOMETRY.PAGE_SIZE_LOG2."]
    #[inline(always)]
    pub fn set_page_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Number of rows (minus 1). ROW_COUNT is typically less than GEOMETRY.ROW_COUNT"]
    #[inline(always)]
    pub const fn row_count(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of rows (minus 1). ROW_COUNT is typically less than GEOMETRY.ROW_COUNT"]
    #[inline(always)]
    pub fn set_row_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
    }
    #[doc = "Number of banks (minus 1). BANK_COUNT is less or equal to GEOMETRY.BANK_COUNT."]
    #[inline(always)]
    pub const fn bank_count(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Number of banks (minus 1). BANK_COUNT is less or equal to GEOMETRY.BANK_COUNT."]
    #[inline(always)]
    pub fn set_bank_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for GeometrySupervisory {
    #[inline(always)]
    fn default() -> GeometrySupervisory {
        GeometrySupervisory(0)
    }
}
#[doc = "High voltage control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HvCtl(pub u32);
impl HvCtl {
    #[doc = "Specifies the frequency in MHz of the timer clock 'clk_t' as provide to the flash macro. E.g., if '4', the timer clock 'clk_t' has a frequency of 4 MHz."]
    #[inline(always)]
    pub const fn timer_clock_freq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the frequency in MHz of the timer clock 'clk_t' as provide to the flash macro. E.g., if '4', the timer clock 'clk_t' has a frequency of 4 MHz."]
    #[inline(always)]
    pub fn set_timer_clock_freq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for HvCtl {
    #[inline(always)]
    fn default() -> HvCtl {
        HvCtl(0)
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
#[doc = "Monitor Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MonitorStatus(pub u32);
impl MonitorStatus {
    #[doc = "POS pump VLO"]
    #[inline(always)]
    pub const fn pos_pump_vlo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "POS pump VLO"]
    #[inline(always)]
    pub fn set_pos_pump_vlo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NEG pump VHI"]
    #[inline(always)]
    pub const fn neg_pump_vhi(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NEG pump VHI"]
    #[inline(always)]
    pub fn set_neg_pump_vhi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for MonitorStatus {
    #[inline(always)]
    fn default() -> MonitorStatus {
        MonitorStatus(0)
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
    #[doc = "'1': Redundancy Enable for Sector 0"]
    #[inline(always)]
    pub const fn red_en_0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "'1': Redundancy Enable for Sector 0"]
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
    #[doc = "'1': Redundancy Enable for Sector 1"]
    #[inline(always)]
    pub const fn red_en_1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "'1': Redundancy Enable for Sector 1"]
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
#[doc = "Redundancy Controll normal sectors 2,3"]
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
    #[doc = "1': Redundancy Enable for Sector 2"]
    #[inline(always)]
    pub const fn red_en_2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1': Redundancy Enable for Sector 2"]
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
    #[doc = "1': Redundancy Enable for Sector 3"]
    #[inline(always)]
    pub const fn red_en_3(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "1': Redundancy Enable for Sector 3"]
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
#[doc = "Redundancy Controll normal sectors 4,5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RedCtl45(pub u32);
impl RedCtl45 {
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_45_1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_45_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub const fn reg_act_hv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn set_reg_act_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_45_3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_45_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "'2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub const fn fdiv_trim_hv_0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "'2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn set_fdiv_trim_hv_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_45_5(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_45_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "'2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub const fn fdiv_trim_hv_1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "'2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn set_fdiv_trim_hv_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_45_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_45_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "'2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub const fn vlim_trim_hv_0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "'2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn set_vlim_trim_hv_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_45_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_45_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_45_23_16(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_45_23_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for RedCtl45 {
    #[inline(always)]
    fn default() -> RedCtl45 {
        RedCtl45(0)
    }
}
#[doc = "Redundancy Controll normal sectors 6,7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RedCtl67(pub u32);
impl RedCtl67 {
    #[doc = "'2b00' V2 = 650mV see vlim_trim_hv<0> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub const fn vlim_trim_hv_1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "'2b00' V2 = 650mV see vlim_trim_hv<0> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn set_vlim_trim_hv_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_67_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_67_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub const fn vprot_act_hv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub fn set_vprot_act_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_67_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_67_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reduces the IPREF Tempco by not subtracting ICREF form IPREF - IPREF will be 1uA"]
    #[inline(always)]
    pub const fn ipref_tc_hv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reduces the IPREF Tempco by not subtracting ICREF form IPREF - IPREF will be 1uA"]
    #[inline(always)]
    pub fn set_ipref_tc_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_67_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_67_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Adds 200-300nA boost on IPREF_HI"]
    #[inline(always)]
    pub const fn ipref_trima_hi_hv(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Adds 200-300nA boost on IPREF_HI"]
    #[inline(always)]
    pub fn set_ipref_trima_hi_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_67_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_67_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Adds 200-300nA boost on IPREF_LO"]
    #[inline(always)]
    pub const fn ipref_trima_lo_hv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Adds 200-300nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn set_ipref_trima_lo_hv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub const fn dnu_67_23_16(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Not Used"]
    #[inline(always)]
    pub fn set_dnu_67_23_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for RedCtl67 {
    #[inline(always)]
    fn default() -> RedCtl67 {
        RedCtl67(0)
    }
}
#[doc = "Redundancy Controll special sectors 0,1"]
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
    #[doc = "Sense Amp Control tracking delay"]
    #[inline(always)]
    pub const fn trkd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Sense Amp Control tracking delay"]
    #[inline(always)]
    pub fn set_trkd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "'0': r_grant handshake disabled, r_grant always 1. '1': r_grand handshake enabled"]
    #[inline(always)]
    pub const fn r_grant_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "'0': r_grant handshake disabled, r_grant always 1. '1': r_grand handshake enabled"]
    #[inline(always)]
    pub fn set_r_grant_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RedCtlSm01 {
    #[inline(always)]
    fn default() -> RedCtlSm01 {
        RedCtlSm01(0)
    }
}
#[doc = "Scratch Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScratchCtl(pub u32);
impl ScratchCtl {
    #[doc = "Scratchpad register fields. Provided for test purposes."]
    #[inline(always)]
    pub const fn dummy32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Scratchpad register fields. Provided for test purposes."]
    #[inline(always)]
    pub fn set_dummy32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ScratchCtl {
    #[inline(always)]
    fn default() -> ScratchCtl {
        ScratchCtl(0)
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Indicates if the high voltage timer is running: '0': not running '1': running"]
    #[inline(always)]
    pub const fn hv_timer_running(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if the high voltage timer is running: '0': not running '1': running"]
    #[inline(always)]
    pub fn set_hv_timer_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the isolation status at HV trim and redundancy registers inputs '0' - Not isolated, writing permitted '1' - isolated writing disabled"]
    #[inline(always)]
    pub const fn hv_regs_isolated(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the isolation status at HV trim and redundancy registers inputs '0' - Not isolated, writing permitted '1' - isolated writing disabled"]
    #[inline(always)]
    pub fn set_hv_regs_isolated(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates a bulk, sector erase, program has been requested when axa=1 '0' - no error '1' - illegal HV operation error"]
    #[inline(always)]
    pub const fn illegal_hvop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates a bulk, sector erase, program has been requested when axa=1 '0' - no error '1' - illegal HV operation error"]
    #[inline(always)]
    pub fn set_illegal_hvop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. '0' - turbo mode '1' - normal mode"]
    #[inline(always)]
    pub const fn turbo_n(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. '0' - turbo mode '1' - normal mode"]
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
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Test mode control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TestCtl(pub u32);
impl TestCtl {
    #[doc = "Test mode control: '0'-'31': TBD"]
    #[inline(always)]
    pub const fn test_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Test mode control: '0'-'31': TBD"]
    #[inline(always)]
    pub fn set_test_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
    #[inline(always)]
    pub const fn pn_ctl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
    #[inline(always)]
    pub fn set_pn_ctl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
    #[inline(always)]
    pub const fn tm_pe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
    #[inline(always)]
    pub fn set_tm_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Test mode positive pump disable"]
    #[inline(always)]
    pub const fn tm_dispos(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Test mode positive pump disable"]
    #[inline(always)]
    pub fn set_tm_dispos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Test mode negative pump disable"]
    #[inline(always)]
    pub const fn tm_disneg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Test mode negative pump disable"]
    #[inline(always)]
    pub fn set_tm_disneg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "1: enables the oscillator output monitor"]
    #[inline(always)]
    pub const fn en_clk_mon(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "1: enables the oscillator output monitor"]
    #[inline(always)]
    pub fn set_en_clk_mon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Engineering Debug Register"]
    #[inline(always)]
    pub const fn csl_debug(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Engineering Debug Register"]
    #[inline(always)]
    pub fn set_csl_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
    #[inline(always)]
    pub const fn enable_osc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
    #[inline(always)]
    pub fn set_enable_osc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
    #[inline(always)]
    pub const fn unscramble_wa(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
    #[inline(always)]
    pub fn set_unscramble_wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TestCtl {
    #[inline(always)]
    fn default() -> TestCtl {
        TestCtl(0)
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
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn set_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
    #[inline(always)]
    pub const fn scale(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
    #[inline(always)]
    pub fn set_scale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pump clock select: '0': internal clock. '1': external clock."]
    #[inline(always)]
    pub const fn pump_clock_sel(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pump clock select: '0': internal clock. '1': external clock."]
    #[inline(always)]
    pub fn set_pump_clock_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "'1' during pre-program operation"]
    #[inline(always)]
    pub const fn pre_prog(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "'1' during pre-program operation"]
    #[inline(always)]
    pub fn set_pre_prog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "'0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub const fn pre_prog_csl(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "'0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn set_pre_prog_csl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub const fn pump_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn set_pump_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub const fn aclk_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn set_aclk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub const fn timer_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
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
#[doc = "Do Not Use"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmCmpr(pub u32);
impl TmCmpr {
    #[doc = "The result of a comparison between the flash macro data output and the content of the high voltage page latches. The comparison result for a given column 'Column_Number' is updated in this register field on a read to address: 0x100+4*Column_Number. The number of wait states is given by WAIT_CTL.WAIT_FM_HV_RD. '0': FALSE (not equal) '1': TRUE (equal)"]
    #[inline(always)]
    pub const fn data_comp_result(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "The result of a comparison between the flash macro data output and the content of the high voltage page latches. The comparison result for a given column 'Column_Number' is updated in this register field on a read to address: 0x100+4*Column_Number. The number of wait states is given by WAIT_CTL.WAIT_FM_HV_RD. '0': FALSE (not equal) '1': TRUE (equal)"]
    #[inline(always)]
    pub fn set_data_comp_result(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for TmCmpr {
    #[inline(always)]
    fn default() -> TmCmpr {
        TmCmpr(0)
    }
}
#[doc = "Wiat State control"]
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
    #[doc = "See CM4_PWR_CTL"]
    OFF = 0,
    #[doc = "undefined"]
    RSVD = 0x01,
    #[doc = "See CM4_PWR_CTL"]
    RETAINED = 0x02,
    #[doc = "See CM4_PWR_CTL"]
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
    #[doc = "See CM4_PWR_CTL"]
    OFF = 0,
    #[doc = "undefined"]
    RSVD = 0x01,
    #[doc = "See CM4_PWR_CTL"]
    RETAINED = 0x02,
    #[doc = "See CM4_PWR_CTL"]
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
