#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Peripheral group structure"]
use core::prelude::v1::derive;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr {
    ptr: *mut u8,
}
unsafe impl Send for Gr {}
unsafe impl Sync for Gr {}
impl Gr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock control"]
    #[inline(always)]
    pub const fn clock_ctl(self) -> crate::common::Reg<GrClockCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Slave control"]
    #[inline(always)]
    pub const fn sl_ctl(self) -> crate::common::Reg<SlCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Timeout control"]
    #[inline(always)]
    pub const fn timeout_ctl(self) -> crate::common::Reg<TimeoutCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
}
#[doc = "Peripheral interconnect"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peri {
    ptr: *mut u8,
}
unsafe impl Send for Peri {}
unsafe impl Sync for Peri {}
impl Peri {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Peripheral group structure"]
    #[inline(always)]
    pub const fn gr(self, n: usize) -> Gr {
        assert!(n < 11usize);
        unsafe { Gr::from_ptr(self.ptr.add(0usize + n * 64usize) as _) }
    }
    #[doc = "Divider command register"]
    #[inline(always)]
    pub const fn div_cmd(self) -> crate::common::Reg<DivCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "Divider control register (for 8.0 divider)"]
    #[inline(always)]
    pub const fn div_8_ctl(self, n: usize) -> crate::common::Reg<Div8Ctl, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2048usize + n * 4usize) as _) }
    }
    #[doc = "Divider control register (for 16.0 divider)"]
    #[inline(always)]
    pub const fn div_16_ctl(self, n: usize) -> crate::common::Reg<Div16Ctl, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2304usize + n * 4usize) as _) }
    }
    #[doc = "Divider control register (for 16.5 divider)"]
    #[inline(always)]
    pub const fn div_16_5_ctl(self, n: usize) -> crate::common::Reg<Div165Ctl, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2560usize + n * 4usize) as _) }
    }
    #[doc = "Divider control register (for 24.5 divider)"]
    #[inline(always)]
    pub const fn div_24_5_ctl(self, n: usize) -> crate::common::Reg<Div245Ctl, crate::common::RW> {
        assert!(n < 63usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2816usize + n * 4usize) as _) }
    }
    #[doc = "Clock control register"]
    #[inline(always)]
    pub const fn clock_ctl(self, n: usize) -> crate::common::Reg<PeriClockCtl, crate::common::RW> {
        assert!(n < 128usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3072usize + n * 4usize) as _) }
    }
    #[doc = "Trigger command register"]
    #[inline(always)]
    pub const fn tr_cmd(self) -> crate::common::Reg<TrCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4096usize) as _) }
    }
    #[doc = "Trigger group"]
    #[inline(always)]
    pub const fn tr_gr(self, n: usize) -> TrGr {
        assert!(n < 15usize);
        unsafe { TrGr::from_ptr(self.ptr.add(8192usize + n * 512usize) as _) }
    }
    #[doc = "PPU structure with programmable address"]
    #[inline(always)]
    pub const fn ppu_pr(self, n: usize) -> PpuPr {
        assert!(n < 16usize);
        unsafe { PpuPr::from_ptr(self.ptr.add(16384usize + n * 64usize) as _) }
    }
    #[doc = "PPU structure with fixed/constant address for a peripheral group"]
    #[inline(always)]
    pub const fn ppu_gr(self, n: usize) -> PpuGr {
        assert!(n < 11usize);
        unsafe { PpuGr::from_ptr(self.ptr.add(20480usize + n * 64usize) as _) }
    }
}
#[doc = "PPU structure with fixed/constant address for a peripheral group"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuGr {
    ptr: *mut u8,
}
unsafe impl Send for PpuGr {}
unsafe impl Sync for PpuGr {}
impl PpuGr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PPU region address 0 (slave structure)"]
    #[inline(always)]
    pub const fn addr0(self) -> crate::common::Reg<PpuGrAddr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "PPU region attributes 0 (slave structure)"]
    #[inline(always)]
    pub const fn att0(self) -> crate::common::Reg<PpuGrAtt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "PPU region address 1 (master structure)"]
    #[inline(always)]
    pub const fn addr1(self) -> crate::common::Reg<PpuGrAddr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "PPU region attributes 1 (master structure)"]
    #[inline(always)]
    pub const fn att1(self) -> crate::common::Reg<PpuGrAtt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
}
#[doc = "PPU structure with programmable address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPr {
    ptr: *mut u8,
}
unsafe impl Send for PpuPr {}
unsafe impl Sync for PpuPr {}
impl PpuPr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PPU region address 0 (slave structure)"]
    #[inline(always)]
    pub const fn addr0(self) -> crate::common::Reg<PpuPrAddr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "PPU region attributes 0 (slave structure)"]
    #[inline(always)]
    pub const fn att0(self) -> crate::common::Reg<PpuPrAtt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "PPU region address 1 (master structure)"]
    #[inline(always)]
    pub const fn addr1(self) -> crate::common::Reg<PpuPrAddr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "PPU region attributes 1 (master structure)"]
    #[inline(always)]
    pub const fn att1(self) -> crate::common::Reg<PpuPrAtt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
}
#[doc = "Trigger group"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrGr {
    ptr: *mut u8,
}
unsafe impl Send for TrGr {}
unsafe impl Sync for TrGr {}
impl TrGr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Trigger control register"]
    #[inline(always)]
    pub const fn tr_out_ctl(self, n: usize) -> crate::common::Reg<TrOutCtl, crate::common::RW> {
        assert!(n < 128usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize + n * 4usize) as _) }
    }
}
#[doc = "Divider control register (for 16.5 divider)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div165Ctl(pub u32);
impl Div165Ctl {
    #[doc = "Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn frac5_div(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_frac5_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\] in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn int16_div(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\] in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_int16_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
    }
}
impl Default for Div165Ctl {
    #[inline(always)]
    fn default() -> Div165Ctl {
        Div165Ctl(0)
    }
}
#[doc = "Divider control register (for 16.0 divider)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div16Ctl(pub u32);
impl Div16Ctl {
    #[doc = "Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 65,536\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is restricted to even numbers in the range \\[2, 65,536\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn int16_div(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 65,536\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is restricted to even numbers in the range \\[2, 65,536\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_int16_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
    }
}
impl Default for Div16Ctl {
    #[inline(always)]
    fn default() -> Div16Ctl {
        Div16Ctl(0)
    }
}
#[doc = "Divider control register (for 24.5 divider)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div245Ctl(pub u32);
impl Div245Ctl {
    #[doc = "Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn frac5_div(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_frac5_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "Integer division by (1+INT24_DIV). Allows for integer divisions in the range \\[1, 16,777,216\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 16,777,216 31/32\\] in 1/32 increments. For the generation of a divided clock, the integer division range is restricted to \\[2, 16,777,216 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 16,777,216\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn int24_div(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Integer division by (1+INT24_DIV). Allows for integer divisions in the range \\[1, 16,777,216\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 16,777,216 31/32\\] in 1/32 increments. For the generation of a divided clock, the integer division range is restricted to \\[2, 16,777,216 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 16,777,216\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_int24_div(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Div245Ctl {
    #[inline(always)]
    fn default() -> Div245Ctl {
        Div245Ctl(0)
    }
}
#[doc = "Divider control register (for 8.0 divider)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div8Ctl(pub u32);
impl Div8Ctl {
    #[doc = "Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn int8_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_int8_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Div8Ctl {
    #[inline(always)]
    fn default() -> Div8Ctl {
        Div8Ctl(0)
    }
}
#[doc = "Divider command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DivCmd(pub u32);
impl DivCmd {
    #[doc = "(TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
    #[inline(always)]
    pub const fn div_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "(TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
    #[inline(always)]
    pub fn set_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub const fn type_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn set_type_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "(PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '63' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
    #[inline(always)]
    pub const fn pa_div_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "(PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '63' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
    #[inline(always)]
    pub fn set_pa_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub const fn pa_type_sel(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn set_pa_type_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
    #[inline(always)]
    pub const fn disable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
    #[inline(always)]
    pub fn set_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DivCmd {
    #[inline(always)]
    fn default() -> DivCmd {
        DivCmd(0)
    }
}
#[doc = "Clock control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrClockCtl(pub u32);
impl GrClockCtl {
    #[doc = "Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn int8_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_int8_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for GrClockCtl {
    #[inline(always)]
    fn default() -> GrClockCtl {
        GrClockCtl(0)
    }
}
#[doc = "Clock control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PeriClockCtl(pub u32);
impl PeriClockCtl {
    #[doc = "Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '63' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    pub const fn div_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '63' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    pub fn set_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub const fn type_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn set_type_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for PeriClockCtl {
    #[inline(always)]
    fn default() -> PeriClockCtl {
        PeriClockCtl(0)
    }
}
#[doc = "PPU region address 0 (slave structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuGrAddr0(pub u32);
impl PpuGrAddr0 {
    #[doc = "See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub const fn subregion_disable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub fn set_subregion_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': address of protected region. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub const fn addr24(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': address of protected region. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub fn set_addr24(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for PpuGrAddr0 {
    #[inline(always)]
    fn default() -> PpuGrAddr0 {
        PpuGrAddr0(0)
    }
}
#[doc = "PPU region address 1 (master structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuGrAddr1(pub u32);
impl PpuGrAddr1 {
    #[doc = "See corresponding field for PPU structure with programmable address. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub const fn subregion_disable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub fn set_subregion_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub const fn addr24(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub fn set_addr24(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for PpuGrAddr1 {
    #[inline(always)]
    fn default() -> PpuGrAddr1 {
        PpuGrAddr1(0)
    }
}
#[doc = "PPU region attributes 0 (slave structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuGrAtt0(pub u32);
impl PpuGrAtt0 {
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
    #[inline(always)]
    pub const fn ux(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn set_ux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pw(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
    #[inline(always)]
    pub const fn px(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn set_px(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x7fff;
        val as u16
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 9usize)) | (((val as u32) & 0x7fff) << 9usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pc_match(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PpuGrAtt0 {
    #[inline(always)]
    fn default() -> PpuGrAtt0 {
        PpuGrAtt0(0)
    }
}
#[doc = "PPU region attributes 1 (master structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuGrAtt1(pub u32);
impl PpuGrAtt1 {
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub const fn ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn set_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
    #[inline(always)]
    pub const fn ux(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn set_ux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pw(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
    #[inline(always)]
    pub const fn px(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn set_px(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x7fff;
        val as u16
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 9usize)) | (((val as u32) & 0x7fff) << 9usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. '7': 256 B region"]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "See corresponding field for PPU structure with programmable address. '7': 256 B region"]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pc_match(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PpuGrAtt1 {
    #[inline(always)]
    fn default() -> PpuGrAtt1 {
        PpuGrAtt1(0)
    }
}
#[doc = "PPU region address 0 (slave structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrAddr0(pub u32);
impl PpuPrAddr0 {
    #[doc = "This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by PPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub const fn subregion_disable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by PPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub fn set_subregion_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by PPU_REGION_ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\] are ignored."]
    #[inline(always)]
    pub const fn addr24(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by PPU_REGION_ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\] are ignored."]
    #[inline(always)]
    pub fn set_addr24(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for PpuPrAddr0 {
    #[inline(always)]
    fn default() -> PpuPrAddr0 {
        PpuPrAddr0(0)
    }
}
#[doc = "PPU region address 1 (master structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrAddr1(pub u32);
impl PpuPrAddr1 {
    #[doc = "See corresponding field for PPU structure with programmable address. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub const fn subregion_disable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub fn set_subregion_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub const fn addr24(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub fn set_addr24(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for PpuPrAddr1 {
    #[inline(always)]
    fn default() -> PpuPrAddr1 {
        PpuPrAddr1(0)
    }
}
#[doc = "PPU region attributes 0 (slave structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrAtt0(pub u32);
impl PpuPrAtt0 {
    #[doc = "User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub const fn ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub const fn uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    pub const fn ux(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    pub fn set_ux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pw(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    pub const fn px(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    pub fn set_px(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This field specifies protection context identifier based access control for protection context '0'."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies protection context identifier based access control for protection context '0'."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x7fff;
        val as u16
    }
    #[doc = "This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 9usize)) | (((val as u32) & 0x7fff) << 9usize);
    }
    #[doc = "This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub const fn pc_match(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub fn set_pc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PpuPrAtt0 {
    #[inline(always)]
    fn default() -> PpuPrAtt0 {
        PpuPrAtt0(0)
    }
}
#[doc = "PPU region attributes 1 (master structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrAtt1(pub u32);
impl PpuPrAtt1 {
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub const fn ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn set_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
    #[inline(always)]
    pub const fn ux(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn set_ux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pw(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
    #[inline(always)]
    pub const fn px(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address. Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn set_px(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x7fff;
        val as u16
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 9usize)) | (((val as u32) & 0x7fff) << 9usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address. '7': 256 B region"]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "See corresponding field for PPU structure with programmable address. '7': 256 B region"]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn pc_match(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_pc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PpuPrAtt1 {
    #[inline(always)]
    fn default() -> PpuPrAtt1 {
        PpuPrAtt1(0)
    }
}
#[doc = "Slave control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlCtl(pub u32);
impl SlCtl {
    #[doc = "Peripheral group, slave 0 enable. This field is for the peripheral group internal MMIO register slave (PPU structures) and is a constant '1'. This slave can NOT be disabled."]
    #[inline(always)]
    pub const fn enabled_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral group, slave 0 enable. This field is for the peripheral group internal MMIO register slave (PPU structures) and is a constant '1'. This slave can NOT be disabled."]
    #[inline(always)]
    pub fn set_enabled_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub const fn enabled_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn set_enabled_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Peripheral group, slave 2 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub const fn enabled_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral group, slave 2 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn set_enabled_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_enabled_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for SlCtl {
    #[inline(always)]
    fn default() -> SlCtl {
        SlCtl(0)
    }
}
#[doc = "Timeout control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimeoutCtl(pub u32);
impl TimeoutCtl {
    #[doc = "This field specifies a number of peripheral group (clk_group) clock cycles. If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of peripheral group clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
    #[inline(always)]
    pub const fn timeout(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field specifies a number of peripheral group (clk_group) clock cycles. If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of peripheral group clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
    #[inline(always)]
    pub fn set_timeout(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for TimeoutCtl {
    #[inline(always)]
    fn default() -> TimeoutCtl {
        TimeoutCtl(0)
    }
}
#[doc = "Trigger command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCmd(pub u32);
impl TrCmd {
    #[doc = "Specifies the activated trigger when ACTIVATE is '1'. OUT_SEL specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (ACTIVATE is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub const fn tr_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the activated trigger when ACTIVATE is '1'. OUT_SEL specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (ACTIVATE is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn set_tr_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Specifies the trigger group."]
    #[inline(always)]
    pub const fn group_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the trigger group."]
    #[inline(always)]
    pub fn set_group_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Amount of 'clk_peri' cycles a specific trigger is activated. During activation (ACTIVATE is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of ACTIVATE when ACTIVATE is '1' the trigger is activated and when ACTIVATE is '0' the trigger is deactivated."]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Amount of 'clk_peri' cycles a specific trigger is activated. During activation (ACTIVATE is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of ACTIVATE when ACTIVATE is '1' the trigger is activated and when ACTIVATE is '0' the trigger is deactivated."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
    #[inline(always)]
    pub const fn out_sel(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
    #[inline(always)]
    pub fn set_out_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and OUT_SEL for COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a COUNT value of 255 is a special case and trigger activation is under direct control of the ACTIVATE field (the counter is not decremented)."]
    #[inline(always)]
    pub const fn activate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and OUT_SEL for COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a COUNT value of 255 is a special case and trigger activation is under direct control of the ACTIVATE field (the counter is not decremented)."]
    #[inline(always)]
    pub fn set_activate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TrCmd {
    #[inline(always)]
    fn default() -> TrCmd {
        TrCmd(0)
    }
}
#[doc = "Trigger control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrOutCtl(pub u32);
impl TrOutCtl {
    #[doc = "Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    pub const fn tr_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    pub fn set_tr_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Specifies if the output trigger is inverted."]
    #[inline(always)]
    pub const fn tr_inv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the output trigger is inverted."]
    #[inline(always)]
    pub fn set_tr_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    pub const fn tr_edge(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    pub fn set_tr_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for TrOutCtl {
    #[inline(always)]
    fn default() -> TrOutCtl {
        TrOutCtl(0)
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
