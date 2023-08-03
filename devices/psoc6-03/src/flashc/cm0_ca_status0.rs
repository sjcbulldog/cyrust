#[doc = "Reader of register CM0_CA_STATUS0"]
pub type R = crate::R<u32, super::CM0_CA_STATUS0>;
#[doc = "Reader of field `VALID32`"]
pub type VALID32_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sixteen valid bits of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn valid32(&self) -> VALID32_R {
        VALID32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
