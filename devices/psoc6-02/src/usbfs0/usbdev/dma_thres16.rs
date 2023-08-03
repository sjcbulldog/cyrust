#[doc = "Reader of register DMA_THRES16"]
pub type R = crate::R<u32, super::DMA_THRES16>;
#[doc = "Writer for register DMA_THRES16"]
pub type W = crate::W<u32, super::DMA_THRES16>;
#[doc = "Register DMA_THRES16 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_THRES16 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `DMA_THS16`"]
pub type DMA_THS16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMA_THS16`"]
pub struct DMA_THS16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_THS16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths16(&self) -> DMA_THS16_R {
        DMA_THS16_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths16(&mut self) -> DMA_THS16_W {
        DMA_THS16_W { w: self }
    }
}
