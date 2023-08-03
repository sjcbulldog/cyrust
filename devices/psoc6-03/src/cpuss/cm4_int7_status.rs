#[doc = "Reader of register CM4_INT7_STATUS"]
pub type R = crate::R<u32, super::CM4_INT7_STATUS>;
#[doc = "Reader of field `SYSTEM_INT_IDX`"]
pub type SYSTEM_INT_IDX_R = crate::R<u16, u16>;
#[doc = "Reader of field `SYSTEM_INT_VALID`"]
pub type SYSTEM_INT_VALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:9 - Lowest CM4 activated system interrupt index for CPU interrupt 7. See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SYSTEM_INT_IDX_R {
        SYSTEM_INT_IDX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_valid(&self) -> SYSTEM_INT_VALID_R {
        SYSTEM_INT_VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
