#[doc = "Reader of register CLK_MF_SELECT"]
pub type R = crate::R<u32, super::CLK_MF_SELECT>;
#[doc = "Writer for register CLK_MF_SELECT"]
pub type W = crate::W<u32, super::CLK_MF_SELECT>;
#[doc = "Register CLK_MF_SELECT `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_MF_SELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select source for MFCLK (clk_mf). Note that not all products support all clock sources. Selecting a clock source that is not supported results in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MFCLK_SEL_A {
    #[doc = "0: MFO - medium frequency oscillator"]
    MFO = 0,
}
impl From<MFCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MFCLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MFCLK_SEL`"]
pub type MFCLK_SEL_R = crate::R<u8, MFCLK_SEL_A>;
impl MFCLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MFCLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MFCLK_SEL_A::MFO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MFO`"]
    #[inline(always)]
    pub fn is_mfo(&self) -> bool {
        *self == MFCLK_SEL_A::MFO
    }
}
#[doc = "Write proxy for field `MFCLK_SEL`"]
pub struct MFCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MFCLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MFCLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MFO - medium frequency oscillator"]
    #[inline(always)]
    pub fn mfo(self) -> &'a mut W {
        self.variant(MFCLK_SEL_A::MFO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `MFCLK_DIV`"]
pub type MFCLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MFCLK_DIV`"]
pub struct MFCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MFCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 0:2 - Select source for MFCLK (clk_mf). Note that not all products support all clock sources. Selecting a clock source that is not supported results in undefined behavior."]
    #[inline(always)]
    pub fn mfclk_sel(&self) -> MFCLK_SEL_R {
        MFCLK_SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Divide selected clock source by (1+MFCLK_DIV). The output of this divider is MFCLK (clk_mf). Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while ENABLE==1."]
    #[inline(always)]
    pub fn mfclk_div(&self) -> MFCLK_DIV_R {
        MFCLK_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enable for MFCLK (clk_mf)."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select source for MFCLK (clk_mf). Note that not all products support all clock sources. Selecting a clock source that is not supported results in undefined behavior."]
    #[inline(always)]
    pub fn mfclk_sel(&mut self) -> MFCLK_SEL_W {
        MFCLK_SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - Divide selected clock source by (1+MFCLK_DIV). The output of this divider is MFCLK (clk_mf). Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while ENABLE==1."]
    #[inline(always)]
    pub fn mfclk_div(&mut self) -> MFCLK_DIV_W {
        MFCLK_DIV_W { w: self }
    }
    #[doc = "Bit 31 - Enable for MFCLK (clk_mf)."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
