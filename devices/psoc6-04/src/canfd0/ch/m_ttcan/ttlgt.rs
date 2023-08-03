#[doc = "Reader of register TTLGT"]
pub type R = crate::R<u32, super::TTLGT>;
#[doc = "Reader of field `LT`"]
pub type LT_R = crate::R<u16, u16>;
#[doc = "Reader of field `GT`"]
pub type GT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Local Time Non-fractional part of local time, incremented once each local NTU (see Section 4.5). 0x0000-FFFF Local time value of TTCAN node"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Global Time Non-fractional part of the sum of the node's local time and its local offset (see Section 4.5). 0x0000-FFFF Global time value of TTCAN network"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
