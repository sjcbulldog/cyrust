#[doc = "Reader of register ALT_JTAG_EN"]
pub type R = crate::R<u32, super::ALT_JTAG_EN>;
#[doc = "Writer for register ALT_JTAG_EN"]
pub type W = crate::W<u32, super::ALT_JTAG_EN>;
#[doc = "Register ALT_JTAG_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::ALT_JTAG_EN {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bit 31 - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
