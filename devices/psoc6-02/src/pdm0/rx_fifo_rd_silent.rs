#[doc = "Reader of register RX_FIFO_RD_SILENT"]
pub type R = crate::R<u32, super::RX_FIFO_RD_SILENT>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data read from the RX FIFO. Reading a data frame will NOT remove the data frame from the RX FIFO; i.e. behavior is similar to that of a PEEK operation. This field is used only for debugging purposes. Note: Don't access to this bit while RX_FIFO_CTL.CLEAR is '1'."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
