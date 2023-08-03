#[doc = "Reader of register CM4_CA_CTL0"]
pub type R = crate::R<u32, super::CM4_CA_CTL0>;
#[doc = "Writer for register CM4_CA_CTL0"]
pub type W = crate::W<u32, super::CM4_CA_CTL0>;
#[doc = "Register CM4_CA_CTL0 `reset()`'s with value 0xc000_0001"]
impl crate::ResetValue for super::CM4_CA_CTL0 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0001
    }
}
#[doc = "Reader of field `RAM_ECC_EN`"]
pub type RAM_ECC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_ECC_EN`"]
pub struct RAM_ECC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_ECC_EN_W<'a> {
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
#[doc = "Reader of field `RAM_ECC_INJ_EN`"]
pub type RAM_ECC_INJ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_ECC_INJ_EN`"]
pub struct RAM_ECC_INJ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_ECC_INJ_EN_W<'a> {
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
#[doc = "Reader of field `WAY`"]
pub type WAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAY`"]
pub struct WAY_W<'a> {
    w: &'a mut W,
}
impl<'a> WAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SET_ADDR`"]
pub type SET_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SET_ADDR`"]
pub struct SET_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `PREF_EN`"]
pub type PREF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREF_EN`"]
pub struct PREF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CA_EN`"]
pub type CA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CA_EN`"]
pub struct CA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CA_EN_W<'a> {
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
    #[doc = "Bit 0 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn ram_ecc_en(&self) -> RAM_ECC_EN_R {
        RAM_ECC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn ram_ecc_inj_en(&self) -> RAM_ECC_INJ_EN_R {
        RAM_ECC_INJ_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn way(&self) -> WAY_R {
        WAY_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_addr(&self) -> SET_ADDR_R {
        SET_ADDR_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 30 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn ca_en(&self) -> CA_EN_R {
        CA_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn ram_ecc_en(&mut self) -> RAM_ECC_EN_W {
        RAM_ECC_EN_W { w: self }
    }
    #[doc = "Bit 1 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn ram_ecc_inj_en(&mut self) -> RAM_ECC_INJ_EN_W {
        RAM_ECC_INJ_EN_W { w: self }
    }
    #[doc = "Bits 16:17 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn way(&mut self) -> WAY_W {
        WAY_W { w: self }
    }
    #[doc = "Bits 24:26 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_addr(&mut self) -> SET_ADDR_W {
        SET_ADDR_W { w: self }
    }
    #[doc = "Bit 30 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W {
        PREF_EN_W { w: self }
    }
    #[doc = "Bit 31 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn ca_en(&mut self) -> CA_EN_W {
        CA_EN_W { w: self }
    }
}
