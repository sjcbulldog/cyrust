#[doc = "Reader of register MS_ATT0"]
pub type R = crate::R<u32, super::MS_ATT0>;
#[doc = "Writer for register MS_ATT0"]
pub type W = crate::W<u32, super::MS_ATT0>;
#[doc = "Register MS_ATT0 `reset()`'s with value 0x1f1f_1f1f"]
impl crate::ResetValue for super::MS_ATT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f1f_1f1f
    }
}
#[doc = "Reader of field `PC0_UR`"]
pub type PC0_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC0_UW`"]
pub type PC0_UW_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC0_PR`"]
pub type PC0_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC0_PW`"]
pub type PC0_PW_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC0_NS`"]
pub type PC0_NS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC1_UR`"]
pub type PC1_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC1_UW`"]
pub type PC1_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC1_UW`"]
pub struct PC1_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_UW_W<'a> {
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
#[doc = "Reader of field `PC1_PR`"]
pub type PC1_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC1_PW`"]
pub type PC1_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC1_PW`"]
pub struct PC1_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_PW_W<'a> {
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
#[doc = "Reader of field `PC1_NS`"]
pub type PC1_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC1_NS`"]
pub struct PC1_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_NS_W<'a> {
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
#[doc = "Reader of field `PC2_UR`"]
pub type PC2_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC2_UW`"]
pub type PC2_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC2_UW`"]
pub struct PC2_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_UW_W<'a> {
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
#[doc = "Reader of field `PC2_PR`"]
pub type PC2_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC2_PW`"]
pub type PC2_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC2_PW`"]
pub struct PC2_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_PW_W<'a> {
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
#[doc = "Reader of field `PC2_NS`"]
pub type PC2_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC2_NS`"]
pub struct PC2_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_NS_W<'a> {
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
#[doc = "Reader of field `PC3_UR`"]
pub type PC3_UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC3_UW`"]
pub type PC3_UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC3_UW`"]
pub struct PC3_UW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3_UW_W<'a> {
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
#[doc = "Reader of field `PC3_PR`"]
pub type PC3_PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC3_PW`"]
pub type PC3_PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC3_PW`"]
pub struct PC3_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3_PW_W<'a> {
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
#[doc = "Reader of field `PC3_NS`"]
pub type PC3_NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC3_NS`"]
pub struct PC3_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3_NS_W<'a> {
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
    #[doc = "Bit 0 - Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_ur(&self) -> PC0_UR_R {
        PC0_UR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_uw(&self) -> PC0_UW_R {
        PC0_UW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_pr(&self) -> PC0_PR_R {
        PC0_PR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_pw(&self) -> PC0_PW_R {
        PC0_PW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn pc0_ns(&self) -> PC0_NS_R {
        PC0_NS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Protection context 1, user read enable."]
    #[inline(always)]
    pub fn pc1_ur(&self) -> PC1_UR_R {
        PC1_UR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protection context 1, user write enable."]
    #[inline(always)]
    pub fn pc1_uw(&self) -> PC1_UW_R {
        PC1_UW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Protection context 1, privileged read enable."]
    #[inline(always)]
    pub fn pc1_pr(&self) -> PC1_PR_R {
        PC1_PR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Protection context 1, privileged write enable."]
    #[inline(always)]
    pub fn pc1_pw(&self) -> PC1_PW_R {
        PC1_PW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protection context 1, non-secure."]
    #[inline(always)]
    pub fn pc1_ns(&self) -> PC1_NS_R {
        PC1_NS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Protection context 2, user read enable."]
    #[inline(always)]
    pub fn pc2_ur(&self) -> PC2_UR_R {
        PC2_UR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Protection context 2, user write enable."]
    #[inline(always)]
    pub fn pc2_uw(&self) -> PC2_UW_R {
        PC2_UW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Protection context 2, privileged read enable."]
    #[inline(always)]
    pub fn pc2_pr(&self) -> PC2_PR_R {
        PC2_PR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Protection context 2, privileged write enable."]
    #[inline(always)]
    pub fn pc2_pw(&self) -> PC2_PW_R {
        PC2_PW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Protection context 2, non-secure."]
    #[inline(always)]
    pub fn pc2_ns(&self) -> PC2_NS_R {
        PC2_NS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Protection context 3, user read enable."]
    #[inline(always)]
    pub fn pc3_ur(&self) -> PC3_UR_R {
        PC3_UR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Protection context 3, user write enable."]
    #[inline(always)]
    pub fn pc3_uw(&self) -> PC3_UW_R {
        PC3_UW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Protection context 3, privileged read enable."]
    #[inline(always)]
    pub fn pc3_pr(&self) -> PC3_PR_R {
        PC3_PR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protection context 3, privileged write enable."]
    #[inline(always)]
    pub fn pc3_pw(&self) -> PC3_PW_R {
        PC3_PW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protection context 3, non-secure."]
    #[inline(always)]
    pub fn pc3_ns(&self) -> PC3_NS_R {
        PC3_NS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Protection context 1, user write enable."]
    #[inline(always)]
    pub fn pc1_uw(&mut self) -> PC1_UW_W {
        PC1_UW_W { w: self }
    }
    #[doc = "Bit 11 - Protection context 1, privileged write enable."]
    #[inline(always)]
    pub fn pc1_pw(&mut self) -> PC1_PW_W {
        PC1_PW_W { w: self }
    }
    #[doc = "Bit 12 - Protection context 1, non-secure."]
    #[inline(always)]
    pub fn pc1_ns(&mut self) -> PC1_NS_W {
        PC1_NS_W { w: self }
    }
    #[doc = "Bit 17 - Protection context 2, user write enable."]
    #[inline(always)]
    pub fn pc2_uw(&mut self) -> PC2_UW_W {
        PC2_UW_W { w: self }
    }
    #[doc = "Bit 19 - Protection context 2, privileged write enable."]
    #[inline(always)]
    pub fn pc2_pw(&mut self) -> PC2_PW_W {
        PC2_PW_W { w: self }
    }
    #[doc = "Bit 20 - Protection context 2, non-secure."]
    #[inline(always)]
    pub fn pc2_ns(&mut self) -> PC2_NS_W {
        PC2_NS_W { w: self }
    }
    #[doc = "Bit 25 - Protection context 3, user write enable."]
    #[inline(always)]
    pub fn pc3_uw(&mut self) -> PC3_UW_W {
        PC3_UW_W { w: self }
    }
    #[doc = "Bit 27 - Protection context 3, privileged write enable."]
    #[inline(always)]
    pub fn pc3_pw(&mut self) -> PC3_PW_W {
        PC3_PW_W { w: self }
    }
    #[doc = "Bit 28 - Protection context 3, non-secure."]
    #[inline(always)]
    pub fn pc3_ns(&mut self) -> PC3_NS_W {
        PC3_NS_W { w: self }
    }
}
