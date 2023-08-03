#[doc = "Reader of register TXBCR"]
pub type R = crate::R<u32, super::TXBCR>;
#[doc = "Writer for register TXBCR"]
pub type W = crate::W<u32, super::TXBCR>;
#[doc = "Register TXBCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXBCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CR`"]
pub type CR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CR`"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a '1' will set the corresponding Cancellation Request bit; writing a '0' has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a '1' will set the corresponding Cancellation Request bit; writing a '0' has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
}
