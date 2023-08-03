#[doc = "Reader of register SAR_TR_SCAN_CNT"]
pub type R = crate::R<u32, super::SAR_TR_SCAN_CNT>;
#[doc = "Writer for register SAR_TR_SCAN_CNT"]
pub type W = crate::W<u32, super::SAR_TR_SCAN_CNT>;
#[doc = "Register SAR_TR_SCAN_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TR_SCAN_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCAN_CNT`"]
pub type SCAN_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCAN_CNT`"]
pub struct SCAN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SAR trigger sample counter. This field determines the number of samples a SAR will take when triggered. The number of samples is SCAN_COUNT+1. This feature can be enabled for individual SARs by setting the appropriate bit of SAR_TR_CTRL.TR_SCAN_CNT_SEL. This feature can be enabled for simultaneously sampled SARs by setting SAR_SIMULT_TR_CTRL.SIMULT_TR_SCAN_CNT_SEL. If SAR.SAMPLE_CTRL.AVG_MODE is set to INTERLEAVED, the SCAN_CNT must be set an integer multiple of (1<<AVG_CNTR+1)."]
    #[inline(always)]
    pub fn scan_cnt(&self) -> SCAN_CNT_R {
        SCAN_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SAR trigger sample counter. This field determines the number of samples a SAR will take when triggered. The number of samples is SCAN_COUNT+1. This feature can be enabled for individual SARs by setting the appropriate bit of SAR_TR_CTRL.TR_SCAN_CNT_SEL. This feature can be enabled for simultaneously sampled SARs by setting SAR_SIMULT_TR_CTRL.SIMULT_TR_SCAN_CNT_SEL. If SAR.SAMPLE_CTRL.AVG_MODE is set to INTERLEAVED, the SCAN_CNT must be set an integer multiple of (1<<AVG_CNTR+1)."]
    #[inline(always)]
    pub fn scan_cnt(&mut self) -> SCAN_CNT_W {
        SCAN_CNT_W { w: self }
    }
}
