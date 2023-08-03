#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `RD_PTR`"]
pub type RD_PTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `WR_PTR`"]
pub type WR_PTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - FIFO read pointer: FIFO location from which a data is read. Note: This functionality is intended for debugging purposes. If CONFIG.CHAIN_TO_NXT is enabled, only FIFO\\[0\\].STATUS.RD_PTR should be read to detemine the read pointer location of the chained FIFO."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - FIFO write pointer: FIFO location at which a new data is written by the hardware. Note: This functionality is intended for debugging purposes. If CONFIG.CHAIN_TO_NXT is enabled, only FIFO\\[0\\].STATUS.WR_PTR should be read to detemine the write pointer location of the chained FIFO."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
