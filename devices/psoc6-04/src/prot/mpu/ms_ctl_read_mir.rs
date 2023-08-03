#[doc = "Reader of register MS_CTL_READ_MIR[%s]"]
pub type R = crate::R<u32, super::MS_CTL_READ_MIR>;
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u8, u8>;
#[doc = "Reader of field `PC_SAVED`"]
pub type PC_SAVED_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Read-only mirror of MS_CTL.PC"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Read-only mirror of MS_CTL.PC_SAVED"]
    #[inline(always)]
    pub fn pc_saved(&self) -> PC_SAVED_R {
        PC_SAVED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
