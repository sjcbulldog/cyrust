#[doc = "Reader of register RESP23_R"]
pub type R = crate::R<u32, super::RESP23_R>;
#[doc = "Reader of field `RESP23`"]
pub type RESP23_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 71-40 bits of the SD/eMMC Response"]
    #[inline(always)]
    pub fn resp23(&self) -> RESP23_R {
        RESP23_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
