#[doc = "Reader of register TSCV"]
pub type R = crate::R<u32, super::TSCV>;
#[doc = "Writer for register TSCV"]
pub type W = crate::W<u32, super::TSCV>;
#[doc = "Register TSCV `reset()`'s with value 0"]
impl crate::ResetValue for super::TSCV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSC`"]
pub type TSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSC`"]
pub struct TSC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter, not used for M_TTCAN The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx). When TSCC.TSS = '01', the Timestamp Counter is incremented in multiples of CAN bit times \\[1...16\\]
depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = '10', TSC reflects the external Timestamp Counter value. A write access has no impact."]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Counter, not used for M_TTCAN The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx). When TSCC.TSS = '01', the Timestamp Counter is incremented in multiples of CAN bit times \\[1...16\\]
depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = '10', TSC reflects the external Timestamp Counter value. A write access has no impact."]
    #[inline(always)]
    pub fn tsc(&mut self) -> TSC_W {
        TSC_W { w: self }
    }
}
