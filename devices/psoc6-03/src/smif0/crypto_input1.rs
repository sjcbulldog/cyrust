#[doc = "Reader of register CRYPTO_INPUT1"]
pub type R = crate::R<u32, super::CRYPTO_INPUT1>;
#[doc = "Writer for register CRYPTO_INPUT1"]
pub type W = crate::W<u32, super::CRYPTO_INPUT1>;
#[doc = "Register CRYPTO_INPUT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYPTO_INPUT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INPUT`"]
pub type INPUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INPUT`"]
pub struct INPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[63:32\\]
= CRYPTO_INPUT1.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[63:32\\]
= CRYPTO_INPUT1.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn input(&mut self) -> INPUT_W {
        INPUT_W { w: self }
    }
}
