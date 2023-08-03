#[doc = "Reader of register CM0_INT0_STATUS"]
pub type R = crate::R<u32, super::CM0_INT0_STATUS>;
#[doc = "Reader of field `SYSTEM_INT_IDX`"]
pub type SYSTEM_INT_IDX_R = crate::R<u16, u16>;
#[doc = "Reader of field `SYSTEM_INT_VALID`"]
pub type SYSTEM_INT_VALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:9 - Lowest CM0+ activated system interrupt index for CPU interrupt 0. Multiple system interrupts can be mapped on the same CPU interrupt. The selected system interrupt is the system interrupt with the lowest system interrupt index that has an activated interrupt request at the time of the fetch (system_interrupts\\[SYSTEM_INT_IDX\\]
is '1'). The CPU interrupt handler SW can read SYSTEM_INT_IDX to determine the system interrupt that activated the handler."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SYSTEM_INT_IDX_R {
        SYSTEM_INT_IDX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Valid indication for SYSTEM_INT_IDX. When '0', no system interrupt for CPU interrupt 0 is valid/activated."]
    #[inline(always)]
    pub fn system_int_valid(&self) -> SYSTEM_INT_VALID_R {
        SYSTEM_INT_VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
