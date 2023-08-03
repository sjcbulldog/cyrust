#[doc = "Reader of register SW_RST_R"]
pub type R = crate::R<u8, super::SW_RST_R>;
#[doc = "Writer for register SW_RST_R"]
pub type W = crate::W<u8, super::SW_RST_R>;
#[doc = "Register SW_RST_R `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_RST_R {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_RST_ALL`"]
pub type SW_RST_ALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_RST_ALL`"]
pub struct SW_RST_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_ALL_W<'a> {
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
#[doc = "Reader of field `SW_RST_CMD`"]
pub type SW_RST_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_RST_CMD`"]
pub struct SW_RST_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_CMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SW_RST_DAT`"]
pub type SW_RST_DAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_RST_DAT`"]
pub struct SW_RST_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_DAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SW_RST_ALL_R {
        SW_RST_ALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_cmd(&self) -> SW_RST_CMD_R {
        SW_RST_CMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_dat(&self) -> SW_RST_DAT_R {
        SW_RST_DAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_all(&mut self) -> SW_RST_ALL_W {
        SW_RST_ALL_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_cmd(&mut self) -> SW_RST_CMD_W {
        SW_RST_CMD_W { w: self }
    }
    #[doc = "Bit 2 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_dat(&mut self) -> SW_RST_DAT_W {
        SW_RST_DAT_W { w: self }
    }
}
