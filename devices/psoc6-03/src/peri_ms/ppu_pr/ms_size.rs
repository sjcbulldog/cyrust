#[doc = "Reader of register MS_SIZE"]
pub type R = crate::R<u32, super::MS_SIZE>;
#[doc = "Reader of field `REGION_SIZE`"]
pub type REGION_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 24:28 - This field specifies the size of the master region: '5': 64 B region The master region includes the SL_ADDR, SL_SIZE, SL_ATT0, ..., SL_ATT3, MS_ADDR, MS_SIZE, MS_ATT0, ..., MS_ATT3 registers. Therefore, the access privileges for all these registers is determined by MS_ATT0, ..., MS_ATT3."]
    #[inline(always)]
    pub fn region_size(&self) -> REGION_SIZE_R {
        REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Master region enable: '1': Enabled."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
