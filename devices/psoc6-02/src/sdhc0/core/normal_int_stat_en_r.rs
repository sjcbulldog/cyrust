#[doc = "Reader of register NORMAL_INT_STAT_EN_R"]
pub type R = crate::R<u16, super::NORMAL_INT_STAT_EN_R>;
#[doc = "Writer for register NORMAL_INT_STAT_EN_R"]
pub type W = crate::W<u16, super::NORMAL_INT_STAT_EN_R>;
#[doc = "Register NORMAL_INT_STAT_EN_R `reset()`'s with value 0"]
impl crate::ResetValue for super::NORMAL_INT_STAT_EN_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `CMD_COMPLETE_STAT_EN`"]
pub type CMD_COMPLETE_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_COMPLETE_STAT_EN`"]
pub struct CMD_COMPLETE_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_COMPLETE_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `XFER_COMPLETE_STAT_EN`"]
pub type XFER_COMPLETE_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XFER_COMPLETE_STAT_EN`"]
pub struct XFER_COMPLETE_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XFER_COMPLETE_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `BGAP_EVENT_STAT_EN`"]
pub type BGAP_EVENT_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BGAP_EVENT_STAT_EN`"]
pub struct BGAP_EVENT_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BGAP_EVENT_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMA_INTERRUPT_STAT_EN`"]
pub type DMA_INTERRUPT_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_INTERRUPT_STAT_EN`"]
pub struct DMA_INTERRUPT_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INTERRUPT_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BUF_WR_READY_STAT_EN`"]
pub type BUF_WR_READY_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUF_WR_READY_STAT_EN`"]
pub struct BUF_WR_READY_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_WR_READY_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `BUF_RD_READY_STAT_EN`"]
pub type BUF_RD_READY_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUF_RD_READY_STAT_EN`"]
pub struct BUF_RD_READY_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_RD_READY_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CARD_INSERTION_STAT_EN`"]
pub type CARD_INSERTION_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_INSERTION_STAT_EN`"]
pub struct CARD_INSERTION_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INSERTION_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CARD_REMOVAL_STAT_EN`"]
pub type CARD_REMOVAL_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_REMOVAL_STAT_EN`"]
pub struct CARD_REMOVAL_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_REMOVAL_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CARD_INTERRUPT_STAT_EN`"]
pub type CARD_INTERRUPT_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_INTERRUPT_STAT_EN`"]
pub struct CARD_INTERRUPT_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INTERRUPT_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `INT_A_STAT_EN`"]
pub type INT_A_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_A_STAT_EN`"]
pub struct INT_A_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_A_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `INT_B_STAT_EN`"]
pub type INT_B_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_B_STAT_EN`"]
pub struct INT_B_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_B_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `INT_C_STAT_EN`"]
pub type INT_C_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_C_STAT_EN`"]
pub struct INT_C_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_C_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RE_TUNE_EVENT_STAT_EN`"]
pub type RE_TUNE_EVENT_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RE_TUNE_EVENT_STAT_EN`"]
pub struct RE_TUNE_EVENT_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_TUNE_EVENT_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FX_EVENT_STAT_EN`"]
pub type FX_EVENT_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FX_EVENT_STAT_EN`"]
pub struct FX_EVENT_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FX_EVENT_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CQE_EVENT_STAT_EN`"]
pub type CQE_EVENT_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQE_EVENT_STAT_EN`"]
pub struct CQE_EVENT_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CQE_EVENT_STAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_complete_stat_en(&self) -> CMD_COMPLETE_STAT_EN_R {
        CMD_COMPLETE_STAT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn xfer_complete_stat_en(&self) -> XFER_COMPLETE_STAT_EN_R {
        XFER_COMPLETE_STAT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn bgap_event_stat_en(&self) -> BGAP_EVENT_STAT_EN_R {
        BGAP_EVENT_STAT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn dma_interrupt_stat_en(&self) -> DMA_INTERRUPT_STAT_EN_R {
        DMA_INTERRUPT_STAT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready_stat_en(&self) -> BUF_WR_READY_STAT_EN_R {
        BUF_WR_READY_STAT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready_stat_en(&self) -> BUF_RD_READY_STAT_EN_R {
        BUF_RD_READY_STAT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_insertion_stat_en(&self) -> CARD_INSERTION_STAT_EN_R {
        CARD_INSERTION_STAT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_removal_stat_en(&self) -> CARD_REMOVAL_STAT_EN_R {
        CARD_REMOVAL_STAT_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_interrupt_stat_en(&self) -> CARD_INTERRUPT_STAT_EN_R {
        CARD_INTERRUPT_STAT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn int_a_stat_en(&self) -> INT_A_STAT_EN_R {
        INT_A_STAT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn int_b_stat_en(&self) -> INT_B_STAT_EN_R {
        INT_B_STAT_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn int_c_stat_en(&self) -> INT_C_STAT_EN_R {
        INT_C_STAT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn re_tune_event_stat_en(&self) -> RE_TUNE_EVENT_STAT_EN_R {
        RE_TUNE_EVENT_STAT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn fx_event_stat_en(&self) -> FX_EVENT_STAT_EN_R {
        FX_EVENT_STAT_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cqe_event_stat_en(&self) -> CQE_EVENT_STAT_EN_R {
        CQE_EVENT_STAT_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_complete_stat_en(&mut self) -> CMD_COMPLETE_STAT_EN_W {
        CMD_COMPLETE_STAT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn xfer_complete_stat_en(&mut self) -> XFER_COMPLETE_STAT_EN_W {
        XFER_COMPLETE_STAT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn bgap_event_stat_en(&mut self) -> BGAP_EVENT_STAT_EN_W {
        BGAP_EVENT_STAT_EN_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn dma_interrupt_stat_en(&mut self) -> DMA_INTERRUPT_STAT_EN_W {
        DMA_INTERRUPT_STAT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready_stat_en(&mut self) -> BUF_WR_READY_STAT_EN_W {
        BUF_WR_READY_STAT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready_stat_en(&mut self) -> BUF_RD_READY_STAT_EN_W {
        BUF_RD_READY_STAT_EN_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_insertion_stat_en(&mut self) -> CARD_INSERTION_STAT_EN_W {
        CARD_INSERTION_STAT_EN_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_removal_stat_en(&mut self) -> CARD_REMOVAL_STAT_EN_W {
        CARD_REMOVAL_STAT_EN_W { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_interrupt_stat_en(&mut self) -> CARD_INTERRUPT_STAT_EN_W {
        CARD_INTERRUPT_STAT_EN_W { w: self }
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn int_a_stat_en(&mut self) -> INT_A_STAT_EN_W {
        INT_A_STAT_EN_W { w: self }
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn int_b_stat_en(&mut self) -> INT_B_STAT_EN_W {
        INT_B_STAT_EN_W { w: self }
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn int_c_stat_en(&mut self) -> INT_C_STAT_EN_W {
        INT_C_STAT_EN_W { w: self }
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn re_tune_event_stat_en(&mut self) -> RE_TUNE_EVENT_STAT_EN_W {
        RE_TUNE_EVENT_STAT_EN_W { w: self }
    }
    #[doc = "Bit 13 - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn fx_event_stat_en(&mut self) -> FX_EVENT_STAT_EN_W {
        FX_EVENT_STAT_EN_W { w: self }
    }
    #[doc = "Bit 14 - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cqe_event_stat_en(&mut self) -> CQE_EVENT_STAT_EN_W {
        CQE_EVENT_STAT_EN_W { w: self }
    }
}
