#[doc = "Reader of register TXESC"]
pub type R = crate::R<u32, super::TXESC>;
#[doc = "Writer for register TXESC"]
pub type W = crate::W<u32, super::TXESC>;
#[doc = "Register TXESC `reset()`'s with value 0"]
impl crate::ResetValue for super::TXESC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBDS`"]
pub type TBDS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBDS`"]
pub struct TBDS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub fn tbds(&mut self) -> TBDS_W {
        TBDS_W { w: self }
    }
}
