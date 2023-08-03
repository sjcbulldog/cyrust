#[doc = "Reader of register PSTATE_REG"]
pub type R = crate::R<u32, super::PSTATE_REG>;
#[doc = "Reader of field `CMD_INHIBIT`"]
pub type CMD_INHIBIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMD_INHIBIT_DAT`"]
pub type CMD_INHIBIT_DAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAT_LINE_ACTIVE`"]
pub type DAT_LINE_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAT_7_4`"]
pub type DAT_7_4_R = crate::R<u8, u8>;
#[doc = "Reader of field `WR_XFER_ACTIVE`"]
pub type WR_XFER_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RD_XFER_ACTIVE`"]
pub type RD_XFER_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUF_WR_ENABLE`"]
pub type BUF_WR_ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUF_RD_ENABLE`"]
pub type BUF_RD_ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARD_INSERTED`"]
pub type CARD_INSERTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARD_STABLE`"]
pub type CARD_STABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARD_DETECT_PIN_LEVEL`"]
pub type CARD_DETECT_PIN_LEVEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `WR_PROTECT_SW_LVL`"]
pub type WR_PROTECT_SW_LVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAT_3_0`"]
pub type DAT_3_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `CMD_LINE_LVL`"]
pub type CMD_LINE_LVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `HOST_REG_VOL`"]
pub type HOST_REG_VOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMD_ISSU_ERR`"]
pub type CMD_ISSU_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUB_CMD_STAT`"]
pub type SUB_CMD_STAT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD) This bit indicates the following : - SD/eMMC mode: If this bit is set to 0, it indicates that the CMD line is not in use and the Host controller can issue an SD/eMMC command using the CMD line. This bit is set when the command register is written. This bit is cleared when the command response is received. This bit is not cleared by the response of auto CMD12/23 but cleared by the response of read/write command. Values: - 0x0 (READY): Host Controller is ready to issue a command - 0x1 (NOT_READY): Host Controller is not ready to issue a command"]
    #[inline(always)]
    pub fn cmd_inhibit(&self) -> CMD_INHIBIT_R {
        CMD_INHIBIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT) This bit is applicable for SD/eMMC mode and is generated if either DAT line active or Read transfer active is set to 1. If this bit is set to 0, it indicates that the Host Controller can issue subsequent SD/eMMC commands. Values: - 0x0 (READY): Can issue command which used DAT line - 0x1 (NOT_READY): Cannot issue command which used DAT line"]
    #[inline(always)]
    pub fn cmd_inhibit_dat(&self) -> CMD_INHIBIT_DAT_R {
        CMD_INHIBIT_DAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active (SD/eMMC Mode only) This bit indicates whether one of the DAT lines on the SD/eMMC bus is in use. In the case of read transactions, this bit indicates whether a read transfer is executing on the SD/eMMC bus. In the case of write transactions, this bit indicates whether a write transfer is executing on the SD/eMMC bus. For a command with busy, this status indicates whether the command executing busy is executing on an SD or eMMC bus. Values: - 0x0 (INACTIVE): DAT Line Inactive - 0x1 (ACTIVE): DAT Line Active"]
    #[inline(always)]
    pub fn dat_line_active(&self) -> DAT_LINE_ACTIVE_R {
        DAT_LINE_ACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - DAT\\[7:4\\]
Line Signal Level This bit is used to check the DAT line level to recover from errors and for debugging. These bits reflect the value of the sd_dat_in (upper nibble) signal."]
    #[inline(always)]
    pub fn dat_7_4(&self) -> DAT_7_4_R {
        DAT_7_4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Write Transfer Active This status indicates whether a write transfer is active for SD/eMMC mode. Values: - 0x0 (INACTIVE): No valid data - 0x1 (ACTIVE): Transferring data"]
    #[inline(always)]
    pub fn wr_xfer_active(&self) -> WR_XFER_ACTIVE_R {
        WR_XFER_ACTIVE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active This bit indicates whether a read transfer is active for SD/eMMC mode. Values: - 0x0 (INACTIVE): No valid data - 0x1 (ACTIVE): Transferring data"]
    #[inline(always)]
    pub fn rd_xfer_active(&self) -> RD_XFER_ACTIVE_R {
        RD_XFER_ACTIVE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable This bit is used for non-DMA transfers. This bit is set if space is available for writing data. Values: - 0x0 (DISABLED): Write disable - 0x1 (ENABLED): Write enable"]
    #[inline(always)]
    pub fn buf_wr_enable(&self) -> BUF_WR_ENABLE_R {
        BUF_WR_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable This bit is used for non-DMA transfers. This bit is set if valid data exists in the Host buffer. Values: - 0x0 (DISABLED): Read disable - 0x1 (ENABLED): Read enable"]
    #[inline(always)]
    pub fn buf_rd_enable(&self) -> BUF_RD_ENABLE_R {
        BUF_RD_ENABLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Card Inserted This bit indicates whether a card has been inserted. The Host Controller debounces this signal so that Host Driver need not wait for it to stabilize. Values: - 0x0 (FALSE): Reset, Debouncing, or No card - 0x1 (TRUE): Card Inserted"]
    #[inline(always)]
    pub fn card_inserted(&self) -> CARD_INSERTED_R {
        CARD_INSERTED_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Card Stable This bit indicates the stability of the Card Detect Pin Level. A card is not detected if this bit is set to 1 and the value of the CARD_INSERTED bit is 0. Values: - 0x0 (FALSE): Reset or Debouncing - 0x1 (TRUE): No Card or Inserted"]
    #[inline(always)]
    pub fn card_stable(&self) -> CARD_STABLE_R {
        CARD_STABLE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level This bit reflects the inverse synchronized value of the card_detect_n signal. Values: - 0x0 (FALSE): No card present - 0x1 (TRUE): Card Present"]
    #[inline(always)]
    pub fn card_detect_pin_level(&self) -> CARD_DETECT_PIN_LEVEL_R {
        CARD_DETECT_PIN_LEVEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level This bit is supported only for memory and combo cards. This bit reflects the synchronized value of the card_write_prot signal. Values: - 0x0 (FALSE): Write protected - 0x1 (TRUE): Write enabled"]
    #[inline(always)]
    pub fn wr_protect_sw_lvl(&self) -> WR_PROTECT_SW_LVL_R {
        WR_PROTECT_SW_LVL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - DAT\\[3:0\\]
Line Signal Level This bit is used to check the DAT line level to recover from errors and for debugging. These bits reflect the value of the sd_dat_in (lower nibble) signal."]
    #[inline(always)]
    pub fn dat_3_0(&self) -> DAT_3_0_R {
        DAT_3_0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Command-Line Signal Level This bit is used to check the CMD line level to recover from errors and for debugging. These bits reflect the value of the sd_cmd_in signal."]
    #[inline(always)]
    pub fn cmd_line_lvl(&self) -> CMD_LINE_LVL_R {
        CMD_LINE_LVL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host Regulator Voltage Stable This bit is used to check whether the host regulator voltage is stable for switching the voltage of UHS-I mode. This bit reflects the synchronized value of the host_reg_vol_stable signal. Values: - 0x0 (FALSE): Host Regulator Voltage is not stable - 0x1 (TRUE): Host Regulator Voltage is stable"]
    #[inline(always)]
    pub fn host_reg_vol(&self) -> HOST_REG_VOL_R {
        HOST_REG_VOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Command Not Issued by Error This bit is set if a command cannot be issued after setting the command register due to an error except the Auto CMD12 error. Values: - 0x0 (FALSE): No error for issuing a command - 0x1 (TRUE): Command cannot be issued"]
    #[inline(always)]
    pub fn cmd_issu_err(&self) -> CMD_ISSU_ERR_R {
        CMD_ISSU_ERR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Sub Command Status This bit is used to distinguish between a main command and a sub command status. Values: - 0x0 (FALSE): Main Command Status - 0x1 (TRUE): Sub Command Status"]
    #[inline(always)]
    pub fn sub_cmd_stat(&self) -> SUB_CMD_STAT_R {
        SUB_CMD_STAT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
