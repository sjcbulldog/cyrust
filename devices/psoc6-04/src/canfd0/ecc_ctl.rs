#[doc = "Reader of register ECC_CTL"]
pub type R = crate::R<u32, super::ECC_CTL>;
#[doc = "Writer for register ECC_CTL"]
pub type W = crate::W<u32, super::ECC_CTL>;
#[doc = "Register ECC_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ECC_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
impl R {
    #[doc = "Bit 16 - Enable ECC for CANFD SRAM When disabled also all error injection functionality is disabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enable ECC for CANFD SRAM When disabled also all error injection functionality is disabled."]
    #[inline(always)]
    pub fn ecc_en(&mut self) -> ECC_EN_W {
        ECC_EN_W { w: self }
    }
}
