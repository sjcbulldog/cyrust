#[doc = "Reader of register CTB_SW_SQ_CTRL"]
pub type R = crate::R<u32, super::CTB_SW_SQ_CTRL>;
#[doc = "Writer for register CTB_SW_SQ_CTRL"]
pub type W = crate::W<u32, super::CTB_SW_SQ_CTRL>;
#[doc = "Register CTB_SW_SQ_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTB_SW_SQ_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2_SQ_CTRL23`"]
pub type P2_SQ_CTRL23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_SQ_CTRL23`"]
pub struct P2_SQ_CTRL23_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_SQ_CTRL23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `P3_SQ_CTRL23`"]
pub type P3_SQ_CTRL23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3_SQ_CTRL23`"]
pub struct P3_SQ_CTRL23_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_SQ_CTRL23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - for D51"]
    #[inline(always)]
    pub fn p2_sq_ctrl23(&self) -> P2_SQ_CTRL23_R {
        P2_SQ_CTRL23_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - for D52, D62"]
    #[inline(always)]
    pub fn p3_sq_ctrl23(&self) -> P3_SQ_CTRL23_R {
        P3_SQ_CTRL23_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - for D51"]
    #[inline(always)]
    pub fn p2_sq_ctrl23(&mut self) -> P2_SQ_CTRL23_W {
        P2_SQ_CTRL23_W { w: self }
    }
    #[doc = "Bit 11 - for D52, D62"]
    #[inline(always)]
    pub fn p3_sq_ctrl23(&mut self) -> P3_SQ_CTRL23_W {
        P3_SQ_CTRL23_W { w: self }
    }
}
