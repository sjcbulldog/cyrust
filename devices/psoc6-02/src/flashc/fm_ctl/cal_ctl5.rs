#[doc = "Reader of register CAL_CTL5"]
pub type R = crate::R<u32, super::CAL_CTL5>;
#[doc = "Writer for register CAL_CTL5"]
pub type W = crate::W<u32, super::CAL_CTL5>;
#[doc = "Register CAL_CTL5 `reset()`'s with value 0x2ae0"]
impl crate::ResetValue for super::CAL_CTL5 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2ae0
    }
}
#[doc = "Reader of field `VLIM_TRIM_LP_HV`"]
pub type VLIM_TRIM_LP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VLIM_TRIM_LP_HV`"]
pub struct VLIM_TRIM_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VLIM_TRIM_LP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `IDAC_LP_HV`"]
pub type IDAC_LP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDAC_LP_HV`"]
pub struct IDAC_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC_LP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `SDAC_LP_HV`"]
pub type SDAC_LP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDAC_LP_HV`"]
pub struct SDAC_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDAC_LP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `ITIM_LP_HV`"]
pub type ITIM_LP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITIM_LP_HV`"]
pub struct ITIM_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ITIM_LP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FM_READY_DEL_LP_HV`"]
pub type FM_READY_DEL_LP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FM_READY_DEL_LP_HV`"]
pub struct FM_READY_DEL_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_READY_DEL_LP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `SPARE451_LP_HV`"]
pub type SPARE451_LP_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE451_LP_HV`"]
pub struct SPARE451_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE451_LP_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SPARE52_HV`"]
pub type SPARE52_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE52_HV`"]
pub struct SPARE52_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE52_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `AMUX_SEL_HV`"]
pub type AMUX_SEL_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMUX_SEL_HV`"]
pub struct AMUX_SEL_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> AMUX_SEL_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_lp_hv(&self) -> VLIM_TRIM_LP_HV_R {
        VLIM_TRIM_LP_HV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub fn idac_lp_hv(&self) -> IDAC_LP_HV_R {
        IDAC_LP_HV_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub fn sdac_lp_hv(&self) -> SDAC_LP_HV_R {
        SDAC_LP_HV_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_lp_hv(&self) -> ITIM_LP_HV_R {
        ITIM_LP_HV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - 00: Delayed by 1us 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub fn fm_ready_del_lp_hv(&self) -> FM_READY_DEL_LP_HV_R {
        FM_READY_DEL_LP_HV_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn spare451_lp_hv(&self) -> SPARE451_LP_HV_R {
        SPARE451_LP_HV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn spare52_hv(&self) -> SPARE52_HV_R {
        SPARE52_HV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Amux Select in AMUX_UGB 00: Bypass UGB for both amuxbusa and amuxbusb 01: Bypass UGB for amuxbusb while passing amuxbusa through UGB. 10: Bypass UGB for amuxbusa while passing amuxbusb through UGB. 11: UGB Calibrate mode"]
    #[inline(always)]
    pub fn amux_sel_hv(&self) -> AMUX_SEL_HV_R {
        AMUX_SEL_HV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_lp_hv(&mut self) -> VLIM_TRIM_LP_HV_W {
        VLIM_TRIM_LP_HV_W { w: self }
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub fn idac_lp_hv(&mut self) -> IDAC_LP_HV_W {
        IDAC_LP_HV_W { w: self }
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub fn sdac_lp_hv(&mut self) -> SDAC_LP_HV_W {
        SDAC_LP_HV_W { w: self }
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_lp_hv(&mut self) -> ITIM_LP_HV_W {
        ITIM_LP_HV_W { w: self }
    }
    #[doc = "Bits 13:14 - 00: Delayed by 1us 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub fn fm_ready_del_lp_hv(&mut self) -> FM_READY_DEL_LP_HV_W {
        FM_READY_DEL_LP_HV_W { w: self }
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn spare451_lp_hv(&mut self) -> SPARE451_LP_HV_W {
        SPARE451_LP_HV_W { w: self }
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn spare52_hv(&mut self) -> SPARE52_HV_W {
        SPARE52_HV_W { w: self }
    }
    #[doc = "Bits 18:19 - Amux Select in AMUX_UGB 00: Bypass UGB for both amuxbusa and amuxbusb 01: Bypass UGB for amuxbusb while passing amuxbusa through UGB. 10: Bypass UGB for amuxbusa while passing amuxbusb through UGB. 11: UGB Calibrate mode"]
    #[inline(always)]
    pub fn amux_sel_hv(&mut self) -> AMUX_SEL_HV_W {
        AMUX_SEL_HV_W { w: self }
    }
}
