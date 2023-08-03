#[doc = "Reader of register CRC_REM_CTL"]
pub type R = crate::R<u32, super::CRC_REM_CTL>;
#[doc = "Writer for register CRC_REM_CTL"]
pub type W = crate::W<u32, super::CRC_REM_CTL>;
#[doc = "Register CRC_REM_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_REM_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REM_XOR`"]
pub type REM_XOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REM_XOR`"]
pub struct REM_XOR_W<'a> {
    w: &'a mut W,
}
impl<'a> REM_XOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
    #[inline(always)]
    pub fn rem_xor(&self) -> REM_XOR_R {
        REM_XOR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
    #[inline(always)]
    pub fn rem_xor(&mut self) -> REM_XOR_W {
        REM_XOR_W { w: self }
    }
}
