#[doc = "Reader of register ADMA_ID_LOW_R"]
pub type R = crate::R<u32, super::ADMA_ID_LOW_R>;
#[doc = "Writer for register ADMA_ID_LOW_R"]
pub type W = crate::W<u32, super::ADMA_ID_LOW_R>;
#[doc = "Register ADMA_ID_LOW_R `reset()`'s with value 0"]
impl crate::ResetValue for super::ADMA_ID_LOW_R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADMA_ID_LOW`"]
pub type ADMA_ID_LOW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADMA_ID_LOW`"]
pub struct ADMA_ID_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMA_ID_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
    #[inline(always)]
    pub fn adma_id_low(&self) -> ADMA_ID_LOW_R {
        ADMA_ID_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
    #[inline(always)]
    pub fn adma_id_low(&mut self) -> ADMA_ID_LOW_W {
        ADMA_ID_LOW_W { w: self }
    }
}
