#[doc = "Reader of register PENDING0"]
pub type R = crate::R<u32, super::PENDING0>;
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0: CM0 MPU. Bit 1: CRYPTO MPU. Bit 2: DW 0 MPU. Bit 3: DW 1 MPU. Bit 4: DMA controller MPU. ... Bit 15: DAP MPU. Bit 16: CM4 system bus MPU. Bit 17: CM4 code bus MPU (for non FLASH controller accesses). Bit 18: CM4 code bus MPU (for FLASH controller accesses)."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
