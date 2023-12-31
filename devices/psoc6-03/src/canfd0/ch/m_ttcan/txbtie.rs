#[doc = "Reader of register TXBTIE"]
pub type R = crate::R<u32, super::TXBTIE>;
#[doc = "Writer for register TXBTIE"]
pub type W = crate::W<u32, super::TXBTIE>;
#[doc = "Register TXBTIE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXBTIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
}
