#[doc = "Reader of register CQTCLR"]
pub type R = crate::R<u32, super::CQTCLR>;
#[doc = "Writer for register CQTCLR"]
pub type W = crate::W<u32, super::CQTCLR>;
#[doc = "Register CQTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CQTCLR {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `TCLR`"]
pub type TCLR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TCLR`"]
pub struct TCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is cleared to verify that a clear operation was done."]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is cleared to verify that a clear operation was done."]
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W {
        TCLR_W { w: self }
    }
}
