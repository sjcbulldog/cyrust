#[doc = "Reader of register CMD_R"]
pub type R = crate::R<u16, super::CMD_R>;
#[doc = "Writer for register CMD_R"]
pub type W = crate::W<u16, super::CMD_R>;
#[doc = "Register CMD_R `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD_R {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESP_TYPE_SELECT`"]
pub type RESP_TYPE_SELECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESP_TYPE_SELECT`"]
pub struct RESP_TYPE_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_TYPE_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SUB_CMD_FLAG`"]
pub type SUB_CMD_FLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUB_CMD_FLAG`"]
pub struct SUB_CMD_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUB_CMD_FLAG_W<'a> {
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
#[doc = "Reader of field `CMD_CRC_CHK_ENABLE`"]
pub type CMD_CRC_CHK_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_CRC_CHK_ENABLE`"]
pub struct CMD_CRC_CHK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CRC_CHK_ENABLE_W<'a> {
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
#[doc = "Reader of field `CMD_IDX_CHK_ENABLE`"]
pub type CMD_IDX_CHK_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_IDX_CHK_ENABLE`"]
pub struct CMD_IDX_CHK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_IDX_CHK_ENABLE_W<'a> {
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
#[doc = "Reader of field `DATA_PRESENT_SEL`"]
pub type DATA_PRESENT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_PRESENT_SEL`"]
pub struct DATA_PRESENT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_PRESENT_SEL_W<'a> {
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
#[doc = "Reader of field `CMD_TYPE`"]
pub type CMD_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD_TYPE`"]
pub struct CMD_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `CMD_INDEX`"]
pub type CMD_INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD_INDEX`"]
pub struct CMD_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
    #[inline(always)]
    pub fn resp_type_select(&self) -> RESP_TYPE_SELECT_R {
        RESP_TYPE_SELECT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
    #[inline(always)]
    pub fn sub_cmd_flag(&self) -> SUB_CMD_FLAG_R {
        SUB_CMD_FLAG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_crc_chk_enable(&self) -> CMD_CRC_CHK_ENABLE_R {
        CMD_CRC_CHK_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_idx_chk_enable(&self) -> CMD_IDX_CHK_ENABLE_R {
        CMD_IDX_CHK_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
    #[inline(always)]
    pub fn data_present_sel(&self) -> DATA_PRESENT_SEL_R {
        DATA_PRESENT_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CMD_TYPE_R {
        CMD_TYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
    #[inline(always)]
    pub fn resp_type_select(&mut self) -> RESP_TYPE_SELECT_W {
        RESP_TYPE_SELECT_W { w: self }
    }
    #[doc = "Bit 2 - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
    #[inline(always)]
    pub fn sub_cmd_flag(&mut self) -> SUB_CMD_FLAG_W {
        SUB_CMD_FLAG_W { w: self }
    }
    #[doc = "Bit 3 - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_crc_chk_enable(&mut self) -> CMD_CRC_CHK_ENABLE_W {
        CMD_CRC_CHK_ENABLE_W { w: self }
    }
    #[doc = "Bit 4 - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_idx_chk_enable(&mut self) -> CMD_IDX_CHK_ENABLE_W {
        CMD_IDX_CHK_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
    #[inline(always)]
    pub fn data_present_sel(&mut self) -> DATA_PRESENT_SEL_W {
        DATA_PRESENT_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
    #[inline(always)]
    pub fn cmd_type(&mut self) -> CMD_TYPE_W {
        CMD_TYPE_W { w: self }
    }
    #[doc = "Bits 8:13 - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    #[inline(always)]
    pub fn cmd_index(&mut self) -> CMD_INDEX_W {
        CMD_INDEX_W { w: self }
    }
}
