#[doc = "Reader of register RAM0_STATUS"]
pub type R = crate::R<u32, super::RAM0_STATUS>;
#[doc = "Reader of field `WB_EMPTY`"]
pub type WB_EMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Write buffer empty. This information is used when entering DeepSleep power mode: WB_EMPTY must be '1' before a transition to system DeepSleep power mode. '0': Write buffer NOT empty. '1': Write buffer empty. Note: the SRAM controller write buffer is only used when ECC checking is enabled. (RAMi_CTL.ECC_EN is '1')."]
    #[inline(always)]
    pub fn wb_empty(&self) -> WB_EMPTY_R {
        WB_EMPTY_R::new((self.bits & 0x01) != 0)
    }
}
