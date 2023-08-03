#[doc = "Reader of register INTR_MASK"]
pub type R = crate::R<u32, super::INTR_MASK>;
#[doc = "Writer for register INTR_MASK"]
pub type W = crate::W<u32, super::INTR_MASK>;
#[doc = "Register INTR_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_MASK {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `FAULT`"]
pub type FAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT`"]
pub struct FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT_W<'a> {
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
    #[doc = "Bit 0 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn fault(&mut self) -> FAULT_W {
        FAULT_W { w: self }
    }
}
