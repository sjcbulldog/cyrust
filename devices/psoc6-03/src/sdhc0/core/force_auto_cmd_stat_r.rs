#[doc = "Writer for register FORCE_AUTO_CMD_STAT_R"]
pub type W = crate::W<u16, super::FORCE_AUTO_CMD_STAT_R>;
#[doc = "Register FORCE_AUTO_CMD_STAT_R `reset()`'s with value 0"]
impl crate::ResetValue for super::FORCE_AUTO_CMD_STAT_R {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FORCE_AUTO_CMD12_NOT_EXEC`"]
pub struct FORCE_AUTO_CMD12_NOT_EXEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_AUTO_CMD12_NOT_EXEC_W<'a> {
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
#[doc = "Write proxy for field `FORCE_AUTO_CMD_TOUT_ERR`"]
pub struct FORCE_AUTO_CMD_TOUT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_AUTO_CMD_TOUT_ERR_W<'a> {
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
#[doc = "Write proxy for field `FORCE_AUTO_CMD_CRC_ERR`"]
pub struct FORCE_AUTO_CMD_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_AUTO_CMD_CRC_ERR_W<'a> {
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
#[doc = "Write proxy for field `FORCE_AUTO_CMD_EBIT_ERR`"]
pub struct FORCE_AUTO_CMD_EBIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_AUTO_CMD_EBIT_ERR_W<'a> {
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
#[doc = "Write proxy for field `FORCE_AUTO_CMD_IDX_ERR`"]
pub struct FORCE_AUTO_CMD_IDX_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_AUTO_CMD_IDX_ERR_W<'a> {
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
#[doc = "Write proxy for field `FORCE_AUTO_CMD_RESP_ERR`"]
pub struct FORCE_AUTO_CMD_RESP_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_AUTO_CMD_RESP_ERR_W<'a> {
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
#[doc = "Write proxy for field `FORCE_CMD_NOT_ISSUED_AUTO_CMD12`"]
pub struct FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 Not Executed Values: - 0x1 (TRUE): Auto CMD12 Not Executed Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd12_not_exec(&mut self) -> FORCE_AUTO_CMD12_NOT_EXEC_W {
        FORCE_AUTO_CMD12_NOT_EXEC_W { w: self }
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error Values: - 0x1 (TRUE): Auto CMD Timeout Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_tout_err(&mut self) -> FORCE_AUTO_CMD_TOUT_ERR_W {
        FORCE_AUTO_CMD_TOUT_ERR_W { w: self }
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error Values: - 0x1 (TRUE): Auto CMD CRC Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_crc_err(&mut self) -> FORCE_AUTO_CMD_CRC_ERR_W {
        FORCE_AUTO_CMD_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error Values: - 0x1 (TRUE): Auto CMD End Bit Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_ebit_err(&mut self) -> FORCE_AUTO_CMD_EBIT_ERR_W {
        FORCE_AUTO_CMD_EBIT_ERR_W { w: self }
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error Values: - 0x1 (TRUE): Auto CMD Index Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_idx_err(&mut self) -> FORCE_AUTO_CMD_IDX_ERR_W {
        FORCE_AUTO_CMD_IDX_ERR_W { w: self }
    }
    #[doc = "Bit 5 - Force Event for Auto CMD Response Error Values: - 0x1 (TRUE): Auto CMD Response Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_resp_err(&mut self) -> FORCE_AUTO_CMD_RESP_ERR_W {
        FORCE_AUTO_CMD_RESP_ERR_W { w: self }
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error Values: - 0x1 (TRUE): Command Not Issued By Auto CMD12 Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_cmd_not_issued_auto_cmd12(&mut self) -> FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W {
        FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W { w: self }
    }
}
