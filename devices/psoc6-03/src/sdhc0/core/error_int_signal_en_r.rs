#[doc = "Reader of register ERROR_INT_SIGNAL_EN_R"]
pub type R = crate::R<u16, super::ERROR_INT_SIGNAL_EN_R>;
#[doc = "Writer for register ERROR_INT_SIGNAL_EN_R"]
pub type W = crate::W<u16, super::ERROR_INT_SIGNAL_EN_R>;
#[doc = "Register ERROR_INT_SIGNAL_EN_R `reset()`'s with value 0"]
impl crate::ResetValue for super::ERROR_INT_SIGNAL_EN_R {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD_TOUT_ERR_SIGNAL_EN`"]
pub type CMD_TOUT_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_TOUT_ERR_SIGNAL_EN`"]
pub struct CMD_TOUT_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_TOUT_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `CMD_CRC_ERR_SIGNAL_EN`"]
pub type CMD_CRC_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_CRC_ERR_SIGNAL_EN`"]
pub struct CMD_CRC_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CRC_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `CMD_END_BIT_ERR_SIGNAL_EN`"]
pub type CMD_END_BIT_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_END_BIT_ERR_SIGNAL_EN`"]
pub struct CMD_END_BIT_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_END_BIT_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `CMD_IDX_ERR_SIGNAL_EN`"]
pub type CMD_IDX_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_IDX_ERR_SIGNAL_EN`"]
pub struct CMD_IDX_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_IDX_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `DATA_TOUT_ERR_SIGNAL_EN`"]
pub type DATA_TOUT_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_TOUT_ERR_SIGNAL_EN`"]
pub struct DATA_TOUT_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TOUT_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `DATA_CRC_ERR_SIGNAL_EN`"]
pub type DATA_CRC_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_CRC_ERR_SIGNAL_EN`"]
pub struct DATA_CRC_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CRC_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `DATA_END_BIT_ERR_SIGNAL_EN`"]
pub type DATA_END_BIT_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_END_BIT_ERR_SIGNAL_EN`"]
pub struct DATA_END_BIT_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_END_BIT_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `CUR_LMT_ERR_SIGNAL_EN`"]
pub type CUR_LMT_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CUR_LMT_ERR_SIGNAL_EN`"]
pub struct CUR_LMT_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_LMT_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `AUTO_CMD_ERR_SIGNAL_EN`"]
pub type AUTO_CMD_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_CMD_ERR_SIGNAL_EN`"]
pub struct AUTO_CMD_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CMD_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `ADMA_ERR_SIGNAL_EN`"]
pub type ADMA_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADMA_ERR_SIGNAL_EN`"]
pub struct ADMA_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMA_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `TUNING_ERR_SIGNAL_EN`"]
pub type TUNING_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNING_ERR_SIGNAL_EN`"]
pub struct TUNING_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `RESP_ERR_SIGNAL_EN`"]
pub type RESP_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESP_ERR_SIGNAL_EN`"]
pub struct RESP_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `BOOT_ACK_ERR_SIGNAL_EN`"]
pub type BOOT_ACK_ERR_SIGNAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_ACK_ERR_SIGNAL_EN`"]
pub struct BOOT_ACK_ERR_SIGNAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ACK_ERR_SIGNAL_EN_W<'a> {
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
#[doc = "Reader of field `VENDOR_ERR_SIGNAL_EN1`"]
pub type VENDOR_ERR_SIGNAL_EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VENDOR_ERR_SIGNAL_EN1`"]
pub struct VENDOR_ERR_SIGNAL_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> VENDOR_ERR_SIGNAL_EN1_W<'a> {
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
#[doc = "Reader of field `VENDOR_ERR_SIGNAL_EN2`"]
pub type VENDOR_ERR_SIGNAL_EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VENDOR_ERR_SIGNAL_EN2`"]
pub struct VENDOR_ERR_SIGNAL_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> VENDOR_ERR_SIGNAL_EN2_W<'a> {
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
#[doc = "Reader of field `VENDOR_ERR_SIGNAL_EN3`"]
pub type VENDOR_ERR_SIGNAL_EN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VENDOR_ERR_SIGNAL_EN3`"]
pub struct VENDOR_ERR_SIGNAL_EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> VENDOR_ERR_SIGNAL_EN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_tout_err_signal_en(&self) -> CMD_TOUT_ERR_SIGNAL_EN_R {
        CMD_TOUT_ERR_SIGNAL_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_crc_err_signal_en(&self) -> CMD_CRC_ERR_SIGNAL_EN_R {
        CMD_CRC_ERR_SIGNAL_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_end_bit_err_signal_en(&self) -> CMD_END_BIT_ERR_SIGNAL_EN_R {
        CMD_END_BIT_ERR_SIGNAL_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err_signal_en(&self) -> CMD_IDX_ERR_SIGNAL_EN_R {
        CMD_IDX_ERR_SIGNAL_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_tout_err_signal_en(&self) -> DATA_TOUT_ERR_SIGNAL_EN_R {
        DATA_TOUT_ERR_SIGNAL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_crc_err_signal_en(&self) -> DATA_CRC_ERR_SIGNAL_EN_R {
        DATA_CRC_ERR_SIGNAL_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_end_bit_err_signal_en(&self) -> DATA_END_BIT_ERR_SIGNAL_EN_R {
        DATA_END_BIT_ERR_SIGNAL_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cur_lmt_err_signal_en(&self) -> CUR_LMT_ERR_SIGNAL_EN_R {
        CUR_LMT_ERR_SIGNAL_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn auto_cmd_err_signal_en(&self) -> AUTO_CMD_ERR_SIGNAL_EN_R {
        AUTO_CMD_ERR_SIGNAL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn adma_err_signal_en(&self) -> ADMA_ERR_SIGNAL_EN_R {
        ADMA_ERR_SIGNAL_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tuning_err_signal_en(&self) -> TUNING_ERR_SIGNAL_EN_R {
        TUNING_ERR_SIGNAL_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn resp_err_signal_en(&self) -> RESP_ERR_SIGNAL_EN_R {
        RESP_ERR_SIGNAL_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn boot_ack_err_signal_en(&self) -> BOOT_ACK_ERR_SIGNAL_EN_R {
        BOOT_ACK_ERR_SIGNAL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en1(&self) -> VENDOR_ERR_SIGNAL_EN1_R {
        VENDOR_ERR_SIGNAL_EN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en2(&self) -> VENDOR_ERR_SIGNAL_EN2_R {
        VENDOR_ERR_SIGNAL_EN2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en3(&self) -> VENDOR_ERR_SIGNAL_EN3_R {
        VENDOR_ERR_SIGNAL_EN3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_tout_err_signal_en(&mut self) -> CMD_TOUT_ERR_SIGNAL_EN_W {
        CMD_TOUT_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_crc_err_signal_en(&mut self) -> CMD_CRC_ERR_SIGNAL_EN_W {
        CMD_CRC_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_end_bit_err_signal_en(&mut self) -> CMD_END_BIT_ERR_SIGNAL_EN_W {
        CMD_END_BIT_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err_signal_en(&mut self) -> CMD_IDX_ERR_SIGNAL_EN_W {
        CMD_IDX_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_tout_err_signal_en(&mut self) -> DATA_TOUT_ERR_SIGNAL_EN_W {
        DATA_TOUT_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_crc_err_signal_en(&mut self) -> DATA_CRC_ERR_SIGNAL_EN_W {
        DATA_CRC_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_end_bit_err_signal_en(&mut self) -> DATA_END_BIT_ERR_SIGNAL_EN_W {
        DATA_END_BIT_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cur_lmt_err_signal_en(&mut self) -> CUR_LMT_ERR_SIGNAL_EN_W {
        CUR_LMT_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn auto_cmd_err_signal_en(&mut self) -> AUTO_CMD_ERR_SIGNAL_EN_W {
        AUTO_CMD_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn adma_err_signal_en(&mut self) -> ADMA_ERR_SIGNAL_EN_W {
        ADMA_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tuning_err_signal_en(&mut self) -> TUNING_ERR_SIGNAL_EN_W {
        TUNING_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 11 - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn resp_err_signal_en(&mut self) -> RESP_ERR_SIGNAL_EN_W {
        RESP_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn boot_ack_err_signal_en(&mut self) -> BOOT_ACK_ERR_SIGNAL_EN_W {
        BOOT_ACK_ERR_SIGNAL_EN_W { w: self }
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en1(&mut self) -> VENDOR_ERR_SIGNAL_EN1_W {
        VENDOR_ERR_SIGNAL_EN1_W { w: self }
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en2(&mut self) -> VENDOR_ERR_SIGNAL_EN2_W {
        VENDOR_ERR_SIGNAL_EN2_W { w: self }
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en3(&mut self) -> VENDOR_ERR_SIGNAL_EN3_W {
        VENDOR_ERR_SIGNAL_EN3_W { w: self }
    }
}
