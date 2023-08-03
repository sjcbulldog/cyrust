#[doc = "Reader of register DPSLP_CLOCK_SEL"]
pub type R = crate::R<u32, super::DPSLP_CLOCK_SEL>;
#[doc = "Writer for register DPSLP_CLOCK_SEL"]
pub type W = crate::W<u32, super::DPSLP_CLOCK_SEL>;
#[doc = "Register DPSLP_CLOCK_SEL `reset()`'s with value 0x20"]
impl crate::ResetValue for super::DPSLP_CLOCK_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Select source for PASS DPSLP Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPSLP_CLOCK_SEL_A {
    #[doc = "0: CLK_DPSLP is set to CLK_LPOSC"]
    CLK_LPOSC = 0,
    #[doc = "1: CLK_DPSLP is set to CLK_MF"]
    CLK_MF = 1,
}
impl From<DPSLP_CLOCK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DPSLP_CLOCK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPSLP_CLOCK_SEL`"]
pub type DPSLP_CLOCK_SEL_R = crate::R<bool, DPSLP_CLOCK_SEL_A>;
impl DPSLP_CLOCK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSLP_CLOCK_SEL_A {
        match self.bits {
            false => DPSLP_CLOCK_SEL_A::CLK_LPOSC,
            true => DPSLP_CLOCK_SEL_A::CLK_MF,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_LPOSC`"]
    #[inline(always)]
    pub fn is_clk_lposc(&self) -> bool {
        *self == DPSLP_CLOCK_SEL_A::CLK_LPOSC
    }
    #[doc = "Checks if the value of the field is `CLK_MF`"]
    #[inline(always)]
    pub fn is_clk_mf(&self) -> bool {
        *self == DPSLP_CLOCK_SEL_A::CLK_MF
    }
}
#[doc = "Write proxy for field `DPSLP_CLOCK_SEL`"]
pub struct DPSLP_CLOCK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_CLOCK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPSLP_CLOCK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CLK_DPSLP is set to CLK_LPOSC"]
    #[inline(always)]
    pub fn clk_lposc(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_SEL_A::CLK_LPOSC)
    }
    #[doc = "CLK_DPSLP is set to CLK_MF"]
    #[inline(always)]
    pub fn clk_mf(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_SEL_A::CLK_MF)
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
#[doc = "CLK_DPSLP divider\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DPSLP_CLOCK_DIV_A {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing."]
    NO_DIV = 0,
    #[doc = "1: Divide selected clock source by 2"]
    DIV_BY_2 = 1,
    #[doc = "2: Divide selected clock source by 4"]
    DIV_BY_4 = 2,
    #[doc = "3: Divide selected clock source by 8"]
    DIV_BY_8 = 3,
    #[doc = "4: Divide selected clock source by 16"]
    DIV_BY_16 = 4,
    #[doc = "5: N/A"]
    RSVD_0 = 5,
    #[doc = "6: N/A"]
    RSVD_1 = 6,
    #[doc = "7: N/A"]
    RSVD_2 = 7,
}
impl From<DPSLP_CLOCK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DPSLP_CLOCK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DPSLP_CLOCK_DIV`"]
pub type DPSLP_CLOCK_DIV_R = crate::R<u8, DPSLP_CLOCK_DIV_A>;
impl DPSLP_CLOCK_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSLP_CLOCK_DIV_A {
        match self.bits {
            0 => DPSLP_CLOCK_DIV_A::NO_DIV,
            1 => DPSLP_CLOCK_DIV_A::DIV_BY_2,
            2 => DPSLP_CLOCK_DIV_A::DIV_BY_4,
            3 => DPSLP_CLOCK_DIV_A::DIV_BY_8,
            4 => DPSLP_CLOCK_DIV_A::DIV_BY_16,
            5 => DPSLP_CLOCK_DIV_A::RSVD_0,
            6 => DPSLP_CLOCK_DIV_A::RSVD_1,
            7 => DPSLP_CLOCK_DIV_A::RSVD_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == DPSLP_CLOCK_DIV_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == DPSLP_CLOCK_DIV_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == DPSLP_CLOCK_DIV_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == DPSLP_CLOCK_DIV_A::DIV_BY_8
    }
    #[doc = "Checks if the value of the field is `DIV_BY_16`"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == DPSLP_CLOCK_DIV_A::DIV_BY_16
    }
    #[doc = "Checks if the value of the field is `RSVD_0`"]
    #[inline(always)]
    pub fn is_rsvd_0(&self) -> bool {
        *self == DPSLP_CLOCK_DIV_A::RSVD_0
    }
    #[doc = "Checks if the value of the field is `RSVD_1`"]
    #[inline(always)]
    pub fn is_rsvd_1(&self) -> bool {
        *self == DPSLP_CLOCK_DIV_A::RSVD_1
    }
    #[doc = "Checks if the value of the field is `RSVD_2`"]
    #[inline(always)]
    pub fn is_rsvd_2(&self) -> bool {
        *self == DPSLP_CLOCK_DIV_A::RSVD_2
    }
}
#[doc = "Write proxy for field `DPSLP_CLOCK_DIV`"]
pub struct DPSLP_CLOCK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_CLOCK_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPSLP_CLOCK_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_DIV_A::NO_DIV)
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_DIV_A::DIV_BY_2)
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_DIV_A::DIV_BY_4)
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_DIV_A::DIV_BY_8)
    }
    #[doc = "Divide selected clock source by 16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_DIV_A::DIV_BY_16)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd_0(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_DIV_A::RSVD_0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd_1(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_DIV_A::RSVD_1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd_2(self) -> &'a mut W {
        self.variant(DPSLP_CLOCK_DIV_A::RSVD_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select source for PASS DPSLP Clock"]
    #[inline(always)]
    pub fn dpslp_clock_sel(&self) -> DPSLP_CLOCK_SEL_R {
        DPSLP_CLOCK_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - CLK_DPSLP divider"]
    #[inline(always)]
    pub fn dpslp_clock_div(&self) -> DPSLP_CLOCK_DIV_R {
        DPSLP_CLOCK_DIV_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Select source for PASS DPSLP Clock"]
    #[inline(always)]
    pub fn dpslp_clock_sel(&mut self) -> DPSLP_CLOCK_SEL_W {
        DPSLP_CLOCK_SEL_W { w: self }
    }
    #[doc = "Bits 4:6 - CLK_DPSLP divider"]
    #[inline(always)]
    pub fn dpslp_clock_div(&mut self) -> DPSLP_CLOCK_DIV_W {
        DPSLP_CLOCK_DIV_W { w: self }
    }
}
