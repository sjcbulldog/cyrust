#[doc = "Reader of register ECC_CTL"]
pub type R = crate::R<u32, super::ECC_CTL>;
#[doc = "Writer for register ECC_CTL"]
pub type W = crate::W<u32, super::ECC_CTL>;
#[doc = "Register ECC_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ECC_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `WORD_ADDR`"]
pub type WORD_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WORD_ADDR`"]
pub struct WORD_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `PARITY`"]
pub type PARITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PARITY`"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub fn word_addr(&self) -> WORD_ADDR_R {
        WORD_ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 25:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub fn word_addr(&mut self) -> WORD_ADDR_W {
        WORD_ADDR_W { w: self }
    }
    #[doc = "Bits 25:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
}
