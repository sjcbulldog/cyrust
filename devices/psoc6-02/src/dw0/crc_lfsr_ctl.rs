#[doc = "Reader of register CRC_LFSR_CTL"]
pub type R = crate::R<u32, super::CRC_LFSR_CTL>;
#[doc = "Writer for register CRC_LFSR_CTL"]
pub type W = crate::W<u32, super::CRC_LFSR_CTL>;
#[doc = "Register CRC_LFSR_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_LFSR_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `LFSR32`"]
pub type LFSR32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LFSR32`"]
pub struct LFSR32_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSR32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
    #[inline(always)]
    pub fn lfsr32(&self) -> LFSR32_R {
        LFSR32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
    #[inline(always)]
    pub fn lfsr32(&mut self) -> LFSR32_W {
        LFSR32_W { w: self }
    }
}
