#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Peripheral group structure"]
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
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
    #[doc = "Timeout control"]
    #[inline(always)]
    pub const fn timeout_ctl(self) -> crate::common::Reg<TimeoutCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Trigger command"]
    #[inline(always)]
    pub const fn tr_cmd(self) -> crate::common::Reg<TrCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize) as _) }
    }
    #[doc = "Divider command"]
    #[inline(always)]
    pub const fn div_cmd(self) -> crate::common::Reg<DivCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "Clock control"]
    #[inline(always)]
    pub const fn clock_ctl(self, n: usize) -> crate::common::Reg<PeriClockCtl, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3072usize + n * 4usize) as _) }
    }
    #[doc = "Divider control (for 8.0 divider)"]
    #[inline(always)]
    pub const fn div_8_ctl(self, n: usize) -> crate::common::Reg<Div8Ctl, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4096usize + n * 4usize) as _) }
    }
    #[doc = "Divider control (for 16.0 divider)"]
    #[inline(always)]
    pub const fn div_16_ctl(self, n: usize) -> crate::common::Reg<Div16Ctl, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(5120usize + n * 4usize) as _) }
    }
    #[doc = "Divider control (for 16.5 divider)"]
    #[inline(always)]
    pub const fn div_16_5_ctl(self, n: usize) -> crate::common::Reg<Div165Ctl, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(6144usize + n * 4usize) as _) }
    }
    #[doc = "Divider control (for 24.5 divider)"]
    #[inline(always)]
    pub const fn div_24_5_ctl(self, n: usize) -> crate::common::Reg<Div245Ctl, crate::common::RW> {
        assert!(n < 255usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7168usize + n * 4usize) as _) }
    }
    #[doc = "ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(self) -> crate::common::Reg<EccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8192usize) as _) }
    }
    #[doc = "Peripheral group structure"]
    #[inline(always)]
    pub const fn gr(self, n: usize) -> Gr {
        assert!(n < 11usize);
        unsafe { Gr::from_ptr(self.ptr.add(16384usize + n * 32usize) as _) }
    }
    #[doc = "Trigger group"]
    #[inline(always)]
    pub const fn tr_gr(self, n: usize) -> TrGr {
        assert!(n < 10usize);
        unsafe { TrGr::from_ptr(self.ptr.add(32768usize + n * 1024usize) as _) }
    }
    #[doc = "Trigger 1-to-1 group"]
    #[inline(always)]
    pub const fn tr_1to1_gr(self, n: usize) -> Tr1to1Gr {
        assert!(n < 7usize);
        unsafe { Tr1to1Gr::from_ptr(self.ptr.add(49152usize + n * 1024usize) as _) }
    }
}
#[doc = "Trigger 1-to-1 group"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr1to1Gr {
    ptr: *mut u8,
}
unsafe impl Send for Tr1to1Gr {}
unsafe impl Sync for Tr1to1Gr {}
impl Tr1to1Gr {
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
    pub const fn tr_ctl(self, n: usize) -> crate::common::Reg<Tr1to1GrTrCtl, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize + n * 4usize) as _) }
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
    pub const fn tr_ctl(self, n: usize) -> crate::common::Reg<TrGrTrCtl, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize + n * 4usize) as _) }
    }
}
#[doc = "Divider control (for 16.5 divider)"]
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
#[doc = "Divider control (for 16.0 divider)"]
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
#[doc = "Divider control (for 24.5 divider)"]
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
#[doc = "Divider control (for 8.0 divider)"]
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
    #[doc = "Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is restricted to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn int8_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is restricted to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
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
#[doc = "Divider command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DivCmd(pub u32);
impl DivCmd {
    #[doc = "(TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
    #[inline(always)]
    pub const fn div_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "(TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
    #[inline(always)]
    pub fn set_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub const fn type_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn set_type_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "(PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '255' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
    #[inline(always)]
    pub const fn pa_div_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "(PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '255' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
    #[inline(always)]
    pub fn set_pa_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub const fn pa_type_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn set_pa_type_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
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
#[doc = "ECC control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccCtl(pub u32);
impl EccCtl {
    #[doc = "Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub const fn word_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub fn set_word_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn ecc_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_ecc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub const fn ecc_inj_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn set_ecc_inj_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    pub const fn parity(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "ECC parity to use for ECC error injection at address WORD_ADDR."]
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
#[doc = "Clock control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PeriClockCtl(pub u32);
impl PeriClockCtl {
    #[doc = "Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '255' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    pub const fn div_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '255' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '255' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    pub fn set_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub const fn type_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn set_type_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for PeriClockCtl {
    #[inline(always)]
    fn default() -> PeriClockCtl {
        PeriClockCtl(0)
    }
}
#[doc = "Slave control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlCtl(pub u32);
impl SlCtl {
    #[doc = "Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub const fn enabled_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn set_enabled_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub const fn enabled_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn set_enabled_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn enabled_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
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
    #[doc = "Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
    #[inline(always)]
    pub const fn disabled_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
    #[inline(always)]
    pub fn set_disabled_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_14(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn disabled_15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_disabled_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
    #[doc = "This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
    #[inline(always)]
    pub const fn timeout(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
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
#[doc = "Trigger control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr1to1GrTrCtl(pub u32);
impl Tr1to1GrTrCtl {
    #[doc = "Specifies input trigger: '0'': constant signal level '0'. '1': input trigger."]
    #[inline(always)]
    pub const fn tr_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies input trigger: '0'': constant signal level '0'. '1': input trigger."]
    #[inline(always)]
    pub fn set_tr_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
    #[doc = "Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    pub const fn dbg_freeze_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    pub fn set_dbg_freeze_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Tr1to1GrTrCtl {
    #[inline(always)]
    fn default() -> Tr1to1GrTrCtl {
        Tr1to1GrTrCtl(0)
    }
}
#[doc = "Trigger command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCmd(pub u32);
impl TrCmd {
    #[doc = "Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub const fn tr_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn set_tr_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
    #[inline(always)]
    pub const fn group_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn set_group_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
    #[inline(always)]
    pub const fn tr_edge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
    #[inline(always)]
    pub fn set_tr_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
    #[inline(always)]
    pub const fn out_sel(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn set_out_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields."]
    #[inline(always)]
    pub const fn activate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields."]
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
pub struct TrGrTrCtl(pub u32);
impl TrGrTrCtl {
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
    #[doc = "Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    pub const fn dbg_freeze_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    pub fn set_dbg_freeze_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for TrGrTrCtl {
    #[inline(always)]
    fn default() -> TrGrTrCtl {
        TrGrTrCtl(0)
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
