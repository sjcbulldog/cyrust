#[doc = "Reader of register INTR_SET"]
pub type R = crate::R<u32, super::INTR_SET>;
#[doc = "Writer for register INTR_SET"]
pub type W = crate::W<u32, super::INTR_SET>;
#[doc = "Register INTR_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER_EXPIRED`"]
pub type TIMER_EXPIRED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_EXPIRED`"]
pub struct TIMER_EXPIRED_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_EXPIRED_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn timer_expired(&self) -> TIMER_EXPIRED_R {
        TIMER_EXPIRED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn timer_expired(&mut self) -> TIMER_EXPIRED_W {
        TIMER_EXPIRED_W { w: self }
    }
}
