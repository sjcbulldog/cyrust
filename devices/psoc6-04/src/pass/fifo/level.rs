#[doc = "Reader of register LEVEL"]
pub type R = crate::R<u32, super::LEVEL>;
#[doc = "Writer for register LEVEL"]
pub type W = crate::W<u32, super::LEVEL>;
#[doc = "Register LEVEL `reset()`'s with value 0"]
impl crate::ResetValue for super::LEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEVEL`"]
pub type LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEVEL`"]
pub struct LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - FIFO level set. A trigger (and optional interrupt) event occurs when USED.USED = LEVEL+1. (Trigger generation is also affect by CONFIG.TR_CLR_RD_EN). If CHAIN_TO_NXT is disabled, the Max LEVEL is limited to 63. If CHAIN_TO_NXT is enabled, only FIFO\\[0\\].config.level should be configured and the Max LEVEL is set by the number of FIFOs in the chain."]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO level set. A trigger (and optional interrupt) event occurs when USED.USED = LEVEL+1. (Trigger generation is also affect by CONFIG.TR_CLR_RD_EN). If CHAIN_TO_NXT is disabled, the Max LEVEL is limited to 63. If CHAIN_TO_NXT is enabled, only FIFO\\[0\\].config.level should be configured and the Max LEVEL is set by the number of FIFOs in the chain."]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W { w: self }
    }
}
