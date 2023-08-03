#[doc = "Reader of register MSHC_VER_ID_R"]
pub type R = crate::R<u32, super::MSHC_VER_ID_R>;
#[doc = "Reader of field `MSHC_VER_ID`"]
pub type MSHC_VER_ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current release number This field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release number that is read by an application. For example, release number '1.60a' is represented in ASCII as 0x313630. Lower 8 bits read from this register can be ignored by the application. An application reading this register in conjunction with the MSHC_VER_TYPE_R register, gathers details of the current release."]
    #[inline(always)]
    pub fn mshc_ver_id(&self) -> MSHC_VER_ID_R {
        MSHC_VER_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
