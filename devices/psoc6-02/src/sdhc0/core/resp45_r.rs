#[doc = "Reader of register RESP45_R"]
pub type R = crate::R<u32, super::RESP45_R>;
#[doc = "Reader of field `RESP45`"]
pub type RESP45_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 103-72 bits of the Response Field."]
    #[inline(always)]
    pub fn resp45(&self) -> RESP45_R {
        RESP45_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
