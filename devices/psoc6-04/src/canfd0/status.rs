#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `STOP_ACK`"]
pub type STOP_ACK_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Clock Stop Acknowledge for each TTCAN IP. These bits are directly driven by m_ttcan_clkstop_ack of each TTCAN IP. When this bit is set the corresponding TTCAN IP clocks will be gated off, except HCLK will enabled for each AHB write"]
    #[inline(always)]
    pub fn stop_ack(&self) -> STOP_ACK_R {
        STOP_ACK_R::new((self.bits & 0xff) as u8)
    }
}
