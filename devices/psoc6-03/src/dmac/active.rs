#[doc = "Reader of register ACTIVE"]
pub type R = crate::R<u32, super::ACTIVE>;
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Specifies active channels; i.e. enabled channels whose trigger got activated."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0xff) as u8)
    }
}
