#[doc = "Reader of register TRIM_MXD[%s]"]
pub type R = crate::R<u32, super::TRIM_MXD>;
#[doc = "Writer for register TRIM_MXD[%s]"]
pub type W = crate::W<u32, super::TRIM_MXD>;
#[doc = "Register TRIM_MXD[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TRIM_MXD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MXD_TRIM_BITS`"]
pub type MXD_TRIM_BITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MXD_TRIM_BITS`"]
pub struct MXD_TRIM_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> MXD_TRIM_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - MXD trim bits"]
    #[inline(always)]
    pub fn mxd_trim_bits(&self) -> MXD_TRIM_BITS_R {
        MXD_TRIM_BITS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MXD trim bits"]
    #[inline(always)]
    pub fn mxd_trim_bits(&mut self) -> MXD_TRIM_BITS_W {
        MXD_TRIM_BITS_W { w: self }
    }
}
