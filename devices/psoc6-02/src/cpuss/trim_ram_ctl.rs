#[doc = "Reader of register TRIM_RAM_CTL"]
pub type R = crate::R<u32, super::TRIM_RAM_CTL>;
#[doc = "Writer for register TRIM_RAM_CTL"]
pub type W = crate::W<u32, super::TRIM_RAM_CTL>;
#[doc = "Register TRIM_RAM_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIM_RAM_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `TRIM`"]
pub type TRIM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TRIM`"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
}
