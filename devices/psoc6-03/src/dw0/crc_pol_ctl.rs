#[doc = "Reader of register CRC_POL_CTL"]
pub type R = crate::R<u32, super::CRC_POL_CTL>;
#[doc = "Writer for register CRC_POL_CTL"]
pub type W = crate::W<u32, super::CRC_POL_CTL>;
#[doc = "Register CRC_POL_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_POL_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POLYNOMIAL`"]
pub type POLYNOMIAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `POLYNOMIAL`"]
pub struct POLYNOMIAL_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYNOMIAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
    #[inline(always)]
    pub fn polynomial(&self) -> POLYNOMIAL_R {
        POLYNOMIAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
    #[inline(always)]
    pub fn polynomial(&mut self) -> POLYNOMIAL_W {
        POLYNOMIAL_W { w: self }
    }
}
