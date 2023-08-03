#[doc = "Reader of register DESCR_Y_SIZE"]
pub type R = crate::R<u32, super::DESCR_Y_SIZE>;
#[doc = "Reader of field `Y_COUNT`"]
pub type Y_COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn y_count(&self) -> Y_COUNT_R {
        Y_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
