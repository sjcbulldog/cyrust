#[doc = "Reader of register INTR1_CAUSE"]
pub type R = crate::R<u32, super::INTR1_CAUSE>;
#[doc = "Reader of field `INT1`"]
pub type INT1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Show pending m_ttcan_int1 of each channel"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new((self.bits & 0xff) as u8)
    }
}
