#[doc = "Reader of register DESCR_X_INCR"]
pub type R = crate::R<u32, super::DESCR_X_INCR>;
#[doc = "Reader of field `SRC_X`"]
pub type SRC_X_R = crate::R<u16, u16>;
#[doc = "Reader of field `DST_X`"]
pub type DST_X_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn src_x(&self) -> SRC_X_R {
        SRC_X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn dst_x(&self) -> DST_X_R {
        DST_X_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
