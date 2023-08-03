#[doc = "Reader of register CQSSC2"]
pub type R = crate::R<u32, super::CQSSC2>;
#[doc = "Writer for register CQSSC2"]
pub type W = crate::W<u32, super::CQSSC2>;
#[doc = "Register CQSSC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CQSSC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SQSCMD_RCA`"]
pub type SQSCMD_RCA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SQSCMD_RCA`"]
pub struct SQSCMD_RCA_W<'a> {
    w: &'a mut W,
}
impl<'a> SQSCMD_RCA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
    #[inline(always)]
    pub fn sqscmd_rca(&self) -> SQSCMD_RCA_R {
        SQSCMD_RCA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
    #[inline(always)]
    pub fn sqscmd_rca(&mut self) -> SQSCMD_RCA_W {
        SQSCMD_RCA_W { w: self }
    }
}
