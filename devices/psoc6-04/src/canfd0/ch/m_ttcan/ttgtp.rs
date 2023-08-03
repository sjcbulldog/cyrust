#[doc = "Reader of register TTGTP"]
pub type R = crate::R<u32, super::TTGTP>;
#[doc = "Writer for register TTGTP"]
pub type W = crate::W<u32, super::TTGTP>;
#[doc = "Register TTGTP `reset()`'s with value 0"]
impl crate::ResetValue for super::TTGTP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TP`"]
pub type TP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TP`"]
pub struct TP_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `CTP`"]
pub type CTP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTP`"]
pub struct CTP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase CTP is write-protected while TTOCN.ESCN or TTOST.SPL are set (see Section 4.11). 0x0000-FFFF Defines target value of cycle time when a rising edge of m_ttcan_evt is expected"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn tp(&mut self) -> TP_W {
        TP_W { w: self }
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase CTP is write-protected while TTOCN.ESCN or TTOST.SPL are set (see Section 4.11). 0x0000-FFFF Defines target value of cycle time when a rising edge of m_ttcan_evt is expected"]
    #[inline(always)]
    pub fn ctp(&mut self) -> CTP_W {
        CTP_W { w: self }
    }
}
