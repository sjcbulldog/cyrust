#[doc = "Reader of register MBIU_CTRL_R"]
pub type R = crate::R<u8, super::MBIU_CTRL_R>;
#[doc = "Writer for register MBIU_CTRL_R"]
pub type W = crate::W<u8, super::MBIU_CTRL_R>;
#[doc = "Register MBIU_CTRL_R `reset()`'s with value 0x01"]
impl crate::ResetValue for super::MBIU_CTRL_R {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `UNDEFL_INCR_EN`"]
pub type UNDEFL_INCR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDEFL_INCR_EN`"]
pub struct UNDEFL_INCR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDEFL_INCR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `BURST_INCR4_EN`"]
pub type BURST_INCR4_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURST_INCR4_EN`"]
pub struct BURST_INCR4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_INCR4_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `BURST_INCR8_EN`"]
pub type BURST_INCR8_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURST_INCR8_EN`"]
pub struct BURST_INCR8_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_INCR8_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BURST_INCR16_EN`"]
pub type BURST_INCR16_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURST_INCR16_EN`"]
pub struct BURST_INCR16_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_INCR16_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
    #[inline(always)]
    pub fn undefl_incr_en(&self) -> UNDEFL_INCR_EN_R {
        UNDEFL_INCR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr4_en(&self) -> BURST_INCR4_EN_R {
        BURST_INCR4_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr8_en(&self) -> BURST_INCR8_EN_R {
        BURST_INCR8_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr16_en(&self) -> BURST_INCR16_EN_R {
        BURST_INCR16_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Undefined INCR Burst Controls generation of undefined length INCR transfer on Master interface. Values: - 0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F - 0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
    #[inline(always)]
    pub fn undefl_incr_en(&mut self) -> UNDEFL_INCR_EN_W {
        UNDEFL_INCR_EN_W { w: self }
    }
    #[doc = "Bit 1 - INCR4 Burst Controls generation of INCR4 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr4_en(&mut self) -> BURST_INCR4_EN_W {
        BURST_INCR4_EN_W { w: self }
    }
    #[doc = "Bit 2 - INCR8 Burst Controls generation of INCR8 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr8_en(&mut self) -> BURST_INCR8_EN_W {
        BURST_INCR8_EN_W { w: self }
    }
    #[doc = "Bit 3 - INCR16 Burst Controls generation of INCR16 transfers on Master interface. Values: - 0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F - 0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr16_en(&mut self) -> BURST_INCR16_EN_W {
        BURST_INCR16_EN_W { w: self }
    }
}
