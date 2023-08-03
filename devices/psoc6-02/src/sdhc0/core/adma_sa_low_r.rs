#[doc = "Reader of register ADMA_SA_LOW_R"]
pub type R = crate::R<u32, super::ADMA_SA_LOW_R>;
#[doc = "Writer for register ADMA_SA_LOW_R"]
pub type W = crate::W<u32, super::ADMA_SA_LOW_R>;
#[doc = "Register ADMA_SA_LOW_R `reset()`'s with value 0"]
impl crate::ResetValue for super::ADMA_SA_LOW_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `ADMA_SA_LOW`"]
pub type ADMA_SA_LOW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADMA_SA_LOW`"]
pub struct ADMA_SA_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMA_SA_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. - SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location - ADMA2: This register stores the byte address of the executing command of the descriptor table - ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
    #[inline(always)]
    pub fn adma_sa_low(&self) -> ADMA_SA_LOW_R {
        ADMA_SA_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. - SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location - ADMA2: This register stores the byte address of the executing command of the descriptor table - ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
    #[inline(always)]
    pub fn adma_sa_low(&mut self) -> ADMA_SA_LOW_W {
        ADMA_SA_LOW_W { w: self }
    }
}
