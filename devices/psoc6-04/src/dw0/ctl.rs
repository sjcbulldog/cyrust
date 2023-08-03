#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&mut self) -> ECC_EN_W {
        ECC_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W {
        ECC_INJ_EN_W { w: self }
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
