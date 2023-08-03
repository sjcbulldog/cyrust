#[doc = "Reader of register CQISGE"]
pub type R = crate::R<u32, super::CQISGE>;
#[doc = "Writer for register CQISGE"]
pub type W = crate::W<u32, super::CQISGE>;
#[doc = "Register CQISGE `reset()`'s with value 0"]
impl crate::ResetValue for super::CQISGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HAC_SGE`"]
pub type HAC_SGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HAC_SGE`"]
pub struct HAC_SGE_W<'a> {
    w: &'a mut W,
}
impl<'a> HAC_SGE_W<'a> {
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
#[doc = "Reader of field `TCC_SGE`"]
pub type TCC_SGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCC_SGE`"]
pub struct TCC_SGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC_SGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RED_SGE`"]
pub type RED_SGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_SGE`"]
pub struct RED_SGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_SGE_W<'a> {
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
#[doc = "Reader of field `TCL_SGE`"]
pub type TCL_SGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCL_SGE`"]
pub struct TCL_SGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCL_SGE_W<'a> {
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
#[doc = "Reader of field `GCE_SGE`"]
pub type GCE_SGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCE_SGE`"]
pub struct GCE_SGE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCE_SGE_W<'a> {
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
#[doc = "Reader of field `ICCE_SGE`"]
pub type ICCE_SGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICCE_SGE`"]
pub struct ICCE_SGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICCE_SGE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Halt complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn hac_sge(&self) -> HAC_SGE_R {
        HAC_SGE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcc_sge(&self) -> TCC_SGE_R {
        TCC_SGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn red_sge(&self) -> RED_SGE_R {
        RED_SGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Task cleared interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcl_sge(&self) -> TCL_SGE_R {
        TCL_SGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - General Crypto Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.GCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.GCE interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn gce_sge(&self) -> GCE_SGE_R {
        GCE_SGE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.ICCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.ICCE interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn icce_sge(&self) -> ICCE_SGE_R {
        ICCE_SGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn hac_sge(&mut self) -> HAC_SGE_W {
        HAC_SGE_W { w: self }
    }
    #[doc = "Bit 1 - Task complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcc_sge(&mut self) -> TCC_SGE_W {
        TCC_SGE_W { w: self }
    }
    #[doc = "Bit 2 - Response error detected interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn red_sge(&mut self) -> RED_SGE_W {
        RED_SGE_W { w: self }
    }
    #[doc = "Bit 3 - Task cleared interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcl_sge(&mut self) -> TCL_SGE_W {
        TCL_SGE_W { w: self }
    }
    #[doc = "Bit 4 - General Crypto Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.GCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.GCE interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn gce_sge(&mut self) -> GCE_SGE_W {
        GCE_SGE_W { w: self }
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.ICCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.ICCE interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn icce_sge(&mut self) -> ICCE_SGE_W {
        ICCE_SGE_W { w: self }
    }
}
