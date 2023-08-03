#[doc = "Reader of register RXFTOP0_DATA"]
pub type R = crate::R<u32, super::RXFTOP0_DATA>;
#[doc = "Reader of field `F0TD`"]
pub type F0TD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - When enabled (F0TPE=1) read data from MRAM at location FnTA. This register can have a read side effect if the following conditions are met: - M_TTCAN not being reconfigured (CCCR.CCE=0) - FIFO Top Pointer logic is enabled (FnTPE=1) - FIFO is not empty (FnFL!=0) The read side effect is as follows: - if FnMWC pointed to the last word of the message (as indicated by FnDS) then the corresponding message index (FnGI) is automatically acknowledge by a write to FnAI - FnMWC is incremented (or restarted if FnMWC pointed to the last word of the message) - the FIFO top address FnTA is incremented (with FIFO wrap around) When this logic is disabled (F0TPE=0) a Read from this register returns undefined data."]
    #[inline(always)]
    pub fn f0td(&self) -> F0TD_R {
        F0TD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
