#[doc = "Reader of register CM0_SYSTEM_INT_CTL[%s]"]
pub type R = crate::R<u32, super::CM0_SYSTEM_INT_CTL>;
#[doc = "Writer for register CM0_SYSTEM_INT_CTL[%s]"]
pub type W = crate::W<u32, super::CM0_SYSTEM_INT_CTL>;
#[doc = "Register CM0_SYSTEM_INT_CTL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CM0_SYSTEM_INT_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `CPU_INT_IDX`"]
pub type CPU_INT_IDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU_INT_IDX`"]
pub struct CPU_INT_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_INT_IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CPU_INT_VALID`"]
pub type CPU_INT_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_INT_VALID`"]
pub struct CPU_INT_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_INT_VALID_W<'a> {
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
    #[doc = "Bits 0:2 - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
    #[inline(always)]
    pub fn cpu_int_idx(&self) -> CPU_INT_IDX_R {
        CPU_INT_IDX_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
    #[inline(always)]
    pub fn cpu_int_valid(&self) -> CPU_INT_VALID_R {
        CPU_INT_VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
    #[inline(always)]
    pub fn cpu_int_idx(&mut self) -> CPU_INT_IDX_W {
        CPU_INT_IDX_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
    #[inline(always)]
    pub fn cpu_int_valid(&mut self) -> CPU_INT_VALID_W {
        CPU_INT_VALID_W { w: self }
    }
}
