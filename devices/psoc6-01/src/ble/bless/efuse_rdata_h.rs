#[doc = "Reader of register EFUSE_RDATA_H"]
pub type R = crate::R<u32, super::EFUSE_RDATA_H>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register has the read value from the Efuse macro, fuse bits\\[63:32\\]"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
