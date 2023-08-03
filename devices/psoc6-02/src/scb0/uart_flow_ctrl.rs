#[doc = "Reader of register UART_FLOW_CTRL"]
pub type R = crate::R<u32, super::UART_FLOW_CTRL>;
#[doc = "Writer for register UART_FLOW_CTRL"]
pub type W = crate::W<u32, super::UART_FLOW_CTRL>;
#[doc = "Register UART_FLOW_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_FLOW_CTRL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `TRIGGER_LEVEL`"]
pub type TRIGGER_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIGGER_LEVEL`"]
pub struct TRIGGER_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RTS_POLARITY`"]
pub type RTS_POLARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTS_POLARITY`"]
pub struct RTS_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTS_POLARITY`"]
pub type CTS_POLARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTS_POLARITY`"]
pub struct CTS_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CTS_ENABLED`"]
pub type CTS_ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTS_ENABLED`"]
pub struct CTS_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal 'uart_rts_out' is activated. By setting this field to '0', flow control is effectively SW disabled (may be useful for debug purposes)."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal 'uart_rts_out': '0': RTS is low/'0' active; 'uart_rts_out' is '0' when active and 'uart_rts_out' is '1' when inactive. '1': RTS is high/'1' active; 'uart_rts_out' is '1' when active and 'uart_rts_out' is '0' when inactive. During IP reset (Hibernate system power mode), 'uart_rts_out' is '1'. This represents an inactive state assuming a low/'0' active polarity."]
    #[inline(always)]
    pub fn rts_polarity(&self) -> RTS_POLARITY_R {
        RTS_POLARITY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal 'uart_cts_in': '0': CTS is low/'0' active; 'uart_cts_in' is '0' when active and 'uart_cts_in' is '1' when inactive. '1': CTS is high/'1' active; 'uart_cts_in' is '1' when active and 'uart_cts_in' is '0' when inactive."]
    #[inline(always)]
    pub fn cts_polarity(&self) -> CTS_POLARITY_R {
        CTS_POLARITY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable use of CTS input signal 'uart_cts_in' by the UART transmitter: '0': Disabled. The UART transmitter ignores 'uart_cts_in', and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses 'uart_cts_in' to qualify the transmission of data. It transmits when 'uart_cts_in' is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', 'uart_cts_in' is connected to 'uart_rts_out' in the IP (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    pub fn cts_enabled(&self) -> CTS_ENABLED_R {
        CTS_ENABLED_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal 'uart_rts_out' is activated. By setting this field to '0', flow control is effectively SW disabled (may be useful for debug purposes)."]
    #[inline(always)]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W {
        TRIGGER_LEVEL_W { w: self }
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal 'uart_rts_out': '0': RTS is low/'0' active; 'uart_rts_out' is '0' when active and 'uart_rts_out' is '1' when inactive. '1': RTS is high/'1' active; 'uart_rts_out' is '1' when active and 'uart_rts_out' is '0' when inactive. During IP reset (Hibernate system power mode), 'uart_rts_out' is '1'. This represents an inactive state assuming a low/'0' active polarity."]
    #[inline(always)]
    pub fn rts_polarity(&mut self) -> RTS_POLARITY_W {
        RTS_POLARITY_W { w: self }
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal 'uart_cts_in': '0': CTS is low/'0' active; 'uart_cts_in' is '0' when active and 'uart_cts_in' is '1' when inactive. '1': CTS is high/'1' active; 'uart_cts_in' is '1' when active and 'uart_cts_in' is '0' when inactive."]
    #[inline(always)]
    pub fn cts_polarity(&mut self) -> CTS_POLARITY_W {
        CTS_POLARITY_W { w: self }
    }
    #[doc = "Bit 25 - Enable use of CTS input signal 'uart_cts_in' by the UART transmitter: '0': Disabled. The UART transmitter ignores 'uart_cts_in', and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses 'uart_cts_in' to qualify the transmission of data. It transmits when 'uart_cts_in' is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', 'uart_cts_in' is connected to 'uart_rts_out' in the IP (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    pub fn cts_enabled(&mut self) -> CTS_ENABLED_W {
        CTS_ENABLED_W { w: self }
    }
}
