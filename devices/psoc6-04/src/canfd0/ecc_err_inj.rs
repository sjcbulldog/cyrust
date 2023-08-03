#[doc = "Reader of register ECC_ERR_INJ"]
pub type R = crate::R<u32, super::ECC_ERR_INJ>;
#[doc = "Writer for register ECC_ERR_INJ"]
pub type W = crate::W<u32, super::ECC_ERR_INJ>;
#[doc = "Register ECC_ERR_INJ `reset()`'s with value 0xfffc"]
impl crate::ResetValue for super::ECC_ERR_INJ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfffc
    }
}
#[doc = "Reader of field `ERR_ADDR`"]
pub type ERR_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ERR_ADDR`"]
pub struct ERR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `ERR_EN`"]
pub type ERR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR_EN`"]
pub struct ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `ERR_PAR`"]
pub type ERR_PAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERR_PAR`"]
pub struct ERR_PAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_PAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Specifies the address of the word where an error will be injected on write or an non-correctable error will be suppressed. When the ERR_EN bit is set an error parity (ERR_PAR) is injected when any write, from bus or a CAN channel, is done to this address. When the ERR_EN bit is set and the access address matches ERR_ADDR then a non-correctable ECC error or an Address error will NOT result in a bus error or CAN channel shutdown. Note that error reporting to the fault structure cannot be suppressed."]
    #[inline(always)]
    pub fn err_addr(&self) -> ERR_ADDR_R {
        ERR_ADDR_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 20 - Enable error injection (ECC_EN must be 1). When this bit is set the error parity (ERR_PAR) will be used when an AHB write is done to the ERR_ADDR address. When the error word is read a single or double error will be reported to the fault structure just like for a real ECC error (even if this bit is no longer set). When this bit is set (and ECC_EN=1) a non-correctable error (ECC or address error) for the ERR_ADDR will not be reported back to the CAN channel or AHB bus."]
    #[inline(always)]
    pub fn err_en(&self) -> ERR_EN_R {
        ERR_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - ECC Parity bits to use for ECC error injection at address ERR_ADDR."]
    #[inline(always)]
    pub fn err_par(&self) -> ERR_PAR_R {
        ERR_PAR_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Specifies the address of the word where an error will be injected on write or an non-correctable error will be suppressed. When the ERR_EN bit is set an error parity (ERR_PAR) is injected when any write, from bus or a CAN channel, is done to this address. When the ERR_EN bit is set and the access address matches ERR_ADDR then a non-correctable ECC error or an Address error will NOT result in a bus error or CAN channel shutdown. Note that error reporting to the fault structure cannot be suppressed."]
    #[inline(always)]
    pub fn err_addr(&mut self) -> ERR_ADDR_W {
        ERR_ADDR_W { w: self }
    }
    #[doc = "Bit 20 - Enable error injection (ECC_EN must be 1). When this bit is set the error parity (ERR_PAR) will be used when an AHB write is done to the ERR_ADDR address. When the error word is read a single or double error will be reported to the fault structure just like for a real ECC error (even if this bit is no longer set). When this bit is set (and ECC_EN=1) a non-correctable error (ECC or address error) for the ERR_ADDR will not be reported back to the CAN channel or AHB bus."]
    #[inline(always)]
    pub fn err_en(&mut self) -> ERR_EN_W {
        ERR_EN_W { w: self }
    }
    #[doc = "Bits 24:30 - ECC Parity bits to use for ECC error injection at address ERR_ADDR."]
    #[inline(always)]
    pub fn err_par(&mut self) -> ERR_PAR_W {
        ERR_PAR_W { w: self }
    }
}
