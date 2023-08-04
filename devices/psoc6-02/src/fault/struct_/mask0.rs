#[doc = "Reader of register MASK0"]
pub type R = crate::R<u32, super::MASK0>;
#[doc = "Writer for register MASK0"]
pub type W = crate::W<u32, super::MASK0>;
#[doc = "Register MASK0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK0 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SOURCE`"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 31 to 0."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 31 to 0."]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W {
        SOURCE_W { w: self }
    }
}