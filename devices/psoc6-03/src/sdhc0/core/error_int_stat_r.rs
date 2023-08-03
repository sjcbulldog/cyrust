#[doc = "Reader of register ERROR_INT_STAT_R"]
pub type R = crate::R<u16, super::ERROR_INT_STAT_R>;
#[doc = "Writer for register ERROR_INT_STAT_R"]
pub type W = crate::W<u16, super::ERROR_INT_STAT_R>;
#[doc = "Register ERROR_INT_STAT_R `reset()`'s with value 0"]
impl crate::ResetValue for super::ERROR_INT_STAT_R {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD_TOUT_ERR`"]
pub type CMD_TOUT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_TOUT_ERR`"]
pub struct CMD_TOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_TOUT_ERR_W<'a> {
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
#[doc = "Reader of field `CMD_CRC_ERR`"]
pub type CMD_CRC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_CRC_ERR`"]
pub struct CMD_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CRC_ERR_W<'a> {
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
#[doc = "Reader of field `CMD_END_BIT_ERR`"]
pub type CMD_END_BIT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_END_BIT_ERR`"]
pub struct CMD_END_BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_END_BIT_ERR_W<'a> {
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
#[doc = "Reader of field `CMD_IDX_ERR`"]
pub type CMD_IDX_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_IDX_ERR`"]
pub struct CMD_IDX_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_IDX_ERR_W<'a> {
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
#[doc = "Reader of field `DATA_TOUT_ERR`"]
pub type DATA_TOUT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_TOUT_ERR`"]
pub struct DATA_TOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TOUT_ERR_W<'a> {
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
#[doc = "Reader of field `DATA_CRC_ERR`"]
pub type DATA_CRC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_CRC_ERR`"]
pub struct DATA_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CRC_ERR_W<'a> {
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
#[doc = "Reader of field `DATA_END_BIT_ERR`"]
pub type DATA_END_BIT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_END_BIT_ERR`"]
pub struct DATA_END_BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_END_BIT_ERR_W<'a> {
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
#[doc = "Reader of field `CUR_LMT_ERR`"]
pub type CUR_LMT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CUR_LMT_ERR`"]
pub struct CUR_LMT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_LMT_ERR_W<'a> {
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
#[doc = "Reader of field `AUTO_CMD_ERR`"]
pub type AUTO_CMD_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_CMD_ERR`"]
pub struct AUTO_CMD_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CMD_ERR_W<'a> {
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
#[doc = "Reader of field `ADMA_ERR`"]
pub type ADMA_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADMA_ERR`"]
pub struct ADMA_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMA_ERR_W<'a> {
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
#[doc = "Reader of field `TUNING_ERR`"]
pub type TUNING_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNING_ERR`"]
pub struct TUNING_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_ERR_W<'a> {
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
#[doc = "Reader of field `RESP_ERR`"]
pub type RESP_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESP_ERR`"]
pub struct RESP_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_ERR_W<'a> {
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
#[doc = "Reader of field `BOOT_ACK_ERR`"]
pub type BOOT_ACK_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_ACK_ERR`"]
pub struct BOOT_ACK_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ACK_ERR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn cmd_tout_err(&self) -> CMD_TOUT_ERR_R {
        CMD_TOUT_ERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
    #[inline(always)]
    pub fn cmd_crc_err(&self) -> CMD_CRC_ERR_R {
        CMD_CRC_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&self) -> CMD_END_BIT_ERR_R {
        CMD_END_BIT_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err(&self) -> CMD_IDX_ERR_R {
        CMD_IDX_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn data_tout_err(&self) -> DATA_TOUT_ERR_R {
        DATA_TOUT_ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_crc_err(&self) -> DATA_CRC_ERR_R {
        DATA_CRC_ERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&self) -> DATA_END_BIT_ERR_R {
        DATA_END_BIT_ERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
    #[inline(always)]
    pub fn cur_lmt_err(&self) -> CUR_LMT_ERR_R {
        CUR_LMT_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn auto_cmd_err(&self) -> AUTO_CMD_ERR_R {
        AUTO_CMD_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn adma_err(&self) -> ADMA_ERR_R {
        ADMA_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tuning_err(&self) -> TUNING_ERR_R {
        TUNING_ERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn resp_err(&self) -> RESP_ERR_R {
        RESP_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn boot_ack_err(&self) -> BOOT_ACK_ERR_R {
        BOOT_ACK_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn cmd_tout_err(&mut self) -> CMD_TOUT_ERR_W {
        CMD_TOUT_ERR_W { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
    #[inline(always)]
    pub fn cmd_crc_err(&mut self) -> CMD_CRC_ERR_W {
        CMD_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&mut self) -> CMD_END_BIT_ERR_W {
        CMD_END_BIT_ERR_W { w: self }
    }
    #[doc = "Bit 3 - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err(&mut self) -> CMD_IDX_ERR_W {
        CMD_IDX_ERR_W { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn data_tout_err(&mut self) -> DATA_TOUT_ERR_W {
        DATA_TOUT_ERR_W { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_crc_err(&mut self) -> DATA_CRC_ERR_W {
        DATA_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&mut self) -> DATA_END_BIT_ERR_W {
        DATA_END_BIT_ERR_W { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
    #[inline(always)]
    pub fn cur_lmt_err(&mut self) -> CUR_LMT_ERR_W {
        CUR_LMT_ERR_W { w: self }
    }
    #[doc = "Bit 8 - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn auto_cmd_err(&mut self) -> AUTO_CMD_ERR_W {
        AUTO_CMD_ERR_W { w: self }
    }
    #[doc = "Bit 9 - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn adma_err(&mut self) -> ADMA_ERR_W {
        ADMA_ERR_W { w: self }
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tuning_err(&mut self) -> TUNING_ERR_W {
        TUNING_ERR_W { w: self }
    }
    #[doc = "Bit 11 - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn resp_err(&mut self) -> RESP_ERR_W {
        RESP_ERR_W { w: self }
    }
    #[doc = "Bit 12 - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn boot_ack_err(&mut self) -> BOOT_ACK_ERR_W {
        BOOT_ACK_ERR_W { w: self }
    }
}
