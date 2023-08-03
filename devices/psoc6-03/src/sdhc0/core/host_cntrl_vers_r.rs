#[doc = "Reader of register HOST_CNTRL_VERS_R"]
pub type R = crate::R<u16, super::HOST_CNTRL_VERS_R>;
#[doc = "Reader of field `SPEC_VERSION_NUM`"]
pub type SPEC_VERSION_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `VENDOR_VERSION_NUM`"]
pub type VENDOR_VERSION_NUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn spec_version_num(&self) -> SPEC_VERSION_NUM_R {
        SPEC_VERSION_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - N/A"]
    #[inline(always)]
    pub fn vendor_version_num(&self) -> VENDOR_VERSION_NUM_R {
        VENDOR_VERSION_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
