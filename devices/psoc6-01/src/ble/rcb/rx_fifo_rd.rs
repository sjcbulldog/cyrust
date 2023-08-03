#[doc = "Reader of register RX_FIFO_RD"]
pub type R = crate::R<u32, super::RX_FIFO_RD>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. During debug it may be required to read the FIFO without a corresponding POP of the FIFO. This can be achieved by using the RX_FIFO_RD_SILENT register A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
