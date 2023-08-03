#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC0_MATCH`"]
pub type CC0_MATCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC1_MATCH`"]
pub type CC1_MATCH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn cc0_match(&self) -> CC0_MATCH_R {
        CC0_MATCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn cc1_match(&self) -> CC1_MATCH_R {
        CC1_MATCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
