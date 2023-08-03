#[doc = "Writer for register CRYPTO_KEY1"]
pub type W = crate::W<u32, super::CRYPTO_KEY1>;
#[doc = "Register CRYPTO_KEY1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYPTO_KEY1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the key KEY\\[63:32\\]
= CRYPTO_KEY1.KEY\\[31:0\\]."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
