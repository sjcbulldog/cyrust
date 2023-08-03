#[doc = "Reader of register SL_ADDR"]
pub type R = crate::R<u32, super::SL_ADDR>;
#[doc = "Reader of field `ADDR30`"]
pub type ADDR30_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 2:31 - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
    #[inline(always)]
    pub fn addr30(&self) -> ADDR30_R {
        ADDR30_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
