#[doc = "Reader of register CTDAC_VAL_NXT"]
pub type R = crate::R<u32, super::CTDAC_VAL_NXT>;
#[doc = "Writer for register CTDAC_VAL_NXT"]
pub type W = crate::W<u32, super::CTDAC_VAL_NXT>;
#[doc = "Register CTDAC_VAL_NXT `reset()`'s with value 0"]
impl crate::ResetValue for super::CTDAC_VAL_NXT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Next value for CTDAC_VAL.VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Next value for CTDAC_VAL.VALUE"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
