#[doc = "Reader of register INTR_S"]
pub type R = crate::R<u32, super::INTR_S>;
#[doc = "Writer for register INTR_S"]
pub type W = crate::W<u32, super::INTR_S>;
#[doc = "Register INTR_S `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_S {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `I2C_ARB_LOST`"]
pub type I2C_ARB_LOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ARB_LOST`"]
pub struct I2C_ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ARB_LOST_W<'a> {
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
#[doc = "Reader of field `I2C_NACK`"]
pub type I2C_NACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_NACK`"]
pub struct I2C_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_NACK_W<'a> {
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
#[doc = "Reader of field `I2C_ACK`"]
pub type I2C_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ACK`"]
pub struct I2C_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ACK_W<'a> {
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
#[doc = "Reader of field `I2C_WRITE_STOP`"]
pub type I2C_WRITE_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_WRITE_STOP`"]
pub struct I2C_WRITE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_WRITE_STOP_W<'a> {
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
#[doc = "Reader of field `I2C_STOP`"]
pub type I2C_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_STOP`"]
pub struct I2C_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_STOP_W<'a> {
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
#[doc = "Reader of field `I2C_START`"]
pub type I2C_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_START`"]
pub struct I2C_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_START_W<'a> {
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
#[doc = "Reader of field `I2C_ADDR_MATCH`"]
pub type I2C_ADDR_MATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ADDR_MATCH`"]
pub struct I2C_ADDR_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ADDR_MATCH_W<'a> {
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
#[doc = "Reader of field `I2C_GENERAL`"]
pub type I2C_GENERAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_GENERAL`"]
pub struct I2C_GENERAL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_GENERAL_W<'a> {
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
#[doc = "Reader of field `I2C_BUS_ERROR`"]
pub type I2C_BUS_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_BUS_ERROR`"]
pub struct I2C_BUS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_BUS_ERROR_W<'a> {
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
#[doc = "Reader of field `SPI_EZ_WRITE_STOP`"]
pub type SPI_EZ_WRITE_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_EZ_WRITE_STOP`"]
pub struct SPI_EZ_WRITE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_EZ_WRITE_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPI_EZ_STOP`"]
pub type SPI_EZ_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_EZ_STOP`"]
pub struct SPI_EZ_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_EZ_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPI_BUS_ERROR`"]
pub type SPI_BUS_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_BUS_ERROR`"]
pub struct SPI_BUS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BUS_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2C_WRITE_STOP_R {
        I2C_WRITE_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2C_ADDR_MATCH_R {
        I2C_ADDR_MATCH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2C_GENERAL_R {
        I2C_GENERAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI slave deselected after a write EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SPI_EZ_WRITE_STOP_R {
        SPI_EZ_WRITE_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI slave deselected after any EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SPI_EZ_STOP_R {
        SPI_EZ_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SPI_BUS_ERROR_R {
        SPI_BUS_ERROR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_arb_lost(&mut self) -> I2C_ARB_LOST_W {
        I2C_ARB_LOST_W { w: self }
    }
    #[doc = "Bit 1 - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W {
        I2C_NACK_W { w: self }
    }
    #[doc = "Bit 2 - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W {
        I2C_ACK_W { w: self }
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
    #[inline(always)]
    pub fn i2c_write_stop(&mut self) -> I2C_WRITE_STOP_W {
        I2C_WRITE_STOP_W { w: self }
    }
    #[doc = "Bit 4 - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W {
        I2C_STOP_W { w: self }
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    pub fn i2c_start(&mut self) -> I2C_START_W {
        I2C_START_W { w: self }
    }
    #[doc = "Bit 6 - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_addr_match(&mut self) -> I2C_ADDR_MATCH_W {
        I2C_ADDR_MATCH_W { w: self }
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_general(&mut self) -> I2C_GENERAL_W {
        I2C_GENERAL_W { w: self }
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_bus_error(&mut self) -> I2C_BUS_ERROR_W {
        I2C_BUS_ERROR_W { w: self }
    }
    #[doc = "Bit 9 - SPI slave deselected after a write EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&mut self) -> SPI_EZ_WRITE_STOP_W {
        SPI_EZ_WRITE_STOP_W { w: self }
    }
    #[doc = "Bit 10 - SPI slave deselected after any EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_stop(&mut self) -> SPI_EZ_STOP_W {
        SPI_EZ_STOP_W { w: self }
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn spi_bus_error(&mut self) -> SPI_BUS_ERROR_W {
        SPI_BUS_ERROR_W { w: self }
    }
}
