#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Low Power Comparators"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpcomp {
    ptr: *mut u8,
}
unsafe impl Send for Lpcomp {}
unsafe impl Sync for Lpcomp {}
impl Lpcomp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LPCOMP Configuration Register"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "LPCOMP Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "LPCOMP Interrupt request register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "LPCOMP Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "LPCOMP Interrupt request mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "LPCOMP Interrupt request masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Comparator 0 control Register"]
    #[inline(always)]
    pub const fn cmp0_ctrl(self) -> crate::common::Reg<Cmp0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Comparator 0 switch control"]
    #[inline(always)]
    pub const fn cmp0_sw(self) -> crate::common::Reg<Cmp0Sw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Comparator 0 switch control clear"]
    #[inline(always)]
    pub const fn cmp0_sw_clear(self) -> crate::common::Reg<Cmp0SwClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Comparator 1 control Register"]
    #[inline(always)]
    pub const fn cmp1_ctrl(self) -> crate::common::Reg<Cmp1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Comparator 1 switch control"]
    #[inline(always)]
    pub const fn cmp1_sw(self) -> crate::common::Reg<Cmp1Sw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "Comparator 1 switch control clear"]
    #[inline(always)]
    pub const fn cmp1_sw_clear(self) -> crate::common::Reg<Cmp1SwClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
}
#[doc = "Comparator 0 control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0Ctrl(pub u32);
impl Cmp0Ctrl {
    #[doc = "Operating mode for the comparator"]
    #[inline(always)]
    pub const fn mode0(&self) -> Mode0 {
        let val = (self.0 >> 0usize) & 0x03;
        Mode0::from_bits(val as u8)
    }
    #[doc = "Operating mode for the comparator"]
    #[inline(always)]
    pub fn set_mode0(&mut self, val: Mode0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub const fn hyst0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn set_hyst0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub const fn inttype0(&self) -> Inttype0 {
        let val = (self.0 >> 6usize) & 0x03;
        Inttype0::from_bits(val as u8)
    }
    #[doc = "Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn set_inttype0(&mut self, val: Inttype0) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub const fn dsi_bypass0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn set_dsi_bypass0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub const fn dsi_level0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn set_dsi_level0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Cmp0Ctrl {
    #[inline(always)]
    fn default() -> Cmp0Ctrl {
        Cmp0Ctrl(0)
    }
}
#[doc = "Comparator 0 switch control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0Sw(pub u32);
impl Cmp0Sw {
    #[doc = "Comparator 0 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub const fn cmp0_ip0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 0 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn set_cmp0_ip0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 0 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub const fn cmp0_ap0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 0 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn set_cmp0_ap0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator 0 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub const fn cmp0_bp0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 0 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn set_cmp0_bp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparator 0 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub const fn cmp0_in0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 0 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn set_cmp0_in0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Comparator 0 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub const fn cmp0_an0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 0 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn set_cmp0_an0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Comparator 0 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub const fn cmp0_bn0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 0 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn set_cmp0_bn0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Comparator 0 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub const fn cmp0_vn0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 0 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn set_cmp0_vn0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Cmp0Sw {
    #[inline(always)]
    fn default() -> Cmp0Sw {
        Cmp0Sw(0)
    }
}
#[doc = "Comparator 0 switch control clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0SwClear(pub u32);
impl Cmp0SwClear {
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub const fn cmp0_ip0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn set_cmp0_ip0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub const fn cmp0_ap0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn set_cmp0_ap0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub const fn cmp0_bp0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn set_cmp0_bp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub const fn cmp0_in0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn set_cmp0_in0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub const fn cmp0_an0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn set_cmp0_an0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub const fn cmp0_bn0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn set_cmp0_bn0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub const fn cmp0_vn0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn set_cmp0_vn0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Cmp0SwClear {
    #[inline(always)]
    fn default() -> Cmp0SwClear {
        Cmp0SwClear(0)
    }
}
#[doc = "Comparator 1 control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1Ctrl(pub u32);
impl Cmp1Ctrl {
    #[doc = "Operating mode for the comparator"]
    #[inline(always)]
    pub const fn mode1(&self) -> Mode1 {
        let val = (self.0 >> 0usize) & 0x03;
        Mode1::from_bits(val as u8)
    }
    #[doc = "Operating mode for the comparator"]
    #[inline(always)]
    pub fn set_mode1(&mut self, val: Mode1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub const fn hyst1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn set_hyst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub const fn inttype1(&self) -> Inttype1 {
        let val = (self.0 >> 6usize) & 0x03;
        Inttype1::from_bits(val as u8)
    }
    #[doc = "Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn set_inttype1(&mut self, val: Inttype1) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub const fn dsi_bypass1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn set_dsi_bypass1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub const fn dsi_level1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn set_dsi_level1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Cmp1Ctrl {
    #[inline(always)]
    fn default() -> Cmp1Ctrl {
        Cmp1Ctrl(0)
    }
}
#[doc = "Comparator 1 switch control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1Sw(pub u32);
impl Cmp1Sw {
    #[doc = "Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub const fn cmp1_ip1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn set_cmp1_ip1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub const fn cmp1_ap1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn set_cmp1_ap1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub const fn cmp1_bp1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn set_cmp1_bp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub const fn cmp1_in1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn set_cmp1_in1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub const fn cmp1_an1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn set_cmp1_an1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub const fn cmp1_bn1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn set_cmp1_bn1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub const fn cmp1_vn1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn set_cmp1_vn1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Cmp1Sw {
    #[inline(always)]
    fn default() -> Cmp1Sw {
        Cmp1Sw(0)
    }
}
#[doc = "Comparator 1 switch control clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1SwClear(pub u32);
impl Cmp1SwClear {
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub const fn cmp1_ip1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn set_cmp1_ip1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub const fn cmp1_ap1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn set_cmp1_ap1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub const fn cmp1_bp1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn set_cmp1_bp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub const fn cmp1_in1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn set_cmp1_in1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub const fn cmp1_an1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn set_cmp1_an1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub const fn cmp1_bn1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn set_cmp1_bn1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub const fn cmp1_vn1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn set_cmp1_vn1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Cmp1SwClear {
    #[inline(always)]
    fn default() -> Cmp1SwClear {
        Cmp1SwClear(0)
    }
}
#[doc = "LPCOMP Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
    #[inline(always)]
    pub const fn lpref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
    #[inline(always)]
    pub fn set_lpref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "- 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "LPCOMP Interrupt request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn comp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_comp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn comp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_comp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "LPCOMP Interrupt request mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp0_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp0_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp1_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp1_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "LPCOMP Interrupt request masked"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn comp0_masked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_comp0_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn comp1_masked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_comp1_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "LPCOMP Interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "LPCOMP Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Current output value of the comparator 0."]
    #[inline(always)]
    pub const fn out0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current output value of the comparator 0."]
    #[inline(always)]
    pub fn set_out0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Current output value of the comparator 1."]
    #[inline(always)]
    pub const fn out1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Current output value of the comparator 1."]
    #[inline(always)]
    pub fn set_out1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Inttype0 {
    #[doc = "Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl Inttype0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inttype0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inttype0 {
    #[inline(always)]
    fn from(val: u8) -> Inttype0 {
        Inttype0::from_bits(val)
    }
}
impl From<Inttype0> for u8 {
    #[inline(always)]
    fn from(val: Inttype0) -> u8 {
        Inttype0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Inttype1 {
    #[doc = "Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl Inttype1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inttype1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inttype1 {
    #[inline(always)]
    fn from(val: u8) -> Inttype1 {
        Inttype1::from_bits(val)
    }
}
impl From<Inttype1> for u8 {
    #[inline(always)]
    fn from(val: Inttype1) -> u8 {
        Inttype1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode0 {
    #[doc = "Off"]
    OFF = 0,
    #[doc = "Ultra lowpower operating mode (uses less power, < 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    ULP = 0x01,
    #[doc = "Low Power operating mode (uses more power, <10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    LP = 0x02,
    #[doc = "Normal, full speed power operating mode (uses <150uA). In this mode the iref from SRSS will be used."]
    NORMAL = 0x03,
}
impl Mode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode0 {
    #[inline(always)]
    fn from(val: u8) -> Mode0 {
        Mode0::from_bits(val)
    }
}
impl From<Mode0> for u8 {
    #[inline(always)]
    fn from(val: Mode0) -> u8 {
        Mode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode1 {
    #[doc = "Off"]
    OFF = 0,
    #[doc = "Ultra lowpower operating mode (uses less power, < 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    ULP = 0x01,
    #[doc = "Low Power operating mode (uses more power, <10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    LP = 0x02,
    #[doc = "Normal, full speed power operating mode (uses <150uA). In this mode the iref from SRSS will be used."]
    NORMAL = 0x03,
}
impl Mode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode1 {
    #[inline(always)]
    fn from(val: u8) -> Mode1 {
        Mode1::from_bits(val)
    }
}
impl From<Mode1> for u8 {
    #[inline(always)]
    fn from(val: Mode1) -> u8 {
        Mode1::to_bits(val)
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
