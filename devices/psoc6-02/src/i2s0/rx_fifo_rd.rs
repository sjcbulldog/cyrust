#[doc = "Reader of register RX_FIFO_RD"]
pub type R = crate::R<u32, super::RX_FIFO_RD>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data read from the RX FIFO. Reading a data frame will remove the data frame from the RX FIFO; i.e. behavior is similar to that of a POP operation. Notes: - Don't access to this register while RX_FIFO_CTL.CLEAR is '1'. - Two stored data may be not valid after CMD.RX_START is set '1'. Therefore we recommend software discard those data."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
