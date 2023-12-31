#[doc = "Reader of register XIDAM"]
pub type R = crate::R<u32, super::XIDAM>;
#[doc = "Writer for register XIDAM"]
pub type W = crate::W<u32, super::XIDAM>;
#[doc = "Register XIDAM `reset()`'s with value 0"]
impl crate::ResetValue for super::XIDAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EIDM`"]
pub type EIDM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EIDM`"]
pub struct EIDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EIDM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - Extended ID Mask For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active."]
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID Mask For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active."]
    #[inline(always)]
    pub fn eidm(&mut self) -> EIDM_W {
        EIDM_W { w: self }
    }
}
