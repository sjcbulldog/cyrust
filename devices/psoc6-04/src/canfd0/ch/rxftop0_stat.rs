#[doc = "Reader of register RXFTOP0_STAT"]
pub type R = crate::R<u32, super::RXFTOP0_STAT>;
#[doc = "Reader of field `F0TA`"]
pub type F0TA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current FIFO 0 Top Address. This is a pointer to the next word in the message buffer defined by the FIFO Start Address (FnSA), Get Index (FnGI), the FIFO message size (FnDS) and the message word counter (FnMWC) FnTA = FnSA + FnGI * msg_size\\[FnDS\\]
+ FnMWC"]
    #[inline(always)]
    pub fn f0ta(&self) -> F0TA_R {
        F0TA_R::new((self.bits & 0xffff) as u16)
    }
}
