#[doc = "Reader of register PERIOD"]
pub type R = crate::R<u32, super::PERIOD>;
#[doc = "Writer for register PERIOD"]
pub type W = crate::W<u32, super::PERIOD>;
#[doc = "Register PERIOD `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PER_VAL`"]
pub type PER_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PER_VAL`"]
pub struct PER_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Actual timer period is PER_VAL+1 (1 to 65536)"]
    #[inline(always)]
    pub fn per_val(&self) -> PER_VAL_R {
        PER_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Actual timer period is PER_VAL+1 (1 to 65536)"]
    #[inline(always)]
    pub fn per_val(&mut self) -> PER_VAL_W {
        PER_VAL_W { w: self }
    }
}
