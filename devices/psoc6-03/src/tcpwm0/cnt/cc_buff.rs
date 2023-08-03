#[doc = "Reader of register CC_BUFF"]
pub type R = crate::R<u32, super::CC_BUFF>;
#[doc = "Writer for register CC_BUFF"]
pub type W = crate::W<u32, super::CC_BUFF>;
#[doc = "Register CC_BUFF `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CC_BUFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `CC`"]
pub type CC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CC`"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Additional buffer for counter CC register."]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional buffer for counter CC register."]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
}
