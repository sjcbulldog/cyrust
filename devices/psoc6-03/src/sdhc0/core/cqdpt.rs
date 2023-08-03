#[doc = "Reader of register CQDPT"]
pub type R = crate::R<u32, super::CQDPT>;
#[doc = "Reader of field `DPT`"]
pub type DPT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device-Pending Tasks Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has been successfully queued into the device and is awaiting execution - Bit-N(0): Task-N is not yet queued. Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task has not been executed. The controller sets this bit after receiving a successful response for CMD45. CQE clears this bit after the task has completed execution. Software reads this register in the task-discard procedure to determine if the task is queued in the device."]
    #[inline(always)]
    pub fn dpt(&self) -> DPT_R {
        DPT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
