#[doc = "Reader of register MS_ADDR"]
pub type R = crate::R<u32, super::MS_ADDR>;
#[doc = "Reader of field `ADDR26`"]
pub type ADDR26_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 6:31 - This field specifies the base address of the master region. The base address of the region is the address of the SL_ADDR register."]
    #[inline(always)]
    pub fn addr26(&self) -> ADDR26_R {
        ADDR26_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
}
