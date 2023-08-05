#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "SRSS Backup Domain"]
use core::prelude::v1::derive;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Backup {
    ptr: *mut u8,
}
unsafe impl Send for Backup {}
unsafe impl Sync for Backup {}
impl Backup {
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
    pub const fn ctl(self) -> crate::common::Reg<Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "RTC Read Write register"]
    #[inline(always)]
    pub const fn rtc_rw(self) -> crate::common::Reg<RtcRw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Oscillator calibration for absolute frequency"]
    #[inline(always)]
    pub const fn cal_ctl(self) -> crate::common::Reg<CalCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Calendar Seconds, Minutes, Hours, Day of Week"]
    #[inline(always)]
    pub const fn rtc_time(self) -> crate::common::Reg<RtcTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Calendar Day of Month, Month, Year"]
    #[inline(always)]
    pub const fn rtc_date(self) -> crate::common::Reg<RtcDate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week"]
    #[inline(always)]
    pub const fn alm1_time(self) -> crate::common::Reg<Alm1Time, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Alarm 1 Day of Month, Month"]
    #[inline(always)]
    pub const fn alm1_date(self) -> crate::common::Reg<Alm1Date, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Alarm 2 Seconds, Minute, Hours, Day of Week"]
    #[inline(always)]
    pub const fn alm2_time(self) -> crate::common::Reg<Alm2Time, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Alarm 2 Day of Month, Month"]
    #[inline(always)]
    pub const fn alm2_date(self) -> crate::common::Reg<Alm2Date, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Interrupt request register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "32kHz oscillator counter"]
    #[inline(always)]
    pub const fn osccnt(self) -> crate::common::Reg<Osccnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "128Hz tick counter"]
    #[inline(always)]
    pub const fn ticks(self) -> crate::common::Reg<Ticks, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "PMIC control register"]
    #[inline(always)]
    pub const fn pmic_ctl(self) -> crate::common::Reg<PmicCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Backup reset register"]
    #[inline(always)]
    pub const fn reset(self) -> crate::common::Reg<Reset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Backup register region"]
    #[inline(always)]
    pub const fn breg(self, n: usize) -> crate::common::Reg<Breg, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4096usize + n * 4usize) as _) }
    }
    #[doc = "Trim Register"]
    #[inline(always)]
    pub const fn trim(self) -> crate::common::Reg<Trim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65280usize) as _) }
    }
}
#[doc = "Alarm 1 Day of Month, Month"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alm1Date(pub u32);
impl Alm1Date {
    #[doc = "Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub const fn alm_date(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn set_alm_date(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_date_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_date_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub const fn alm_mon(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn set_alm_mon(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_mon_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_mon_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Master enable for alarm 1. 0: Alarm 1 is disabled. Fields for date and time are ignored. 1: Alarm 1 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub const fn alm_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for alarm 1. 0: Alarm 1 is disabled. Fields for date and time are ignored. 1: Alarm 1 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn set_alm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Alm1Date {
    #[inline(always)]
    fn default() -> Alm1Date {
        Alm1Date(0)
    }
}
#[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alm1Time(pub u32);
impl Alm1Time {
    #[doc = "Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub const fn alm_sec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn set_alm_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_sec_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_sec_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub const fn alm_min(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn set_alm_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_min_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_min_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub const fn alm_hour(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn set_alm_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_hour_en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_hour_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub const fn alm_day(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn set_alm_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_day_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_day_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Alm1Time {
    #[inline(always)]
    fn default() -> Alm1Time {
        Alm1Time(0)
    }
}
#[doc = "Alarm 2 Day of Month, Month"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alm2Date(pub u32);
impl Alm2Date {
    #[doc = "Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub const fn alm_date(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn set_alm_date(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_date_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_date_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub const fn alm_mon(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn set_alm_mon(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_mon_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_mon_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub const fn alm_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn set_alm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Alm2Date {
    #[inline(always)]
    fn default() -> Alm2Date {
        Alm2Date(0)
    }
}
#[doc = "Alarm 2 Seconds, Minute, Hours, Day of Week"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alm2Time(pub u32);
impl Alm2Time {
    #[doc = "Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub const fn alm_sec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn set_alm_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_sec_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_sec_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub const fn alm_min(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn set_alm_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_min_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_min_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub const fn alm_hour(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn set_alm_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_hour_en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_hour_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub const fn alm_day(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn set_alm_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub const fn alm_day_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn set_alm_day_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Alm2Time {
    #[inline(always)]
    fn default() -> Alm2Time {
        Alm2Time(0)
    }
}
#[doc = "Backup register region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Breg(pub u32);
impl Breg {
    #[doc = "Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
    #[inline(always)]
    pub const fn breg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
    #[inline(always)]
    pub fn set_breg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Breg {
    #[inline(always)]
    fn default() -> Breg {
        Breg(0)
    }
}
#[doc = "Oscillator calibration for absolute frequency"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCtl(pub u32);
impl CalCtl {
    #[doc = "Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    pub const fn calib_val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    pub fn set_calib_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    pub const fn calib_sign(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    pub fn set_calib_sign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    pub const fn cal_out(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    pub fn set_cal_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CalCtl {
    #[inline(always)]
    fn default() -> CalCtl {
        CalCtl(0)
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
    #[inline(always)]
    pub const fn wco_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
    #[inline(always)]
    pub fn set_wco_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock select for BAK clock"]
    #[inline(always)]
    pub const fn clk_sel(&self) -> ClkSel {
        let val = (self.0 >> 8usize) & 0x03;
        ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock select for BAK clock"]
    #[inline(always)]
    pub fn set_clk_sel(&mut self, val: ClkSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn prescaler(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_prescaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
    #[inline(always)]
    pub const fn wco_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
    #[inline(always)]
    pub fn set_wco_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
    #[inline(always)]
    pub const fn vddbak_ctl(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
    #[inline(always)]
    pub fn set_vddbak_ctl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
    #[inline(always)]
    pub const fn vbackup_meas(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
    #[inline(always)]
    pub fn set_vbackup_meas(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
    #[inline(always)]
    pub const fn en_charge_key(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
    #[inline(always)]
    pub fn set_en_charge_key(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "Interrupt request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Alarm 1 Interrupt"]
    #[inline(always)]
    pub const fn alarm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm 1 Interrupt"]
    #[inline(always)]
    pub fn set_alarm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Alarm 2 Interrupt"]
    #[inline(always)]
    pub const fn alarm2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm 2 Interrupt"]
    #[inline(always)]
    pub fn set_alarm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Century overflow interrupt"]
    #[inline(always)]
    pub const fn century(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Century overflow interrupt"]
    #[inline(always)]
    pub fn set_century(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn alarm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_alarm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn alarm2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_alarm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn century(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_century(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn alarm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_alarm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn alarm2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_alarm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn century(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_century(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn alarm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_alarm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn alarm2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_alarm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn century(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_century(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "32kHz oscillator counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osccnt(pub u32);
impl Osccnt {
    #[doc = "32kHz oscillator count (msb=128Hz), calibration can cause bit 6 to skip. Reset when RTC_TIME.RTC_SEC fields is written."]
    #[inline(always)]
    pub const fn cnt32khz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "32kHz oscillator count (msb=128Hz), calibration can cause bit 6 to skip. Reset when RTC_TIME.RTC_SEC fields is written."]
    #[inline(always)]
    pub fn set_cnt32khz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Osccnt {
    #[inline(always)]
    fn default() -> Osccnt {
        Osccnt(0)
    }
}
#[doc = "PMIC control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmicCtl(pub u32);
impl PmicCtl {
    #[doc = "This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
    #[inline(always)]
    pub const fn unlock(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
    #[inline(always)]
    pub fn set_unlock(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn polarity(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
    #[inline(always)]
    pub const fn pmic_en_outen(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
    #[inline(always)]
    pub fn set_pmic_en_outen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
    #[inline(always)]
    pub const fn pmic_alwaysen(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
    #[inline(always)]
    pub fn set_pmic_alwaysen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
    #[inline(always)]
    pub const fn pmic_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
    #[inline(always)]
    pub fn set_pmic_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PmicCtl {
    #[inline(always)]
    fn default() -> PmicCtl {
        PmicCtl(0)
    }
}
#[doc = "Backup reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reset(pub u32);
impl Reset {
    #[doc = "Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Reset {
    #[inline(always)]
    fn default() -> Reset {
        Reset(0)
    }
}
#[doc = "Calendar Day of Month, Month, Year"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcDate(pub u32);
impl RtcDate {
    #[doc = "Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
    #[inline(always)]
    pub const fn rtc_date(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
    #[inline(always)]
    pub fn set_rtc_date(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calendar Month in BCD, 1-12"]
    #[inline(always)]
    pub const fn rtc_mon(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Calendar Month in BCD, 1-12"]
    #[inline(always)]
    pub fn set_rtc_mon(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Calendar year in BCD, 0-99"]
    #[inline(always)]
    pub const fn rtc_year(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Calendar year in BCD, 0-99"]
    #[inline(always)]
    pub fn set_rtc_year(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for RtcDate {
    #[inline(always)]
    fn default() -> RtcDate {
        RtcDate(0)
    }
}
#[doc = "RTC Read Write register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcRw(pub u32);
impl RtcRw {
    #[doc = "Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
    #[inline(always)]
    pub const fn read(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
    #[inline(always)]
    pub fn set_read(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
    #[inline(always)]
    pub const fn write(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
    #[inline(always)]
    pub fn set_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RtcRw {
    #[inline(always)]
    fn default() -> RtcRw {
        RtcRw(0)
    }
}
#[doc = "Calendar Seconds, Minutes, Hours, Day of Week"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcTime(pub u32);
impl RtcTime {
    #[doc = "Calendar seconds in BCD, 0-59"]
    #[inline(always)]
    pub const fn rtc_sec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Calendar seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn set_rtc_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Calendar minutes in BCD, 0-59"]
    #[inline(always)]
    pub const fn rtc_min(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Calendar minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn set_rtc_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
    #[inline(always)]
    pub const fn rtc_hour(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
    #[inline(always)]
    pub fn set_rtc_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    pub const fn ctrl_12hr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    pub fn set_ctrl_12hr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub const fn rtc_day(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn set_rtc_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for RtcTime {
    #[inline(always)]
    fn default() -> RtcTime {
        RtcTime(0)
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "pending RTC write"]
    #[inline(always)]
    pub const fn rtc_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "pending RTC write"]
    #[inline(always)]
    pub fn set_rtc_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates that output has transitioned."]
    #[inline(always)]
    pub const fn wco_ok(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that output has transitioned."]
    #[inline(always)]
    pub fn set_wco_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "128Hz tick counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ticks(pub u32);
impl Ticks {
    #[doc = "128Hz counter (msb=2Hz) When SECONDS is written this field will be reset."]
    #[inline(always)]
    pub const fn cnt128hz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "128Hz counter (msb=2Hz) When SECONDS is written this field will be reset."]
    #[inline(always)]
    pub fn set_cnt128hz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Ticks {
    #[inline(always)]
    fn default() -> Ticks {
        Ticks(0)
    }
}
#[doc = "Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trim(pub u32);
impl Trim {
    #[doc = "WCO trim"]
    #[inline(always)]
    pub const fn trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "WCO trim"]
    #[inline(always)]
    pub fn set_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Trim {
    #[inline(always)]
    fn default() -> Trim {
        Trim(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClkSel {
    #[doc = "Watch-crystal oscillator input."]
    WCO = 0,
    #[doc = "This allows to use the LFCLK selection as an alternate backup domain clock. Note that LFCLK is not available in all power modes, and clock glitches can propagate into the backup logic when the clock is stopped. For this reason, if the WCO is intended as the clock source then choose it directly instead of routing through LFCLK."]
    ALTBAK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel {
    #[inline(always)]
    fn from(val: u8) -> ClkSel {
        ClkSel::from_bits(val)
    }
}
impl From<ClkSel> for u8 {
    #[inline(always)]
    fn from(val: ClkSel) -> u8 {
        ClkSel::to_bits(val)
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
