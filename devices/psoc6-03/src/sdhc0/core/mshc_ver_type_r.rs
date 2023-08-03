#[doc = "Reader of register MSHC_VER_TYPE_R"]
pub type R = crate::R<u32, super::MSHC_VER_TYPE_R>;
#[doc = "Reader of field `MSHC_VER_TYPE`"]
pub type MSHC_VER_TYPE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current release type This field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release type that is read by an application. For example, release type is 'ga' is represented in ASCII as 0x6761. Lower 16 bits read from this register can be ignored by the application. An application reading this register in conjunction with the MSHC_VER_ID_R register, gathers details of the current release."]
    #[inline(always)]
    pub fn mshc_ver_type(&self) -> MSHC_VER_TYPE_R {
        MSHC_VER_TYPE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
