#[doc = "Reader of register CM4_CTL"]
pub type R = crate::R<u32, super::CM4_CTL>;
#[doc = "Writer for register CM4_CTL"]
pub type W = crate::W<u32, super::CM4_CTL>;
#[doc = "Register CM4_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CM4_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `IOC_MASK`"]
pub type IOC_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOC_MASK`"]
pub struct IOC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> IOC_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DZC_MASK`"]
pub type DZC_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DZC_MASK`"]
pub struct DZC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DZC_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `OFC_MASK`"]
pub type OFC_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OFC_MASK`"]
pub struct OFC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> OFC_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `UFC_MASK`"]
pub type UFC_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UFC_MASK`"]
pub struct UFC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> UFC_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `IXC_MASK`"]
pub type IXC_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IXC_MASK`"]
pub struct IXC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> IXC_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `IDC_MASK`"]
pub type IDC_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDC_MASK`"]
pub struct IDC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> IDC_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ioc_mask(&self) -> IOC_MASK_R {
        IOC_MASK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn dzc_mask(&self) -> DZC_MASK_R {
        DZC_MASK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ofc_mask(&self) -> OFC_MASK_R {
        OFC_MASK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ufc_mask(&self) -> UFC_MASK_R {
        UFC_MASK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
    #[inline(always)]
    pub fn ixc_mask(&self) -> IXC_MASK_R {
        IXC_MASK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
    #[inline(always)]
    pub fn idc_mask(&self) -> IDC_MASK_R {
        IDC_MASK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ioc_mask(&mut self) -> IOC_MASK_W {
        IOC_MASK_W { w: self }
    }
    #[doc = "Bit 25 - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn dzc_mask(&mut self) -> DZC_MASK_W {
        DZC_MASK_W { w: self }
    }
    #[doc = "Bit 26 - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ofc_mask(&mut self) -> OFC_MASK_W {
        OFC_MASK_W { w: self }
    }
    #[doc = "Bit 27 - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ufc_mask(&mut self) -> UFC_MASK_W {
        UFC_MASK_W { w: self }
    }
    #[doc = "Bit 28 - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
    #[inline(always)]
    pub fn ixc_mask(&mut self) -> IXC_MASK_W {
        IXC_MASK_W { w: self }
    }
    #[doc = "Bit 31 - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
    #[inline(always)]
    pub fn idc_mask(&mut self) -> IDC_MASK_W {
        IDC_MASK_W { w: self }
    }
}
