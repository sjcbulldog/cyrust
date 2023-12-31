#[doc = "Reader of register TDCR"]
pub type R = crate::R<u32, super::TDCR>;
#[doc = "Writer for register TDCR"]
pub type W = crate::W<u32, super::TDCR>;
#[doc = "Register TDCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDCF`"]
pub type TDCF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDCF`"]
pub struct TDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `TDCO`"]
pub type TDCO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDCO`"]
pub struct TDCO_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Window Length 0x00-0x7F Defines the minimum value for the SSP position, dominant edges on m_ttcan_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq"]
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset 0x00-0x7F Offset value defining the distance between the measured delay from m_ttcan_tx to m_ttcan_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Window Length 0x00-0x7F Defines the minimum value for the SSP position, dominant edges on m_ttcan_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq"]
    #[inline(always)]
    pub fn tdcf(&mut self) -> TDCF_W {
        TDCF_W { w: self }
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset 0x00-0x7F Offset value defining the distance between the measured delay from m_ttcan_tx to m_ttcan_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn tdco(&mut self) -> TDCO_W {
        TDCO_W { w: self }
    }
}
