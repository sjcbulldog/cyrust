#[doc = "Reader of register ARB_RW1_WA"]
pub type R = crate::R<u32, super::ARB_RW1_WA>;
#[doc = "Writer for register ARB_RW1_WA"]
pub type W = crate::W<u32, super::ARB_RW1_WA>;
#[doc = "Register ARB_RW1_WA `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_RW1_WA {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `WA`"]
pub type WA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WA`"]
pub struct WA_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Write Address for EP"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Address for EP"]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W {
        WA_W { w: self }
    }
}
