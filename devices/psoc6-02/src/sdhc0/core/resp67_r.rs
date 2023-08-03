#[doc = "Reader of register RESP67_R"]
pub type R = crate::R<u32, super::RESP67_R>;
#[doc = "Reader of field `RESP67`"]
pub type RESP67_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect bits 135-104 of SD/EMMC Response Field. Note: For Auto CMD, this register also reflects the 32-bit response (bits 39-8 of the Response Field)."]
    #[inline(always)]
    pub fn resp67(&self) -> RESP67_R {
        RESP67_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
