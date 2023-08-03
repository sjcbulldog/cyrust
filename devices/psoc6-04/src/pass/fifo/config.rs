#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHAN_ID_EN`"]
pub type CHAN_ID_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHAN_ID_EN`"]
pub struct CHAN_ID_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAN_ID_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CHAIN_TO_NXT`"]
pub type CHAIN_TO_NXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHAIN_TO_NXT`"]
pub struct CHAIN_TO_NXT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAIN_TO_NXT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TR_INTR_CLR_RD_EN`"]
pub type TR_INTR_CLR_RD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR_INTR_CLR_RD_EN`"]
pub struct TR_INTR_CLR_RD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_INTR_CLR_RD_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - channel number (ID) enable bit -0: CHAN_ID field in RD_DATA is disabled. A read from RD_DATA will result in (4'b0,16'b RESULT) -1: CHAN_ID field in RD_DATA is enabled. A read from RD_DATA will result in (4'b CHAN_ID, 16'b RESULT) If CHAIN_EN is enabled, only FIFO\\[0\\].config.chan_id_en should be configured and the other FIFOs in the chain will inherit the FIFO\\[0\\]
config."]
    #[inline(always)]
    pub fn chan_id_en(&self) -> CHAN_ID_EN_R {
        CHAN_ID_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Chain FIFO to next FIFO (i.e. chain FIFO0 and FIFO1). - 0: FIFO not chained . FIFO operates independently (FIFO depth of 64) and only operates on result data from its associated SAR. - 1: FIFO chained to next FIFO. FIFO is part of a chain of FIFOs (effectively extending the FIFO depth beyond 64) and only operates on result data from SAR0."]
    #[inline(always)]
    pub fn chain_to_nxt(&self) -> CHAIN_TO_NXT_R {
        CHAIN_TO_NXT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable for FIFO read clearing the FIFO level trigger and level interrupt. - 0: Disabled, FIFO level trigger and level interrupt generation follows LEVEL.LEVEL. - 1: Enabled, after initial FIFO level trigger and level interrupt generation, subsequent FIFO level triggers and level intterrupts are blocked until at least FIFO.LEVEL+1 reads have occurred. If CHAIN_TO_NXT is enabled, only FIFO\\[0\\].CONFIG. TR_CLR_RD_EN should be configured."]
    #[inline(always)]
    pub fn tr_intr_clr_rd_en(&self) -> TR_INTR_CLR_RD_EN_R {
        TR_INTR_CLR_RD_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - channel number (ID) enable bit -0: CHAN_ID field in RD_DATA is disabled. A read from RD_DATA will result in (4'b0,16'b RESULT) -1: CHAN_ID field in RD_DATA is enabled. A read from RD_DATA will result in (4'b CHAN_ID, 16'b RESULT) If CHAIN_EN is enabled, only FIFO\\[0\\].config.chan_id_en should be configured and the other FIFOs in the chain will inherit the FIFO\\[0\\]
config."]
    #[inline(always)]
    pub fn chan_id_en(&mut self) -> CHAN_ID_EN_W {
        CHAN_ID_EN_W { w: self }
    }
    #[doc = "Bit 1 - Chain FIFO to next FIFO (i.e. chain FIFO0 and FIFO1). - 0: FIFO not chained . FIFO operates independently (FIFO depth of 64) and only operates on result data from its associated SAR. - 1: FIFO chained to next FIFO. FIFO is part of a chain of FIFOs (effectively extending the FIFO depth beyond 64) and only operates on result data from SAR0."]
    #[inline(always)]
    pub fn chain_to_nxt(&mut self) -> CHAIN_TO_NXT_W {
        CHAIN_TO_NXT_W { w: self }
    }
    #[doc = "Bit 2 - Enable for FIFO read clearing the FIFO level trigger and level interrupt. - 0: Disabled, FIFO level trigger and level interrupt generation follows LEVEL.LEVEL. - 1: Enabled, after initial FIFO level trigger and level interrupt generation, subsequent FIFO level triggers and level intterrupts are blocked until at least FIFO.LEVEL+1 reads have occurred. If CHAIN_TO_NXT is enabled, only FIFO\\[0\\].CONFIG. TR_CLR_RD_EN should be configured."]
    #[inline(always)]
    pub fn tr_intr_clr_rd_en(&mut self) -> TR_INTR_CLR_RD_EN_W {
        TR_INTR_CLR_RD_EN_W { w: self }
    }
}
