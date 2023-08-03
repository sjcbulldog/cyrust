#[doc = "Reader of register INTR0_CAUSE"]
pub type R = crate::R<u32, super::INTR0_CAUSE>;
#[doc = "Reader of field `INT0`"]
pub type INT0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Show pending m_ttcan_int0 of each channel"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 0xff) as u8)
    }
}
