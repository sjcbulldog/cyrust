#[doc = "Reader of register DESCR_NEXT"]
pub type R = crate::R<u32, super::DESCR_NEXT>;
#[doc = "Reader of field `PTR`"]
pub type PTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 2:31 - N/A"]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
