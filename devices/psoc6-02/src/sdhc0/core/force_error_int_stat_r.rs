#[doc = "Reader of register FORCE_ERROR_INT_STAT_R"]
pub type R = crate::R<u16, super::FORCE_ERROR_INT_STAT_R>;
#[doc = "Writer for register FORCE_ERROR_INT_STAT_R"]
pub type W = crate::W<u16, super::FORCE_ERROR_INT_STAT_R>;
#[doc = "Register FORCE_ERROR_INT_STAT_R `reset()`'s with value 0"]
impl crate::ResetValue for super::FORCE_ERROR_INT_STAT_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `FORCE_CMD_TOUT_ERR`"]
pub type FORCE_CMD_TOUT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_CMD_TOUT_ERR`"]
pub struct FORCE_CMD_TOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_CMD_TOUT_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_CMD_CRC_ERR`"]
pub type FORCE_CMD_CRC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_CMD_CRC_ERR`"]
pub struct FORCE_CMD_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_CMD_CRC_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_CMD_END_BIT_ERR`"]
pub type FORCE_CMD_END_BIT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_CMD_END_BIT_ERR`"]
pub struct FORCE_CMD_END_BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_CMD_END_BIT_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_CMD_IDX_ERR`"]
pub type FORCE_CMD_IDX_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_CMD_IDX_ERR`"]
pub struct FORCE_CMD_IDX_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_CMD_IDX_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_DATA_TOUT_ERR`"]
pub type FORCE_DATA_TOUT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_DATA_TOUT_ERR`"]
pub struct FORCE_DATA_TOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_DATA_TOUT_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_DATA_CRC_ERR`"]
pub type FORCE_DATA_CRC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_DATA_CRC_ERR`"]
pub struct FORCE_DATA_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_DATA_CRC_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_DATA_END_BIT_ERR`"]
pub type FORCE_DATA_END_BIT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_DATA_END_BIT_ERR`"]
pub struct FORCE_DATA_END_BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_DATA_END_BIT_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_CUR_LMT_ERR`"]
pub type FORCE_CUR_LMT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_CUR_LMT_ERR`"]
pub struct FORCE_CUR_LMT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_CUR_LMT_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_AUTO_CMD_ERR`"]
pub type FORCE_AUTO_CMD_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_AUTO_CMD_ERR`"]
pub struct FORCE_AUTO_CMD_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_AUTO_CMD_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_ADMA_ERR`"]
pub type FORCE_ADMA_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_ADMA_ERR`"]
pub struct FORCE_ADMA_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_ADMA_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_TUNING_ERR`"]
pub type FORCE_TUNING_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_TUNING_ERR`"]
pub struct FORCE_TUNING_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_TUNING_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_RESP_ERR`"]
pub type FORCE_RESP_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_RESP_ERR`"]
pub struct FORCE_RESP_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_RESP_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_BOOT_ACK_ERR`"]
pub type FORCE_BOOT_ACK_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_BOOT_ACK_ERR`"]
pub struct FORCE_BOOT_ACK_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_BOOT_ACK_ERR_W<'a> {
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
#[doc = "Reader of field `FORCE_VENDOR_ERR1`"]
pub type FORCE_VENDOR_ERR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_VENDOR_ERR1`"]
pub struct FORCE_VENDOR_ERR1_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_VENDOR_ERR1_W<'a> {
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
#[doc = "Reader of field `FORCE_VENDOR_ERR2`"]
pub type FORCE_VENDOR_ERR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_VENDOR_ERR2`"]
pub struct FORCE_VENDOR_ERR2_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_VENDOR_ERR2_W<'a> {
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
#[doc = "Reader of field `FORCE_VENDOR_ERR3`"]
pub type FORCE_VENDOR_ERR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_VENDOR_ERR3`"]
pub struct FORCE_VENDOR_ERR3_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_VENDOR_ERR3_W<'a> {
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
    #[doc = "Bit 0 - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_tout_err(&self) -> FORCE_CMD_TOUT_ERR_R {
        FORCE_CMD_TOUT_ERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_crc_err(&self) -> FORCE_CMD_CRC_ERR_R {
        FORCE_CMD_CRC_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_end_bit_err(&self) -> FORCE_CMD_END_BIT_ERR_R {
        FORCE_CMD_END_BIT_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_idx_err(&self) -> FORCE_CMD_IDX_ERR_R {
        FORCE_CMD_IDX_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_data_tout_err(&self) -> FORCE_DATA_TOUT_ERR_R {
        FORCE_DATA_TOUT_ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
    #[inline(always)]
    pub fn force_data_crc_err(&self) -> FORCE_DATA_CRC_ERR_R {
        FORCE_DATA_CRC_ERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_data_end_bit_err(&self) -> FORCE_DATA_END_BIT_ERR_R {
        FORCE_DATA_END_BIT_ERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
    #[inline(always)]
    pub fn force_cur_lmt_err(&self) -> FORCE_CUR_LMT_ERR_R {
        FORCE_CUR_LMT_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
    #[inline(always)]
    pub fn force_auto_cmd_err(&self) -> FORCE_AUTO_CMD_ERR_R {
        FORCE_AUTO_CMD_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
    #[inline(always)]
    pub fn force_adma_err(&self) -> FORCE_ADMA_ERR_R {
        FORCE_ADMA_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
    #[inline(always)]
    pub fn force_tuning_err(&self) -> FORCE_TUNING_ERR_R {
        FORCE_TUNING_ERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
    #[inline(always)]
    pub fn force_resp_err(&self) -> FORCE_RESP_ERR_R {
        FORCE_RESP_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
    #[inline(always)]
    pub fn force_boot_ack_err(&self) -> FORCE_BOOT_ACK_ERR_R {
        FORCE_BOOT_ACK_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err1(&self) -> FORCE_VENDOR_ERR1_R {
        FORCE_VENDOR_ERR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err2(&self) -> FORCE_VENDOR_ERR2_R {
        FORCE_VENDOR_ERR2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err3(&self) -> FORCE_VENDOR_ERR3_R {
        FORCE_VENDOR_ERR3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_tout_err(&mut self) -> FORCE_CMD_TOUT_ERR_W {
        FORCE_CMD_TOUT_ERR_W { w: self }
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_crc_err(&mut self) -> FORCE_CMD_CRC_ERR_W {
        FORCE_CMD_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_end_bit_err(&mut self) -> FORCE_CMD_END_BIT_ERR_W {
        FORCE_CMD_END_BIT_ERR_W { w: self }
    }
    #[doc = "Bit 3 - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_idx_err(&mut self) -> FORCE_CMD_IDX_ERR_W {
        FORCE_CMD_IDX_ERR_W { w: self }
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_data_tout_err(&mut self) -> FORCE_DATA_TOUT_ERR_W {
        FORCE_DATA_TOUT_ERR_W { w: self }
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
    #[inline(always)]
    pub fn force_data_crc_err(&mut self) -> FORCE_DATA_CRC_ERR_W {
        FORCE_DATA_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_data_end_bit_err(&mut self) -> FORCE_DATA_END_BIT_ERR_W {
        FORCE_DATA_END_BIT_ERR_W { w: self }
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
    #[inline(always)]
    pub fn force_cur_lmt_err(&mut self) -> FORCE_CUR_LMT_ERR_W {
        FORCE_CUR_LMT_ERR_W { w: self }
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
    #[inline(always)]
    pub fn force_auto_cmd_err(&mut self) -> FORCE_AUTO_CMD_ERR_W {
        FORCE_AUTO_CMD_ERR_W { w: self }
    }
    #[doc = "Bit 9 - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
    #[inline(always)]
    pub fn force_adma_err(&mut self) -> FORCE_ADMA_ERR_W {
        FORCE_ADMA_ERR_W { w: self }
    }
    #[doc = "Bit 10 - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
    #[inline(always)]
    pub fn force_tuning_err(&mut self) -> FORCE_TUNING_ERR_W {
        FORCE_TUNING_ERR_W { w: self }
    }
    #[doc = "Bit 11 - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
    #[inline(always)]
    pub fn force_resp_err(&mut self) -> FORCE_RESP_ERR_W {
        FORCE_RESP_ERR_W { w: self }
    }
    #[doc = "Bit 12 - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
    #[inline(always)]
    pub fn force_boot_ack_err(&mut self) -> FORCE_BOOT_ACK_ERR_W {
        FORCE_BOOT_ACK_ERR_W { w: self }
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err1(&mut self) -> FORCE_VENDOR_ERR1_W {
        FORCE_VENDOR_ERR1_W { w: self }
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err2(&mut self) -> FORCE_VENDOR_ERR2_W {
        FORCE_VENDOR_ERR2_W { w: self }
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err3(&mut self) -> FORCE_VENDOR_ERR3_W {
        FORCE_VENDOR_ERR3_W { w: self }
    }
}
