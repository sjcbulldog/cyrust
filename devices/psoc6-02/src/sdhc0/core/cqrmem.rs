#[doc = "Reader of register CQRMEM"]
pub type R = crate::R<u32, super::CQRMEM>;
#[doc = "Writer for register CQRMEM"]
pub type W = crate::W<u32, super::CQRMEM>;
#[doc = "Register CQRMEM `reset()`'s with value 0xfdf9_a080"]
impl crate::ResetValue for super::CQRMEM {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfdf9_a080
    }
}
#[doc = "Reader of field `RESP_ERR_MASK`"]
pub type RESP_ERR_MASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESP_ERR_MASK`"]
pub struct RESP_ERR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_ERR_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
    #[inline(always)]
    pub fn resp_err_mask(&self) -> RESP_ERR_MASK_R {
        RESP_ERR_MASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. - 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. - 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all 'Error' type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
    #[inline(always)]
    pub fn resp_err_mask(&mut self) -> RESP_ERR_MASK_W {
        RESP_ERR_MASK_W { w: self }
    }
}
