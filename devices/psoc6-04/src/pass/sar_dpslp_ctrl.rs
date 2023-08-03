#[doc = "Reader of register SAR_DPSLP_CTRL[%s]"]
pub type R = crate::R<u32, super::SAR_DPSLP_CTRL>;
#[doc = "Writer for register SAR_DPSLP_CTRL[%s]"]
pub type W = crate::W<u32, super::SAR_DPSLP_CTRL>;
#[doc = "Register SAR_DPSLP_CTRL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_DPSLP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - enable for SAR deepsleep operation. SAR_CLOCK_SEL.CLOCK_SEL must be set to 1 for this field to affect SAR operation. - 0: SAR deeepsleep operation disabled - 1: SAR deepsleep operation enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - enable for SAR deepsleep operation. SAR_CLOCK_SEL.CLOCK_SEL must be set to 1 for this field to affect SAR operation. - 0: SAR deeepsleep operation disabled - 1: SAR deepsleep operation enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
