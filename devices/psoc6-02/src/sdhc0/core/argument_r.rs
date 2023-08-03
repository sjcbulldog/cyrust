#[doc = "Reader of register ARGUMENT_R"]
pub type R = crate::R<u32, super::ARGUMENT_R>;
#[doc = "Writer for register ARGUMENT_R"]
pub type W = crate::W<u32, super::ARGUMENT_R>;
#[doc = "Register ARGUMENT_R `reset()`'s with value 0"]
impl crate::ResetValue for super::ARGUMENT_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `ARGUMENT`"]
pub type ARGUMENT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ARGUMENT`"]
pub struct ARGUMENT_W<'a> {
    w: &'a mut W,
}
impl<'a> ARGUMENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    pub fn argument(&self) -> ARGUMENT_R {
        ARGUMENT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    pub fn argument(&mut self) -> ARGUMENT_W {
        ARGUMENT_W { w: self }
    }
}
