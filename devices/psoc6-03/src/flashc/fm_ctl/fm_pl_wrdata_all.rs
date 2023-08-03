#[doc = "Reader of register FM_PL_WRDATA_ALL"]
pub type R = crate::R<u32, super::FM_PL_WRDATA_ALL>;
#[doc = "Writer for register FM_PL_WRDATA_ALL"]
pub type W = crate::W<u32, super::FM_PL_WRDATA_ALL>;
#[doc = "Register FM_PL_WRDATA_ALL `reset()`'s with value 0"]
impl crate::ResetValue for super::FM_PL_WRDATA_ALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA32`"]
pub type DATA32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA32`"]
pub struct DATA32_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
    #[inline(always)]
    pub fn data32(&mut self) -> DATA32_W {
        DATA32_W { w: self }
    }
}
