#[doc = "Reader of register ADFT"]
pub type R = crate::R<u32, super::ADFT>;
#[doc = "Writer for register ADFT"]
pub type W = crate::W<u32, super::ADFT>;
#[doc = "Register ADFT `reset()`'s with value 0"]
impl crate::ResetValue for super::ADFT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADFT_SEL`"]
pub type ADFT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADFT_SEL`"]
pub struct ADFT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADFT selection for LPOSC. 0: DFT disabled 1: Measure Vdo; expect ~0.8V 2: Measure Ibias_ptat; expect ~250nA 3: Measure Ibias_ctat; expect ~550nA"]
    #[inline(always)]
    pub fn adft_sel(&self) -> ADFT_SEL_R {
        ADFT_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADFT selection for LPOSC. 0: DFT disabled 1: Measure Vdo; expect ~0.8V 2: Measure Ibias_ptat; expect ~250nA 3: Measure Ibias_ctat; expect ~550nA"]
    #[inline(always)]
    pub fn adft_sel(&mut self) -> ADFT_SEL_W {
        ADFT_SEL_W { w: self }
    }
}
