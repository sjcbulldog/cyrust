#[doc = "Reader of register TEST"]
pub type R = crate::R<u32, super::TEST>;
#[doc = "Writer for register TEST"]
pub type W = crate::W<u32, super::TEST>;
#[doc = "Register TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAM`"]
pub type TAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAM`"]
pub struct TAM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAM_W<'a> {
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
#[doc = "Reader of field `TAT`"]
pub type TAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAT`"]
pub struct TAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAT_W<'a> {
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
#[doc = "Reader of field `CAM`"]
pub type CAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAM`"]
pub struct CAM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_W<'a> {
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
#[doc = "Reader of field `CAT`"]
pub type CAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAT`"]
pub struct CAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAT_W<'a> {
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
#[doc = "Reader of field `LBCK`"]
pub type LBCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBCK`"]
pub struct LBCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LBCK_W<'a> {
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
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ASC is not supported by M_TTCAN Test ASC Multiplexer Control Controls output pin m_ttcan_ascm in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_ascm controlled by FSE 1= Level at pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub fn tam(&self) -> TAM_R {
        TAM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ASC is not supported by M_TTCAN Test ASC Transmit Control Controls output pin m_ttcan_asct in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_asct controlled by FSE 1= Level at pin m_ttcan_asct = '1'"]
    #[inline(always)]
    pub fn tat(&self) -> TAT_R {
        TAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ASC is not supported by M_TTCAN Check ASC Multiplexer Control Monitors level at output pin m_ttcan_ascm. 0= Output pin m_ttcan_ascm = '0' 1= Output pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub fn cam(&self) -> CAM_R {
        CAM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ASC is not supported by M_TTCAN Check ASC Transmit Control Monitors level at output pin m_ttcan_asct. 0= Output pin m_ttcan_asct = '0'"]
    #[inline(always)]
    pub fn cat(&self) -> CAT_R {
        CAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled (see Section 3.1.9, Test Modes)"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin 00 Reset value, m_ttcan_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_ttcan_tx 10 Dominant ('0') level at pin m_ttcan_tx 11 Recessive ('1') at pin m_ttcan_tx"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Receive Pin Monitors the actual value of pin m_ttcan_rx 0= The CAN bus is dominant (m_ttcan_rx = '0') 1= The CAN bus is recessive (m_ttcan_rx = '1')"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ASC is not supported by M_TTCAN Test ASC Multiplexer Control Controls output pin m_ttcan_ascm in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_ascm controlled by FSE 1= Level at pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub fn tam(&mut self) -> TAM_W {
        TAM_W { w: self }
    }
    #[doc = "Bit 1 - ASC is not supported by M_TTCAN Test ASC Transmit Control Controls output pin m_ttcan_asct in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_asct controlled by FSE 1= Level at pin m_ttcan_asct = '1'"]
    #[inline(always)]
    pub fn tat(&mut self) -> TAT_W {
        TAT_W { w: self }
    }
    #[doc = "Bit 2 - ASC is not supported by M_TTCAN Check ASC Multiplexer Control Monitors level at output pin m_ttcan_ascm. 0= Output pin m_ttcan_ascm = '0' 1= Output pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub fn cam(&mut self) -> CAM_W {
        CAM_W { w: self }
    }
    #[doc = "Bit 3 - ASC is not supported by M_TTCAN Check ASC Transmit Control Monitors level at output pin m_ttcan_asct. 0= Output pin m_ttcan_asct = '0'"]
    #[inline(always)]
    pub fn cat(&mut self) -> CAT_W {
        CAT_W { w: self }
    }
    #[doc = "Bit 4 - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled (see Section 3.1.9, Test Modes)"]
    #[inline(always)]
    pub fn lbck(&mut self) -> LBCK_W {
        LBCK_W { w: self }
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin 00 Reset value, m_ttcan_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_ttcan_tx 10 Dominant ('0') level at pin m_ttcan_tx 11 Recessive ('1') at pin m_ttcan_tx"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
}
