#[doc = "Reader of register CAL_CTL7"]
pub type R = crate::R<u32, super::CAL_CTL7>;
#[doc = "Writer for register CAL_CTL7"]
pub type W = crate::W<u32, super::CAL_CTL7>;
#[doc = "Register CAL_CTL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAL_CTL7 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `ERSX8_CLK_SEL_HV`"]
pub type ERSX8_CLK_SEL_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERSX8_CLK_SEL_HV`"]
pub struct ERSX8_CLK_SEL_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSX8_CLK_SEL_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FM_ACTIVE_HV`"]
pub type FM_ACTIVE_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FM_ACTIVE_HV`"]
pub struct FM_ACTIVE_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_ACTIVE_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TURBO_EXT_HV`"]
pub type TURBO_EXT_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TURBO_EXT_HV`"]
pub struct TURBO_EXT_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> TURBO_EXT_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `NPDAC_HWCTL_DIS_HV`"]
pub type NPDAC_HWCTL_DIS_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NPDAC_HWCTL_DIS_HV`"]
pub struct NPDAC_HWCTL_DIS_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> NPDAC_HWCTL_DIS_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FM_READY_DIS_HV`"]
pub type FM_READY_DIS_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FM_READY_DIS_HV`"]
pub struct FM_READY_DIS_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_READY_DIS_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ERSX8_EN_ALL_HV`"]
pub type ERSX8_EN_ALL_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERSX8_EN_ALL_HV`"]
pub struct ERSX8_EN_ALL_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSX8_EN_ALL_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DISABLE_LOAD_ONCE_HV`"]
pub type DISABLE_LOAD_ONCE_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_LOAD_ONCE_HV`"]
pub struct DISABLE_LOAD_ONCE_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_LOAD_ONCE_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SPARE7_HV`"]
pub type SPARE7_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE7_HV`"]
pub struct SPARE7_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE7_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPARE7_ULP_HV`"]
pub type SPARE7_ULP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE7_ULP_HV`"]
pub struct SPARE7_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE7_ULP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPARE7_LP_HV`"]
pub type SPARE7_LP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE7_LP_HV`"]
pub struct SPARE7_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE7_LP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock frequency into the ersx8 shift register block 00: Oscillator clock 01: Oscillator clock / 2 10: Oscillator clock / 4 11: Oscillator clock"]
    #[inline(always)]
    pub fn ersx8_clk_sel_hv(&self) -> ERSX8_CLK_SEL_HV_R {
        ERSX8_CLK_SEL_HV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - 0: Normal operation 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub fn fm_active_hv(&self) -> FM_ACTIVE_HV_R {
        FM_ACTIVE_HV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0: Normal operation 1: Uses external turbo pulse"]
    #[inline(always)]
    pub fn turbo_ext_hv(&self) -> TURBO_EXT_HV_R {
        TURBO_EXT_HV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0': ndac, pdac staircase hardware controlled 1: ndac, pdac staircase disabled. Enables FW control."]
    #[inline(always)]
    pub fn npdac_hwctl_dis_hv(&self) -> NPDAC_HWCTL_DIS_HV_R {
        NPDAC_HWCTL_DIS_HV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0': fm ready is enabled 1: fm ready is disabled (fm_ready is always '1')"]
    #[inline(always)]
    pub fn fm_ready_dis_hv(&self) -> FM_READY_DIS_HV_R {
        FM_READY_DIS_HV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 0': Staggered turn on/off of GWL 1: GWL are turned on/off at the same time (old FM legacy)"]
    #[inline(always)]
    pub fn ersx8_en_all_hv(&self) -> ERSX8_EN_ALL_HV_R {
        ERSX8_EN_ALL_HV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0: Load common HV params during API HV operations depends on the HV_PARAMS_LOADED bit in RGRANT_DELAY_PRG register. 1: All HV params are loaded during every API HV operation irrespective of HV_PARAMS_LOADED bit in the RGRANT_DELAY_PRG register."]
    #[inline(always)]
    pub fn disable_load_once_hv(&self) -> DISABLE_LOAD_ONCE_HV_R {
        DISABLE_LOAD_ONCE_HV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn spare7_hv(&self) -> SPARE7_HV_R {
        SPARE7_HV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:14 - N/A"]
    #[inline(always)]
    pub fn spare7_ulp_hv(&self) -> SPARE7_ULP_HV_R {
        SPARE7_ULP_HV_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - N/A"]
    #[inline(always)]
    pub fn spare7_lp_hv(&self) -> SPARE7_LP_HV_R {
        SPARE7_LP_HV_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock frequency into the ersx8 shift register block 00: Oscillator clock 01: Oscillator clock / 2 10: Oscillator clock / 4 11: Oscillator clock"]
    #[inline(always)]
    pub fn ersx8_clk_sel_hv(&mut self) -> ERSX8_CLK_SEL_HV_W {
        ERSX8_CLK_SEL_HV_W { w: self }
    }
    #[doc = "Bit 2 - 0: Normal operation 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub fn fm_active_hv(&mut self) -> FM_ACTIVE_HV_W {
        FM_ACTIVE_HV_W { w: self }
    }
    #[doc = "Bit 3 - 0: Normal operation 1: Uses external turbo pulse"]
    #[inline(always)]
    pub fn turbo_ext_hv(&mut self) -> TURBO_EXT_HV_W {
        TURBO_EXT_HV_W { w: self }
    }
    #[doc = "Bit 4 - 0': ndac, pdac staircase hardware controlled 1: ndac, pdac staircase disabled. Enables FW control."]
    #[inline(always)]
    pub fn npdac_hwctl_dis_hv(&mut self) -> NPDAC_HWCTL_DIS_HV_W {
        NPDAC_HWCTL_DIS_HV_W { w: self }
    }
    #[doc = "Bit 5 - 0': fm ready is enabled 1: fm ready is disabled (fm_ready is always '1')"]
    #[inline(always)]
    pub fn fm_ready_dis_hv(&mut self) -> FM_READY_DIS_HV_W {
        FM_READY_DIS_HV_W { w: self }
    }
    #[doc = "Bit 6 - 0': Staggered turn on/off of GWL 1: GWL are turned on/off at the same time (old FM legacy)"]
    #[inline(always)]
    pub fn ersx8_en_all_hv(&mut self) -> ERSX8_EN_ALL_HV_W {
        ERSX8_EN_ALL_HV_W { w: self }
    }
    #[doc = "Bit 7 - 0: Load common HV params during API HV operations depends on the HV_PARAMS_LOADED bit in RGRANT_DELAY_PRG register. 1: All HV params are loaded during every API HV operation irrespective of HV_PARAMS_LOADED bit in the RGRANT_DELAY_PRG register."]
    #[inline(always)]
    pub fn disable_load_once_hv(&mut self) -> DISABLE_LOAD_ONCE_HV_W {
        DISABLE_LOAD_ONCE_HV_W { w: self }
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn spare7_hv(&mut self) -> SPARE7_HV_W {
        SPARE7_HV_W { w: self }
    }
    #[doc = "Bits 10:14 - N/A"]
    #[inline(always)]
    pub fn spare7_ulp_hv(&mut self) -> SPARE7_ULP_HV_W {
        SPARE7_ULP_HV_W { w: self }
    }
    #[doc = "Bits 15:19 - N/A"]
    #[inline(always)]
    pub fn spare7_lp_hv(&mut self) -> SPARE7_LP_HV_W {
        SPARE7_LP_HV_W { w: self }
    }
}
