#[doc = "Reader of register INTR_RX"]
pub type R = crate::R<u32, super::INTR_RX>;
#[doc = "Writer for register INTR_RX"]
pub type W = crate::W<u32, super::INTR_RX>;
#[doc = "Register INTR_RX `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_RX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIGGER`"]
pub type TRIGGER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIGGER`"]
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
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
#[doc = "Reader of field `NOT_EMPTY`"]
pub type NOT_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOT_EMPTY`"]
pub struct NOT_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> NOT_EMPTY_W<'a> {
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
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULL`"]
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
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
#[doc = "Reader of field `OVERFLOW`"]
pub type OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERFLOW`"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
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
#[doc = "Reader of field `UNDERFLOW`"]
pub type UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDERFLOW`"]
pub struct UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_W<'a> {
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
#[doc = "Reader of field `BLOCKED`"]
pub type BLOCKED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCKED`"]
pub struct BLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKED_W<'a> {
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
#[doc = "Reader of field `FRAME_ERROR`"]
pub type FRAME_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAME_ERROR`"]
pub struct FRAME_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_ERROR_W<'a> {
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
#[doc = "Reader of field `PARITY_ERROR`"]
pub type PARITY_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARITY_ERROR`"]
pub struct PARITY_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ERROR_W<'a> {
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
#[doc = "Reader of field `BAUD_DETECT`"]
pub type BAUD_DETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BAUD_DETECT`"]
pub struct BAUD_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_DETECT_W<'a> {
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
#[doc = "Reader of field `BREAK_DETECT`"]
pub type BREAK_DETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BREAK_DETECT`"]
pub struct BREAK_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAK_DETECT_W<'a> {
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
    #[doc = "Bit 0 - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX FIFO is not empty. Only used in FIFO mode."]
    #[inline(always)]
    pub fn not_empty(&self) -> NOT_EMPTY_R {
        NOT_EMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    pub fn frame_error(&self) -> FRAME_ERROR_R {
        FRAME_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    pub fn parity_error(&self) -> PARITY_ERROR_R {
        PARITY_ERROR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BAUD_DETECT_R {
        BAUD_DETECT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn break_detect(&self) -> BREAK_DETECT_R {
        BREAK_DETECT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
    #[doc = "Bit 2 - RX FIFO is not empty. Only used in FIFO mode."]
    #[inline(always)]
    pub fn not_empty(&mut self) -> NOT_EMPTY_W {
        NOT_EMPTY_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
    #[doc = "Bit 5 - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 6 - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 7 - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn blocked(&mut self) -> BLOCKED_W {
        BLOCKED_W { w: self }
    }
    #[doc = "Bit 8 - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    pub fn frame_error(&mut self) -> FRAME_ERROR_W {
        FRAME_ERROR_W { w: self }
    }
    #[doc = "Bit 9 - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    pub fn parity_error(&mut self) -> PARITY_ERROR_W {
        PARITY_ERROR_W { w: self }
    }
    #[doc = "Bit 10 - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn baud_detect(&mut self) -> BAUD_DETECT_W {
        BAUD_DETECT_W { w: self }
    }
    #[doc = "Bit 11 - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn break_detect(&mut self) -> BREAK_DETECT_W {
        BREAK_DETECT_W { w: self }
    }
}
