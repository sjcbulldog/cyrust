#[doc = "Reader of register WAIT_CTL"]
pub type R = crate::R<u32, super::WAIT_CTL>;
#[doc = "Writer for register WAIT_CTL"]
pub type W = crate::W<u32, super::WAIT_CTL>;
#[doc = "Register WAIT_CTL `reset()`'s with value 0x0003_0b09"]
impl crate::ResetValue for super::WAIT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_0b09
    }
}
#[doc = "Reader of field `WAIT_FM_MEM_RD`"]
pub type WAIT_FM_MEM_RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT_FM_MEM_RD`"]
pub struct WAIT_FM_MEM_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_FM_MEM_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `WAIT_FM_HV_RD`"]
pub type WAIT_FM_HV_RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT_FM_HV_RD`"]
pub struct WAIT_FM_HV_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_FM_HV_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WAIT_FM_HV_WR`"]
pub type WAIT_FM_HV_WR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT_FM_HV_WR`"]
pub struct WAIT_FM_HV_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_FM_HV_WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `FM_RWW_MODE`"]
pub type FM_RWW_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FM_RWW_MODE`"]
pub struct FM_RWW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_RWW_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `LV_SPARE_1`"]
pub type LV_SPARE_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LV_SPARE_1`"]
pub struct LV_SPARE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> LV_SPARE_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DRMM`"]
pub type DRMM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRMM`"]
pub struct DRMM_W<'a> {
    w: &'a mut W,
}
impl<'a> DRMM_W<'a> {
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
#[doc = "Reader of field `MBA`"]
pub type MBA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MBA`"]
pub struct MBA_W<'a> {
    w: &'a mut W,
}
impl<'a> MBA_W<'a> {
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
#[doc = "Reader of field `PL_SOFT_SET_EN`"]
pub type PL_SOFT_SET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PL_SOFT_SET_EN`"]
pub struct PL_SOFT_SET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_SOFT_SET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub fn wait_fm_mem_rd(&self) -> WAIT_FM_MEM_RD_R {
        WAIT_FM_MEM_RD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub fn wait_fm_hv_rd(&self) -> WAIT_FM_HV_RD_R {
        WAIT_FM_HV_RD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub fn wait_fm_hv_wr(&self) -> WAIT_FM_HV_WR_R {
        WAIT_FM_HV_WR_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
    #[inline(always)]
    pub fn fm_rww_mode(&self) -> FM_RWW_MODE_R {
        FM_RWW_MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Spare register"]
    #[inline(always)]
    pub fn lv_spare_1(&self) -> LV_SPARE_1_R {
        LV_SPARE_1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
    #[inline(always)]
    pub fn drmm(&self) -> DRMM_R {
        DRMM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
    #[inline(always)]
    pub fn mba(&self) -> MBA_R {
        MBA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
    #[inline(always)]
    pub fn pl_soft_set_en(&self) -> PL_SOFT_SET_EN_R {
        PL_SOFT_SET_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub fn wait_fm_mem_rd(&mut self) -> WAIT_FM_MEM_RD_W {
        WAIT_FM_MEM_RD_W { w: self }
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub fn wait_fm_hv_rd(&mut self) -> WAIT_FM_HV_RD_W {
        WAIT_FM_HV_RD_W { w: self }
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub fn wait_fm_hv_wr(&mut self) -> WAIT_FM_HV_WR_W {
        WAIT_FM_HV_WR_W { w: self }
    }
    #[doc = "Bits 24:25 - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
    #[inline(always)]
    pub fn fm_rww_mode(&mut self) -> FM_RWW_MODE_W {
        FM_RWW_MODE_W { w: self }
    }
    #[doc = "Bit 26 - Spare register"]
    #[inline(always)]
    pub fn lv_spare_1(&mut self) -> LV_SPARE_1_W {
        LV_SPARE_1_W { w: self }
    }
    #[doc = "Bit 27 - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
    #[inline(always)]
    pub fn drmm(&mut self) -> DRMM_W {
        DRMM_W { w: self }
    }
    #[doc = "Bit 28 - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
    #[inline(always)]
    pub fn mba(&mut self) -> MBA_W {
        MBA_W { w: self }
    }
    #[doc = "Bit 29 - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
    #[inline(always)]
    pub fn pl_soft_set_en(&mut self) -> PL_SOFT_SET_EN_W {
        PL_SOFT_SET_EN_W { w: self }
    }
}
