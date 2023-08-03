#[doc = "Reader of register CAL_CTL2"]
pub type R = crate::R<u32, super::CAL_CTL2>;
#[doc = "Writer for register CAL_CTL2"]
pub type W = crate::W<u32, super::CAL_CTL2>;
#[doc = "Register CAL_CTL2 `reset()`'s with value 0x0007_be10"]
impl crate::ResetValue for super::CAL_CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_be10
    }
}
#[doc = "Reader of field `ICREF_TRIM_LO_HV`"]
pub type ICREF_TRIM_LO_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICREF_TRIM_LO_HV`"]
pub struct ICREF_TRIM_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICREF_TRIM_LO_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ICREF_TRIM_HI_HV`"]
pub type ICREF_TRIM_HI_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICREF_TRIM_HI_HV`"]
pub struct ICREF_TRIM_HI_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICREF_TRIM_HI_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `IPREF_TRIM_LO_HV`"]
pub type IPREF_TRIM_LO_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IPREF_TRIM_LO_HV`"]
pub struct IPREF_TRIM_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREF_TRIM_LO_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `IPREF_TRIM_HI_HV`"]
pub type IPREF_TRIM_HI_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IPREF_TRIM_HI_HV`"]
pub struct IPREF_TRIM_HI_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREF_TRIM_HI_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_lo_hv(&self) -> ICREF_TRIM_LO_HV_R {
        ICREF_TRIM_LO_HV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_hi_hv(&self) -> ICREF_TRIM_HI_HV_R {
        ICREF_TRIM_HI_HV_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - LO Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_lo_hv(&self) -> IPREF_TRIM_LO_HV_R {
        IPREF_TRIM_LO_HV_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_hi_hv(&self) -> IPREF_TRIM_HI_HV_R {
        IPREF_TRIM_HI_HV_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_lo_hv(&mut self) -> ICREF_TRIM_LO_HV_W {
        ICREF_TRIM_LO_HV_W { w: self }
    }
    #[doc = "Bits 5:9 - HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_hi_hv(&mut self) -> ICREF_TRIM_HI_HV_W {
        ICREF_TRIM_HI_HV_W { w: self }
    }
    #[doc = "Bits 10:14 - LO Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_lo_hv(&mut self) -> IPREF_TRIM_LO_HV_W {
        IPREF_TRIM_LO_HV_W { w: self }
    }
    #[doc = "Bits 15:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_hi_hv(&mut self) -> IPREF_TRIM_HI_HV_W {
        IPREF_TRIM_HI_HV_W { w: self }
    }
}
