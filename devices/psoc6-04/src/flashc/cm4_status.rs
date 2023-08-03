#[doc = "Reader of register CM4_STATUS"]
pub type R = crate::R<u32, super::CM4_STATUS>;
#[doc = "Writer for register CM4_STATUS"]
pub type W = crate::W<u32, super::CM4_STATUS>;
#[doc = "Register CM4_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CM4_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAIN_INTERNAL_ERR`"]
pub type MAIN_INTERNAL_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAIN_INTERNAL_ERR`"]
pub struct MAIN_INTERNAL_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_INTERNAL_ERR_W<'a> {
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
#[doc = "Reader of field `WORK_INTERNAL_ERR`"]
pub type WORK_INTERNAL_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WORK_INTERNAL_ERR`"]
pub struct WORK_INTERNAL_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WORK_INTERNAL_ERR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn main_internal_err(&self) -> MAIN_INTERNAL_ERR_R {
        MAIN_INTERNAL_ERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn work_internal_err(&self) -> WORK_INTERNAL_ERR_R {
        WORK_INTERNAL_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn main_internal_err(&mut self) -> MAIN_INTERNAL_ERR_W {
        MAIN_INTERNAL_ERR_W { w: self }
    }
    #[doc = "Bit 1 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn work_internal_err(&mut self) -> WORK_INTERNAL_ERR_W {
        WORK_INTERNAL_ERR_W { w: self }
    }
}
