#[doc = "Reader of register FM_SRAM_ECC_CTL3"]
pub type R = crate::R<u32, super::FM_SRAM_ECC_CTL3>;
#[doc = "Writer for register FM_SRAM_ECC_CTL3"]
pub type W = crate::W<u32, super::FM_SRAM_ECC_CTL3>;
#[doc = "Register FM_SRAM_ECC_CTL3 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FM_SRAM_ECC_CTL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `ECC_ENABLE`"]
pub type ECC_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECC_ENABLE`"]
pub struct ECC_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ECC_TEST_FAIL`"]
pub type ECC_TEST_FAIL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ECC generation/check enable for eCT Flash SRAM memory."]
    #[inline(always)]
    pub fn ecc_enable(&self) -> ECC_ENABLE_R {
        ECC_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - eCT Flash SRAM ECC error injection test enable. Follow the steps below for ECC logic test: 1. Write corrupted or uncorrupted 39-bit data to FM_SRAM_ECC_CTL0/1 registers. 2. Set the ECC_INJ_EN bit to '1'. 3. Confirm that the bit ECC_TEST_FAIL is '0'. If this is not the case, start over at item 1 because the eCT Flash was not idle. 4. Check the corrected data in FM_SRAM_ECC_CTL2. 5. Confirm that fault was reported to fault structure, and check syndrome (only applicable if corrupted data was written in step 1). 6. If not finished, start over at 1 with different data."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of ECC test. 1 : ECC test failed because eCT Flash macro is busy and using the SRAM. 0: ECC was performed."]
    #[inline(always)]
    pub fn ecc_test_fail(&self) -> ECC_TEST_FAIL_R {
        ECC_TEST_FAIL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC generation/check enable for eCT Flash SRAM memory."]
    #[inline(always)]
    pub fn ecc_enable(&mut self) -> ECC_ENABLE_W {
        ECC_ENABLE_W { w: self }
    }
    #[doc = "Bit 4 - eCT Flash SRAM ECC error injection test enable. Follow the steps below for ECC logic test: 1. Write corrupted or uncorrupted 39-bit data to FM_SRAM_ECC_CTL0/1 registers. 2. Set the ECC_INJ_EN bit to '1'. 3. Confirm that the bit ECC_TEST_FAIL is '0'. If this is not the case, start over at item 1 because the eCT Flash was not idle. 4. Check the corrected data in FM_SRAM_ECC_CTL2. 5. Confirm that fault was reported to fault structure, and check syndrome (only applicable if corrupted data was written in step 1). 6. If not finished, start over at 1 with different data."]
    #[inline(always)]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W {
        ECC_INJ_EN_W { w: self }
    }
}
