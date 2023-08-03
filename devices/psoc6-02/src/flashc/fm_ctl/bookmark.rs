#[doc = "Reader of register BOOKMARK"]
pub type R = crate::R<u32, super::BOOKMARK>;
#[doc = "Writer for register BOOKMARK"]
pub type W = crate::W<u32, super::BOOKMARK>;
#[doc = "Register BOOKMARK `reset()`'s with value 0"]
impl crate::ResetValue for super::BOOKMARK {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `BOOKMARK`"]
pub type BOOKMARK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BOOKMARK`"]
pub struct BOOKMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOKMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    pub fn bookmark(&self) -> BOOKMARK_R {
        BOOKMARK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    pub fn bookmark(&mut self) -> BOOKMARK_W {
        BOOKMARK_W { w: self }
    }
}
