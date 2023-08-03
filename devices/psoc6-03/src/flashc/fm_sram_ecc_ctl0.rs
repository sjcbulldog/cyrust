#[doc = "Reader of register FM_SRAM_ECC_CTL0"]
pub type R = crate::R<u32, super::FM_SRAM_ECC_CTL0>;
#[doc = "Writer for register FM_SRAM_ECC_CTL0"]
pub type W = crate::W<u32, super::FM_SRAM_ECC_CTL0>;
#[doc = "Register FM_SRAM_ECC_CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FM_SRAM_ECC_CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ECC_INJ_DATA`"]
pub type ECC_INJ_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ECC_INJ_DATA`"]
pub struct ECC_INJ_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_INJ_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn ecc_inj_data(&self) -> ECC_INJ_DATA_R {
        ECC_INJ_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn ecc_inj_data(&mut self) -> ECC_INJ_DATA_W {
        ECC_INJ_DATA_W { w: self }
    }
}
