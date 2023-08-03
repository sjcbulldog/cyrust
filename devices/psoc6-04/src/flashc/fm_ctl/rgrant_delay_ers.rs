#[doc = "Reader of register RGRANT_DELAY_ERS"]
pub type R = crate::R<u32, super::RGRANT_DELAY_ERS>;
#[doc = "Writer for register RGRANT_DELAY_ERS"]
pub type W = crate::W<u32, super::RGRANT_DELAY_ERS>;
#[doc = "Register RGRANT_DELAY_ERS `reset()`'s with value 0"]
impl crate::ResetValue for super::RGRANT_DELAY_ERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RGRANT_DELAY_ERS_SEQ01`"]
pub type RGRANT_DELAY_ERS_SEQ01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_ERS_SEQ01`"]
pub struct RGRANT_DELAY_ERS_SEQ01_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_ERS_SEQ01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_ERS_SEQ12`"]
pub type RGRANT_DELAY_ERS_SEQ12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_ERS_SEQ12`"]
pub struct RGRANT_DELAY_ERS_SEQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_ERS_SEQ12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_ERS_SEQ23`"]
pub type RGRANT_DELAY_ERS_SEQ23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_ERS_SEQ23`"]
pub struct RGRANT_DELAY_ERS_SEQ23_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_ERS_SEQ23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq01(&self) -> RGRANT_DELAY_ERS_SEQ01_R {
        RGRANT_DELAY_ERS_SEQ01_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq12(&self) -> RGRANT_DELAY_ERS_SEQ12_R {
        RGRANT_DELAY_ERS_SEQ12_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq23(&self) -> RGRANT_DELAY_ERS_SEQ23_R {
        RGRANT_DELAY_ERS_SEQ23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq01(&mut self) -> RGRANT_DELAY_ERS_SEQ01_W {
        RGRANT_DELAY_ERS_SEQ01_W { w: self }
    }
    #[doc = "Bits 8:15 - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq12(&mut self) -> RGRANT_DELAY_ERS_SEQ12_W {
        RGRANT_DELAY_ERS_SEQ12_W { w: self }
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq23(&mut self) -> RGRANT_DELAY_ERS_SEQ23_W {
        RGRANT_DELAY_ERS_SEQ23_W { w: self }
    }
}
