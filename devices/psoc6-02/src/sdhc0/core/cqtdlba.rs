#[doc = "Reader of register CQTDLBA"]
pub type R = crate::R<u32, super::CQTDLBA>;
#[doc = "Writer for register CQTDLBA"]
pub type W = crate::W<u32, super::CQTDLBA>;
#[doc = "Register CQTDLBA `reset()`'s with value 0"]
impl crate::ResetValue for super::CQTDLBA {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `TDLBA`"]
pub type TDLBA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TDLBA`"]
pub struct TDLBA_W<'a> {
    w: &'a mut W,
}
impl<'a> TDLBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register stores the LSB bits (31:0) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by the host driver. This address is set on 1 KB boundary. The lower 10 bits of this register are set to 0 by the software and are ignored by CQE."]
    #[inline(always)]
    pub fn tdlba(&self) -> TDLBA_R {
        TDLBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the LSB bits (31:0) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by the host driver. This address is set on 1 KB boundary. The lower 10 bits of this register are set to 0 by the software and are ignored by CQE."]
    #[inline(always)]
    pub fn tdlba(&mut self) -> TDLBA_W {
        TDLBA_W { w: self }
    }
}
