#[doc = "Reader of register TR_CMD"]
pub type R = crate::R<u32, super::TR_CMD>;
#[doc = "Writer for register TR_CMD"]
pub type W = crate::W<u32, super::TR_CMD>;
#[doc = "Register TR_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::TR_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACTIVATE`"]
pub type ACTIVATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVATE`"]
pub struct ACTIVATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVATE_W<'a> {
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
    #[doc = "Bit 0 - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    pub fn activate(&self) -> ACTIVATE_R {
        ACTIVATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    pub fn activate(&mut self) -> ACTIVATE_W {
        ACTIVATE_W { w: self }
    }
}
