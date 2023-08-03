#[doc = "Reader of register INTR_I2C_EC"]
pub type R = crate::R<u32, super::INTR_I2C_EC>;
#[doc = "Writer for register INTR_I2C_EC"]
pub type W = crate::W<u32, super::INTR_I2C_EC>;
#[doc = "Register INTR_I2C_EC `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_I2C_EC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `WAKE_UP`"]
pub type WAKE_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKE_UP`"]
pub struct WAKE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_UP_W<'a> {
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
#[doc = "Reader of field `EZ_STOP`"]
pub type EZ_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EZ_STOP`"]
pub struct EZ_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EZ_STOP_W<'a> {
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
#[doc = "Reader of field `EZ_WRITE_STOP`"]
pub type EZ_WRITE_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EZ_WRITE_STOP`"]
pub struct EZ_WRITE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EZ_WRITE_STOP_W<'a> {
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
#[doc = "Reader of field `EZ_READ_STOP`"]
pub type EZ_READ_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EZ_READ_STOP`"]
pub struct EZ_READ_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EZ_READ_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request (with address match). Only used when EC_AM is '1'."]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (I2C STOP). Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EZ_STOP_R {
        EZ_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EZ_WRITE_STOP_R {
        EZ_WRITE_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EZ_READ_STOP_R {
        EZ_READ_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request (with address match). Only used when EC_AM is '1'."]
    #[inline(always)]
    pub fn wake_up(&mut self) -> WAKE_UP_W {
        WAKE_UP_W { w: self }
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (I2C STOP). Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_stop(&mut self) -> EZ_STOP_W {
        EZ_STOP_W { w: self }
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_write_stop(&mut self) -> EZ_WRITE_STOP_W {
        EZ_WRITE_STOP_W { w: self }
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_read_stop(&mut self) -> EZ_READ_STOP_W {
        EZ_READ_STOP_W { w: self }
    }
}
