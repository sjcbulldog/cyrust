#[doc = "Reader of register ANA_TRIM1"]
pub type R = crate::R<u32, super::ANA_TRIM1>;
#[doc = "Writer for register ANA_TRIM1"]
pub type W = crate::W<u32, super::ANA_TRIM1>;
#[doc = "Register ANA_TRIM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_TRIM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR_REF_BUF_TRIM`"]
pub type SAR_REF_BUF_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR_REF_BUF_TRIM`"]
pub struct SAR_REF_BUF_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_REF_BUF_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    pub fn sar_ref_buf_trim(&self) -> SAR_REF_BUF_TRIM_R {
        SAR_REF_BUF_TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    pub fn sar_ref_buf_trim(&mut self) -> SAR_REF_BUF_TRIM_W {
        SAR_REF_BUF_TRIM_W { w: self }
    }
}
