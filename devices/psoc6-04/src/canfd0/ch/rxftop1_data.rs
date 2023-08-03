#[doc = "Reader of register RXFTOP1_DATA"]
pub type R = crate::R<u32, super::RXFTOP1_DATA>;
#[doc = "Reader of field `F1TD`"]
pub type F1TD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See F0TD description"]
    #[inline(always)]
    pub fn f1td(&self) -> F1TD_R {
        F1TD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
