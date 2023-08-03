#[doc = "Reader of register CRC_DATA_CTL"]
pub type R = crate::R<u32, super::CRC_DATA_CTL>;
#[doc = "Writer for register CRC_DATA_CTL"]
pub type W = crate::W<u32, super::CRC_DATA_CTL>;
#[doc = "Register CRC_DATA_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_DATA_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `DATA_XOR`"]
pub type DATA_XOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_XOR`"]
pub struct DATA_XOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_XOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
    #[inline(always)]
    pub fn data_xor(&self) -> DATA_XOR_R {
        DATA_XOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
    #[inline(always)]
    pub fn data_xor(&mut self) -> DATA_XOR_W {
        DATA_XOR_W { w: self }
    }
}
