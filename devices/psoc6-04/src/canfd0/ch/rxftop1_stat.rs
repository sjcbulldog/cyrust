#[doc = "Reader of register RXFTOP1_STAT"]
pub type R = crate::R<u32, super::RXFTOP1_STAT>;
#[doc = "Reader of field `F1TA`"]
pub type F1TA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - See F0TA description"]
    #[inline(always)]
    pub fn f1ta(&self) -> F1TA_R {
        F1TA_R::new((self.bits & 0xffff) as u16)
    }
}
