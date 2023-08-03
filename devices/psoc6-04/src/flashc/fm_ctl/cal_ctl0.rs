#[doc = "Reader of register CAL_CTL0"]
pub type R = crate::R<u32, super::CAL_CTL0>;
#[doc = "Writer for register CAL_CTL0"]
pub type W = crate::W<u32, super::CAL_CTL0>;
#[doc = "Register CAL_CTL0 `reset()`'s with value 0x0003_8f8f"]
impl crate::ResetValue for super::CAL_CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_8f8f
    }
}
#[doc = "Reader of field `VCT_TRIM_LO_HV`"]
pub type VCT_TRIM_LO_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCT_TRIM_LO_HV`"]
pub struct VCT_TRIM_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VCT_TRIM_LO_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CDAC_LO_HV`"]
pub type CDAC_LO_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CDAC_LO_HV`"]
pub struct CDAC_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> CDAC_LO_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `VBG_TRIM_LO_HV`"]
pub type VBG_TRIM_LO_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VBG_TRIM_LO_HV`"]
pub struct VBG_TRIM_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VBG_TRIM_LO_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `VBG_TC_TRIM_LO_HV`"]
pub type VBG_TC_TRIM_LO_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VBG_TC_TRIM_LO_HV`"]
pub struct VBG_TC_TRIM_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VBG_TC_TRIM_LO_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `ICREF_TC_TRIM_LO_HV`"]
pub type ICREF_TC_TRIM_LO_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICREF_TC_TRIM_LO_HV`"]
pub struct ICREF_TC_TRIM_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICREF_TC_TRIM_LO_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `IPREF_TRIMA_LO_HV`"]
pub type IPREF_TRIMA_LO_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPREF_TRIMA_LO_HV`"]
pub struct IPREF_TRIMA_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREF_TRIMA_LO_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - LO Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vct_trim_lo_hv(&self) -> VCT_TRIM_LO_HV_R {
        VCT_TRIM_LO_HV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub fn cdac_lo_hv(&self) -> CDAC_LO_HV_R {
        CDAC_LO_HV_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - LO Bandgap Voltage trim control."]
    #[inline(always)]
    pub fn vbg_trim_lo_hv(&self) -> VBG_TRIM_LO_HV_R {
        VBG_TRIM_LO_HV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - LO Bandgap Voltage Temperature Compensation trim control"]
    #[inline(always)]
    pub fn vbg_tc_trim_lo_hv(&self) -> VBG_TC_TRIM_LO_HV_R {
        VBG_TC_TRIM_LO_HV_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn icref_tc_trim_lo_hv(&self) -> ICREF_TC_TRIM_LO_HV_R {
        ICREF_TC_TRIM_LO_HV_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - Adds 100-150nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn ipref_trima_lo_hv(&self) -> IPREF_TRIMA_LO_HV_R {
        IPREF_TRIMA_LO_HV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vct_trim_lo_hv(&mut self) -> VCT_TRIM_LO_HV_W {
        VCT_TRIM_LO_HV_W { w: self }
    }
    #[doc = "Bits 5:7 - LO Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub fn cdac_lo_hv(&mut self) -> CDAC_LO_HV_W {
        CDAC_LO_HV_W { w: self }
    }
    #[doc = "Bits 8:12 - LO Bandgap Voltage trim control."]
    #[inline(always)]
    pub fn vbg_trim_lo_hv(&mut self) -> VBG_TRIM_LO_HV_W {
        VBG_TRIM_LO_HV_W { w: self }
    }
    #[doc = "Bits 13:15 - LO Bandgap Voltage Temperature Compensation trim control"]
    #[inline(always)]
    pub fn vbg_tc_trim_lo_hv(&mut self) -> VBG_TC_TRIM_LO_HV_W {
        VBG_TC_TRIM_LO_HV_W { w: self }
    }
    #[doc = "Bits 16:18 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn icref_tc_trim_lo_hv(&mut self) -> ICREF_TC_TRIM_LO_HV_W {
        ICREF_TC_TRIM_LO_HV_W { w: self }
    }
    #[doc = "Bit 19 - Adds 100-150nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn ipref_trima_lo_hv(&mut self) -> IPREF_TRIMA_LO_HV_W {
        IPREF_TRIMA_LO_HV_W { w: self }
    }
}
