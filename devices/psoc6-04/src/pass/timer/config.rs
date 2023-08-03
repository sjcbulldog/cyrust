#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select Clock source of the Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLOCK_SEL_A {
    #[doc = "0: Timer clocked from CLK_PERI"]
    CLK_PERI = 0,
    #[doc = "1: Timer clocked from CLK_DPSLP"]
    CLK_DPSLP = 1,
    #[doc = "2: Timer clocked from CLK_LF"]
    CLK_LF = 2,
    #[doc = "3: N/A"]
    TBD = 3,
}
impl From<CLOCK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLOCK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLOCK_SEL`"]
pub type CLOCK_SEL_R = crate::R<u8, CLOCK_SEL_A>;
impl CLOCK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_SEL_A {
        match self.bits {
            0 => CLOCK_SEL_A::CLK_PERI,
            1 => CLOCK_SEL_A::CLK_DPSLP,
            2 => CLOCK_SEL_A::CLK_LF,
            3 => CLOCK_SEL_A::TBD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_PERI`"]
    #[inline(always)]
    pub fn is_clk_peri(&self) -> bool {
        *self == CLOCK_SEL_A::CLK_PERI
    }
    #[doc = "Checks if the value of the field is `CLK_DPSLP`"]
    #[inline(always)]
    pub fn is_clk_dpslp(&self) -> bool {
        *self == CLOCK_SEL_A::CLK_DPSLP
    }
    #[doc = "Checks if the value of the field is `CLK_LF`"]
    #[inline(always)]
    pub fn is_clk_lf(&self) -> bool {
        *self == CLOCK_SEL_A::CLK_LF
    }
    #[doc = "Checks if the value of the field is `TBD`"]
    #[inline(always)]
    pub fn is_tbd(&self) -> bool {
        *self == CLOCK_SEL_A::TBD
    }
}
#[doc = "Write proxy for field `CLOCK_SEL`"]
pub struct CLOCK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer clocked from CLK_PERI"]
    #[inline(always)]
    pub fn clk_peri(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::CLK_PERI)
    }
    #[doc = "Timer clocked from CLK_DPSLP"]
    #[inline(always)]
    pub fn clk_dpslp(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::CLK_DPSLP)
    }
    #[doc = "Timer clocked from CLK_LF"]
    #[inline(always)]
    pub fn clk_lf(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::CLK_LF)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tbd(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::TBD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select Clock source of the Timer"]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select Clock source of the Timer"]
    #[inline(always)]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W {
        CLOCK_SEL_W { w: self }
    }
}
