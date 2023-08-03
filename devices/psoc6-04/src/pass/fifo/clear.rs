#[doc = "Reader of register CLEAR"]
pub type R = crate::R<u32, super::CLEAR>;
#[doc = "Writer for register CLEAR"]
pub type W = crate::W<u32, super::CLEAR>;
#[doc = "Register CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLEAR`"]
pub type CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLEAR`"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
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
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger and clearing of the FIFO status registers (excluding interrupts), hardware clears this bit."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger and clearing of the FIFO status registers (excluding interrupts), hardware clears this bit."]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
}
