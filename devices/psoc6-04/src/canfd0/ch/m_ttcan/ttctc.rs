#[doc = "Reader of register TTCTC"]
pub type R = crate::R<u32, super::TTCTC>;
#[doc = "Reader of field `CT`"]
pub type CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `CC`"]
pub type CC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Cycle Time Non-fractional part of the difference of the node's local time and Ref_Mark (see Section 4.5). 0x0000-FFFF Cycle time value of TTCAN Basic Cycle"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Cycle Count 0x00-3F Number of actual Basic Cycle in the System Matrix"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
