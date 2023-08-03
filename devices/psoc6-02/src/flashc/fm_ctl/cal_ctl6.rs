#[doc = "Reader of register CAL_CTL6"]
pub type R = crate::R<u32, super::CAL_CTL6>;
#[doc = "Writer for register CAL_CTL6"]
pub type W = crate::W<u32, super::CAL_CTL6>;
#[doc = "Register CAL_CTL6 `reset()`'s with value 0x0003_6f7f"]
impl crate::ResetValue for super::CAL_CTL6 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_6f7f
    }
}
#[doc = "Reader of field `SA_CTL_TRIM_T1_ULP_HV`"]
pub type SA_CTL_TRIM_T1_ULP_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T1_ULP_HV`"]
pub struct SA_CTL_TRIM_T1_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T1_ULP_HV_W<'a> {
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
#[doc = "Reader of field `SA_CTL_TRIM_T4_ULP_HV`"]
pub type SA_CTL_TRIM_T4_ULP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T4_ULP_HV`"]
pub struct SA_CTL_TRIM_T4_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T4_ULP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `SA_CTL_TRIM_T5_ULP_HV`"]
pub type SA_CTL_TRIM_T5_ULP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T5_ULP_HV`"]
pub struct SA_CTL_TRIM_T5_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T5_ULP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SA_CTL_TRIM_T6_ULP_HV`"]
pub type SA_CTL_TRIM_T6_ULP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T6_ULP_HV`"]
pub struct SA_CTL_TRIM_T6_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T6_ULP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `SA_CTL_TRIM_T8_ULP_HV`"]
pub type SA_CTL_TRIM_T8_ULP_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T8_ULP_HV`"]
pub struct SA_CTL_TRIM_T8_ULP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T8_ULP_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SA_CTL_TRIM_T1_LP_HV`"]
pub type SA_CTL_TRIM_T1_LP_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T1_LP_HV`"]
pub struct SA_CTL_TRIM_T1_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T1_LP_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SA_CTL_TRIM_T4_LP_HV`"]
pub type SA_CTL_TRIM_T4_LP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T4_LP_HV`"]
pub struct SA_CTL_TRIM_T4_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T4_LP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `SA_CTL_TRIM_T5_LP_HV`"]
pub type SA_CTL_TRIM_T5_LP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T5_LP_HV`"]
pub struct SA_CTL_TRIM_T5_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T5_LP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `SA_CTL_TRIM_T6_LP_HV`"]
pub type SA_CTL_TRIM_T6_LP_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T6_LP_HV`"]
pub struct SA_CTL_TRIM_T6_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T6_LP_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `SA_CTL_TRIM_T8_LP_HV`"]
pub type SA_CTL_TRIM_T8_LP_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SA_CTL_TRIM_T8_LP_HV`"]
pub struct SA_CTL_TRIM_T8_LP_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_CTL_TRIM_T8_LP_HV_W<'a> {
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
    #[doc = "Bit 0 - clk_trk delay"]
    #[inline(always)]
    pub fn sa_ctl_trim_t1_ulp_hv(&self) -> SA_CTL_TRIM_T1_ULP_HV_R {
        SA_CTL_TRIM_T1_ULP_HV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - SA_CTL_TRIM_T4_ULP_HV<2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV<1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t4_ulp_hv(&self) -> SA_CTL_TRIM_T4_ULP_HV_R {
        SA_CTL_TRIM_T4_ULP_HV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - SA_CTL_TRIM_T5_ULP_HV<2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV<1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t5_ulp_hv(&self) -> SA_CTL_TRIM_T5_ULP_HV_R {
        SA_CTL_TRIM_T5_ULP_HV_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:8 - SA_CTL_TRIM_T6_ULP_HV<1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV<0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t6_ulp_hv(&self) -> SA_CTL_TRIM_T6_ULP_HV_R {
        SA_CTL_TRIM_T6_ULP_HV_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t8_ulp_hv(&self) -> SA_CTL_TRIM_T8_ULP_HV_R {
        SA_CTL_TRIM_T8_ULP_HV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - clk_trk delay"]
    #[inline(always)]
    pub fn sa_ctl_trim_t1_lp_hv(&self) -> SA_CTL_TRIM_T1_LP_HV_R {
        SA_CTL_TRIM_T1_LP_HV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - SA_CTL_TRIM_T4_LP_HV<2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV<1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t4_lp_hv(&self) -> SA_CTL_TRIM_T4_LP_HV_R {
        SA_CTL_TRIM_T4_LP_HV_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - SA_CTL_TRIM_T5_LP_HV<2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV<1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t5_lp_hv(&self) -> SA_CTL_TRIM_T5_LP_HV_R {
        SA_CTL_TRIM_T5_LP_HV_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:18 - SA_CTL_TRIM_T6_LP_HV<1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV<0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t6_lp_hv(&self) -> SA_CTL_TRIM_T6_LP_HV_R {
        SA_CTL_TRIM_T6_LP_HV_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t8_lp_hv(&self) -> SA_CTL_TRIM_T8_LP_HV_R {
        SA_CTL_TRIM_T8_LP_HV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_trk delay"]
    #[inline(always)]
    pub fn sa_ctl_trim_t1_ulp_hv(&mut self) -> SA_CTL_TRIM_T1_ULP_HV_W {
        SA_CTL_TRIM_T1_ULP_HV_W { w: self }
    }
    #[doc = "Bits 1:3 - SA_CTL_TRIM_T4_ULP_HV<2>= eqi (eq current trim) SA_CTL_TRIM_T4_ULP_HV<1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t4_ulp_hv(&mut self) -> SA_CTL_TRIM_T4_ULP_HV_W {
        SA_CTL_TRIM_T4_ULP_HV_W { w: self }
    }
    #[doc = "Bits 4:6 - SA_CTL_TRIM_T5_ULP_HV<2>= evi (integration current trim) SA_CTL_TRIM_T5_ULP_HV<1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t5_ulp_hv(&mut self) -> SA_CTL_TRIM_T5_ULP_HV_W {
        SA_CTL_TRIM_T5_ULP_HV_W { w: self }
    }
    #[doc = "Bits 7:8 - SA_CTL_TRIM_T6_ULP_HV<1>= eni (enable current trim) SA_CTL_TRIM_T6_ULP_HV<0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t6_ulp_hv(&mut self) -> SA_CTL_TRIM_T6_ULP_HV_W {
        SA_CTL_TRIM_T6_ULP_HV_W { w: self }
    }
    #[doc = "Bit 9 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t8_ulp_hv(&mut self) -> SA_CTL_TRIM_T8_ULP_HV_W {
        SA_CTL_TRIM_T8_ULP_HV_W { w: self }
    }
    #[doc = "Bit 10 - clk_trk delay"]
    #[inline(always)]
    pub fn sa_ctl_trim_t1_lp_hv(&mut self) -> SA_CTL_TRIM_T1_LP_HV_W {
        SA_CTL_TRIM_T1_LP_HV_W { w: self }
    }
    #[doc = "Bits 11:13 - SA_CTL_TRIM_T4_LP_HV<2>= eqi (eq current trim) SA_CTL_TRIM_T4_LP_HV<1:0> = eqc (eq cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t4_lp_hv(&mut self) -> SA_CTL_TRIM_T4_LP_HV_W {
        SA_CTL_TRIM_T4_LP_HV_W { w: self }
    }
    #[doc = "Bits 14:16 - SA_CTL_TRIM_T5_LP_HV<2>= evi (integration current trim) SA_CTL_TRIM_T5_LP_HV<1:0> = evc (integration cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t5_lp_hv(&mut self) -> SA_CTL_TRIM_T5_LP_HV_W {
        SA_CTL_TRIM_T5_LP_HV_W { w: self }
    }
    #[doc = "Bits 17:18 - SA_CTL_TRIM_T6_LP_HV<1>= eni (enable current trim) SA_CTL_TRIM_T6_LP_HV<0> = ecn (enable cap trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t6_lp_hv(&mut self) -> SA_CTL_TRIM_T6_LP_HV_W {
        SA_CTL_TRIM_T6_LP_HV_W { w: self }
    }
    #[doc = "Bit 19 - saen3 pulse width trim (Current trim)"]
    #[inline(always)]
    pub fn sa_ctl_trim_t8_lp_hv(&mut self) -> SA_CTL_TRIM_T8_LP_HV_W {
        SA_CTL_TRIM_T8_LP_HV_W { w: self }
    }
}
