#[doc = "Reader of register TR_CTL[%s]"]
pub type R = crate::R<u32, super::TR_CTL>;
#[doc = "Writer for register TR_CTL[%s]"]
pub type W = crate::W<u32, super::TR_CTL>;
#[doc = "Register TR_CTL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TR_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TR_SEL`"]
pub type TR_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR_SEL`"]
pub struct TR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_SEL_W<'a> {
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
#[doc = "Reader of field `TR_INV`"]
pub type TR_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR_INV`"]
pub struct TR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DBG_FREEZE_EN`"]
pub type DBG_FREEZE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_FREEZE_EN`"]
pub struct DBG_FREEZE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_FREEZE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Specifies input trigger: '0'': constant signal level '0'. '1': input trigger."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TR_SEL_R {
        TR_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    pub fn tr_inv(&self) -> TR_INV_R {
        TR_INV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    pub fn tr_edge(&self) -> TR_EDGE_R {
        TR_EDGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DBG_FREEZE_EN_R {
        DBG_FREEZE_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies input trigger: '0'': constant signal level '0'. '1': input trigger."]
    #[inline(always)]
    pub fn tr_sel(&mut self) -> TR_SEL_W {
        TR_SEL_W { w: self }
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    pub fn tr_inv(&mut self) -> TR_INV_W {
        TR_INV_W { w: self }
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    pub fn tr_edge(&mut self) -> TR_EDGE_W {
        TR_EDGE_W { w: self }
    }
    #[doc = "Bit 12 - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    pub fn dbg_freeze_en(&mut self) -> DBG_FREEZE_EN_W {
        DBG_FREEZE_EN_W { w: self }
    }
}
