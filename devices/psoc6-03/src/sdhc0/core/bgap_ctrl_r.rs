#[doc = "Reader of register BGAP_CTRL_R"]
pub type R = crate::R<u8, super::BGAP_CTRL_R>;
#[doc = "Writer for register BGAP_CTRL_R"]
pub type W = crate::W<u8, super::BGAP_CTRL_R>;
#[doc = "Register BGAP_CTRL_R `reset()`'s with value 0"]
impl crate::ResetValue for super::BGAP_CTRL_R {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STOP_BG_REQ`"]
pub type STOP_BG_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_BG_REQ`"]
pub struct STOP_BG_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_BG_REQ_W<'a> {
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
#[doc = "Reader of field `CONTINUE_REQ`"]
pub type CONTINUE_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONTINUE_REQ`"]
pub struct CONTINUE_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTINUE_REQ_W<'a> {
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
#[doc = "Reader of field `RD_WAIT_CTRL`"]
pub type RD_WAIT_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_WAIT_CTRL`"]
pub struct RD_WAIT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_WAIT_CTRL_W<'a> {
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
#[doc = "Reader of field `INT_AT_BGAP`"]
pub type INT_AT_BGAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_AT_BGAP`"]
pub struct INT_AT_BGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_AT_BGAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
    #[inline(always)]
    pub fn stop_bg_req(&self) -> STOP_BG_REQ_R {
        STOP_BG_REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
    #[inline(always)]
    pub fn continue_req(&self) -> CONTINUE_REQ_R {
        CONTINUE_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rd_wait_ctrl(&self) -> RD_WAIT_CTRL_R {
        RD_WAIT_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
    #[inline(always)]
    pub fn int_at_bgap(&self) -> INT_AT_BGAP_R {
        INT_AT_BGAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
    #[inline(always)]
    pub fn stop_bg_req(&mut self) -> STOP_BG_REQ_W {
        STOP_BG_REQ_W { w: self }
    }
    #[doc = "Bit 1 - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
    #[inline(always)]
    pub fn continue_req(&mut self) -> CONTINUE_REQ_W {
        CONTINUE_REQ_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rd_wait_ctrl(&mut self) -> RD_WAIT_CTRL_W {
        RD_WAIT_CTRL_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
    #[inline(always)]
    pub fn int_at_bgap(&mut self) -> INT_AT_BGAP_W {
        INT_AT_BGAP_W { w: self }
    }
}
