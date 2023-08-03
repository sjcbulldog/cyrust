#[doc = "Reader of register CQDQS"]
pub type R = crate::R<u32, super::CQDQS>;
#[doc = "Reader of field `DQS`"]
pub type DQS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Device has marked task N as ready for execution - Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
    #[inline(always)]
    pub fn dqs(&self) -> DQS_R {
        DQS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
