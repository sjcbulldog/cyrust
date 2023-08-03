#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDX`"]
pub type IDX_R = crate::R<u8, u8>;
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VALID`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. HW sets this field to '1' when new fault source data is captured. New fault source data is ONLY captured when VALID is '0'. SW can clear this field to '0' when the fault is handled (by SW)."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. HW sets this field to '1' when new fault source data is captured. New fault source data is ONLY captured when VALID is '0'. SW can clear this field to '0' when the fault is handled (by SW)."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
}
