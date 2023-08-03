#[doc = "Reader of register SLOW_CA_CMD"]
pub type R = crate::R<u32, super::SLOW_CA_CMD>;
#[doc = "Writer for register SLOW_CA_CMD"]
pub type W = crate::W<u32, super::SLOW_CA_CMD>;
#[doc = "Register SLOW_CA_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::SLOW_CA_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INV`"]
pub type INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV`"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
}
