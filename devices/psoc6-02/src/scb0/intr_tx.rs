#[doc = "Reader of register INTR_TX"]
pub type R = crate::R<u32, super::INTR_TX>;
#[doc = "Writer for register INTR_TX"]
pub type W = crate::W<u32, super::INTR_TX>;
#[doc = "Register INTR_TX `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_TX {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
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
#[doc = "Reader of field `NOT_FULL`"]
pub type NOT_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOT_FULL`"]
pub struct NOT_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> NOT_FULL_W<'a> {
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
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMPTY`"]
pub struct EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTY_W<'a> {
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
#[doc = "Reader of field `UART_NACK`"]
pub type UART_NACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_NACK`"]
pub struct UART_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_NACK_W<'a> {
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
#[doc = "Reader of field `UART_DONE`"]
pub type UART_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_DONE`"]
pub struct UART_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DONE_W<'a> {
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
#[doc = "Reader of field `UART_ARB_LOST`"]
pub type UART_ARB_LOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_ARB_LOST`"]
pub struct UART_ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_ARB_LOST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn not_full(&self) -> NOT_FULL_R {
        NOT_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_nack(&self) -> UART_NACK_R {
        UART_NACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty). Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_done(&self) -> UART_DONE_R {
        UART_DONE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is useful when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UART_ARB_LOST_R {
        UART_ARB_LOST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
    #[doc = "Bit 1 - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn not_full(&mut self) -> NOT_FULL_W {
        NOT_FULL_W { w: self }
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W {
        EMPTY_W { w: self }
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 7 - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn blocked(&mut self) -> BLOCKED_W {
        BLOCKED_W { w: self }
    }
    #[doc = "Bit 8 - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_nack(&mut self) -> UART_NACK_W {
        UART_NACK_W { w: self }
    }
    #[doc = "Bit 9 - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty). Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_done(&mut self) -> UART_DONE_W {
        UART_DONE_W { w: self }
    }
    #[doc = "Bit 10 - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is useful when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_arb_lost(&mut self) -> UART_ARB_LOST_W {
        UART_ARB_LOST_W { w: self }
    }
}
