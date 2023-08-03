#[doc = "Reader of register SAR_SIMULT_FW_START_CTRL"]
pub type R = crate::R<u32, super::SAR_SIMULT_FW_START_CTRL>;
#[doc = "Writer for register SAR_SIMULT_FW_START_CTRL"]
pub type W = crate::W<u32, super::SAR_SIMULT_FW_START_CTRL>;
#[doc = "Register SAR_SIMULT_FW_START_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_SIMULT_FW_START_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FW_TRIGGER`"]
pub type FW_TRIGGER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FW_TRIGGER`"]
pub struct FW_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_TRIGGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CONTINUOUS`"]
pub type CONTINUOUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONTINUOUS`"]
pub struct CONTINUOUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTINUOUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - This field is used to simultaneously FW trigger two or more SARs. <0>: Firmware trigger for SAR0 <1>: Firmware trigger for SAR1 <2>: Firmware trigger for SAR2 <3>: Firmware trigger for SAR3 If less than two bits are set, this field has no effect. When firmware writes to this field it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled. This field cannot be set when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn fw_trigger(&self) -> FW_TRIGGER_R {
        FW_TRIGGER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - This field is used to configure two or more SARs for continuous operation. -0: Continuous mode disabled -1: Continuously scan enabled channels, ignore triggers. <0>: Continuous Mode for SAR0 <1>: Continuous Mode for SAR1 <2>: Continuous Mode for SAR2 <3>: Continuous Mode for SAR3 If less than two bits are set, this field has no effect. This field cannot be set when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn continuous(&self) -> CONTINUOUS_R {
        CONTINUOUS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This field is used to simultaneously FW trigger two or more SARs. <0>: Firmware trigger for SAR0 <1>: Firmware trigger for SAR1 <2>: Firmware trigger for SAR2 <3>: Firmware trigger for SAR3 If less than two bits are set, this field has no effect. When firmware writes to this field it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled. This field cannot be set when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn fw_trigger(&mut self) -> FW_TRIGGER_W {
        FW_TRIGGER_W { w: self }
    }
    #[doc = "Bits 16:19 - This field is used to configure two or more SARs for continuous operation. -0: Continuous mode disabled -1: Continuously scan enabled channels, ignore triggers. <0>: Continuous Mode for SAR0 <1>: Continuous Mode for SAR1 <2>: Continuous Mode for SAR2 <3>: Continuous Mode for SAR3 If less than two bits are set, this field has no effect. This field cannot be set when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn continuous(&mut self) -> CONTINUOUS_W {
        CONTINUOUS_W { w: self }
    }
}
