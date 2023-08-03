#[doc = "Reader of register DESCR_STATUS"]
pub type R = crate::R<u32, super::DESCR_STATUS>;
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - Indicates whether the descriptor information present in DESCR_CTL, DESCR_SRC, DESCR_DST, DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE, DESCR_Y_INCR, DESCR_NEXT status registers is valid or not."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
