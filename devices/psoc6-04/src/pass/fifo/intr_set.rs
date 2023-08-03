#[doc = "Reader of register INTR_SET"]
pub type R = crate::R<u32, super::INTR_SET>;
#[doc = "Writer for register INTR_SET"]
pub type W = crate::W<u32, super::INTR_SET>;
#[doc = "Register INTR_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFO_LEVEL`"]
pub type FIFO_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_LEVEL`"]
pub struct FIFO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_LEVEL_W<'a> {
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
#[doc = "Reader of field `FIFO_OVERFLOW`"]
pub type FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_OVERFLOW`"]
pub struct FIFO_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_OVERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FIFO_UNDERFLOW`"]
pub type FIFO_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_UNDERFLOW`"]
pub struct FIFO_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_UNDERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write this field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn fifo_level(&self) -> FIFO_LEVEL_R {
        FIFO_LEVEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write this field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn fifo_overflow(&self) -> FIFO_OVERFLOW_R {
        FIFO_OVERFLOW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write this field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn fifo_underflow(&self) -> FIFO_UNDERFLOW_R {
        FIFO_UNDERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write this field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn fifo_level(&mut self) -> FIFO_LEVEL_W {
        FIFO_LEVEL_W { w: self }
    }
    #[doc = "Bit 1 - Write this field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn fifo_overflow(&mut self) -> FIFO_OVERFLOW_W {
        FIFO_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 2 - Write this field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn fifo_underflow(&mut self) -> FIFO_UNDERFLOW_W {
        FIFO_UNDERFLOW_W { w: self }
    }
}
