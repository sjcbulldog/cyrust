#[doc = "Reader of register RX_FIFO_RD"]
pub type R = crate::R<u32, super::RX_FIFO_RD>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\]
are used. This register has a side effect when read by software: a data frame is removed from the FIFO. This may be undesirable during debug; i.e. a read during debug should NOT have a side effect. To this end, the IP uses the AHB-Lite 'hmaster\\[0\\]' input signal. When this signal is '1' in the address cycle of a bus transfer, a read transfer will not have a side effect. As a result, a read from this register will not remove a data frame from the FIFO. As a result, a read from this register behaves as a read from the SCB_RX_FIFO_RD_SILENT register. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
