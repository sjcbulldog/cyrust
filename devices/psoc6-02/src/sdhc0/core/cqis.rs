#[doc = "Reader of register CQIS"]
pub type R = crate::R<u32, super::CQIS>;
#[doc = "Writer for register CQIS"]
pub type W = crate::W<u32, super::CQIS>;
#[doc = "Register CQIS `reset()`'s with value 0"]
impl crate::ResetValue for super::CQIS {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `HAC`"]
pub type HAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HAC`"]
pub struct HAC_W<'a> {
    w: &'a mut W,
}
impl<'a> HAC_W<'a> {
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
#[doc = "Reader of field `TCC`"]
pub type TCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCC`"]
pub struct TCC_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC_W<'a> {
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
#[doc = "Reader of field `RED`"]
pub type RED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED`"]
pub struct RED_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_W<'a> {
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
#[doc = "Reader of field `TCL`"]
pub type TCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCL`"]
pub struct TCL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCL_W<'a> {
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
#[doc = "Reader of field `GCE`"]
pub type GCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCE`"]
pub struct GCE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCE_W<'a> {
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
#[doc = "Reader of field `ICCE`"]
pub type ICCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICCE`"]
pub struct ICCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICCE_W<'a> {
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
    #[doc = "Bit 0 - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
    #[inline(always)]
    pub fn hac(&self) -> HAC_R {
        HAC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn gce(&self) -> GCE_R {
        GCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn icce(&self) -> ICCE_R {
        ICCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
    #[inline(always)]
    pub fn hac(&mut self) -> HAC_W {
        HAC_W { w: self }
    }
    #[doc = "Bit 1 - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TCC_W {
        TCC_W { w: self }
    }
    #[doc = "Bit 2 - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W {
        RED_W { w: self }
    }
    #[doc = "Bit 3 - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
    #[inline(always)]
    pub fn tcl(&mut self) -> TCL_W {
        TCL_W { w: self }
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn gce(&mut self) -> GCE_W {
        GCE_W { w: self }
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn icce(&mut self) -> ICCE_W {
        ICCE_W { w: self }
    }
}
