#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "AREF configuration"]
use core::prelude::v1::derive;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aref {
    ptr: *mut u8,
}
unsafe impl Send for Aref {}
unsafe impl Sync for Aref {}
impl Aref {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "global AREF control"]
    #[inline(always)]
    pub const fn aref_ctrl(self) -> crate::common::Reg<ArefCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
}
#[doc = "PASS top-level MMIO (DSABv2, INTR)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pass {
    ptr: *mut u8,
}
unsafe impl Send for Pass {}
unsafe impl Sync for Pass {}
impl Pass {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(self) -> crate::common::Reg<IntrCause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "AREF configuration"]
    #[inline(always)]
    pub const fn aref(self) -> Aref {
        unsafe { Aref::from_ptr(self.ptr.add(3584usize) as _) }
    }
    #[doc = "VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim0(self) -> crate::common::Reg<VrefTrim0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3840usize) as _) }
    }
    #[doc = "VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim1(self) -> crate::common::Reg<VrefTrim1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3844usize) as _) }
    }
    #[doc = "VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim2(self) -> crate::common::Reg<VrefTrim2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3848usize) as _) }
    }
    #[doc = "VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim3(self) -> crate::common::Reg<VrefTrim3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3852usize) as _) }
    }
    #[doc = "IZTAT Trim bits"]
    #[inline(always)]
    pub const fn iztat_trim0(self) -> crate::common::Reg<IztatTrim0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3856usize) as _) }
    }
    #[doc = "IZTAT Trim bits"]
    #[inline(always)]
    pub const fn iztat_trim1(self) -> crate::common::Reg<IztatTrim1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3860usize) as _) }
    }
    #[doc = "IPTAT Trim bits"]
    #[inline(always)]
    pub const fn iptat_trim0(self) -> crate::common::Reg<IptatTrim0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3864usize) as _) }
    }
    #[doc = "ICTAT Trim bits"]
    #[inline(always)]
    pub const fn ictat_trim0(self) -> crate::common::Reg<IctatTrim0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3868usize) as _) }
    }
}
#[doc = "global AREF control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArefCtrl(pub u32);
impl ArefCtrl {
    #[doc = "Control bit to trade off AREF settling and noise performance"]
    #[inline(always)]
    pub const fn aref_mode(&self) -> ArefMode {
        let val = (self.0 >> 0usize) & 0x01;
        ArefMode::from_bits(val as u8)
    }
    #[doc = "Control bit to trade off AREF settling and noise performance"]
    #[inline(always)]
    pub fn set_aref_mode(&mut self, val: ArefMode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
    #[inline(always)]
    pub const fn aref_bias_scale(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
    #[inline(always)]
    pub fn set_aref_bias_scale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
    #[inline(always)]
    pub const fn aref_rmb(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
    #[inline(always)]
    pub fn set_aref_rmb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
    #[inline(always)]
    pub const fn ctb_iptat_scale(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
    #[inline(always)]
    pub fn set_ctb_iptat_scale(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
    #[inline(always)]
    pub const fn ctb_iptat_redirect(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
    #[inline(always)]
    pub fn set_ctb_iptat_redirect(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "iztat current select control"]
    #[inline(always)]
    pub const fn iztat_sel(&self) -> IztatSel {
        let val = (self.0 >> 16usize) & 0x01;
        IztatSel::from_bits(val as u8)
    }
    #[doc = "iztat current select control"]
    #[inline(always)]
    pub fn set_iztat_sel(&mut self, val: IztatSel) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
    #[inline(always)]
    pub const fn clock_pump_peri_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
    #[inline(always)]
    pub fn set_clock_pump_peri_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "bandgap voltage select control"]
    #[inline(always)]
    pub const fn vref_sel(&self) -> VrefSel {
        let val = (self.0 >> 20usize) & 0x03;
        VrefSel::from_bits(val as u8)
    }
    #[doc = "bandgap voltage select control"]
    #[inline(always)]
    pub fn set_vref_sel(&mut self, val: VrefSel) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
    #[inline(always)]
    pub const fn deepsleep_mode(&self) -> DeepsleepMode {
        let val = (self.0 >> 28usize) & 0x03;
        DeepsleepMode::from_bits(val as u8)
    }
    #[doc = "AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
    #[inline(always)]
    pub fn set_deepsleep_mode(&mut self, val: DeepsleepMode) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "- 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub const fn deepsleep_on(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn set_deepsleep_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Disable AREF"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Disable AREF"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ArefCtrl {
    #[inline(always)]
    fn default() -> ArefCtrl {
        ArefCtrl(0)
    }
}
#[doc = "ICTAT Trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IctatTrim0(pub u32);
impl IctatTrim0 {
    #[doc = "ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    pub const fn ictat_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn set_ictat_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for IctatTrim0 {
    #[inline(always)]
    fn default() -> IctatTrim0 {
        IctatTrim0(0)
    }
}
#[doc = "Interrupt cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause(pub u32);
impl IntrCause {
    #[doc = "CTB0 interrupt pending"]
    #[inline(always)]
    pub const fn ctb0_int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CTB0 interrupt pending"]
    #[inline(always)]
    pub fn set_ctb0_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CTB1 interrupt pending"]
    #[inline(always)]
    pub const fn ctb1_int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CTB1 interrupt pending"]
    #[inline(always)]
    pub fn set_ctb1_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTB2 interrupt pending"]
    #[inline(always)]
    pub const fn ctb2_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTB2 interrupt pending"]
    #[inline(always)]
    pub fn set_ctb2_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTB3 interrupt pending"]
    #[inline(always)]
    pub const fn ctb3_int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTB3 interrupt pending"]
    #[inline(always)]
    pub fn set_ctb3_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTDAC0 interrupt pending"]
    #[inline(always)]
    pub const fn ctdac0_int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTDAC0 interrupt pending"]
    #[inline(always)]
    pub fn set_ctdac0_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTDAC1 interrupt pending"]
    #[inline(always)]
    pub const fn ctdac1_int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTDAC1 interrupt pending"]
    #[inline(always)]
    pub fn set_ctdac1_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTDAC2 interrupt pending"]
    #[inline(always)]
    pub const fn ctdac2_int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTDAC2 interrupt pending"]
    #[inline(always)]
    pub fn set_ctdac2_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTDAC3 interrupt pending"]
    #[inline(always)]
    pub const fn ctdac3_int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTDAC3 interrupt pending"]
    #[inline(always)]
    pub fn set_ctdac3_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrCause {
    #[inline(always)]
    fn default() -> IntrCause {
        IntrCause(0)
    }
}
#[doc = "IPTAT Trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IptatTrim0(pub u32);
impl IptatTrim0 {
    #[doc = "IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    pub const fn iptat_core_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn set_iptat_core_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    pub const fn iptat_ctbm_trim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    pub fn set_iptat_ctbm_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for IptatTrim0 {
    #[inline(always)]
    fn default() -> IptatTrim0 {
        IptatTrim0(0)
    }
}
#[doc = "IZTAT Trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IztatTrim0(pub u32);
impl IztatTrim0 {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn iztat_abs_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_iztat_abs_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for IztatTrim0 {
    #[inline(always)]
    fn default() -> IztatTrim0 {
        IztatTrim0(0)
    }
}
#[doc = "IZTAT Trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IztatTrim1(pub u32);
impl IztatTrim1 {
    #[doc = "IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
    #[inline(always)]
    pub const fn iztat_tc_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
    #[inline(always)]
    pub fn set_iztat_tc_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for IztatTrim1 {
    #[inline(always)]
    fn default() -> IztatTrim1 {
        IztatTrim1(0)
    }
}
#[doc = "VREF Trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VrefTrim0(pub u32);
impl VrefTrim0 {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn vref_abs_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_vref_abs_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for VrefTrim0 {
    #[inline(always)]
    fn default() -> VrefTrim0 {
        VrefTrim0(0)
    }
}
#[doc = "VREF Trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VrefTrim1(pub u32);
impl VrefTrim1 {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn vref_tempco_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_vref_tempco_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for VrefTrim1 {
    #[inline(always)]
    fn default() -> VrefTrim1 {
        VrefTrim1(0)
    }
}
#[doc = "VREF Trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VrefTrim2(pub u32);
impl VrefTrim2 {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn vref_curv_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_vref_curv_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for VrefTrim2 {
    #[inline(always)]
    fn default() -> VrefTrim2 {
        VrefTrim2(0)
    }
}
#[doc = "VREF Trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VrefTrim3(pub u32);
impl VrefTrim3 {
    #[doc = "Obsolete"]
    #[inline(always)]
    pub const fn vref_atten_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Obsolete"]
    #[inline(always)]
    pub fn set_vref_atten_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for VrefTrim3 {
    #[inline(always)]
    fn default() -> VrefTrim3 {
        VrefTrim3(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArefMode {
    #[doc = "Nominal noise normal startup mode (meets normal mode settling and noise specifications)"]
    NORMAL = 0,
    #[doc = "High noise fast startup mode (meets fast mode settling and noise specifications)"]
    FAST_START = 0x01,
}
impl ArefMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArefMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArefMode {
    #[inline(always)]
    fn from(val: u8) -> ArefMode {
        ArefMode::from_bits(val)
    }
}
impl From<ArefMode> for u8 {
    #[inline(always)]
    fn from(val: ArefMode) -> u8 {
        ArefMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DeepsleepMode {
    #[doc = "All blocks 'OFF' in DeepSleep"]
    OFF = 0,
    #[doc = "IPTAT bias generator 'ON' in DeepSleep (used for fast AREF wakeup only: IPTAT outputs not available)"]
    IPTAT = 0x01,
    #[doc = "IPTAT bias generator and outputs 'ON' in DeepSleep (used for biasing the CTBm with a PTAT current only in deep sleep) *Note that this mode also requires that the CTB_IPTAT_REDIRECT be set if the CTBm opamp is to operate in DeepSleep"]
    IPTAT_IZTAT = 0x02,
    #[doc = "IPTAT, VREF, and IZTAT generators 'ON' in DeepSleep. This mode provides identical AREF functionality in DeepSleep as in the Active mode."]
    IPTAT_IZTAT_VREF = 0x03,
}
impl DeepsleepMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeepsleepMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeepsleepMode {
    #[inline(always)]
    fn from(val: u8) -> DeepsleepMode {
        DeepsleepMode::from_bits(val)
    }
}
impl From<DeepsleepMode> for u8 {
    #[inline(always)]
    fn from(val: DeepsleepMode) -> u8 {
        DeepsleepMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IztatSel {
    #[doc = "Use 250nA IZTAT from SRSS"]
    SRSS = 0,
    #[doc = "Use locally generated 250nA"]
    LOCAL = 0x01,
}
impl IztatSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IztatSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IztatSel {
    #[inline(always)]
    fn from(val: u8) -> IztatSel {
        IztatSel::from_bits(val)
    }
}
impl From<IztatSel> for u8 {
    #[inline(always)]
    fn from(val: IztatSel) -> u8 {
        IztatSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum VrefSel {
    #[doc = "Use 0.8V Vref from SRSS"]
    SRSS = 0,
    #[doc = "Use locally generated Vref"]
    LOCAL = 0x01,
    #[doc = "Use externally supplied Vref (aref_ext_vref)"]
    EXTERNAL = 0x02,
    _RESERVED_3 = 0x03,
}
impl VrefSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VrefSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VrefSel {
    #[inline(always)]
    fn from(val: u8) -> VrefSel {
        VrefSel::from_bits(val)
    }
}
impl From<VrefSel> for u8 {
    #[inline(always)]
    fn from(val: VrefSel) -> u8 {
        VrefSel::to_bits(val)
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
