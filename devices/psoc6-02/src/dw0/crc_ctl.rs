#[doc = "Reader of register CRC_CTL"]
pub type R = crate::R<u32, super::CRC_CTL>;
#[doc = "Writer for register CRC_CTL"]
pub type W = crate::W<u32, super::CRC_CTL>;
#[doc = "Register CRC_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `DATA_REVERSE`"]
pub type DATA_REVERSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_REVERSE`"]
pub struct DATA_REVERSE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_REVERSE_W<'a> {
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
#[doc = "Reader of field `REM_REVERSE`"]
pub type REM_REVERSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REM_REVERSE`"]
pub struct REM_REVERSE_W<'a> {
    w: &'a mut W,
}
impl<'a> REM_REVERSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
    #[inline(always)]
    pub fn data_reverse(&self) -> DATA_REVERSE_R {
        DATA_REVERSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
    #[inline(always)]
    pub fn rem_reverse(&self) -> REM_REVERSE_R {
        REM_REVERSE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
    #[inline(always)]
    pub fn data_reverse(&mut self) -> DATA_REVERSE_W {
        DATA_REVERSE_W { w: self }
    }
    #[doc = "Bit 8 - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
    #[inline(always)]
    pub fn rem_reverse(&mut self) -> REM_REVERSE_W {
        REM_REVERSE_W { w: self }
    }
}
