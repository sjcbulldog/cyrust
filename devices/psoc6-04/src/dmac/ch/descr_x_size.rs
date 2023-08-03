#[doc = "Reader of register DESCR_X_SIZE"]
pub type R = crate::R<u32, super::DESCR_X_SIZE>;
#[doc = "Reader of field `X_COUNT`"]
pub type X_COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn x_count(&self) -> X_COUNT_R {
        X_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
