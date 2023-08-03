#[doc = "Reader of register SAR_CLOCK_SEL[%s]"]
pub type R = crate::R<u32, super::SAR_CLOCK_SEL>;
#[doc = "Writer for register SAR_CLOCK_SEL[%s]"]
pub type W = crate::W<u32, super::SAR_CLOCK_SEL>;
#[doc = "Register SAR_CLOCK_SEL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_CLOCK_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAR clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_SEL_A {
    #[doc = "0: - 0: legacy: SAR clock source is CLK_PERI (SAR is only operational in chip ACTIVE mode)"]
    LEGACY = 0,
    #[doc = "1: - 1: SAR clock source is CLK_DPSLP (SAR can be operational in both chip ACTIVE and DEEPSLEEP modes)"]
    CLK_DPSLP = 1,
}
impl From<CLOCK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLOCK_SEL`"]
pub type CLOCK_SEL_R = crate::R<bool, CLOCK_SEL_A>;
impl CLOCK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_SEL_A {
        match self.bits {
            false => CLOCK_SEL_A::LEGACY,
            true => CLOCK_SEL_A::CLK_DPSLP,
        }
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == CLOCK_SEL_A::LEGACY
    }
    #[doc = "Checks if the value of the field is `CLK_DPSLP`"]
    #[inline(always)]
    pub fn is_clk_dpslp(&self) -> bool {
        *self == CLOCK_SEL_A::CLK_DPSLP
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
            self.bit(variant.into())
        }
    }
    #[doc = "- 0: legacy: SAR clock source is CLK_PERI (SAR is only operational in chip ACTIVE mode)"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::LEGACY)
    }
    #[doc = "- 1: SAR clock source is CLK_DPSLP (SAR can be operational in both chip ACTIVE and DEEPSLEEP modes)"]
    #[inline(always)]
    pub fn clk_dpslp(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::CLK_DPSLP)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - SAR clock select"]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - SAR clock select"]
    #[inline(always)]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W {
        CLOCK_SEL_W { w: self }
    }
}
