#[doc = "Reader of register RGRANT_SCALE_ERS"]
pub type R = crate::R<u32, super::RGRANT_SCALE_ERS>;
#[doc = "Writer for register RGRANT_SCALE_ERS"]
pub type W = crate::W<u32, super::RGRANT_SCALE_ERS>;
#[doc = "Register RGRANT_SCALE_ERS `reset()`'s with value 0"]
impl crate::ResetValue for super::RGRANT_SCALE_ERS {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `SCALE_ERS_SEQ01`"]
pub type SCALE_ERS_SEQ01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_ERS_SEQ01`"]
pub struct SCALE_ERS_SEQ01_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_ERS_SEQ01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SCALE_ERS_SEQ12`"]
pub type SCALE_ERS_SEQ12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_ERS_SEQ12`"]
pub struct SCALE_ERS_SEQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_ERS_SEQ12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SCALE_ERS_SEQ23`"]
pub type SCALE_ERS_SEQ23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_ERS_SEQ23`"]
pub struct SCALE_ERS_SEQ23_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_ERS_SEQ23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SCALE_ERS_PEON`"]
pub type SCALE_ERS_PEON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_ERS_PEON`"]
pub struct SCALE_ERS_PEON_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_ERS_PEON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SCALE_ERS_PEOFF`"]
pub type SCALE_ERS_PEOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_ERS_PEOFF`"]
pub struct SCALE_ERS_PEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_ERS_PEOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_ERS_PEON`"]
pub type RGRANT_DELAY_ERS_PEON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_ERS_PEON`"]
pub struct RGRANT_DELAY_ERS_PEON_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_ERS_PEON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_ERS_PEOFF`"]
pub type RGRANT_DELAY_ERS_PEOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_ERS_PEOFF`"]
pub struct RGRANT_DELAY_ERS_PEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_ERS_PEOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq01(&self) -> SCALE_ERS_SEQ01_R {
        SCALE_ERS_SEQ01_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq12(&self) -> SCALE_ERS_SEQ12_R {
        SCALE_ERS_SEQ12_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq23(&self) -> SCALE_ERS_SEQ23_R {
        SCALE_ERS_SEQ23_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peon(&self) -> SCALE_ERS_PEON_R {
        SCALE_ERS_PEON_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peoff(&self) -> SCALE_ERS_PEOFF_R {
        SCALE_ERS_PEOFF_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peon(&self) -> RGRANT_DELAY_ERS_PEON_R {
        RGRANT_DELAY_ERS_PEON_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peoff(&self) -> RGRANT_DELAY_ERS_PEOFF_R {
        RGRANT_DELAY_ERS_PEOFF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq01(&mut self) -> SCALE_ERS_SEQ01_W {
        SCALE_ERS_SEQ01_W { w: self }
    }
    #[doc = "Bits 2:3 - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq12(&mut self) -> SCALE_ERS_SEQ12_W {
        SCALE_ERS_SEQ12_W { w: self }
    }
    #[doc = "Bits 4:5 - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq23(&mut self) -> SCALE_ERS_SEQ23_W {
        SCALE_ERS_SEQ23_W { w: self }
    }
    #[doc = "Bits 6:7 - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peon(&mut self) -> SCALE_ERS_PEON_W {
        SCALE_ERS_PEON_W { w: self }
    }
    #[doc = "Bits 8:9 - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peoff(&mut self) -> SCALE_ERS_PEOFF_W {
        SCALE_ERS_PEOFF_W { w: self }
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peon(&mut self) -> RGRANT_DELAY_ERS_PEON_W {
        RGRANT_DELAY_ERS_PEON_W { w: self }
    }
    #[doc = "Bits 24:31 - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peoff(&mut self) -> RGRANT_DELAY_ERS_PEOFF_W {
        RGRANT_DELAY_ERS_PEOFF_W { w: self }
    }
}
