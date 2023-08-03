#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `TIMER_ENABLED`"]
pub type TIMER_ENABLED_R = crate::R<bool, bool>;
#[doc = "Reader of field `HV_REGS_ISOLATED`"]
pub type HV_REGS_ISOLATED_R = crate::R<bool, bool>;
#[doc = "Reader of field `ILLEGAL_HVOP`"]
pub type ILLEGAL_HVOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TURBO_N`"]
pub type TURBO_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `WR_EN_MON`"]
pub type WR_EN_MON_R = crate::R<bool, bool>;
#[doc = "Reader of field `IF_SEL_MON`"]
pub type IF_SEL_MON_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER_STATUS`"]
pub type TIMER_STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `R_GRANT_DELAY_STATUS`"]
pub type R_GRANT_DELAY_STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FM_BUSY`"]
pub type FM_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `FM_READY`"]
pub type FM_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `POS_PUMP_VLO`"]
pub type POS_PUMP_VLO_R = crate::R<bool, bool>;
#[doc = "Reader of field `NEG_PUMP_VHI`"]
pub type NEG_PUMP_VHI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWW`"]
pub type RWW_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAX_DOUT_WIDTH`"]
pub type MAX_DOUT_WIDTH_R = crate::R<bool, bool>;
#[doc = "Reader of field `SECTOR0_SR`"]
pub type SECTOR0_SR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESET_MM`"]
pub type RESET_MM_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROW_ODD`"]
pub type ROW_ODD_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROW_EVEN`"]
pub type ROW_EVEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `HVOP_SUB_SECTOR_N`"]
pub type HVOP_SUB_SECTOR_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `HVOP_SECTOR`"]
pub type HVOP_SECTOR_R = crate::R<bool, bool>;
#[doc = "Reader of field `HVOP_BULK_ALL`"]
pub type HVOP_BULK_ALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBUS_RA_MATCH`"]
pub type CBUS_RA_MATCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBUS_RED_ROW_EN`"]
pub type CBUS_RED_ROW_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RQ_ERROR`"]
pub type RQ_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PUMP_PDAC`"]
pub type PUMP_PDAC_R = crate::R<u8, u8>;
#[doc = "Reader of field `PUMP_NDAC`"]
pub type PUMP_NDAC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - This is the timer_en bit set by writing a '1' in the TIMER_CTL bit 31. It is reset by HW when the timer expires 0: timer not running 1: Timer is enabled and not expired yet"]
    #[inline(always)]
    pub fn timer_enabled(&self) -> TIMER_ENABLED_R {
        TIMER_ENABLED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates the isolation status at HV trim and redundancy registers inputs 0: Not isolated, writing permitted 1: isolated writing disabled"]
    #[inline(always)]
    pub fn hv_regs_isolated(&self) -> HV_REGS_ISOLATED_R {
        HV_REGS_ISOLATED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates a bulk, sector erase, program has been requested when axa=1 0: no error 1: illegal HV operation error"]
    #[inline(always)]
    pub fn illegal_hvop(&self) -> ILLEGAL_HVOP_R {
        ILLEGAL_HVOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. 0: turbo mode 1: normal mode"]
    #[inline(always)]
    pub fn turbo_n(&self) -> TURBO_N_R {
        TURBO_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FM_CTL.WR_EN bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn wr_en_mon(&self) -> WR_EN_MON_R {
        WR_EN_MON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FM_CTL.IF_SEL bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn if_sel_mon(&self) -> IF_SEL_MON_R {
        IF_SEL_MON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The actual timer state sync-ed in clk_c domain: 0: timer is not running: 1: timer is running;"]
    #[inline(always)]
    pub fn timer_status(&self) -> TIMER_STATUS_R {
        TIMER_STATUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0: R_GRANT_DELAY timer is not running 1: R_GRANT_DELAY timer is running"]
    #[inline(always)]
    pub fn r_grant_delay_status(&self) -> R_GRANT_DELAY_STATUS_R {
        R_GRANT_DELAY_STATUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0': FM not busy 1: FM BUSY : R_GRANT is 0 as result of a busy request from FM ready, or from HV operations."]
    #[inline(always)]
    pub fn fm_busy(&self) -> FM_BUSY_R {
        FM_BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 0: FM not ready 1: FM ready"]
    #[inline(always)]
    pub fn fm_ready(&self) -> FM_READY_R {
        FM_READY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - POS pump VLO"]
    #[inline(always)]
    pub fn pos_pump_vlo(&self) -> POS_PUMP_VLO_R {
        POS_PUMP_VLO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - NEG pump VHI"]
    #[inline(always)]
    pub fn neg_pump_vhi(&self) -> NEG_PUMP_VHI_R {
        NEG_PUMP_VHI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FM Type (Read While Write or Not Read While Write): 0: Non RWW FM Type 1: RWW FM Type"]
    #[inline(always)]
    pub fn rww(&self) -> RWW_R {
        RWW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Internal memory core max data out size (number of data out bits per column): 0: x128 bits 1: x256 bits"]
    #[inline(always)]
    pub fn max_dout_width(&self) -> MAX_DOUT_WIDTH_R {
        MAX_DOUT_WIDTH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 0: Sector 0 does not contain special rows. The special rows are located in separate special sectors. 1: Sector 0 contains special rows"]
    #[inline(always)]
    pub fn sector0_sr(&self) -> SECTOR0_SR_R {
        SECTOR0_SR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Test_only, internal node: mpcon reset_mm"]
    #[inline(always)]
    pub fn reset_mm(&self) -> RESET_MM_R {
        RESET_MM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Test_only, internal node: mpcon row_odd"]
    #[inline(always)]
    pub fn row_odd(&self) -> ROW_ODD_R {
        ROW_ODD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Test_only, internal node: mpcon row_even"]
    #[inline(always)]
    pub fn row_even(&self) -> ROW_EVEN_R {
        ROW_EVEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Test_only, internal node: mpcon bk_subb"]
    #[inline(always)]
    pub fn hvop_sub_sector_n(&self) -> HVOP_SUB_SECTOR_N_R {
        HVOP_SUB_SECTOR_N_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Test_only, internal node: mpcon bk_sec"]
    #[inline(always)]
    pub fn hvop_sector(&self) -> HVOP_SECTOR_R {
        HVOP_SECTOR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Test_only, internal node: mpcon bk_all"]
    #[inline(always)]
    pub fn hvop_bulk_all(&self) -> HVOP_BULK_ALL_R {
        HVOP_BULK_ALL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Test_only, internal node: mpcon ra match"]
    #[inline(always)]
    pub fn cbus_ra_match(&self) -> CBUS_RA_MATCH_R {
        CBUS_RA_MATCH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Test_only, internal node: mpcon red_row_en"]
    #[inline(always)]
    pub fn cbus_red_row_en(&self) -> CBUS_RED_ROW_EN_R {
        CBUS_RED_ROW_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Test_only, internal node: rq_error sync-de in clk_c domain"]
    #[inline(always)]
    pub fn rq_error(&self) -> RQ_ERROR_R {
        RQ_ERROR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Test_only, internal node: regif pdac outputs to pos pump"]
    #[inline(always)]
    pub fn pump_pdac(&self) -> PUMP_PDAC_R {
        PUMP_PDAC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Test_only, internal node: regif ndac outputs to pos pump"]
    #[inline(always)]
    pub fn pump_ndac(&self) -> PUMP_NDAC_R {
        PUMP_NDAC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
