#[doc = "Reader of register ANA_CTL0"]
pub type R = crate::R<u32, super::ANA_CTL0>;
#[doc = "Writer for register ANA_CTL0"]
pub type W = crate::W<u32, super::ANA_CTL0>;
#[doc = "Register ANA_CTL0 `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::ANA_CTL0 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
#[doc = "Reader of field `MDAC`"]
pub type MDAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MDAC`"]
pub struct MDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CSLDAC`"]
pub type CSLDAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSLDAC`"]
pub struct CSLDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSLDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `FLIP_AMUXBUS_AB`"]
pub type FLIP_AMUXBUS_AB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLIP_AMUXBUS_AB`"]
pub struct FLIP_AMUXBUS_AB_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIP_AMUXBUS_AB_W<'a> {
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
#[doc = "Reader of field `NDAC_MIN`"]
pub type NDAC_MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDAC_MIN`"]
pub struct NDAC_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> NDAC_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PDAC_MIN`"]
pub type PDAC_MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDAC_MIN`"]
pub struct PDAC_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SCALE_PRG_SEQ01`"]
pub type SCALE_PRG_SEQ01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_PRG_SEQ01`"]
pub struct SCALE_PRG_SEQ01_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_PRG_SEQ01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `SCALE_PRG_SEQ12`"]
pub type SCALE_PRG_SEQ12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_PRG_SEQ12`"]
pub struct SCALE_PRG_SEQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_PRG_SEQ12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `SCALE_PRG_SEQ23`"]
pub type SCALE_PRG_SEQ23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_PRG_SEQ23`"]
pub struct SCALE_PRG_SEQ23_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_PRG_SEQ23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `SCALE_SEQ30`"]
pub type SCALE_SEQ30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_SEQ30`"]
pub struct SCALE_SEQ30_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_SEQ30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `SCALE_PRG_PEON`"]
pub type SCALE_PRG_PEON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_PRG_PEON`"]
pub struct SCALE_PRG_PEON_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_PRG_PEON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `SCALE_PRG_PEOFF`"]
pub type SCALE_PRG_PEOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCALE_PRG_PEOFF`"]
pub struct SCALE_PRG_PEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_PRG_PEOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&self) -> MDAC_R {
        MDAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&self) -> CSLDAC_R {
        CSLDAC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&self) -> FLIP_AMUXBUS_AB_R {
        FLIP_AMUXBUS_AB_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - NDAC staircase min value"]
    #[inline(always)]
    pub fn ndac_min(&self) -> NDAC_MIN_R {
        NDAC_MIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PDAC staircase min value"]
    #[inline(always)]
    pub fn pdac_min(&self) -> PDAC_MIN_R {
        PDAC_MIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq01(&self) -> SCALE_PRG_SEQ01_R {
        SCALE_PRG_SEQ01_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq12(&self) -> SCALE_PRG_SEQ12_R {
        SCALE_PRG_SEQ12_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq23(&self) -> SCALE_PRG_SEQ23_R {
        SCALE_PRG_SEQ23_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - PROG&PRE_PROG& ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_seq30(&self) -> SCALE_SEQ30_R {
        SCALE_SEQ30_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_peon(&self) -> SCALE_PRG_PEON_R {
        SCALE_PRG_PEON_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_peoff(&self) -> SCALE_PRG_PEOFF_R {
        SCALE_PRG_PEOFF_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&mut self) -> MDAC_W {
        MDAC_W { w: self }
    }
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&mut self) -> CSLDAC_W {
        CSLDAC_W { w: self }
    }
    #[doc = "Bit 11 - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&mut self) -> FLIP_AMUXBUS_AB_W {
        FLIP_AMUXBUS_AB_W { w: self }
    }
    #[doc = "Bits 12:15 - NDAC staircase min value"]
    #[inline(always)]
    pub fn ndac_min(&mut self) -> NDAC_MIN_W {
        NDAC_MIN_W { w: self }
    }
    #[doc = "Bits 16:19 - PDAC staircase min value"]
    #[inline(always)]
    pub fn pdac_min(&mut self) -> PDAC_MIN_W {
        PDAC_MIN_W { w: self }
    }
    #[doc = "Bits 20:21 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq01(&mut self) -> SCALE_PRG_SEQ01_W {
        SCALE_PRG_SEQ01_W { w: self }
    }
    #[doc = "Bits 22:23 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq12(&mut self) -> SCALE_PRG_SEQ12_W {
        SCALE_PRG_SEQ12_W { w: self }
    }
    #[doc = "Bits 24:25 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq23(&mut self) -> SCALE_PRG_SEQ23_W {
        SCALE_PRG_SEQ23_W { w: self }
    }
    #[doc = "Bits 26:27 - PROG&PRE_PROG& ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_seq30(&mut self) -> SCALE_SEQ30_W {
        SCALE_SEQ30_W { w: self }
    }
    #[doc = "Bits 28:29 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_peon(&mut self) -> SCALE_PRG_PEON_W {
        SCALE_PRG_PEON_W { w: self }
    }
    #[doc = "Bits 30:31 - PROG&PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_peoff(&mut self) -> SCALE_PRG_PEOFF_W {
        SCALE_PRG_PEOFF_W { w: self }
    }
}
