#[doc = "Reader of register DESCR_Y_INCR"]
pub type R = crate::R<u32, super::DESCR_Y_INCR>;
#[doc = "Reader of field `SRC_Y`"]
pub type SRC_Y_R = crate::R<u16, u16>;
#[doc = "Reader of field `DST_Y`"]
pub type DST_Y_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn src_y(&self) -> SRC_Y_R {
        SRC_Y_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn dst_y(&self) -> DST_Y_R {
        DST_Y_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
