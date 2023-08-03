#[doc = "Reader of register CM4_CA_STATUS0"]
pub type R = crate::R<u32, super::CM4_CA_STATUS0>;
#[doc = "Reader of field `VALID32`"]
pub type VALID32_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See CM0_CA_STATUS0."]
    #[inline(always)]
    pub fn valid32(&self) -> VALID32_R {
        VALID32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
