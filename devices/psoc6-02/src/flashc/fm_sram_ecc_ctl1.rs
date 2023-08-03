#[doc = "Reader of register FM_SRAM_ECC_CTL1"]
pub type R = crate::R<u32, super::FM_SRAM_ECC_CTL1>;
#[doc = "Writer for register FM_SRAM_ECC_CTL1"]
pub type W = crate::W<u32, super::FM_SRAM_ECC_CTL1>;
#[doc = "Register FM_SRAM_ECC_CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FM_SRAM_ECC_CTL1 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `ECC_INJ_PARITY`"]
pub type ECC_INJ_PARITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECC_INJ_PARITY`"]
pub struct ECC_INJ_PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_INJ_PARITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn ecc_inj_parity(&self) -> ECC_INJ_PARITY_R {
        ECC_INJ_PARITY_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn ecc_inj_parity(&mut self) -> ECC_INJ_PARITY_W {
        ECC_INJ_PARITY_W { w: self }
    }
}
