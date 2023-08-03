#[doc = "Reader of register CAL_CTL4"]
pub type R = crate::R<u32, super::CAL_CTL4>;
#[doc = "Writer for register CAL_CTL4"]
pub type W = crate::W<u32, super::CAL_CTL4>;
#[doc = "Register CAL_CTL4 `reset()`'s with value 0x0001_2ae0"]
impl crate::ResetValue for super::CAL_CTL4 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_2ae0
    }
}
#[doc = "Reader of field `VLIM_TRIM_ULP_HV`"]
pub type VLIM_TRIM_ULP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VLIM_TRIM_ULP_HV`"]
pub struct VLIM_TRIM_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VLIM_TRIM_ULP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `IDAC_ULP_HV`"]
pub type IDAC_ULP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDAC_ULP_HV`"]
pub struct IDAC_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC_ULP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `SDAC_ULP_HV`"]
pub type SDAC_ULP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDAC_ULP_HV`"]
pub struct SDAC_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDAC_ULP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `ITIM_ULP_HV`"]
pub type ITIM_ULP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITIM_ULP_HV`"]
pub struct ITIM_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ITIM_ULP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FM_READY_DEL_ULP_HV`"]
pub type FM_READY_DEL_ULP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FM_READY_DEL_ULP_HV`"]
pub struct FM_READY_DEL_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_READY_DEL_ULP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `SPARE451_ULP_HV`"]
pub type SPARE451_ULP_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE451_ULP_HV`"]
pub struct SPARE451_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE451_ULP_HV_W<'a> {
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
#[doc = "Reader of field `READY_RESTART_N_HV`"]
pub type READY_RESTART_N_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READY_RESTART_N_HV`"]
pub struct READY_RESTART_N_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_RESTART_N_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `VBST_S_DIS_HV`"]
pub type VBST_S_DIS_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBST_S_DIS_HV`"]
pub struct VBST_S_DIS_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VBST_S_DIS_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `AUTO_HVPULSE_HV`"]
pub type AUTO_HVPULSE_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_HVPULSE_HV`"]
pub struct AUTO_HVPULSE_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_HVPULSE_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `UGB_EN_HV`"]
pub type UGB_EN_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UGB_EN_HV`"]
pub struct UGB_EN_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> UGB_EN_HV_W<'a> {
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
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_ulp_hv(&self) -> VLIM_TRIM_ULP_HV_R {
        VLIM_TRIM_ULP_HV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub fn idac_ulp_hv(&self) -> IDAC_ULP_HV_R {
        IDAC_ULP_HV_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub fn sdac_ulp_hv(&self) -> SDAC_ULP_HV_R {
        SDAC_ULP_HV_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_ulp_hv(&self) -> ITIM_ULP_HV_R {
        ITIM_ULP_HV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub fn fm_ready_del_ulp_hv(&self) -> FM_READY_DEL_ULP_HV_R {
        FM_READY_DEL_ULP_HV_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn spare451_ulp_hv(&self) -> SPARE451_ULP_HV_R {
        SPARE451_ULP_HV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
    #[inline(always)]
    pub fn ready_restart_n_hv(&self) -> READY_RESTART_N_HV_R {
        READY_RESTART_N_HV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
    #[inline(always)]
    pub fn vbst_s_dis_hv(&self) -> VBST_S_DIS_HV_R {
        VBST_S_DIS_HV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
    #[inline(always)]
    pub fn auto_hvpulse_hv(&self) -> AUTO_HVPULSE_HV_R {
        AUTO_HVPULSE_HV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UGB enable in TM control"]
    #[inline(always)]
    pub fn ugb_en_hv(&self) -> UGB_EN_HV_R {
        UGB_EN_HV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_ulp_hv(&mut self) -> VLIM_TRIM_ULP_HV_W {
        VLIM_TRIM_ULP_HV_W { w: self }
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub fn idac_ulp_hv(&mut self) -> IDAC_ULP_HV_W {
        IDAC_ULP_HV_W { w: self }
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub fn sdac_ulp_hv(&mut self) -> SDAC_ULP_HV_W {
        SDAC_ULP_HV_W { w: self }
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_ulp_hv(&mut self) -> ITIM_ULP_HV_W {
        ITIM_ULP_HV_W { w: self }
    }
    #[doc = "Bits 13:14 - 00: Default : delay 1ns 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub fn fm_ready_del_ulp_hv(&mut self) -> FM_READY_DEL_ULP_HV_W {
        FM_READY_DEL_ULP_HV_W { w: self }
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn spare451_ulp_hv(&mut self) -> SPARE451_ULP_HV_W {
        SPARE451_ULP_HV_W { w: self }
    }
    #[doc = "Bit 16 - Toggle: 1-->0, ready goes low, ready will remain low as long as the bit is low. Toggle the bit back to 1 to activate the ready logic. To be used by API only."]
    #[inline(always)]
    pub fn ready_restart_n_hv(&mut self) -> READY_RESTART_N_HV_W {
        READY_RESTART_N_HV_W { w: self }
    }
    #[doc = "Bit 17 - 0: VBST_S voltage for each sector to allow VBST level to be dropped to VCC during Erase in the selected sector, reducing coupling to GBL. 1: VBST_S voltage for each sector stays at VBST level during Erase in the selected sector."]
    #[inline(always)]
    pub fn vbst_s_dis_hv(&mut self) -> VBST_S_DIS_HV_W {
        VBST_S_DIS_HV_W { w: self }
    }
    #[doc = "Bit 18 - 0: HV Pulse controlled by FW 1: HV Pulse controlled by Hardware"]
    #[inline(always)]
    pub fn auto_hvpulse_hv(&mut self) -> AUTO_HVPULSE_HV_W {
        AUTO_HVPULSE_HV_W { w: self }
    }
    #[doc = "Bit 19 - UGB enable in TM control"]
    #[inline(always)]
    pub fn ugb_en_hv(&mut self) -> UGB_EN_HV_W {
        UGB_EN_HV_W { w: self }
    }
}
