#[doc = "Reader of register USED"]
pub type R = crate::R<u32, super::USED>;
#[doc = "Reader of field `USED`"]
pub type USED_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of used/occupied entries in the FIFO. If CONFIG.CHAIN_EN is disabled, the field value is in the range \\[0, 64\\]. When '0', the FIFO is empty. When '64', the FIFO is full. If CONFIG.CHAIN_EN is enabled, only FIFO\\[0\\].USED.USED should be read to detemine the used status."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0xff) as u8)
    }
}
