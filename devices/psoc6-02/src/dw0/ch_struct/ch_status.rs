#[doc = "Reader of register CH_STATUS"]
pub type R = crate::R<u32, super::CH_STATUS>;
#[doc = "Reader of field `INTR_CAUSE`"]
pub type INTR_CAUSE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PENDING`"]
pub type PENDING_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Specifies the source of the interrupt cause: '0': No interrupt generated '1': Interrupt based on transfer complettion configuration based on INTR_TYPE '2': Source transfer bus error '3': Destination transfer bus error '4': Source address misalignment '5': Destination address misalignment '6': Current descriptor pointer is null '7': Active channel is disabled '8': Descriptor bus error '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
    #[inline(always)]
    pub fn intr_cause(&self) -> INTR_CAUSE_R {
        INTR_CAUSE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
