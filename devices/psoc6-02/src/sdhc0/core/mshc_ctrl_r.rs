#[doc = "Reader of register MSHC_CTRL_R"]
pub type R = crate::R<u8, super::MSHC_CTRL_R>;
#[doc = "Writer for register MSHC_CTRL_R"]
pub type W = crate::W<u8, super::MSHC_CTRL_R>;
#[doc = "Register MSHC_CTRL_R `reset()`'s with value 0x01"]
impl crate::ResetValue for super::MSHC_CTRL_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
#[doc = "Reader of field `CMD_CONFLICT_CHECK`"]
pub type CMD_CONFLICT_CHECK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_CONFLICT_CHECK`"]
pub struct CMD_CONFLICT_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CONFLICT_CHECK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SW_CG_DIS`"]
pub type SW_CG_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_CG_DIS`"]
pub struct SW_CG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CG_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
    #[inline(always)]
    pub fn cmd_conflict_check(&self) -> CMD_CONFLICT_CHECK_R {
        CMD_CONFLICT_CHECK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
    #[inline(always)]
    pub fn sw_cg_dis(&self) -> SW_CG_DIS_R {
        SW_CG_DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
    #[inline(always)]
    pub fn cmd_conflict_check(&mut self) -> CMD_CONFLICT_CHECK_W {
        CMD_CONFLICT_CHECK_W { w: self }
    }
    #[doc = "Bit 4 - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
    #[inline(always)]
    pub fn sw_cg_dis(&mut self) -> SW_CG_DIS_W {
        SW_CG_DIS_W { w: self }
    }
}
