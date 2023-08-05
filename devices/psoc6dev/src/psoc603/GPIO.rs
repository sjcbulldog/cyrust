#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "GPIO port control/configuration"]
use core::prelude::v1::derive;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO port registers"]
    #[inline(always)]
    pub const fn prt(self, n: usize) -> Prt {
        assert!(n < 15usize);
        unsafe { Prt::from_ptr(self.ptr.add(0usize + n * 128usize) as _) }
    }
    #[doc = "Interrupt port cause register 0"]
    #[inline(always)]
    pub const fn intr_cause0(self) -> crate::common::Reg<IntrCause0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16384usize) as _) }
    }
    #[doc = "Interrupt port cause register 1"]
    #[inline(always)]
    pub const fn intr_cause1(self) -> crate::common::Reg<IntrCause1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16388usize) as _) }
    }
    #[doc = "Interrupt port cause register 2"]
    #[inline(always)]
    pub const fn intr_cause2(self) -> crate::common::Reg<IntrCause2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16392usize) as _) }
    }
    #[doc = "Interrupt port cause register 3"]
    #[inline(always)]
    pub const fn intr_cause3(self) -> crate::common::Reg<IntrCause3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16396usize) as _) }
    }
    #[doc = "Extern power supply detection register"]
    #[inline(always)]
    pub const fn vdd_active(self) -> crate::common::Reg<VddActive, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16400usize) as _) }
    }
    #[doc = "Supply detection interrupt register"]
    #[inline(always)]
    pub const fn vdd_intr(self) -> crate::common::Reg<VddIntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16404usize) as _) }
    }
    #[doc = "Supply detection interrupt mask register"]
    #[inline(always)]
    pub const fn vdd_intr_mask(self) -> crate::common::Reg<VddIntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16408usize) as _) }
    }
    #[doc = "Supply detection interrupt masked register"]
    #[inline(always)]
    pub const fn vdd_intr_masked(self) -> crate::common::Reg<VddIntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16412usize) as _) }
    }
    #[doc = "Supply detection interrupt set register"]
    #[inline(always)]
    pub const fn vdd_intr_set(self) -> crate::common::Reg<VddIntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16416usize) as _) }
    }
}
#[doc = "GPIO port registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prt {
    ptr: *mut u8,
}
unsafe impl Send for Prt {}
unsafe impl Sync for Prt {}
impl Prt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port output data register"]
    #[inline(always)]
    pub const fn out(self) -> crate::common::Reg<Out, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Port output data clear register"]
    #[inline(always)]
    pub const fn out_clr(self) -> crate::common::Reg<OutClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Port output data set register"]
    #[inline(always)]
    pub const fn out_set(self) -> crate::common::Reg<OutSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Port output data invert register"]
    #[inline(always)]
    pub const fn out_inv(self) -> crate::common::Reg<OutInv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Port input state register"]
    #[inline(always)]
    pub const fn in_(self) -> crate::common::Reg<In, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Port interrupt status register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Port interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Port interrupt masked status register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Port interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Port interrupt configuration register"]
    #[inline(always)]
    pub const fn intr_cfg(self) -> crate::common::Reg<IntrCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Port configuration register"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Port input buffer configuration register"]
    #[inline(always)]
    pub const fn cfg_in(self) -> crate::common::Reg<CfgIn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Port output buffer configuration register"]
    #[inline(always)]
    pub const fn cfg_out(self) -> crate::common::Reg<CfgOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Port SIO configuration register"]
    #[inline(always)]
    pub const fn cfg_sio(self) -> crate::common::Reg<CfgSio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Port input buffer AUTOLVL configuration register"]
    #[inline(always)]
    pub const fn cfg_in_autolvl(self) -> crate::common::Reg<CfgInAutolvl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
}
#[doc = "Port configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
    #[inline(always)]
    pub const fn drive_mode0(&self) -> DriveMode0 {
        let val = (self.0 >> 0usize) & 0x07;
        DriveMode0::from_bits(val as u8)
    }
    #[doc = "The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
    #[inline(always)]
    pub fn set_drive_mode0(&mut self, val: DriveMode0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
    #[inline(always)]
    pub const fn in_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
    #[inline(always)]
    pub fn set_in_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The GPIO drive mode for IO pin 1"]
    #[inline(always)]
    pub const fn drive_mode1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pin 1"]
    #[inline(always)]
    pub fn set_drive_mode1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Enables the input buffer for IO pin 1"]
    #[inline(always)]
    pub const fn in_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the input buffer for IO pin 1"]
    #[inline(always)]
    pub fn set_in_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "The GPIO drive mode for IO pin 2"]
    #[inline(always)]
    pub const fn drive_mode2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pin 2"]
    #[inline(always)]
    pub fn set_drive_mode2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Enables the input buffer for IO pin 2"]
    #[inline(always)]
    pub const fn in_en2(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the input buffer for IO pin 2"]
    #[inline(always)]
    pub fn set_in_en2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "The GPIO drive mode for IO pin 3"]
    #[inline(always)]
    pub const fn drive_mode3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pin 3"]
    #[inline(always)]
    pub fn set_drive_mode3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Enables the input buffer for IO pin 3"]
    #[inline(always)]
    pub const fn in_en3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the input buffer for IO pin 3"]
    #[inline(always)]
    pub fn set_in_en3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "The GPIO drive mode for IO pin4"]
    #[inline(always)]
    pub const fn drive_mode4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pin4"]
    #[inline(always)]
    pub fn set_drive_mode4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Enables the input buffer for IO pin 4"]
    #[inline(always)]
    pub const fn in_en4(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the input buffer for IO pin 4"]
    #[inline(always)]
    pub fn set_in_en4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "The GPIO drive mode for IO pin 5"]
    #[inline(always)]
    pub const fn drive_mode5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pin 5"]
    #[inline(always)]
    pub fn set_drive_mode5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Enables the input buffer for IO pin 5"]
    #[inline(always)]
    pub const fn in_en5(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the input buffer for IO pin 5"]
    #[inline(always)]
    pub fn set_in_en5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "The GPIO drive mode for IO pin 6"]
    #[inline(always)]
    pub const fn drive_mode6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pin 6"]
    #[inline(always)]
    pub fn set_drive_mode6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Enables the input buffer for IO pin 6"]
    #[inline(always)]
    pub const fn in_en6(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the input buffer for IO pin 6"]
    #[inline(always)]
    pub fn set_in_en6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "The GPIO drive mode for IO pin 7"]
    #[inline(always)]
    pub const fn drive_mode7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pin 7"]
    #[inline(always)]
    pub fn set_drive_mode7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "Enables the input buffer for IO pin 7"]
    #[inline(always)]
    pub const fn in_en7(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the input buffer for IO pin 7"]
    #[inline(always)]
    pub fn set_in_en7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
#[doc = "Port input buffer configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfgIn(pub u32);
impl CfgIn {
    #[doc = "Configures the pin 0 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub const fn vtrip_sel0_0(&self) -> VtripSel00 {
        let val = (self.0 >> 0usize) & 0x01;
        VtripSel00::from_bits(val as u8)
    }
    #[doc = "Configures the pin 0 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn set_vtrip_sel0_0(&mut self, val: VtripSel00) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Configures the pin 1 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub const fn vtrip_sel1_0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Configures the pin 1 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn set_vtrip_sel1_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configures the pin 2 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub const fn vtrip_sel2_0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Configures the pin 2 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn set_vtrip_sel2_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Configures the pin 3 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub const fn vtrip_sel3_0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Configures the pin 3 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn set_vtrip_sel3_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Configures the pin 4 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub const fn vtrip_sel4_0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Configures the pin 4 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn set_vtrip_sel4_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Configures the pin 5 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub const fn vtrip_sel5_0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Configures the pin 5 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn set_vtrip_sel5_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Configures the pin 6 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub const fn vtrip_sel6_0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Configures the pin 6 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn set_vtrip_sel6_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Configures the pin 7 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub const fn vtrip_sel7_0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Configures the pin 7 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn set_vtrip_sel7_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for CfgIn {
    #[inline(always)]
    fn default() -> CfgIn {
        CfgIn(0)
    }
}
#[doc = "Port input buffer AUTOLVL configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfgInAutolvl(pub u32);
impl CfgInAutolvl {
    #[doc = "Configures the input buffer mode (trip points and hysteresis) for GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
    #[inline(always)]
    pub const fn vtrip_sel0_1(&self) -> VtripSel01 {
        let val = (self.0 >> 0usize) & 0x01;
        VtripSel01::from_bits(val as u8)
    }
    #[doc = "Configures the input buffer mode (trip points and hysteresis) for GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
    #[inline(always)]
    pub fn set_vtrip_sel0_1(&mut self, val: VtripSel01) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub const fn vtrip_sel1_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn set_vtrip_sel1_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub const fn vtrip_sel2_1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn set_vtrip_sel2_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub const fn vtrip_sel3_1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn set_vtrip_sel3_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub const fn vtrip_sel4_1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn set_vtrip_sel4_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub const fn vtrip_sel5_1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn set_vtrip_sel5_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub const fn vtrip_sel6_1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn set_vtrip_sel6_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub const fn vtrip_sel7_1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn set_vtrip_sel7_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for CfgInAutolvl {
    #[inline(always)]
    fn default() -> CfgInAutolvl {
        CfgInAutolvl(0)
    }
}
#[doc = "Port output buffer configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfgOut(pub u32);
impl CfgOut {
    #[doc = "Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    pub const fn slow0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    pub fn set_slow0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    pub const fn slow1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    pub fn set_slow1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    pub const fn slow2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    pub fn set_slow2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    pub const fn slow3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    pub fn set_slow3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    pub const fn slow4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    pub fn set_slow4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    pub const fn slow5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    pub fn set_slow5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    pub const fn slow6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    pub fn set_slow6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    pub const fn slow7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    pub fn set_slow7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    pub const fn drive_sel0(&self) -> DriveSel0 {
        let val = (self.0 >> 16usize) & 0x03;
        DriveSel0::from_bits(val as u8)
    }
    #[doc = "Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    pub fn set_drive_sel0(&mut self, val: DriveSel0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    pub const fn drive_sel1(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    pub fn set_drive_sel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    pub const fn drive_sel2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    pub fn set_drive_sel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    pub const fn drive_sel3(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    pub fn set_drive_sel3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    pub const fn drive_sel4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    pub fn set_drive_sel4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    pub const fn drive_sel5(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    pub fn set_drive_sel5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    pub const fn drive_sel6(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    pub fn set_drive_sel6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    pub const fn drive_sel7(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    pub fn set_drive_sel7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for CfgOut {
    #[inline(always)]
    fn default() -> CfgOut {
        CfgOut(0)
    }
}
#[doc = "Port SIO configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfgSio(pub u32);
impl CfgSio {
    #[doc = "Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub const fn vreg_en01(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn set_vreg_en01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
    #[inline(always)]
    pub const fn ibuf_sel01(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
    #[inline(always)]
    pub fn set_ibuf_sel01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
    #[inline(always)]
    pub const fn vtrip_sel01(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
    #[inline(always)]
    pub fn set_vtrip_sel01(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
    #[inline(always)]
    pub const fn vref_sel01(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
    #[inline(always)]
    pub fn set_vref_sel01(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
    #[inline(always)]
    pub const fn voh_sel01(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
    #[inline(always)]
    pub fn set_voh_sel01(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn vreg_en23(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_vreg_en23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn ibuf_sel23(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_ibuf_sel23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn vtrip_sel23(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_vtrip_sel23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn vref_sel23(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_vref_sel23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn voh_sel23(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_voh_sel23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn vreg_en45(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_vreg_en45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn ibuf_sel45(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_ibuf_sel45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn vtrip_sel45(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_vtrip_sel45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn vref_sel45(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_vref_sel45(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn voh_sel45(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_voh_sel45(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn vreg_en67(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_vreg_en67(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn ibuf_sel67(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_ibuf_sel67(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn vtrip_sel67(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_vtrip_sel67(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn vref_sel67(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_vref_sel67(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub const fn voh_sel67(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn set_voh_sel67(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for CfgSio {
    #[inline(always)]
    fn default() -> CfgSio {
        CfgSio(0)
    }
}
#[doc = "Port input state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In(pub u32);
impl In {
    #[doc = "IO pin state for pin 0 '0': Low logic level present on pin. '1': High logic level present on pin."]
    #[inline(always)]
    pub const fn in0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 0 '0': Low logic level present on pin. '1': High logic level present on pin."]
    #[inline(always)]
    pub fn set_in0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO pin state for pin 1"]
    #[inline(always)]
    pub const fn in1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 1"]
    #[inline(always)]
    pub fn set_in1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IO pin state for pin 2"]
    #[inline(always)]
    pub const fn in2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 2"]
    #[inline(always)]
    pub fn set_in2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IO pin state for pin 3"]
    #[inline(always)]
    pub const fn in3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 3"]
    #[inline(always)]
    pub fn set_in3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IO pin state for pin 4"]
    #[inline(always)]
    pub const fn in4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 4"]
    #[inline(always)]
    pub fn set_in4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO pin state for pin 5"]
    #[inline(always)]
    pub const fn in5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 5"]
    #[inline(always)]
    pub fn set_in5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IO pin state for pin 6"]
    #[inline(always)]
    pub const fn in6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 6"]
    #[inline(always)]
    pub fn set_in6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IO pin state for pin 7"]
    #[inline(always)]
    pub const fn in7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 7"]
    #[inline(always)]
    pub fn set_in7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Reads of this register return the logical state of the filtered pin as selected in the INTR_CFG.FLT_SEL register."]
    #[inline(always)]
    pub const fn flt_in(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Reads of this register return the logical state of the filtered pin as selected in the INTR_CFG.FLT_SEL register."]
    #[inline(always)]
    pub fn set_flt_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for In {
    #[inline(always)]
    fn default() -> In {
        In(0)
    }
}
#[doc = "Port interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
    #[inline(always)]
    pub const fn edge0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
    #[inline(always)]
    pub fn set_edge0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Edge detect for IO pin 1"]
    #[inline(always)]
    pub const fn edge1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detect for IO pin 1"]
    #[inline(always)]
    pub fn set_edge1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Edge detect for IO pin 2"]
    #[inline(always)]
    pub const fn edge2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detect for IO pin 2"]
    #[inline(always)]
    pub fn set_edge2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Edge detect for IO pin 3"]
    #[inline(always)]
    pub const fn edge3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detect for IO pin 3"]
    #[inline(always)]
    pub fn set_edge3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Edge detect for IO pin 4"]
    #[inline(always)]
    pub const fn edge4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detect for IO pin 4"]
    #[inline(always)]
    pub fn set_edge4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Edge detect for IO pin 5"]
    #[inline(always)]
    pub const fn edge5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detect for IO pin 5"]
    #[inline(always)]
    pub fn set_edge5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Edge detect for IO pin 6"]
    #[inline(always)]
    pub const fn edge6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detect for IO pin 6"]
    #[inline(always)]
    pub fn set_edge6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Edge detect for IO pin 7"]
    #[inline(always)]
    pub const fn edge7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detect for IO pin 7"]
    #[inline(always)]
    pub fn set_edge7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub const fn flt_edge(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn set_flt_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "IO pin state for pin 0"]
    #[inline(always)]
    pub const fn in_in0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 0"]
    #[inline(always)]
    pub fn set_in_in0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "IO pin state for pin 1"]
    #[inline(always)]
    pub const fn in_in1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 1"]
    #[inline(always)]
    pub fn set_in_in1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "IO pin state for pin 2"]
    #[inline(always)]
    pub const fn in_in2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 2"]
    #[inline(always)]
    pub fn set_in_in2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "IO pin state for pin 3"]
    #[inline(always)]
    pub const fn in_in3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 3"]
    #[inline(always)]
    pub fn set_in_in3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "IO pin state for pin 4"]
    #[inline(always)]
    pub const fn in_in4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 4"]
    #[inline(always)]
    pub fn set_in_in4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "IO pin state for pin 5"]
    #[inline(always)]
    pub const fn in_in5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 5"]
    #[inline(always)]
    pub fn set_in_in5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "IO pin state for pin 6"]
    #[inline(always)]
    pub const fn in_in6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 6"]
    #[inline(always)]
    pub fn set_in_in6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "IO pin state for pin 7"]
    #[inline(always)]
    pub const fn in_in7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IO pin state for pin 7"]
    #[inline(always)]
    pub fn set_in_in7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Filtered pin state for pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub const fn flt_in_in(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Filtered pin state for pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn set_flt_in_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt port cause register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause0(pub u32);
impl IntrCause0 {
    #[doc = "Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PRT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub const fn port_int(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PRT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub fn set_port_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IntrCause0 {
    #[inline(always)]
    fn default() -> IntrCause0 {
        IntrCause0(0)
    }
}
#[doc = "Interrupt port cause register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause1(pub u32);
impl IntrCause1 {
    #[doc = "Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub const fn port_int(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub fn set_port_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IntrCause1 {
    #[inline(always)]
    fn default() -> IntrCause1 {
        IntrCause1(0)
    }
}
#[doc = "Interrupt port cause register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause2(pub u32);
impl IntrCause2 {
    #[doc = "Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub const fn port_int(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub fn set_port_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IntrCause2 {
    #[inline(always)]
    fn default() -> IntrCause2 {
        IntrCause2(0)
    }
}
#[doc = "Interrupt port cause register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause3(pub u32);
impl IntrCause3 {
    #[doc = "Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub const fn port_int(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub fn set_port_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IntrCause3 {
    #[inline(always)]
    fn default() -> IntrCause3 {
        IntrCause3(0)
    }
}
#[doc = "Port interrupt configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCfg(pub u32);
impl IntrCfg {
    #[doc = "Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    pub const fn edge0_sel(&self) -> Edge0Sel {
        let val = (self.0 >> 0usize) & 0x03;
        Edge0Sel::from_bits(val as u8)
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    pub fn set_edge0_sel(&mut self, val: Edge0Sel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    pub const fn edge1_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    pub fn set_edge1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    pub const fn edge2_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    pub fn set_edge2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    pub const fn edge3_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    pub fn set_edge3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    pub const fn edge4_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    pub fn set_edge4_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    pub const fn edge5_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    pub fn set_edge5_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    pub const fn edge6_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    pub fn set_edge6_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    pub const fn edge7_sel(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    pub fn set_edge7_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub const fn flt_edge_sel(&self) -> FltEdgeSel {
        let val = (self.0 >> 16usize) & 0x03;
        FltEdgeSel::from_bits(val as u8)
    }
    #[doc = "Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn set_flt_edge_sel(&mut self, val: FltEdgeSel) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub const fn flt_sel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn set_flt_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
}
impl Default for IntrCfg {
    #[inline(always)]
    fn default() -> IntrCfg {
        IntrCfg(0)
    }
}
#[doc = "Port interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
    #[inline(always)]
    pub const fn edge0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
    #[inline(always)]
    pub fn set_edge0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Masks edge interrupt on IO pin 1"]
    #[inline(always)]
    pub const fn edge1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Masks edge interrupt on IO pin 1"]
    #[inline(always)]
    pub fn set_edge1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Masks edge interrupt on IO pin 2"]
    #[inline(always)]
    pub const fn edge2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Masks edge interrupt on IO pin 2"]
    #[inline(always)]
    pub fn set_edge2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Masks edge interrupt on IO pin 3"]
    #[inline(always)]
    pub const fn edge3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Masks edge interrupt on IO pin 3"]
    #[inline(always)]
    pub fn set_edge3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Masks edge interrupt on IO pin 4"]
    #[inline(always)]
    pub const fn edge4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Masks edge interrupt on IO pin 4"]
    #[inline(always)]
    pub fn set_edge4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Masks edge interrupt on IO pin 5"]
    #[inline(always)]
    pub const fn edge5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Masks edge interrupt on IO pin 5"]
    #[inline(always)]
    pub fn set_edge5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Masks edge interrupt on IO pin 6"]
    #[inline(always)]
    pub const fn edge6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Masks edge interrupt on IO pin 6"]
    #[inline(always)]
    pub fn set_edge6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Masks edge interrupt on IO pin 7"]
    #[inline(always)]
    pub const fn edge7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Masks edge interrupt on IO pin 7"]
    #[inline(always)]
    pub fn set_edge7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub const fn flt_edge(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn set_flt_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Port interrupt masked status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Edge detected AND masked on IO pin 0 '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
    #[inline(always)]
    pub const fn edge0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected AND masked on IO pin 0 '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
    #[inline(always)]
    pub fn set_edge0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Edge detected and masked on IO pin 1"]
    #[inline(always)]
    pub const fn edge1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected and masked on IO pin 1"]
    #[inline(always)]
    pub fn set_edge1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Edge detected and masked on IO pin 2"]
    #[inline(always)]
    pub const fn edge2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected and masked on IO pin 2"]
    #[inline(always)]
    pub fn set_edge2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Edge detected and masked on IO pin 3"]
    #[inline(always)]
    pub const fn edge3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected and masked on IO pin 3"]
    #[inline(always)]
    pub fn set_edge3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Edge detected and masked on IO pin 4"]
    #[inline(always)]
    pub const fn edge4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected and masked on IO pin 4"]
    #[inline(always)]
    pub fn set_edge4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Edge detected and masked on IO pin 5"]
    #[inline(always)]
    pub const fn edge5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected and masked on IO pin 5"]
    #[inline(always)]
    pub fn set_edge5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Edge detected and masked on IO pin 6"]
    #[inline(always)]
    pub const fn edge6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected and masked on IO pin 6"]
    #[inline(always)]
    pub fn set_edge6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Edge detected and masked on IO pin 7"]
    #[inline(always)]
    pub const fn edge7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected and masked on IO pin 7"]
    #[inline(always)]
    pub fn set_edge7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Edge detected and masked on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub const fn flt_edge(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Edge detected and masked on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn set_flt_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Port interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Sets edge detect interrupt for IO pin 0 '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    pub const fn edge0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sets edge detect interrupt for IO pin 0 '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    pub fn set_edge0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sets edge detect interrupt for IO pin 1"]
    #[inline(always)]
    pub const fn edge1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sets edge detect interrupt for IO pin 1"]
    #[inline(always)]
    pub fn set_edge1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Sets edge detect interrupt for IO pin 2"]
    #[inline(always)]
    pub const fn edge2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Sets edge detect interrupt for IO pin 2"]
    #[inline(always)]
    pub fn set_edge2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Sets edge detect interrupt for IO pin 3"]
    #[inline(always)]
    pub const fn edge3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Sets edge detect interrupt for IO pin 3"]
    #[inline(always)]
    pub fn set_edge3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Sets edge detect interrupt for IO pin 4"]
    #[inline(always)]
    pub const fn edge4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Sets edge detect interrupt for IO pin 4"]
    #[inline(always)]
    pub fn set_edge4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Sets edge detect interrupt for IO pin 5"]
    #[inline(always)]
    pub const fn edge5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Sets edge detect interrupt for IO pin 5"]
    #[inline(always)]
    pub fn set_edge5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Sets edge detect interrupt for IO pin 6"]
    #[inline(always)]
    pub const fn edge6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Sets edge detect interrupt for IO pin 6"]
    #[inline(always)]
    pub fn set_edge6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Sets edge detect interrupt for IO pin 7"]
    #[inline(always)]
    pub const fn edge7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Sets edge detect interrupt for IO pin 7"]
    #[inline(always)]
    pub fn set_edge7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sets edge detect interrupt for filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub const fn flt_edge(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Sets edge detect interrupt for filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn set_flt_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Port output data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out(pub u32);
impl Out {
    #[doc = "IO output data for pin 0 '0': Output state set to '0' '1': Output state set to '1'"]
    #[inline(always)]
    pub const fn out0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IO output data for pin 0 '0': Output state set to '0' '1': Output state set to '1'"]
    #[inline(always)]
    pub fn set_out0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO output data for pin 1"]
    #[inline(always)]
    pub const fn out1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO output data for pin 1"]
    #[inline(always)]
    pub fn set_out1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IO output data for pin 2"]
    #[inline(always)]
    pub const fn out2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IO output data for pin 2"]
    #[inline(always)]
    pub fn set_out2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IO output data for pin 3"]
    #[inline(always)]
    pub const fn out3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IO output data for pin 3"]
    #[inline(always)]
    pub fn set_out3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IO output data for pin 4"]
    #[inline(always)]
    pub const fn out4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IO output data for pin 4"]
    #[inline(always)]
    pub fn set_out4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO output data for pin 5"]
    #[inline(always)]
    pub const fn out5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO output data for pin 5"]
    #[inline(always)]
    pub fn set_out5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IO output data for pin 6"]
    #[inline(always)]
    pub const fn out6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IO output data for pin 6"]
    #[inline(always)]
    pub fn set_out6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IO output data for pin 7"]
    #[inline(always)]
    pub const fn out7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IO output data for pin 7"]
    #[inline(always)]
    pub fn set_out7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Out {
    #[inline(always)]
    fn default() -> Out {
        Out(0)
    }
}
#[doc = "Port output data clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutClr(pub u32);
impl OutClr {
    #[doc = "IO clear output for pin 0: '0': Output state not affected. '1': Output state set to '0'."]
    #[inline(always)]
    pub const fn out0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IO clear output for pin 0: '0': Output state not affected. '1': Output state set to '0'."]
    #[inline(always)]
    pub fn set_out0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO clear output for pin 1"]
    #[inline(always)]
    pub const fn out1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO clear output for pin 1"]
    #[inline(always)]
    pub fn set_out1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IO clear output for pin 2"]
    #[inline(always)]
    pub const fn out2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IO clear output for pin 2"]
    #[inline(always)]
    pub fn set_out2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IO clear output for pin 3"]
    #[inline(always)]
    pub const fn out3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IO clear output for pin 3"]
    #[inline(always)]
    pub fn set_out3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IO clear output for pin 4"]
    #[inline(always)]
    pub const fn out4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IO clear output for pin 4"]
    #[inline(always)]
    pub fn set_out4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO clear output for pin 5"]
    #[inline(always)]
    pub const fn out5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO clear output for pin 5"]
    #[inline(always)]
    pub fn set_out5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IO clear output for pin 6"]
    #[inline(always)]
    pub const fn out6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IO clear output for pin 6"]
    #[inline(always)]
    pub fn set_out6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IO clear output for pin 7"]
    #[inline(always)]
    pub const fn out7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IO clear output for pin 7"]
    #[inline(always)]
    pub fn set_out7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for OutClr {
    #[inline(always)]
    fn default() -> OutClr {
        OutClr(0)
    }
}
#[doc = "Port output data invert register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutInv(pub u32);
impl OutInv {
    #[doc = "IO invert output for pin 0: '0': Output state not affected. '1': Output state inverted ('0' => '1', '1' => '0')."]
    #[inline(always)]
    pub const fn out0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IO invert output for pin 0: '0': Output state not affected. '1': Output state inverted ('0' => '1', '1' => '0')."]
    #[inline(always)]
    pub fn set_out0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO invert output for pin 1"]
    #[inline(always)]
    pub const fn out1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO invert output for pin 1"]
    #[inline(always)]
    pub fn set_out1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IO invert output for pin 2"]
    #[inline(always)]
    pub const fn out2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IO invert output for pin 2"]
    #[inline(always)]
    pub fn set_out2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IO invert output for pin 3"]
    #[inline(always)]
    pub const fn out3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IO invert output for pin 3"]
    #[inline(always)]
    pub fn set_out3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IO invert output for pin 4"]
    #[inline(always)]
    pub const fn out4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IO invert output for pin 4"]
    #[inline(always)]
    pub fn set_out4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO invert output for pin 5"]
    #[inline(always)]
    pub const fn out5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO invert output for pin 5"]
    #[inline(always)]
    pub fn set_out5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IO invert output for pin 6"]
    #[inline(always)]
    pub const fn out6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IO invert output for pin 6"]
    #[inline(always)]
    pub fn set_out6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IO invert output for pin 7"]
    #[inline(always)]
    pub const fn out7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IO invert output for pin 7"]
    #[inline(always)]
    pub fn set_out7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for OutInv {
    #[inline(always)]
    fn default() -> OutInv {
        OutInv(0)
    }
}
#[doc = "Port output data set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSet(pub u32);
impl OutSet {
    #[doc = "IO set output for pin 0: '0': Output state not affected. '1': Output state set to '1'."]
    #[inline(always)]
    pub const fn out0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IO set output for pin 0: '0': Output state not affected. '1': Output state set to '1'."]
    #[inline(always)]
    pub fn set_out0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO set output for pin 1"]
    #[inline(always)]
    pub const fn out1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO set output for pin 1"]
    #[inline(always)]
    pub fn set_out1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IO set output for pin 2"]
    #[inline(always)]
    pub const fn out2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IO set output for pin 2"]
    #[inline(always)]
    pub fn set_out2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IO set output for pin 3"]
    #[inline(always)]
    pub const fn out3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IO set output for pin 3"]
    #[inline(always)]
    pub fn set_out3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IO set output for pin 4"]
    #[inline(always)]
    pub const fn out4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IO set output for pin 4"]
    #[inline(always)]
    pub fn set_out4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO set output for pin 5"]
    #[inline(always)]
    pub const fn out5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO set output for pin 5"]
    #[inline(always)]
    pub fn set_out5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IO set output for pin 6"]
    #[inline(always)]
    pub const fn out6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IO set output for pin 6"]
    #[inline(always)]
    pub fn set_out6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IO set output for pin 7"]
    #[inline(always)]
    pub const fn out7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IO set output for pin 7"]
    #[inline(always)]
    pub fn set_out7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for OutSet {
    #[inline(always)]
    fn default() -> OutSet {
        OutSet(0)
    }
}
#[doc = "Extern power supply detection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VddActive(pub u32);
impl VddActive {
    #[doc = "Indicates presence or absence of VDDIO supplies (i.e. other than VDDD, VDDA) on the device (supplies are numbered 0..n-1). Note that VDDIO supplies have basic (crude) supply detectors only. If separate, robust, brown-out detection is desired on IO supplies, on-chip or off-chip analog resources need to provide it. For these bits to work reliable, the supply must be within valid spec range (per datasheet) or held at ground. Any in-between voltage has an undefined result. '0': Supply is not present '1': Supply is present When multiple VDDIO supplies are present, they will be assigned in alphanumeric ascending order to these bits during implementation. For example 'vddusb, vddio_0, vddio_a, vbackup, vddio_r, vddio_1' are present then they will be assigned to these bits as below: 0: vbackup, 1: vddio_0, 2: vddio_1, 3: vddio_a, 4: vddio_r, 5: vddusb'"]
    #[inline(always)]
    pub const fn vddio_active(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates presence or absence of VDDIO supplies (i.e. other than VDDD, VDDA) on the device (supplies are numbered 0..n-1). Note that VDDIO supplies have basic (crude) supply detectors only. If separate, robust, brown-out detection is desired on IO supplies, on-chip or off-chip analog resources need to provide it. For these bits to work reliable, the supply must be within valid spec range (per datasheet) or held at ground. Any in-between voltage has an undefined result. '0': Supply is not present '1': Supply is present When multiple VDDIO supplies are present, they will be assigned in alphanumeric ascending order to these bits during implementation. For example 'vddusb, vddio_0, vddio_a, vbackup, vddio_r, vddio_1' are present then they will be assigned to these bits as below: 0: vbackup, 1: vddio_0, 2: vddio_1, 3: vddio_a, 4: vddio_r, 5: vddusb'"]
    #[inline(always)]
    pub fn set_vddio_active(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub const fn vdda_active(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn set_vdda_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit indicates presence of the VDDD supply. This bit will always read-back 1. The VDDD supply has robust brown-out protection monitoring and it is not possible to read back this register without a valid supply. (This bit is used in certain test-modes to observe the brown-out detector status.)"]
    #[inline(always)]
    pub const fn vddd_active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates presence of the VDDD supply. This bit will always read-back 1. The VDDD supply has robust brown-out protection monitoring and it is not possible to read back this register without a valid supply. (This bit is used in certain test-modes to observe the brown-out detector status.)"]
    #[inline(always)]
    pub fn set_vddd_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for VddActive {
    #[inline(always)]
    fn default() -> VddActive {
        VddActive(0)
    }
}
#[doc = "Supply detection interrupt register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VddIntr(pub u32);
impl VddIntr {
    #[doc = "Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
    #[inline(always)]
    pub const fn vddio_active(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
    #[inline(always)]
    pub fn set_vddio_active(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub const fn vdda_active(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn set_vdda_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
    #[inline(always)]
    pub const fn vddd_active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
    #[inline(always)]
    pub fn set_vddd_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for VddIntr {
    #[inline(always)]
    fn default() -> VddIntr {
        VddIntr(0)
    }
}
#[doc = "Supply detection interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VddIntrMask(pub u32);
impl VddIntrMask {
    #[doc = "Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
    #[inline(always)]
    pub const fn vddio_active(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
    #[inline(always)]
    pub fn set_vddio_active(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub const fn vdda_active(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn set_vdda_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub const fn vddd_active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn set_vddd_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for VddIntrMask {
    #[inline(always)]
    fn default() -> VddIntrMask {
        VddIntrMask(0)
    }
}
#[doc = "Supply detection interrupt masked register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VddIntrMasked(pub u32);
impl VddIntrMasked {
    #[doc = "Supply transition detected AND masked '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
    #[inline(always)]
    pub const fn vddio_active(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Supply transition detected AND masked '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
    #[inline(always)]
    pub fn set_vddio_active(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub const fn vdda_active(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn set_vdda_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub const fn vddd_active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn set_vddd_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for VddIntrMasked {
    #[inline(always)]
    fn default() -> VddIntrMasked {
        VddIntrMasked(0)
    }
}
#[doc = "Supply detection interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VddIntrSet(pub u32);
impl VddIntrSet {
    #[doc = "Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    pub const fn vddio_active(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    pub fn set_vddio_active(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub const fn vdda_active(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn set_vdda_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub const fn vddd_active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn set_vddd_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for VddIntrSet {
    #[inline(always)]
    fn default() -> VddIntrSet {
        VddIntrSet(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DriveMode0 {
    #[doc = "Output buffer is off creating a high impedance input D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    HIGHZ = 0,
    #[doc = "N/A"]
    RSVD = 0x01,
    #[doc = "Resistive pull up For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Weak/resistive pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull up D_OUT = '1': Weak/resistive pull up"]
    PULLUP = 0x02,
    #[doc = "Resistive pull down For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull down"]
    PULLDOWN = 0x03,
    #[doc = "Open drain, drives low For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': High Impedance When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    OD_DRIVESLOW = 0x04,
    #[doc = "Open drain, drives high For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': High Impedance D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    OD_DRIVESHIGH = 0x05,
    #[doc = "Strong D_OUTput buffer For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    STRONG = 0x06,
    #[doc = "Pull up or pull down For GPIO & UDB/DSI peripherals: When D_OUT_EN = '0': GPIO_DSI_OUT = '0': Weak/resistive pull down GPIO_DSI_OUT = '1': Weak/resistive pull up where 'GPIO_DSI_OUT' is a function of PORT_SEL, OUT & DSI_DATA_OUT. For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull up"]
    PULLUP_DOWN = 0x07,
}
impl DriveMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DriveMode0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DriveMode0 {
    #[inline(always)]
    fn from(val: u8) -> DriveMode0 {
        DriveMode0::from_bits(val)
    }
}
impl From<DriveMode0> for u8 {
    #[inline(always)]
    fn from(val: DriveMode0) -> u8 {
        DriveMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DriveSel0 {
    #[doc = "Traveo II: GPIO_STD/GPIO_ENH: Full drive strengh: GPIO drives current at its max rated spec. Traveo II:_GPIO_SMC: GPIO_SMC default mode. Traveo II:_HSIO_STD: HSIO default mode. PSoC 6: GPIO cells and HSIO_STD cells: Full drive strength: GPIO drives current at its max rated spec."]
    DRIVE_SEL_ZERO = 0,
    #[doc = "Traveo II: GPIO_STD/GPIO_ENH: Full drive strengh: GPIO drives current at its max rated spec. Traveo II:_GPIO_SMC: GPIO full drive strength. Traveo II:_HSIO_STD: GPIO full drive strength. PSoC 6: GPIO cells and HSIO_STD cells: 1/2 drive strength: GPIO drives current at 1/2 of its max rated spec"]
    DRIVE_SEL_ONE = 0x01,
    #[doc = "Traveo II: GPIO_STD/GPIO_ENH: 1/2 drive strength: GPIO drives current at 1/2 of its max rated spec. Traveo II:_GPIO_SMC: GPIO 1/2 drive strength. Traveo II:_HSIO_STD: GPIO 1/2 drive strength. PSoC 6: GPIO cells and HSIO_STD cells: 1/4 drive strength. GPIO drives current at 1/4 of its max rated spec."]
    DRIVE_SEL_TWO = 0x02,
    #[doc = "Traveo II: GPIO_STD/GPIO_ENH: 1/4 drive strength: GPIO drives current at 1/4 of its max rated spec. Traveo II:_GPIO_SMC: GPIO 1/4 drive strength. Traveo II:_HSIO_STD: GPIO 1/4 drive strength. PSoC 6: GPIO cells and HSIO_STD cells: 1/8 drive strength. GPIO drives current at 1/8 of its max rated spec."]
    DRIVE_SEL_THREE = 0x03,
}
impl DriveSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DriveSel0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DriveSel0 {
    #[inline(always)]
    fn from(val: u8) -> DriveSel0 {
        DriveSel0::from_bits(val)
    }
}
impl From<DriveSel0> for u8 {
    #[inline(always)]
    fn from(val: DriveSel0) -> u8 {
        DriveSel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Edge0Sel {
    #[doc = "Disabled"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl Edge0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edge0Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edge0Sel {
    #[inline(always)]
    fn from(val: u8) -> Edge0Sel {
        Edge0Sel::from_bits(val)
    }
}
impl From<Edge0Sel> for u8 {
    #[inline(always)]
    fn from(val: Edge0Sel) -> u8 {
        Edge0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FltEdgeSel {
    #[doc = "Disabled"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl FltEdgeSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FltEdgeSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FltEdgeSel {
    #[inline(always)]
    fn from(val: u8) -> FltEdgeSel {
        FltEdgeSel::from_bits(val)
    }
}
impl From<FltEdgeSel> for u8 {
    #[inline(always)]
    fn from(val: FltEdgeSel) -> u8 {
        FltEdgeSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum VtripSel00 {
    #[doc = "PSoC 6:: Input buffer compatible with CMOS and I2C interfaces Traveo II: Full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    CMOS = 0,
    #[doc = "PSoC 6:: Input buffer compatible with TTL and MediaLB interfaces Traveo II: full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    TTL = 0x01,
}
impl VtripSel00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VtripSel00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VtripSel00 {
    #[inline(always)]
    fn from(val: u8) -> VtripSel00 {
        VtripSel00::from_bits(val)
    }
}
impl From<VtripSel00> for u8 {
    #[inline(always)]
    fn from(val: VtripSel00) -> u8 {
        VtripSel00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum VtripSel01 {
    #[doc = "Input buffer compatible with CMOS/TTL interfaces as described in CFG_IN.VTRIP_SEL0_0."]
    CMOS_OR_TTL = 0,
    #[doc = "Input buffer compatible with AUTO (elevated Vil) interfaces when used along with CFG_IN.VTRIP_SEL0_0."]
    AUTO = 0x01,
}
impl VtripSel01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VtripSel01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VtripSel01 {
    #[inline(always)]
    fn from(val: u8) -> VtripSel01 {
        VtripSel01::from_bits(val)
    }
}
impl From<VtripSel01> for u8 {
    #[inline(always)]
    fn from(val: VtripSel01) -> u8 {
        VtripSel01::to_bits(val)
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
