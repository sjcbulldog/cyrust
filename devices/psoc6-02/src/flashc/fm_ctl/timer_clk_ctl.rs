#[doc = "Reader of register TIMER_CLK_CTL"]
pub type R = crate::R<u32, super::TIMER_CLK_CTL>;
#[doc = "Writer for register TIMER_CLK_CTL"]
pub type W = crate::W<u32, super::TIMER_CLK_CTL>;
#[doc = "Register TIMER_CLK_CTL `reset()`'s with value 0x08"]
impl crate::ResetValue for super::TIMER_CLK_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
#[doc = "Reader of field `TIMER_CLOCK_FREQ`"]
pub type TIMER_CLOCK_FREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER_CLOCK_FREQ`"]
pub struct TIMER_CLOCK_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CLOCK_FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_PRG_PEON`"]
pub type RGRANT_DELAY_PRG_PEON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_PRG_PEON`"]
pub struct RGRANT_DELAY_PRG_PEON_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_PRG_PEON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_PRG_PEOFF`"]
pub type RGRANT_DELAY_PRG_PEOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_PRG_PEOFF`"]
pub struct RGRANT_DELAY_PRG_PEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_PRG_PEOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RGRANT_DELAY_PRG_SEQ01`"]
pub type RGRANT_DELAY_PRG_SEQ01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RGRANT_DELAY_PRG_SEQ01`"]
pub struct RGRANT_DELAY_PRG_SEQ01_W<'a> {
    w: &'a mut W,
}
impl<'a> RGRANT_DELAY_PRG_SEQ01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
    #[inline(always)]
    pub fn timer_clock_freq(&self) -> TIMER_CLOCK_FREQ_R {
        TIMER_CLOCK_FREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PROG&PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_peon(&self) -> RGRANT_DELAY_PRG_PEON_R {
        RGRANT_DELAY_PRG_PEON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PROG&PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_peoff(&self) -> RGRANT_DELAY_PRG_PEOFF_R {
        RGRANT_DELAY_PRG_PEOFF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PROG&PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq01(&self) -> RGRANT_DELAY_PRG_SEQ01_R {
        RGRANT_DELAY_PRG_SEQ01_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
    #[inline(always)]
    pub fn timer_clock_freq(&mut self) -> TIMER_CLOCK_FREQ_W {
        TIMER_CLOCK_FREQ_W { w: self }
    }
    #[doc = "Bits 8:15 - PROG&PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_peon(&mut self) -> RGRANT_DELAY_PRG_PEON_W {
        RGRANT_DELAY_PRG_PEON_W { w: self }
    }
    #[doc = "Bits 16:23 - PROG&PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_peoff(&mut self) -> RGRANT_DELAY_PRG_PEOFF_W {
        RGRANT_DELAY_PRG_PEOFF_W { w: self }
    }
    #[doc = "Bits 24:31 - PROG&PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq01(&mut self) -> RGRANT_DELAY_PRG_SEQ01_W {
        RGRANT_DELAY_PRG_SEQ01_W { w: self }
    }
}
