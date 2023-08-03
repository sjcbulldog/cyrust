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
#[doc = "Reader of field `WORD_ADDR`"]
pub type WORD_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WORD_ADDR`"]
pub struct WORD_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
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
    #[doc = "Bits 0:23 - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
    #[inline(always)]
    pub fn word_addr(&self) -> WORD_ADDR_R {
        WORD_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
    #[inline(always)]
    pub fn word_addr(&mut self) -> WORD_ADDR_W {
        WORD_ADDR_W { w: self }
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
}
