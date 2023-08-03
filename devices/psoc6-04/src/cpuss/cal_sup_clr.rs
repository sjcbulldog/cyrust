#[doc = "Reader of register CAL_SUP_CLR"]
pub type R = crate::R<u32, super::CAL_SUP_CLR>;
#[doc = "Writer for register CAL_SUP_CLR"]
pub type W = crate::W<u32, super::CAL_SUP_CLR>;
#[doc = "Register CAL_SUP_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CAL_SUP_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
