#[doc = "Reader of register MCWDT_INTR"]
pub type R = crate::R<u32, super::MCWDT_INTR>;
#[doc = "Writer for register MCWDT_INTR"]
pub type W = crate::W<u32, super::MCWDT_INTR>;
#[doc = "Register MCWDT_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCWDT_INTR {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `MCWDT_INT0`"]
pub type MCWDT_INT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCWDT_INT0`"]
pub struct MCWDT_INT0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCWDT_INT0_W<'a> {
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
#[doc = "Reader of field `MCWDT_INT1`"]
pub type MCWDT_INT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCWDT_INT1`"]
pub struct MCWDT_INT1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCWDT_INT1_W<'a> {
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
#[doc = "Reader of field `MCWDT_INT2`"]
pub type MCWDT_INT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCWDT_INT2`"]
pub struct MCWDT_INT2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCWDT_INT2_W<'a> {
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
    #[doc = "Bit 0 - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
    #[inline(always)]
    pub fn mcwdt_int0(&self) -> MCWDT_INT0_R {
        MCWDT_INT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
    #[inline(always)]
    pub fn mcwdt_int1(&self) -> MCWDT_INT1_R {
        MCWDT_INT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
    #[inline(always)]
    pub fn mcwdt_int2(&self) -> MCWDT_INT2_R {
        MCWDT_INT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
    #[inline(always)]
    pub fn mcwdt_int0(&mut self) -> MCWDT_INT0_W {
        MCWDT_INT0_W { w: self }
    }
    #[doc = "Bit 1 - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
    #[inline(always)]
    pub fn mcwdt_int1(&mut self) -> MCWDT_INT1_W {
        MCWDT_INT1_W { w: self }
    }
    #[doc = "Bit 2 - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
    #[inline(always)]
    pub fn mcwdt_int2(&mut self) -> MCWDT_INT2_W {
        MCWDT_INT2_W { w: self }
    }
}
