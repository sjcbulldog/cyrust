#[doc = "Reader of register RD_DATA"]
pub type R = crate::R<u32, super::RD_DATA>;
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
#[doc = "Reader of field `CHAN_ID`"]
pub type CHAN_ID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - SAR result. Results from all enabled channels are stored in the buffer. If CONFIG.CHAIN_TO_NXT is enabled, only FIFO\\[0\\].RD_DATA.RESULT should be read."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Channel number for a given SAR result. Requires CTRL.CHAN_ID_EN to be set. If CONFIG.CHAIN_TO_NXT is enabled, only FIFO\\[0\\].RD_DATA.CHAN_ID should be read."]
    #[inline(always)]
    pub fn chan_id(&self) -> CHAN_ID_R {
        CHAN_ID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
