#[doc = "Reader of register TR_CMD"]
pub type R = crate::R<u32, super::TR_CMD>;
#[doc = "Writer for register TR_CMD"]
pub type W = crate::W<u32, super::TR_CMD>;
#[doc = "Register TR_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::TR_CMD {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `TR_SEL`"]
pub type TR_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TR_SEL`"]
pub struct TR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `GROUP_SEL`"]
pub type GROUP_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GROUP_SEL`"]
pub struct GROUP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TR_EDGE`"]
pub type TR_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR_EDGE`"]
pub struct TR_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `OUT_SEL`"]
pub type OUT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_SEL`"]
pub struct OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ACTIVATE`"]
pub type ACTIVATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVATE`"]
pub struct ACTIVATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TR_SEL_R {
        TR_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn group_sel(&self) -> GROUP_SEL_R {
        GROUP_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
    #[inline(always)]
    pub fn tr_edge(&self) -> TR_EDGE_R {
        TR_EDGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields."]
    #[inline(always)]
    pub fn activate(&self) -> ACTIVATE_R {
        ACTIVATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn tr_sel(&mut self) -> TR_SEL_W {
        TR_SEL_W { w: self }
    }
    #[doc = "Bits 8:12 - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn group_sel(&mut self) -> GROUP_SEL_W {
        GROUP_SEL_W { w: self }
    }
    #[doc = "Bit 29 - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
    #[inline(always)]
    pub fn tr_edge(&mut self) -> TR_EDGE_W {
        TR_EDGE_W { w: self }
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OUT_SEL_W {
        OUT_SEL_W { w: self }
    }
    #[doc = "Bit 31 - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields."]
    #[inline(always)]
    pub fn activate(&mut self) -> ACTIVATE_W {
        ACTIVATE_W { w: self }
    }
}
