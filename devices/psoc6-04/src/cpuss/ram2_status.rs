#[doc = "Reader of register RAM2_STATUS"]
pub type R = crate::R<u32, super::RAM2_STATUS>;
#[doc = "Reader of field `WB_EMPTY`"]
pub type WB_EMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - See RAM0_STATUS."]
    #[inline(always)]
    pub fn wb_empty(&self) -> WB_EMPTY_R {
        WB_EMPTY_R::new((self.bits & 0x01) != 0)
    }
}
