#[doc = "Reader of register ARB_RW4_RA"]
pub type R = crate::R<u32, super::ARB_RW4_RA>;
#[doc = "Writer for register ARB_RW4_RA"]
pub type W = crate::W<u32, super::ARB_RW4_RA>;
#[doc = "Register ARB_RW4_RA `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_RW4_RA {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `RA`"]
pub type RA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RA`"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Read Address for EP"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Address for EP"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
}
