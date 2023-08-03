#[doc = "Reader of register RESP01_R"]
pub type R = crate::R<u32, super::RESP01_R>;
#[doc = "Reader of field `RESP01`"]
pub type RESP01_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 39-8 bits of SD/eMMC Response Field. Note: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
    #[inline(always)]
    pub fn resp01(&self) -> RESP01_R {
        RESP01_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
