#[doc = "Reader of register CM0_PC_CTL"]
pub type R = crate::R<u32, super::CM0_PC_CTL>;
#[doc = "Writer for register CM0_PC_CTL"]
pub type W = crate::W<u32, super::CM0_PC_CTL>;
#[doc = "Register CM0_PC_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CM0_PC_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VALID`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
}
