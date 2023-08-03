#[doc = "Reader of register MS_ATT2"]
pub type R = crate::R<u32, super::MS_ATT2>;
#[doc = "Writer for register MS_ATT2"]
pub type W = crate::W<u32, super::MS_ATT2>;
#[doc = "Register MS_ATT2 `reset()`'s with value 0x1f1f_1f1f"]
impl crate::ResetValue for super::MS_ATT2 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f1f_1f1f
    }
}
#[doc = "Reader of field `PC8_UR`"]
pub type PC8_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC8_UW`"]
pub type PC8_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC8_UW`"]
pub struct PC8_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC8_UW_W<'a> {
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
#[doc = "Reader of field `PC8_PR`"]
pub type PC8_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC8_PW`"]
pub type PC8_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC8_PW`"]
pub struct PC8_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC8_PW_W<'a> {
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
#[doc = "Reader of field `PC8_NS`"]
pub type PC8_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC8_NS`"]
pub struct PC8_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC8_NS_W<'a> {
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
#[doc = "Reader of field `PC9_UR`"]
pub type PC9_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC9_UW`"]
pub type PC9_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC9_UW`"]
pub struct PC9_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC9_UW_W<'a> {
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
#[doc = "Reader of field `PC9_PR`"]
pub type PC9_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC9_PW`"]
pub type PC9_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC9_PW`"]
pub struct PC9_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC9_PW_W<'a> {
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
#[doc = "Reader of field `PC9_NS`"]
pub type PC9_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC9_NS`"]
pub struct PC9_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC9_NS_W<'a> {
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
#[doc = "Reader of field `PC10_UR`"]
pub type PC10_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC10_UW`"]
pub type PC10_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC10_UW`"]
pub struct PC10_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC10_UW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PC10_PR`"]
pub type PC10_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC10_PW`"]
pub type PC10_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC10_PW`"]
pub struct PC10_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC10_PW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `PC10_NS`"]
pub type PC10_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC10_NS`"]
pub struct PC10_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC10_NS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PC11_UR`"]
pub type PC11_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC11_UW`"]
pub type PC11_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC11_UW`"]
pub struct PC11_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC11_UW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PC11_PR`"]
pub type PC11_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC11_PW`"]
pub type PC11_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC11_PW`"]
pub struct PC11_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC11_PW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PC11_NS`"]
pub type PC11_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC11_NS`"]
pub struct PC11_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC11_NS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Protection context 8, user read enable."]
    #[inline(always)]
    pub fn pc8_ur(&self) -> PC8_UR_R {
        PC8_UR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protection context 8, user write enable."]
    #[inline(always)]
    pub fn pc8_uw(&self) -> PC8_UW_R {
        PC8_UW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protection context 8, privileged read enable."]
    #[inline(always)]
    pub fn pc8_pr(&self) -> PC8_PR_R {
        PC8_PR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Protection context 8, privileged write enable."]
    #[inline(always)]
    pub fn pc8_pw(&self) -> PC8_PW_R {
        PC8_PW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Protection context 8, non-secure."]
    #[inline(always)]
    pub fn pc8_ns(&self) -> PC8_NS_R {
        PC8_NS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Protection context 9, user read enable."]
    #[inline(always)]
    pub fn pc9_ur(&self) -> PC9_UR_R {
        PC9_UR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protection context 9, user write enable."]
    #[inline(always)]
    pub fn pc9_uw(&self) -> PC9_UW_R {
        PC9_UW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Protection context 9, privileged read enable."]
    #[inline(always)]
    pub fn pc9_pr(&self) -> PC9_PR_R {
        PC9_PR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Protection context 9, privileged write enable."]
    #[inline(always)]
    pub fn pc9_pw(&self) -> PC9_PW_R {
        PC9_PW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protection context 9, non-secure."]
    #[inline(always)]
    pub fn pc9_ns(&self) -> PC9_NS_R {
        PC9_NS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Protection context 10, user read enable."]
    #[inline(always)]
    pub fn pc10_ur(&self) -> PC10_UR_R {
        PC10_UR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Protection context 10, user write enable."]
    #[inline(always)]
    pub fn pc10_uw(&self) -> PC10_UW_R {
        PC10_UW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Protection context 10, privileged read enable."]
    #[inline(always)]
    pub fn pc10_pr(&self) -> PC10_PR_R {
        PC10_PR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Protection context 10, privileged write enable."]
    #[inline(always)]
    pub fn pc10_pw(&self) -> PC10_PW_R {
        PC10_PW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Protection context 10, non-secure."]
    #[inline(always)]
    pub fn pc10_ns(&self) -> PC10_NS_R {
        PC10_NS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Protection context 11, user read enable."]
    #[inline(always)]
    pub fn pc11_ur(&self) -> PC11_UR_R {
        PC11_UR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Protection context 11, user write enable."]
    #[inline(always)]
    pub fn pc11_uw(&self) -> PC11_UW_R {
        PC11_UW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Protection context 11, privileged read enable."]
    #[inline(always)]
    pub fn pc11_pr(&self) -> PC11_PR_R {
        PC11_PR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protection context 11, privileged write enable."]
    #[inline(always)]
    pub fn pc11_pw(&self) -> PC11_PW_R {
        PC11_PW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protection context 11, non-secure."]
    #[inline(always)]
    pub fn pc11_ns(&self) -> PC11_NS_R {
        PC11_NS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Protection context 8, user write enable."]
    #[inline(always)]
    pub fn pc8_uw(&mut self) -> PC8_UW_W {
        PC8_UW_W { w: self }
    }
    #[doc = "Bit 3 - Protection context 8, privileged write enable."]
    #[inline(always)]
    pub fn pc8_pw(&mut self) -> PC8_PW_W {
        PC8_PW_W { w: self }
    }
    #[doc = "Bit 4 - Protection context 8, non-secure."]
    #[inline(always)]
    pub fn pc8_ns(&mut self) -> PC8_NS_W {
        PC8_NS_W { w: self }
    }
    #[doc = "Bit 9 - Protection context 9, user write enable."]
    #[inline(always)]
    pub fn pc9_uw(&mut self) -> PC9_UW_W {
        PC9_UW_W { w: self }
    }
    #[doc = "Bit 11 - Protection context 9, privileged write enable."]
    #[inline(always)]
    pub fn pc9_pw(&mut self) -> PC9_PW_W {
        PC9_PW_W { w: self }
    }
    #[doc = "Bit 12 - Protection context 9, non-secure."]
    #[inline(always)]
    pub fn pc9_ns(&mut self) -> PC9_NS_W {
        PC9_NS_W { w: self }
    }
    #[doc = "Bit 17 - Protection context 10, user write enable."]
    #[inline(always)]
    pub fn pc10_uw(&mut self) -> PC10_UW_W {
        PC10_UW_W { w: self }
    }
    #[doc = "Bit 19 - Protection context 10, privileged write enable."]
    #[inline(always)]
    pub fn pc10_pw(&mut self) -> PC10_PW_W {
        PC10_PW_W { w: self }
    }
    #[doc = "Bit 20 - Protection context 10, non-secure."]
    #[inline(always)]
    pub fn pc10_ns(&mut self) -> PC10_NS_W {
        PC10_NS_W { w: self }
    }
    #[doc = "Bit 25 - Protection context 11, user write enable."]
    #[inline(always)]
    pub fn pc11_uw(&mut self) -> PC11_UW_W {
        PC11_UW_W { w: self }
    }
    #[doc = "Bit 27 - Protection context 11, privileged write enable."]
    #[inline(always)]
    pub fn pc11_pw(&mut self) -> PC11_PW_W {
        PC11_PW_W { w: self }
    }
    #[doc = "Bit 28 - Protection context 11, non-secure."]
    #[inline(always)]
    pub fn pc11_ns(&mut self) -> PC11_NS_W {
        PC11_NS_W { w: self }
    }
}
