#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC`"]
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
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
#[doc = "Reader of field `CC0_MATCH`"]
pub type CC0_MATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC0_MATCH`"]
pub struct CC0_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CC0_MATCH_W<'a> {
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
#[doc = "Reader of field `CC1_MATCH`"]
pub type CC1_MATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC1_MATCH`"]
pub struct CC1_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1_MATCH_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter matches CC0 register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn cc0_match(&self) -> CC0_MATCH_R {
        CC0_MATCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter matches CC1 register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn cc1_match(&self) -> CC1_MATCH_R {
        CC1_MATCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    #[doc = "Bit 1 - Counter matches CC0 register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn cc0_match(&mut self) -> CC0_MATCH_W {
        CC0_MATCH_W { w: self }
    }
    #[doc = "Bit 2 - Counter matches CC1 register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn cc1_match(&mut self) -> CC1_MATCH_W {
        CC1_MATCH_W { w: self }
    }
}
