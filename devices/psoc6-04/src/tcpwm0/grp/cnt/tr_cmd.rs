#[doc = "Reader of register TR_CMD"]
pub type R = crate::R<u32, super::TR_CMD>;
#[doc = "Writer for register TR_CMD"]
pub type W = crate::W<u32, super::TR_CMD>;
#[doc = "Register TR_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::TR_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPTURE0`"]
pub type CAPTURE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTURE0`"]
pub struct CAPTURE0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE0_W<'a> {
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
#[doc = "Reader of field `RELOAD`"]
pub type RELOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RELOAD`"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
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
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CAPTURE1`"]
pub type CAPTURE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTURE1`"]
pub struct CAPTURE1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SW capture 0 trigger. When written with '1', a capture 0 trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn capture0(&self) -> CAPTURE0_R {
        CAPTURE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SW reload trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SW stop trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SW start trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SW capture 1 trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn capture1(&self) -> CAPTURE1_R {
        CAPTURE1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW capture 0 trigger. When written with '1', a capture 0 trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn capture0(&mut self) -> CAPTURE0_W {
        CAPTURE0_W { w: self }
    }
    #[doc = "Bit 2 - SW reload trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
    #[doc = "Bit 3 - SW stop trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 4 - SW start trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 5 - SW capture 1 trigger. For HW behavior, see COUNTER_CAPTURE0 field."]
    #[inline(always)]
    pub fn capture1(&mut self) -> CAPTURE1_W {
        CAPTURE1_W { w: self }
    }
}
