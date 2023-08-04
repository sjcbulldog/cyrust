#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Profile counter structure"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CntStruct {
    ptr: *mut u8,
}
unsafe impl Send for CntStruct {}
unsafe impl Sync for CntStruct {}
impl CntStruct {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Profile counter configuration"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<CntStructCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Profile counter value"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
}
#[doc = "Energy Profiler IP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Profile {
    ptr: *mut u8,
}
unsafe impl Send for Profile {}
unsafe impl Sync for Profile {}
impl Profile {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Profile control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<ProfileCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Profile status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Profile command"]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Profile interrupt"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1984usize) as _) }
    }
    #[doc = "Profile interrupt set"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1988usize) as _) }
    }
    #[doc = "Profile interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1992usize) as _) }
    }
    #[doc = "Profile interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1996usize) as _) }
    }
    #[doc = "Profile counter structure"]
    #[inline(always)]
    pub const fn cnt_struct(self, n: usize) -> CntStruct {
        assert!(n < 8usize);
        unsafe { CntStruct::from_ptr(self.ptr.add(2048usize + n * 16usize) as _) }
    }
}
#[doc = "Profile command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub const fn start_tr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn set_start_tr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub const fn stop_tr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn set_stop_tr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    pub const fn clr_all_cnt(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    pub fn set_clr_all_cnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        Cmd(0)
    }
}
#[doc = "Profile counter value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "This field shows / specifies the actual value of the profiling counter. It allows reading as well as writing the profiling counter."]
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field shows / specifies the actual value of the profiling counter. It allows reading as well as writing the profiling counter."]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
#[doc = "Profile counter configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CntStructCtl(pub u32);
impl CntStructCtl {
    #[doc = "This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
    #[inline(always)]
    pub const fn cnt_duration(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
    #[inline(always)]
    pub fn set_cnt_duration(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
    #[inline(always)]
    pub const fn ref_clk_sel(&self) -> RefClkSel {
        let val = (self.0 >> 4usize) & 0x07;
        RefClkSel::from_bits(val as u8)
    }
    #[doc = "This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
    #[inline(always)]
    pub fn set_ref_clk_sel(&mut self, val: RefClkSel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
    #[inline(always)]
    pub const fn mon_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
    #[inline(always)]
    pub fn set_mon_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Enables the profiling counter: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the profiling counter: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CntStructCtl {
    #[inline(always)]
    fn default() -> CntStructCtl {
        CntStructCtl(0)
    }
}
#[doc = "Profile interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
    #[inline(always)]
    pub const fn cnt_ovflw(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
    #[inline(always)]
    pub fn set_cnt_ovflw(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Profile interrupt mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub const fn cnt_ovflw(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn set_cnt_ovflw(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Profile interrupt masked"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub const fn cnt_ovflw(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn set_cnt_ovflw(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Profile interrupt set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
    #[inline(always)]
    pub const fn cnt_ovflw(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
    #[inline(always)]
    pub fn set_cnt_ovflw(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Profile control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProfileCtl(pub u32);
impl ProfileCtl {
    #[doc = "Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    pub const fn win_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    pub fn set_win_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ProfileCtl {
    #[inline(always)]
    fn default() -> ProfileCtl {
        ProfileCtl(0)
    }
}
#[doc = "Profile status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Indicates if the profiling time window is active. '0': Not active. '1': Active."]
    #[inline(always)]
    pub const fn win_active(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if the profiling time window is active. '0': Not active. '1': Active."]
    #[inline(always)]
    pub fn set_win_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
pub enum RefClkSel {
    #[doc = "Timer clock (divided or undivided high frequency clock, e.g. from IMO). Selection is done in SRSS register CLK_TIMER_CTL.TIMER_SEL."]
    CLK_TIMER = 0,
    #[doc = "IMO - Internal Main Oscillator"]
    CLK_IMO = 0x01,
    #[doc = "ECO - External-Crystal Oscillator"]
    CLK_ECO = 0x02,
    #[doc = "Low frequency clock (ILO, WCO or ALTLF). Selection is done in SRSS register CLK_SELECT.LFCLK_SEL."]
    CLK_LF = 0x03,
    #[doc = "High frequuency clock ('clk_hfx')."]
    CLK_HF = 0x04,
    #[doc = "Peripheral clock ('clk_peri')."]
    CLK_PERI = 0x05,
    #[doc = "N/A"]
    RSVD_6 = 0x06,
    #[doc = "N/A"]
    RSVD_7 = 0x07,
}
impl RefClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefClkSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefClkSel {
    #[inline(always)]
    fn from(val: u8) -> RefClkSel {
        RefClkSel::from_bits(val)
    }
}
impl From<RefClkSel> for u8 {
    #[inline(always)]
    fn from(val: RefClkSel) -> u8 {
        RefClkSel::to_bits(val)
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
