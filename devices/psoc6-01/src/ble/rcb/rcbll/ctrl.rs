#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCBLL_CTRL`"]
pub type RCBLL_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCBLL_CTRL`"]
pub struct RCBLL_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RCBLL_CTRL_W<'a> {
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
#[doc = "Reader of field `RCBLL_CPU_REQ`"]
pub type RCBLL_CPU_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCBLL_CPU_REQ`"]
pub struct RCBLL_CPU_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RCBLL_CPU_REQ_W<'a> {
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
#[doc = "Reader of field `CPU_SINGLE_WRITE`"]
pub type CPU_SINGLE_WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_SINGLE_WRITE`"]
pub struct CPU_SINGLE_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_SINGLE_WRITE_W<'a> {
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
#[doc = "Reader of field `CPU_SINGLE_READ`"]
pub type CPU_SINGLE_READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_SINGLE_READ`"]
pub struct CPU_SINGLE_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_SINGLE_READ_W<'a> {
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
#[doc = "Reader of field `ALLOW_CPU_ACCESS_TX_RX`"]
pub type ALLOW_CPU_ACCESS_TX_RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALLOW_CPU_ACCESS_TX_RX`"]
pub struct ALLOW_CPU_ACCESS_TX_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOW_CPU_ACCESS_TX_RX_W<'a> {
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
#[doc = "Reader of field `ENABLE_RADIO_BOD`"]
pub type ENABLE_RADIO_BOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_RADIO_BOD`"]
pub struct ENABLE_RADIO_BOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_RADIO_BOD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RCB register access control 0: RCB registers can be accessed by CPU 1: RCB registers can be accessed by BLE Link Layer. FW sets this bit to give the access control to BLE link layer HW clears this bit to 0 to give the access control to CPU (HW clears this when the RCB controller is free abd RCB?_LL_CPU_REQ is set to 1)"]
    #[inline(always)]
    pub fn rcbll_ctrl(&self) -> RCBLL_CTRL_R {
        RCBLL_CTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RCB register access control request by CPU FW sets this bit to take the RCB register access control Once the HW is done with the current transactions, it clears this bit to give control to CPU And also indicates this by giving RCB_LL_DONE interrupt"]
    #[inline(always)]
    pub fn rcbll_cpu_req(&self) -> RCBLL_CPU_REQ_R {
        RCBLL_CPU_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn cpu_single_write(&self) -> CPU_SINGLE_WRITE_R {
        CPU_SINGLE_WRITE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn cpu_single_read(&self) -> CPU_SINGLE_READ_R {
        CPU_SINGLE_READ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates if CPU Single Read/Single Write are allowed when Radio RX/TX is ongoing. By default this bit is 0 and no RCB access from CPU are allowed during BLE RX/TX."]
    #[inline(always)]
    pub fn allow_cpu_access_tx_rx(&self) -> ALLOW_CPU_ACCESS_TX_RX_R {
        ALLOW_CPU_ACCESS_TX_RX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit indicates if the active logic in CYBLERD55 is reset on every TX/RX transaction."]
    #[inline(always)]
    pub fn enable_radio_bod(&self) -> ENABLE_RADIO_BOD_R {
        ENABLE_RADIO_BOD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RCB register access control 0: RCB registers can be accessed by CPU 1: RCB registers can be accessed by BLE Link Layer. FW sets this bit to give the access control to BLE link layer HW clears this bit to 0 to give the access control to CPU (HW clears this when the RCB controller is free abd RCB?_LL_CPU_REQ is set to 1)"]
    #[inline(always)]
    pub fn rcbll_ctrl(&mut self) -> RCBLL_CTRL_W {
        RCBLL_CTRL_W { w: self }
    }
    #[doc = "Bit 1 - RCB register access control request by CPU FW sets this bit to take the RCB register access control Once the HW is done with the current transactions, it clears this bit to give control to CPU And also indicates this by giving RCB_LL_DONE interrupt"]
    #[inline(always)]
    pub fn rcbll_cpu_req(&mut self) -> RCBLL_CPU_REQ_W {
        RCBLL_CPU_REQ_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn cpu_single_write(&mut self) -> CPU_SINGLE_WRITE_W {
        CPU_SINGLE_WRITE_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn cpu_single_read(&mut self) -> CPU_SINGLE_READ_W {
        CPU_SINGLE_READ_W { w: self }
    }
    #[doc = "Bit 4 - This bit indicates if CPU Single Read/Single Write are allowed when Radio RX/TX is ongoing. By default this bit is 0 and no RCB access from CPU are allowed during BLE RX/TX."]
    #[inline(always)]
    pub fn allow_cpu_access_tx_rx(&mut self) -> ALLOW_CPU_ACCESS_TX_RX_W {
        ALLOW_CPU_ACCESS_TX_RX_W { w: self }
    }
    #[doc = "Bit 5 - This bit indicates if the active logic in CYBLERD55 is reset on every TX/RX transaction."]
    #[inline(always)]
    pub fn enable_radio_bod(&mut self) -> ENABLE_RADIO_BOD_W {
        ENABLE_RADIO_BOD_W { w: self }
    }
}
