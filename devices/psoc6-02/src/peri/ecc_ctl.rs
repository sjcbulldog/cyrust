#[doc = "Reader of register ECC_CTL"]
pub type R = crate::R<u32, super::ECC_CTL>;
#[doc = "Writer for register ECC_CTL"]
pub type W = crate::W<u32, super::ECC_CTL>;
#[doc = "Register ECC_CTL `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::ECC_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
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
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `ECC_EN`"]
pub type ECC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECC_EN`"]
pub struct ECC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ECC_INJ_EN`"]
pub type ECC_INJ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECC_INJ_EN`"]
pub struct ECC_INJ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_INJ_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
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
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub fn word_addr(&self) -> WORD_ADDR_R {
        WORD_ADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub fn word_addr(&mut self) -> WORD_ADDR_W {
        WORD_ADDR_W { w: self }
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&mut self) -> ECC_EN_W {
        ECC_EN_W { w: self }
    }
    #[doc = "Bit 18 - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W {
        ECC_INJ_EN_W { w: self }
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
}
