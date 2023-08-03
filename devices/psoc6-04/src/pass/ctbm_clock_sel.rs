#[doc = "Reader of register CTBM_CLOCK_SEL"]
pub type R = crate::R<u32, super::CTBM_CLOCK_SEL>;
#[doc = "Writer for register CTBM_CLOCK_SEL"]
pub type W = crate::W<u32, super::CTBM_CLOCK_SEL>;
#[doc = "Register CTBM_CLOCK_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTBM_CLOCK_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select source for CTBm Pump Clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUMP_CLOCK_SEL_A {
    #[doc = "0: CTBm pump clock set by AREF.CTRL.CLOCK_PUMP_PERI_SEL (Legacy)"]
    LEGACY = 0,
    #[doc = "1: CTBm pump clock sourced from CLK_DPSLP"]
    CLK_DPSLP = 1,
}
impl From<PUMP_CLOCK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PUMP_CLOCK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PUMP_CLOCK_SEL`"]
pub type PUMP_CLOCK_SEL_R = crate::R<bool, PUMP_CLOCK_SEL_A>;
impl PUMP_CLOCK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUMP_CLOCK_SEL_A {
        match self.bits {
            false => PUMP_CLOCK_SEL_A::LEGACY,
            true => PUMP_CLOCK_SEL_A::CLK_DPSLP,
        }
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == PUMP_CLOCK_SEL_A::LEGACY
    }
    #[doc = "Checks if the value of the field is `CLK_DPSLP`"]
    #[inline(always)]
    pub fn is_clk_dpslp(&self) -> bool {
        *self == PUMP_CLOCK_SEL_A::CLK_DPSLP
    }
}
#[doc = "Write proxy for field `PUMP_CLOCK_SEL`"]
pub struct PUMP_CLOCK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_CLOCK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUMP_CLOCK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CTBm pump clock set by AREF.CTRL.CLOCK_PUMP_PERI_SEL (Legacy)"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(PUMP_CLOCK_SEL_A::LEGACY)
    }
    #[doc = "CTBm pump clock sourced from CLK_DPSLP"]
    #[inline(always)]
    pub fn clk_dpslp(self) -> &'a mut W {
        self.variant(PUMP_CLOCK_SEL_A::CLK_DPSLP)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select source for CTBm Pump Clock."]
    #[inline(always)]
    pub fn pump_clock_sel(&self) -> PUMP_CLOCK_SEL_R {
        PUMP_CLOCK_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select source for CTBm Pump Clock."]
    #[inline(always)]
    pub fn pump_clock_sel(&mut self) -> PUMP_CLOCK_SEL_W {
        PUMP_CLOCK_SEL_W { w: self }
    }
}
