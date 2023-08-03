#[doc = "Reader of register MS_ATT1"]
pub type R = crate::R<u32, super::MS_ATT1>;
#[doc = "Writer for register MS_ATT1"]
pub type W = crate::W<u32, super::MS_ATT1>;
#[doc = "Register MS_ATT1 `reset()`'s with value 0x1f1f_1f1f"]
impl crate::ResetValue for super::MS_ATT1 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f1f_1f1f
    }
}
#[doc = "Reader of field `PC4_UR`"]
pub type PC4_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC4_UW`"]
pub type PC4_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC4_UW`"]
pub struct PC4_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC4_UW_W<'a> {
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
#[doc = "Reader of field `PC4_PR`"]
pub type PC4_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC4_PW`"]
pub type PC4_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC4_PW`"]
pub struct PC4_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC4_PW_W<'a> {
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
#[doc = "Reader of field `PC4_NS`"]
pub type PC4_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC4_NS`"]
pub struct PC4_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC4_NS_W<'a> {
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
#[doc = "Reader of field `PC5_UR`"]
pub type PC5_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC5_UW`"]
pub type PC5_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC5_UW`"]
pub struct PC5_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5_UW_W<'a> {
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
#[doc = "Reader of field `PC5_PR`"]
pub type PC5_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC5_PW`"]
pub type PC5_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC5_PW`"]
pub struct PC5_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5_PW_W<'a> {
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
#[doc = "Reader of field `PC5_NS`"]
pub type PC5_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC5_NS`"]
pub struct PC5_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5_NS_W<'a> {
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
#[doc = "Reader of field `PC6_UR`"]
pub type PC6_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC6_UW`"]
pub type PC6_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC6_UW`"]
pub struct PC6_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC6_UW_W<'a> {
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
#[doc = "Reader of field `PC6_PR`"]
pub type PC6_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC6_PW`"]
pub type PC6_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC6_PW`"]
pub struct PC6_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC6_PW_W<'a> {
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
#[doc = "Reader of field `PC6_NS`"]
pub type PC6_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC6_NS`"]
pub struct PC6_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC6_NS_W<'a> {
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
#[doc = "Reader of field `PC7_UR`"]
pub type PC7_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC7_UW`"]
pub type PC7_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC7_UW`"]
pub struct PC7_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC7_UW_W<'a> {
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
#[doc = "Reader of field `PC7_PR`"]
pub type PC7_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC7_PW`"]
pub type PC7_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC7_PW`"]
pub struct PC7_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC7_PW_W<'a> {
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
#[doc = "Reader of field `PC7_NS`"]
pub type PC7_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC7_NS`"]
pub struct PC7_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC7_NS_W<'a> {
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
    #[doc = "Bit 0 - Protection context 4, user read enable."]
    #[inline(always)]
    pub fn pc4_ur(&self) -> PC4_UR_R {
        PC4_UR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protection context 4, user write enable."]
    #[inline(always)]
    pub fn pc4_uw(&self) -> PC4_UW_R {
        PC4_UW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protection context 4, privileged read enable."]
    #[inline(always)]
    pub fn pc4_pr(&self) -> PC4_PR_R {
        PC4_PR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Protection context 4, privileged write enable."]
    #[inline(always)]
    pub fn pc4_pw(&self) -> PC4_PW_R {
        PC4_PW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Protection context 4, non-secure."]
    #[inline(always)]
    pub fn pc4_ns(&self) -> PC4_NS_R {
        PC4_NS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Protection context 5, user read enable."]
    #[inline(always)]
    pub fn pc5_ur(&self) -> PC5_UR_R {
        PC5_UR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protection context 5, user write enable."]
    #[inline(always)]
    pub fn pc5_uw(&self) -> PC5_UW_R {
        PC5_UW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Protection context 5, privileged read enable."]
    #[inline(always)]
    pub fn pc5_pr(&self) -> PC5_PR_R {
        PC5_PR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Protection context 5, privileged write enable."]
    #[inline(always)]
    pub fn pc5_pw(&self) -> PC5_PW_R {
        PC5_PW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protection context 5, non-secure."]
    #[inline(always)]
    pub fn pc5_ns(&self) -> PC5_NS_R {
        PC5_NS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Protection context 6, user read enable."]
    #[inline(always)]
    pub fn pc6_ur(&self) -> PC6_UR_R {
        PC6_UR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Protection context 6, user write enable."]
    #[inline(always)]
    pub fn pc6_uw(&self) -> PC6_UW_R {
        PC6_UW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Protection context 6, privileged read enable."]
    #[inline(always)]
    pub fn pc6_pr(&self) -> PC6_PR_R {
        PC6_PR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Protection context 6, privileged write enable."]
    #[inline(always)]
    pub fn pc6_pw(&self) -> PC6_PW_R {
        PC6_PW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Protection context 6, non-secure."]
    #[inline(always)]
    pub fn pc6_ns(&self) -> PC6_NS_R {
        PC6_NS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Protection context 7, user read enable."]
    #[inline(always)]
    pub fn pc7_ur(&self) -> PC7_UR_R {
        PC7_UR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Protection context 7, user write enable."]
    #[inline(always)]
    pub fn pc7_uw(&self) -> PC7_UW_R {
        PC7_UW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Protection context 7, privileged read enable."]
    #[inline(always)]
    pub fn pc7_pr(&self) -> PC7_PR_R {
        PC7_PR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protection context 7, privileged write enable."]
    #[inline(always)]
    pub fn pc7_pw(&self) -> PC7_PW_R {
        PC7_PW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protection context 7, non-secure."]
    #[inline(always)]
    pub fn pc7_ns(&self) -> PC7_NS_R {
        PC7_NS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Protection context 4, user write enable."]
    #[inline(always)]
    pub fn pc4_uw(&mut self) -> PC4_UW_W {
        PC4_UW_W { w: self }
    }
    #[doc = "Bit 3 - Protection context 4, privileged write enable."]
    #[inline(always)]
    pub fn pc4_pw(&mut self) -> PC4_PW_W {
        PC4_PW_W { w: self }
    }
    #[doc = "Bit 4 - Protection context 4, non-secure."]
    #[inline(always)]
    pub fn pc4_ns(&mut self) -> PC4_NS_W {
        PC4_NS_W { w: self }
    }
    #[doc = "Bit 9 - Protection context 5, user write enable."]
    #[inline(always)]
    pub fn pc5_uw(&mut self) -> PC5_UW_W {
        PC5_UW_W { w: self }
    }
    #[doc = "Bit 11 - Protection context 5, privileged write enable."]
    #[inline(always)]
    pub fn pc5_pw(&mut self) -> PC5_PW_W {
        PC5_PW_W { w: self }
    }
    #[doc = "Bit 12 - Protection context 5, non-secure."]
    #[inline(always)]
    pub fn pc5_ns(&mut self) -> PC5_NS_W {
        PC5_NS_W { w: self }
    }
    #[doc = "Bit 17 - Protection context 6, user write enable."]
    #[inline(always)]
    pub fn pc6_uw(&mut self) -> PC6_UW_W {
        PC6_UW_W { w: self }
    }
    #[doc = "Bit 19 - Protection context 6, privileged write enable."]
    #[inline(always)]
    pub fn pc6_pw(&mut self) -> PC6_PW_W {
        PC6_PW_W { w: self }
    }
    #[doc = "Bit 20 - Protection context 6, non-secure."]
    #[inline(always)]
    pub fn pc6_ns(&mut self) -> PC6_NS_W {
        PC6_NS_W { w: self }
    }
    #[doc = "Bit 25 - Protection context 7, user write enable."]
    #[inline(always)]
    pub fn pc7_uw(&mut self) -> PC7_UW_W {
        PC7_UW_W { w: self }
    }
    #[doc = "Bit 27 - Protection context 7, privileged write enable."]
    #[inline(always)]
    pub fn pc7_pw(&mut self) -> PC7_PW_W {
        PC7_PW_W { w: self }
    }
    #[doc = "Bit 28 - Protection context 7, non-secure."]
    #[inline(always)]
    pub fn pc7_ns(&mut self) -> PC7_NS_W {
        PC7_NS_W { w: self }
    }
}
