#[doc = "Reader of register RGRANT_DELAY_PRG"]
pub type R = crate::R<u32, super::RGRANT_DELAY_PRG>;
#[doc = "Writer for register RGRANT_DELAY_PRG"]
pub type W = crate::W<u32, super::RGRANT_DELAY_PRG>;
#[doc = "Register RGRANT_DELAY_PRG `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::RGRANT_DELAY_PRG {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
#[doc = "Reader of field `RGRANT_DELAY_PRG_SEQ12`"]
pub type RGRANT_DELAY_PRG_SEQ12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_PRG_SEQ12`"]
pub struct RGRANT_DELAY_PRG_SEQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_PRG_SEQ12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_PRG_SEQ23`"]
pub type RGRANT_DELAY_PRG_SEQ23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_PRG_SEQ23`"]
pub struct RGRANT_DELAY_PRG_SEQ23_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_PRG_SEQ23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_SEQ30`"]
pub type RGRANT_DELAY_SEQ30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_SEQ30`"]
pub struct RGRANT_DELAY_SEQ30_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_SEQ30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_CLK`"]
pub type RGRANT_DELAY_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_CLK`"]
pub struct RGRANT_DELAY_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `HV_PARAMS_LOADED`"]
pub type HV_PARAMS_LOADED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HV_PARAMS_LOADED`"]
pub struct HV_PARAMS_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> HV_PARAMS_LOADED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PROG&PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq12(&self) -> RGRANT_DELAY_PRG_SEQ12_R {
        RGRANT_DELAY_PRG_SEQ12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PROG&PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq23(&self) -> RGRANT_DELAY_PRG_SEQ23_R {
        RGRANT_DELAY_PRG_SEQ23_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PROG&PRE_PROG & ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_seq30(&self) -> RGRANT_DELAY_SEQ30_R {
        RGRANT_DELAY_SEQ30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
    #[inline(always)]
    pub fn rgrant_delay_clk(&self) -> RGRANT_DELAY_CLK_R {
        RGRANT_DELAY_CLK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
    #[inline(always)]
    pub fn hv_params_loaded(&self) -> HV_PARAMS_LOADED_R {
        HV_PARAMS_LOADED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PROG&PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq12(&mut self) -> RGRANT_DELAY_PRG_SEQ12_W {
        RGRANT_DELAY_PRG_SEQ12_W { w: self }
    }
    #[doc = "Bits 8:15 - PROG&PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq23(&mut self) -> RGRANT_DELAY_PRG_SEQ23_W {
        RGRANT_DELAY_PRG_SEQ23_W { w: self }
    }
    #[doc = "Bits 16:23 - PROG&PRE_PROG & ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_seq30(&mut self) -> RGRANT_DELAY_SEQ30_W {
        RGRANT_DELAY_SEQ30_W { w: self }
    }
    #[doc = "Bits 24:27 - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
    #[inline(always)]
    pub fn rgrant_delay_clk(&mut self) -> RGRANT_DELAY_CLK_W {
        RGRANT_DELAY_CLK_W { w: self }
    }
    #[doc = "Bit 31 - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
    #[inline(always)]
    pub fn hv_params_loaded(&mut self) -> HV_PARAMS_LOADED_W {
        HV_PARAMS_LOADED_W { w: self }
    }
}
