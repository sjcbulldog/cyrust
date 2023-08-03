#[doc = "Reader of register CQISE"]
pub type R = crate::R<u32, super::CQISE>;
#[doc = "Writer for register CQISE"]
pub type W = crate::W<u32, super::CQISE>;
#[doc = "Register CQISE `reset()`'s with value 0"]
impl crate::ResetValue for super::CQISE {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `HAC_STE`"]
pub type HAC_STE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HAC_STE`"]
pub struct HAC_STE_W<'a> {
    w: &'a mut W,
}
impl<'a> HAC_STE_W<'a> {
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
#[doc = "Reader of field `TCC_STE`"]
pub type TCC_STE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCC_STE`"]
pub struct TCC_STE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC_STE_W<'a> {
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
#[doc = "Reader of field `RED_STE`"]
pub type RED_STE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_STE`"]
pub struct RED_STE_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_STE_W<'a> {
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
#[doc = "Reader of field `TCL_STE`"]
pub type TCL_STE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCL_STE`"]
pub struct TCL_STE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCL_STE_W<'a> {
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
#[doc = "Reader of field `GCE_STE`"]
pub type GCE_STE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCE_STE`"]
pub struct GCE_STE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCE_STE_W<'a> {
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
#[doc = "Reader of field `ICCE_STE`"]
pub type ICCE_STE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICCE_STE`"]
pub struct ICCE_STE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICCE_STE_W<'a> {
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
    #[doc = "Bit 0 - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
    #[inline(always)]
    pub fn hac_ste(&self) -> HAC_STE_R {
        HAC_STE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
    #[inline(always)]
    pub fn tcc_ste(&self) -> TCC_STE_R {
        TCC_STE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
    #[inline(always)]
    pub fn red_ste(&self) -> RED_STE_R {
        RED_STE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
    #[inline(always)]
    pub fn tcl_ste(&self) -> TCL_STE_R {
        TCL_STE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
    #[inline(always)]
    pub fn gce_ste(&self) -> GCE_STE_R {
        GCE_STE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
    #[inline(always)]
    pub fn icce_ste(&self) -> ICCE_STE_R {
        ICCE_STE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
    #[inline(always)]
    pub fn hac_ste(&mut self) -> HAC_STE_W {
        HAC_STE_W { w: self }
    }
    #[doc = "Bit 1 - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
    #[inline(always)]
    pub fn tcc_ste(&mut self) -> TCC_STE_W {
        TCC_STE_W { w: self }
    }
    #[doc = "Bit 2 - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
    #[inline(always)]
    pub fn red_ste(&mut self) -> RED_STE_W {
        RED_STE_W { w: self }
    }
    #[doc = "Bit 3 - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
    #[inline(always)]
    pub fn tcl_ste(&mut self) -> TCL_STE_W {
        TCL_STE_W { w: self }
    }
    #[doc = "Bit 4 - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
    #[inline(always)]
    pub fn gce_ste(&mut self) -> GCE_STE_W {
        GCE_STE_W { w: self }
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
    #[inline(always)]
    pub fn icce_ste(&mut self) -> ICCE_STE_W {
        ICCE_STE_W { w: self }
    }
}
