#[doc = "Reader of register UART_CTRL"]
pub type R = crate::R<u32, super::UART_CTRL>;
#[doc = "Writer for register UART_CTRL"]
pub type W = crate::W<u32, super::UART_CTRL>;
#[doc = "Register UART_CTRL `reset()`'s with value 0x0300_0000"]
impl crate::ResetValue for super::UART_CTRL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0000
    }
}
#[doc = "Reader of field `LOOPBACK`"]
pub type LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBACK`"]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Standard UART submode."]
    UART_STD = 0,
    #[doc = "1: SmartCard (ISO7816) submode. Support for negative acknowledgement (NACK) on the receiver side and retransmission on the transmitter side."]
    UART_SMARTCARD = 1,
    #[doc = "2: Infrared Data Association (IrDA) submode. Return to Zero modulation scheme."]
    UART_IRDA = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::UART_STD),
            1 => Val(MODE_A::UART_SMARTCARD),
            2 => Val(MODE_A::UART_IRDA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART_STD`"]
    #[inline(always)]
    pub fn is_uart_std(&self) -> bool {
        *self == MODE_A::UART_STD
    }
    #[doc = "Checks if the value of the field is `UART_SMARTCARD`"]
    #[inline(always)]
    pub fn is_uart_smartcard(&self) -> bool {
        *self == MODE_A::UART_SMARTCARD
    }
    #[doc = "Checks if the value of the field is `UART_IRDA`"]
    #[inline(always)]
    pub fn is_uart_irda(&self) -> bool {
        *self == MODE_A::UART_IRDA
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard UART submode."]
    #[inline(always)]
    pub fn uart_std(self) -> &'a mut W {
        self.variant(MODE_A::UART_STD)
    }
    #[doc = "SmartCard (ISO7816) submode. Support for negative acknowledgement (NACK) on the receiver side and retransmission on the transmitter side."]
    #[inline(always)]
    pub fn uart_smartcard(self) -> &'a mut W {
        self.variant(MODE_A::UART_SMARTCARD)
    }
    #[doc = "Infrared Data Association (IrDA) submode. Return to Zero modulation scheme."]
    #[inline(always)]
    pub fn uart_irda(self) -> &'a mut W {
        self.variant(MODE_A::UART_IRDA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). When '0', the transmitter TX line 'uart_tx_out' is connected to the TX pin and the receiver RX line 'uart_rx_in' is connected to the RX pin. When '1', the transmitter TX line 'uart_tx_out' is connected to the receiver RX line 'uart_rx_in'. A similar connections scheme is followed for 'uart_rts_out' and 'uart_cts_in'. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). When '0', the transmitter TX line 'uart_tx_out' is connected to the TX pin and the receiver RX line 'uart_rx_in' is connected to the RX pin. When '1', the transmitter TX line 'uart_tx_out' is connected to the receiver RX line 'uart_rx_in'. A similar connections scheme is followed for 'uart_rts_out' and 'uart_cts_in'. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
