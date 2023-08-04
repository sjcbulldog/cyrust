#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "LCD Controller Block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcd0 {
    ptr: *mut u8,
}
unsafe impl Send for Lcd0 {}
unsafe impl Sync for Lcd0 {}
impl Lcd0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ID & Revision"]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<Id, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "LCD Divider Register"]
    #[inline(always)]
    pub const fn divider(self) -> crate::common::Reg<Divider, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "LCD Configuration Register"]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data0(self, n: usize) -> crate::common::Reg<Data0, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
    #[doc = "LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data1(self, n: usize) -> crate::common::Reg<Data1, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize + n * 4usize) as _) }
    }
    #[doc = "LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data2(self, n: usize) -> crate::common::Reg<Data2, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize + n * 4usize) as _) }
    }
    #[doc = "LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data3(self, n: usize) -> crate::common::Reg<Data3, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize + n * 4usize) as _) }
    }
}
#[doc = "LCD Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub const fn ls_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn set_ls_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub const fn hs_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn set_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HS/LS Mode selection"]
    #[inline(always)]
    pub const fn lcd_mode(&self) -> LcdMode {
        let val = (self.0 >> 2usize) & 0x01;
        LcdMode::from_bits(val as u8)
    }
    #[doc = "HS/LS Mode selection"]
    #[inline(always)]
    pub fn set_lcd_mode(&mut self, val: LcdMode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LCD driving waveform type configuration."]
    #[inline(always)]
    pub const fn type_(&self) -> Type {
        let val = (self.0 >> 3usize) & 0x01;
        Type::from_bits(val as u8)
    }
    #[doc = "LCD driving waveform type configuration."]
    #[inline(always)]
    pub fn set_type_(&mut self, val: Type) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Driving mode configuration"]
    #[inline(always)]
    pub const fn op_mode(&self) -> OpMode {
        let val = (self.0 >> 4usize) & 0x01;
        OpMode::from_bits(val as u8)
    }
    #[doc = "Driving mode configuration"]
    #[inline(always)]
    pub fn set_op_mode(&mut self, val: OpMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PWM bias selection"]
    #[inline(always)]
    pub const fn bias(&self) -> Bias {
        let val = (self.0 >> 5usize) & 0x03;
        Bias::from_bits(val as u8)
    }
    #[doc = "PWM bias selection"]
    #[inline(always)]
    pub fn set_bias(&mut self, val: Bias) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Low speed (LS) generator clock source selection 1: select clk_mf 0: select clk_lf"]
    #[inline(always)]
    pub const fn clock_ls_sel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low speed (LS) generator clock source selection 1: select clk_mf 0: select clk_lf"]
    #[inline(always)]
    pub fn set_clock_ls_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    pub const fn com_num(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    pub fn set_com_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "LS enable status bit. This bit is a copy of LS_EN that is synchronized to the low speed clock domain and back to the system clock domain. Firmware can use this bit to observe whether LS_EN has taken effect in the low speed clock domain. Firmware should never change the configuration for the LS generator without ensuring this bit is 0. The following procedure should be followed to disable the LS generator: 1. If LS_EN=0 we are done. Exit the procedure. 2. Check that LS_EN_STAT=1. If not, wait until it is. This will catch the case of a recent enable (LS_EN=1) that has not taken effect yet. 3. Set LS_EN=0. 4. Wait until LS_EN_STAT=0."]
    #[inline(always)]
    pub const fn ls_en_stat(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LS enable status bit. This bit is a copy of LS_EN that is synchronized to the low speed clock domain and back to the system clock domain. Firmware can use this bit to observe whether LS_EN has taken effect in the low speed clock domain. Firmware should never change the configuration for the LS generator without ensuring this bit is 0. The following procedure should be followed to disable the LS generator: 1. If LS_EN=0 we are done. Exit the procedure. 2. Check that LS_EN_STAT=1. If not, wait until it is. This will catch the case of a recent enable (LS_EN=1) that has not taken effect yet. 3. Set LS_EN=0. 4. Wait until LS_EN_STAT=0."]
    #[inline(always)]
    pub fn set_ls_en_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
#[doc = "LCD Pin Data Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data0(pub u32);
impl Data0 {
    #[doc = "Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 1-4 (COM1 is lsb)."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 1-4 (COM1 is lsb)."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Data0 {
    #[inline(always)]
    fn default() -> Data0 {
        Data0(0)
    }
}
#[doc = "LCD Pin Data Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data1(pub u32);
impl Data1 {
    #[doc = "Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 5-8 (COM5 is lsb)."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 5-8 (COM5 is lsb)."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Data1 {
    #[inline(always)]
    fn default() -> Data1 {
        Data1(0)
    }
}
#[doc = "LCD Pin Data Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data2(pub u32);
impl Data2 {
    #[doc = "Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 9-12 (COM9 is lsb)."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 9-12 (COM9 is lsb)."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Data2 {
    #[inline(always)]
    fn default() -> Data2 {
        Data2(0)
    }
}
#[doc = "LCD Pin Data Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data3(pub u32);
impl Data3 {
    #[doc = "Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 13-16 (COM13 is lsb)."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 13-16 (COM13 is lsb)."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Data3 {
    #[inline(always)]
    fn default() -> Data3 {
        Data3(0)
    }
}
#[doc = "LCD Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Divider(pub u32);
impl Divider {
    #[doc = "Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub const fn subfr_div(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub fn set_subfr_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub const fn dead_div(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub fn set_dead_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Divider {
    #[inline(always)]
    fn default() -> Divider {
        Divider(0)
    }
}
#[doc = "ID & Revision"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "the ID of LCD controller peripheral is 0xF0F0"]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "the ID of LCD controller peripheral is 0xF0F0"]
    #[inline(always)]
    pub fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "the version number is 0x0002"]
    #[inline(always)]
    pub const fn revision(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "the version number is 0x0002"]
    #[inline(always)]
    pub fn set_revision(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bias {
    #[doc = "1/2 Bias"]
    HALF = 0,
    #[doc = "1/3 Bias"]
    THIRD = 0x01,
    #[doc = "1/4 Bias"]
    FOURTH = 0x02,
    #[doc = "1/5 Bias"]
    FIFTH = 0x03,
}
impl Bias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bias {
    #[inline(always)]
    fn from(val: u8) -> Bias {
        Bias::from_bits(val)
    }
}
impl From<Bias> for u8 {
    #[inline(always)]
    fn from(val: Bias) -> u8 {
        Bias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LcdMode {
    #[doc = "Select Low Speed Generator (Works in Active, Sleep and DeepSleep power modes). Low speed clock (clk_lf) or middle speed clock (clk_mf) can be selected for Low Speed Generator."]
    LS = 0,
    #[doc = "Select High Speed (system clock) Generator (Works in Active and Sleep power modes only)."]
    HS = 0x01,
}
impl LcdMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdMode {
    #[inline(always)]
    fn from(val: u8) -> LcdMode {
        LcdMode::from_bits(val)
    }
}
impl From<LcdMode> for u8 {
    #[inline(always)]
    fn from(val: LcdMode) -> u8 {
        LcdMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OpMode {
    #[doc = "PWM Mode"]
    PWM = 0,
    #[doc = "Digital Correlation Mode"]
    CORRELATION = 0x01,
}
impl OpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpMode {
    #[inline(always)]
    fn from(val: u8) -> OpMode {
        OpMode::from_bits(val)
    }
}
impl From<OpMode> for u8 {
    #[inline(always)]
    fn from(val: OpMode) -> u8 {
        OpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    #[doc = "Type A - Each frame addresses each COM pin only once with a balanced (DC=0) waveform."]
    TYPE_A = 0,
    #[doc = "Type B - Each frame addresses each COM pin twice in sequence with a positive and negative waveform that together are balanced (DC=0)."]
    TYPE_B = 0x01,
}
impl Type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Type {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Type {
    #[inline(always)]
    fn from(val: u8) -> Type {
        Type::from_bits(val)
    }
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(val: Type) -> u8 {
        Type::to_bits(val)
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
