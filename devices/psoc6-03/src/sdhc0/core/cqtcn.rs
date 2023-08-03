#[doc = "Reader of register CQTCN"]
pub type R = crate::R<u32, super::CQTCN>;
#[doc = "Writer for register CQTCN"]
pub type W = crate::W<u32, super::CQTCN>;
#[doc = "Register CQTCN `reset()`'s with value 0"]
impl crate::ResetValue for super::CQTCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCN`"]
pub type TCN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TCN`"]
pub struct TCN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    pub fn tcn(&self) -> TCN_R {
        TCN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    pub fn tcn(&mut self) -> TCN_W {
        TCN_W { w: self }
    }
}
