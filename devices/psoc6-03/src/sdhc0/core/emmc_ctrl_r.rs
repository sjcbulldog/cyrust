#[doc = "Reader of register EMMC_CTRL_R"]
pub type R = crate::R<u16, super::EMMC_CTRL_R>;
#[doc = "Writer for register EMMC_CTRL_R"]
pub type W = crate::W<u16, super::EMMC_CTRL_R>;
#[doc = "Register EMMC_CTRL_R `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::EMMC_CTRL_R {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c
    }
}
#[doc = "Reader of field `CARD_IS_EMMC`"]
pub type CARD_IS_EMMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_IS_EMMC`"]
pub struct CARD_IS_EMMC_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_IS_EMMC_W<'a> {
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
#[doc = "Reader of field `DISABLE_DATA_CRC_CHK`"]
pub type DISABLE_DATA_CRC_CHK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_DATA_CRC_CHK`"]
pub struct DISABLE_DATA_CRC_CHK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_DATA_CRC_CHK_W<'a> {
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
#[doc = "Reader of field `EMMC_RST_N`"]
pub type EMMC_RST_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMMC_RST_N`"]
pub struct EMMC_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> EMMC_RST_N_W<'a> {
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
#[doc = "Reader of field `EMMC_RST_N_OE`"]
pub type EMMC_RST_N_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMMC_RST_N_OE`"]
pub struct EMMC_RST_N_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> EMMC_RST_N_OE_W<'a> {
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
#[doc = "Reader of field `CQE_ALGO_SEL`"]
pub type CQE_ALGO_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQE_ALGO_SEL`"]
pub struct CQE_ALGO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CQE_ALGO_SEL_W<'a> {
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
#[doc = "Reader of field `CQE_PREFETCH_DISABLE`"]
pub type CQE_PREFETCH_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQE_PREFETCH_DISABLE`"]
pub struct CQE_PREFETCH_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CQE_PREFETCH_DISABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
    #[inline(always)]
    pub fn card_is_emmc(&self) -> CARD_IS_EMMC_R {
        CARD_IS_EMMC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
    #[inline(always)]
    pub fn disable_data_crc_chk(&self) -> DISABLE_DATA_CRC_CHK_R {
        DISABLE_DATA_CRC_CHK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    #[inline(always)]
    pub fn emmc_rst_n(&self) -> EMMC_RST_N_R {
        EMMC_RST_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
    #[inline(always)]
    pub fn emmc_rst_n_oe(&self) -> EMMC_RST_N_OE_R {
        EMMC_RST_N_OE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
    #[inline(always)]
    pub fn cqe_algo_sel(&self) -> CQE_ALGO_SEL_R {
        CQE_ALGO_SEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
    #[inline(always)]
    pub fn cqe_prefetch_disable(&self) -> CQE_PREFETCH_DISABLE_R {
        CQE_PREFETCH_DISABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
    #[inline(always)]
    pub fn card_is_emmc(&mut self) -> CARD_IS_EMMC_W {
        CARD_IS_EMMC_W { w: self }
    }
    #[doc = "Bit 1 - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
    #[inline(always)]
    pub fn disable_data_crc_chk(&mut self) -> DISABLE_DATA_CRC_CHK_W {
        DISABLE_DATA_CRC_CHK_W { w: self }
    }
    #[doc = "Bit 2 - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    #[inline(always)]
    pub fn emmc_rst_n(&mut self) -> EMMC_RST_N_W {
        EMMC_RST_N_W { w: self }
    }
    #[doc = "Bit 3 - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
    #[inline(always)]
    pub fn emmc_rst_n_oe(&mut self) -> EMMC_RST_N_OE_W {
        EMMC_RST_N_OE_W { w: self }
    }
    #[doc = "Bit 9 - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
    #[inline(always)]
    pub fn cqe_algo_sel(&mut self) -> CQE_ALGO_SEL_W {
        CQE_ALGO_SEL_W { w: self }
    }
    #[doc = "Bit 10 - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
    #[inline(always)]
    pub fn cqe_prefetch_disable(&mut self) -> CQE_PREFETCH_DISABLE_W {
        CQE_PREFETCH_DISABLE_W { w: self }
    }
}
