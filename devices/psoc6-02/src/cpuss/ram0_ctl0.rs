#[doc = "Reader of register RAM0_CTL0"]
pub type R = crate::R<u32, super::RAM0_CTL0>;
#[doc = "Writer for register RAM0_CTL0"]
pub type W = crate::W<u32, super::RAM0_CTL0>;
#[doc = "Register RAM0_CTL0 `reset()`'s with value 0x0003_0001"]
impl crate::ResetValue for super::RAM0_CTL0 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0001
    }
}
#[doc = "Reader of field `SLOW_WS`"]
pub type SLOW_WS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLOW_WS`"]
pub struct SLOW_WS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW_WS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FAST_WS`"]
pub type FAST_WS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FAST_WS`"]
pub struct FAST_WS_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_WS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
#[doc = "Reader of field `ECC_AUTO_CORRECT`"]
pub type ECC_AUTO_CORRECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECC_AUTO_CORRECT`"]
pub struct ECC_AUTO_CORRECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_AUTO_CORRECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
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
impl R {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SLOW_WS_R {
        SLOW_WS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FAST_WS_R {
        FAST_WS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
    #[inline(always)]
    pub fn ecc_auto_correct(&self) -> ECC_AUTO_CORRECT_R {
        ECC_AUTO_CORRECT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn slow_ws(&mut self) -> SLOW_WS_W {
        SLOW_WS_W { w: self }
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn fast_ws(&mut self) -> FAST_WS_W {
        FAST_WS_W { w: self }
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&mut self) -> ECC_EN_W {
        ECC_EN_W { w: self }
    }
    #[doc = "Bit 17 - HW ECC autocorrect functionality: '0': Disabled. '1': Enabled. HW automatically writes back SRAM with corrected data when a recoverable ECC error is detected."]
    #[inline(always)]
    pub fn ecc_auto_correct(&mut self) -> ECC_AUTO_CORRECT_W {
        ECC_AUTO_CORRECT_W { w: self }
    }
    #[doc = "Bit 18 - Enable error injection for system SRAM 0. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of system SRAM 0."]
    #[inline(always)]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W {
        ECC_INJ_EN_W { w: self }
    }
}
