#[doc = "Reader of register CLK_CTRL_R"]
pub type R = crate::R<u16, super::CLK_CTRL_R>;
#[doc = "Writer for register CLK_CTRL_R"]
pub type W = crate::W<u16, super::CLK_CTRL_R>;
#[doc = "Register CLK_CTRL_R `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CTRL_R {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERNAL_CLK_EN`"]
pub type INTERNAL_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERNAL_CLK_EN`"]
pub struct INTERNAL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `INTERNAL_CLK_STABLE`"]
pub type INTERNAL_CLK_STABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SD_CLK_EN`"]
pub type SD_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SD_CLK_EN`"]
pub struct SD_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PLL_ENABLE`"]
pub type PLL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_ENABLE`"]
pub struct PLL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CLK_GEN_SELECT`"]
pub type CLK_GEN_SELECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_GEN_SELECT`"]
pub struct CLK_GEN_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_GEN_SELECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `UPPER_FREQ_SEL`"]
pub type UPPER_FREQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UPPER_FREQ_SEL`"]
pub struct UPPER_FREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPER_FREQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `FREQ_SEL`"]
pub type FREQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREQ_SEL`"]
pub struct FREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. Certain registers are not accessible when this bit is off. So, to be safe turn it on for any register access. Values: - 0x0 (FALSE): Stop - 0x1 (TRUE): Oscillate"]
    #[inline(always)]
    pub fn internal_clk_en(&self) -> INTERNAL_CLK_EN_R {
        INTERNAL_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable This bit enables the Host Driver to check the clock stability twice after the Internal Clock Enable bit is set and after the PLL Enable bit is set. This bit reflects the synchronized value of the Internal Clock Stable signal after the Internal Clock Enable bit is set to 1 and also reflects the synchronized value of the Card Clock Stable signal after the PLL Enable bit is set to 1. Values: - 0x0 (FALSE): Not Ready - 0x1 (TRUE): Ready"]
    #[inline(always)]
    pub fn internal_clk_stable(&self) -> INTERNAL_CLK_STABLE_R {
        INTERNAL_CLK_STABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SD/eMMC Clock Enable This bit stops the clk_card output when set to 0. The SDCLK Frequency Select bit can be changed when this bit is set to 0. Values: - 0x0 (FALSE): Disable providing clk_card - 0x1 (TRUE): Enable providing clk_card"]
    #[inline(always)]
    pub fn sd_clk_en(&self) -> SD_CLK_EN_R {
        SD_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). Values: - 0x0 (FALSE): PLL is in low power mode - 0x1 (TRUE): PLL is enabled"]
    #[inline(always)]
    pub fn pll_enable(&self) -> PLL_ENABLE_R {
        PLL_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select This bit is used to select the clock generator mode in SDCLK Frequency Select. Values: - 0x0 (FALSE): Divided Clock Mode - 0x1 (TRUE): Programmable Clock Mode"]
    #[inline(always)]
    pub fn clk_gen_select(&self) -> CLK_GEN_SELECT_R {
        CLK_GEN_SELECT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - These bits specify the upper 2 bits of 10-bit SDCLK Frequency Select control."]
    #[inline(always)]
    pub fn upper_freq_sel(&self) -> UPPER_FREQ_SEL_R {
        UPPER_FREQ_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. 10-bit Divided Clock Mode: - 0x3FF - 1/2046 Divided clock - .......... - N - 1/2N Divided Clock - .......... - 0x002 - 1/4 Divided Clock - 0x001 - 1/2 Divided Clock - 0x000 - Base clock (10MHz - 255 MHz)"]
    #[inline(always)]
    pub fn freq_sel(&self) -> FREQ_SEL_R {
        FREQ_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. Certain registers are not accessible when this bit is off. So, to be safe turn it on for any register access. Values: - 0x0 (FALSE): Stop - 0x1 (TRUE): Oscillate"]
    #[inline(always)]
    pub fn internal_clk_en(&mut self) -> INTERNAL_CLK_EN_W {
        INTERNAL_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - SD/eMMC Clock Enable This bit stops the clk_card output when set to 0. The SDCLK Frequency Select bit can be changed when this bit is set to 0. Values: - 0x0 (FALSE): Disable providing clk_card - 0x1 (TRUE): Enable providing clk_card"]
    #[inline(always)]
    pub fn sd_clk_en(&mut self) -> SD_CLK_EN_W {
        SD_CLK_EN_W { w: self }
    }
    #[doc = "Bit 3 - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). Values: - 0x0 (FALSE): PLL is in low power mode - 0x1 (TRUE): PLL is enabled"]
    #[inline(always)]
    pub fn pll_enable(&mut self) -> PLL_ENABLE_W {
        PLL_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Clock Generator Select This bit is used to select the clock generator mode in SDCLK Frequency Select. Values: - 0x0 (FALSE): Divided Clock Mode - 0x1 (TRUE): Programmable Clock Mode"]
    #[inline(always)]
    pub fn clk_gen_select(&mut self) -> CLK_GEN_SELECT_W {
        CLK_GEN_SELECT_W { w: self }
    }
    #[doc = "Bits 6:7 - These bits specify the upper 2 bits of 10-bit SDCLK Frequency Select control."]
    #[inline(always)]
    pub fn upper_freq_sel(&mut self) -> UPPER_FREQ_SEL_W {
        UPPER_FREQ_SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. 10-bit Divided Clock Mode: - 0x3FF - 1/2046 Divided clock - .......... - N - 1/2N Divided Clock - .......... - 0x002 - 1/4 Divided Clock - 0x001 - 1/2 Divided Clock - 0x000 - Base clock (10MHz - 255 MHz)"]
    #[inline(always)]
    pub fn freq_sel(&mut self) -> FREQ_SEL_W {
        FREQ_SEL_W { w: self }
    }
}
