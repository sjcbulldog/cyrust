#[doc = "Reader of register SAR_TR_OUT_CTRL"]
pub type R = crate::R<u32, super::SAR_TR_OUT_CTRL>;
#[doc = "Writer for register SAR_TR_OUT_CTRL"]
pub type W = crate::W<u32, super::SAR_TR_OUT_CTRL>;
#[doc = "Register SAR_TR_OUT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_TR_OUT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAR0 Trigger Out Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAR0_TR_OUT_SEL_A {
    #[doc = "0: sar output trigger is set by SAR.SAMPLE_CTRL.EOS_DSI_OUT_EN condition"]
    LEGACY = 0,
    #[doc = "1: sar output trigger is set by FIFO.CTRL.FIFO_LEVEL condition"]
    BUFFER_TRIGGER_LEVEL = 1,
}
impl From<SAR0_TR_OUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SAR0_TR_OUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAR0_TR_OUT_SEL`"]
pub type SAR0_TR_OUT_SEL_R = crate::R<bool, SAR0_TR_OUT_SEL_A>;
impl SAR0_TR_OUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAR0_TR_OUT_SEL_A {
        match self.bits {
            false => SAR0_TR_OUT_SEL_A::LEGACY,
            true => SAR0_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == SAR0_TR_OUT_SEL_A::LEGACY
    }
    #[doc = "Checks if the value of the field is `BUFFER_TRIGGER_LEVEL`"]
    #[inline(always)]
    pub fn is_buffer_trigger_level(&self) -> bool {
        *self == SAR0_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL
    }
}
#[doc = "Write proxy for field `SAR0_TR_OUT_SEL`"]
pub struct SAR0_TR_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR0_TR_OUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAR0_TR_OUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "sar output trigger is set by SAR.SAMPLE_CTRL.EOS_DSI_OUT_EN condition"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(SAR0_TR_OUT_SEL_A::LEGACY)
    }
    #[doc = "sar output trigger is set by FIFO.CTRL.FIFO_LEVEL condition"]
    #[inline(always)]
    pub fn buffer_trigger_level(self) -> &'a mut W {
        self.variant(SAR0_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "SAR1 Trigger Out Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAR1_TR_OUT_SEL_A {
    #[doc = "0: sar output trigger is set by SAR.SAMPLE_CTRL.EOS_DSI_OUT_EN condition"]
    LEGACY = 0,
    #[doc = "1: sar output trigger is set by FIFO.CTRL.FIFO_LEVEL condition"]
    BUFFER_TRIGGER_LEVEL = 1,
}
impl From<SAR1_TR_OUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SAR1_TR_OUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAR1_TR_OUT_SEL`"]
pub type SAR1_TR_OUT_SEL_R = crate::R<bool, SAR1_TR_OUT_SEL_A>;
impl SAR1_TR_OUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAR1_TR_OUT_SEL_A {
        match self.bits {
            false => SAR1_TR_OUT_SEL_A::LEGACY,
            true => SAR1_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == SAR1_TR_OUT_SEL_A::LEGACY
    }
    #[doc = "Checks if the value of the field is `BUFFER_TRIGGER_LEVEL`"]
    #[inline(always)]
    pub fn is_buffer_trigger_level(&self) -> bool {
        *self == SAR1_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL
    }
}
#[doc = "Write proxy for field `SAR1_TR_OUT_SEL`"]
pub struct SAR1_TR_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_TR_OUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAR1_TR_OUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "sar output trigger is set by SAR.SAMPLE_CTRL.EOS_DSI_OUT_EN condition"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(SAR1_TR_OUT_SEL_A::LEGACY)
    }
    #[doc = "sar output trigger is set by FIFO.CTRL.FIFO_LEVEL condition"]
    #[inline(always)]
    pub fn buffer_trigger_level(self) -> &'a mut W {
        self.variant(SAR1_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "SAR2 Trigger Out Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAR2_TR_OUT_SEL_A {
    #[doc = "0: sar output trigger is set by SAR.SAMPLE_CTRL.EOS_DSI_OUT_EN condition"]
    LEGACY = 0,
    #[doc = "1: sar output trigger is set by FIFO.CTRL.FIFO_LEVEL condition"]
    BUFFER_TRIGGER_LEVEL = 1,
}
impl From<SAR2_TR_OUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SAR2_TR_OUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAR2_TR_OUT_SEL`"]
pub type SAR2_TR_OUT_SEL_R = crate::R<bool, SAR2_TR_OUT_SEL_A>;
impl SAR2_TR_OUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAR2_TR_OUT_SEL_A {
        match self.bits {
            false => SAR2_TR_OUT_SEL_A::LEGACY,
            true => SAR2_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == SAR2_TR_OUT_SEL_A::LEGACY
    }
    #[doc = "Checks if the value of the field is `BUFFER_TRIGGER_LEVEL`"]
    #[inline(always)]
    pub fn is_buffer_trigger_level(&self) -> bool {
        *self == SAR2_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL
    }
}
#[doc = "Write proxy for field `SAR2_TR_OUT_SEL`"]
pub struct SAR2_TR_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_TR_OUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAR2_TR_OUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "sar output trigger is set by SAR.SAMPLE_CTRL.EOS_DSI_OUT_EN condition"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(SAR2_TR_OUT_SEL_A::LEGACY)
    }
    #[doc = "sar output trigger is set by FIFO.CTRL.FIFO_LEVEL condition"]
    #[inline(always)]
    pub fn buffer_trigger_level(self) -> &'a mut W {
        self.variant(SAR2_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "SAR3 Trigger Out Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAR3_TR_OUT_SEL_A {
    #[doc = "0: sar output trigger is set by SAR.SAMPLE_CTRL.EOS_DSI_OUT_EN condition"]
    LEGACY = 0,
    #[doc = "1: sar output trigger is set by FIFO.CTRL.FIFO_LEVEL condition"]
    BUFFER_TRIGGER_LEVEL = 1,
}
impl From<SAR3_TR_OUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SAR3_TR_OUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAR3_TR_OUT_SEL`"]
pub type SAR3_TR_OUT_SEL_R = crate::R<bool, SAR3_TR_OUT_SEL_A>;
impl SAR3_TR_OUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAR3_TR_OUT_SEL_A {
        match self.bits {
            false => SAR3_TR_OUT_SEL_A::LEGACY,
            true => SAR3_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == SAR3_TR_OUT_SEL_A::LEGACY
    }
    #[doc = "Checks if the value of the field is `BUFFER_TRIGGER_LEVEL`"]
    #[inline(always)]
    pub fn is_buffer_trigger_level(&self) -> bool {
        *self == SAR3_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL
    }
}
#[doc = "Write proxy for field `SAR3_TR_OUT_SEL`"]
pub struct SAR3_TR_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR3_TR_OUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAR3_TR_OUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "sar output trigger is set by SAR.SAMPLE_CTRL.EOS_DSI_OUT_EN condition"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(SAR3_TR_OUT_SEL_A::LEGACY)
    }
    #[doc = "sar output trigger is set by FIFO.CTRL.FIFO_LEVEL condition"]
    #[inline(always)]
    pub fn buffer_trigger_level(self) -> &'a mut W {
        self.variant(SAR3_TR_OUT_SEL_A::BUFFER_TRIGGER_LEVEL)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SAR0 Trigger Out Source Select"]
    #[inline(always)]
    pub fn sar0_tr_out_sel(&self) -> SAR0_TR_OUT_SEL_R {
        SAR0_TR_OUT_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SAR1 Trigger Out Source Select"]
    #[inline(always)]
    pub fn sar1_tr_out_sel(&self) -> SAR1_TR_OUT_SEL_R {
        SAR1_TR_OUT_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SAR2 Trigger Out Source Select"]
    #[inline(always)]
    pub fn sar2_tr_out_sel(&self) -> SAR2_TR_OUT_SEL_R {
        SAR2_TR_OUT_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SAR3 Trigger Out Source Select"]
    #[inline(always)]
    pub fn sar3_tr_out_sel(&self) -> SAR3_TR_OUT_SEL_R {
        SAR3_TR_OUT_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAR0 Trigger Out Source Select"]
    #[inline(always)]
    pub fn sar0_tr_out_sel(&mut self) -> SAR0_TR_OUT_SEL_W {
        SAR0_TR_OUT_SEL_W { w: self }
    }
    #[doc = "Bit 1 - SAR1 Trigger Out Source Select"]
    #[inline(always)]
    pub fn sar1_tr_out_sel(&mut self) -> SAR1_TR_OUT_SEL_W {
        SAR1_TR_OUT_SEL_W { w: self }
    }
    #[doc = "Bit 2 - SAR2 Trigger Out Source Select"]
    #[inline(always)]
    pub fn sar2_tr_out_sel(&mut self) -> SAR2_TR_OUT_SEL_W {
        SAR2_TR_OUT_SEL_W { w: self }
    }
    #[doc = "Bit 3 - SAR3 Trigger Out Source Select"]
    #[inline(always)]
    pub fn sar3_tr_out_sel(&mut self) -> SAR3_TR_OUT_SEL_W {
        SAR3_TR_OUT_SEL_W { w: self }
    }
}
