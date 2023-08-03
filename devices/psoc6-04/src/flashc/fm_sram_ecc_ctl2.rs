#[doc = "Reader of register FM_SRAM_ECC_CTL2"]
pub type R = crate::R<u32, super::FM_SRAM_ECC_CTL2>;
#[doc = "Reader of field `CORRECTED_DATA`"]
pub type CORRECTED_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit corrected data output of the ECC syndrome logic."]
    #[inline(always)]
    pub fn corrected_data(&self) -> CORRECTED_DATA_R {
        CORRECTED_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
