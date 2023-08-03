#[doc = "Reader of register INTR_SET"]
pub type R = crate::R<u32, super::INTR_SET>;
#[doc = "Writer for register INTR_SET"]
pub type W = crate::W<u32, super::INTR_SET>;
#[doc = "Register INTR_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_SET {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `COMPLETION`"]
pub type COMPLETION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPLETION`"]
pub struct COMPLETION_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLETION_W<'a> {
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
#[doc = "Reader of field `SRC_BUS_ERROR`"]
pub type SRC_BUS_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC_BUS_ERROR`"]
pub struct SRC_BUS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_BUS_ERROR_W<'a> {
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
#[doc = "Reader of field `DST_BUS_ERROR`"]
pub type DST_BUS_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DST_BUS_ERROR`"]
pub struct DST_BUS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_BUS_ERROR_W<'a> {
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
#[doc = "Reader of field `SRC_MISAL`"]
pub type SRC_MISAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC_MISAL`"]
pub struct SRC_MISAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MISAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DST_MISAL`"]
pub type DST_MISAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DST_MISAL`"]
pub struct DST_MISAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_MISAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CURR_PTR_NULL`"]
pub type CURR_PTR_NULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURR_PTR_NULL`"]
pub struct CURR_PTR_NULL_W<'a> {
    w: &'a mut W,
}
impl<'a> CURR_PTR_NULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ACTIVE_CH_DISABLED`"]
pub type ACTIVE_CH_DISABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVE_CH_DISABLED`"]
pub struct ACTIVE_CH_DISABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE_CH_DISABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DESCR_BUS_ERROR`"]
pub type DESCR_BUS_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DESCR_BUS_ERROR`"]
pub struct DESCR_BUS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCR_BUS_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write this field with '1' to set corresponding INTR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn completion(&self) -> COMPLETION_R {
        COMPLETION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SRC_BUS_ERROR_R {
        SRC_BUS_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DST_BUS_ERROR_R {
        DST_BUS_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn src_misal(&self) -> SRC_MISAL_R {
        SRC_MISAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn dst_misal(&self) -> DST_MISAL_R {
        DST_MISAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CURR_PTR_NULL_R {
        CURR_PTR_NULL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ACTIVE_CH_DISABLED_R {
        ACTIVE_CH_DISABLED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DESCR_BUS_ERROR_R {
        DESCR_BUS_ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write this field with '1' to set corresponding INTR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn completion(&mut self) -> COMPLETION_W {
        COMPLETION_W { w: self }
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn src_bus_error(&mut self) -> SRC_BUS_ERROR_W {
        SRC_BUS_ERROR_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn dst_bus_error(&mut self) -> DST_BUS_ERROR_W {
        DST_BUS_ERROR_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn src_misal(&mut self) -> SRC_MISAL_W {
        SRC_MISAL_W { w: self }
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn dst_misal(&mut self) -> DST_MISAL_W {
        DST_MISAL_W { w: self }
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn curr_ptr_null(&mut self) -> CURR_PTR_NULL_W {
        CURR_PTR_NULL_W { w: self }
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn active_ch_disabled(&mut self) -> ACTIVE_CH_DISABLED_W {
        ACTIVE_CH_DISABLED_W { w: self }
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn descr_bus_error(&mut self) -> DESCR_BUS_ERROR_W {
        DESCR_BUS_ERROR_W { w: self }
    }
}
