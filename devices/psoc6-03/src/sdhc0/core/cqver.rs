#[doc = "Reader of register CQVER"]
pub type R = crate::R<u32, super::CQVER>;
#[doc = "Reader of field `EMMC_VER_SUFFIX`"]
pub type EMMC_VER_SUFFIX_R = crate::R<u8, u8>;
#[doc = "Reader of field `EMMC_VER_MINOR`"]
pub type EMMC_VER_MINOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EMMC_VER_MAJOR`"]
pub type EMMC_VER_MAJOR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - This bit indicates the eMMC version suffix (2nd digit right of decimal point) in BCD format."]
    #[inline(always)]
    pub fn emmc_ver_suffix(&self) -> EMMC_VER_SUFFIX_R {
        EMMC_VER_SUFFIX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This bit indicates the eMMC minor version (1st digit right of decimal point) in BCD format."]
    #[inline(always)]
    pub fn emmc_ver_minor(&self) -> EMMC_VER_MINOR_R {
        EMMC_VER_MINOR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This bit indicates the eMMC major version (1st digit left of decimal point) in BCD format."]
    #[inline(always)]
    pub fn emmc_ver_major(&self) -> EMMC_VER_MAJOR_R {
        EMMC_VER_MAJOR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
