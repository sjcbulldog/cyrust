#[doc = "Reader of register PWR_CTRL_R"]
pub type R = crate::R<u8, super::PWR_CTRL_R>;
#[doc = "Writer for register PWR_CTRL_R"]
pub type W = crate::W<u8, super::PWR_CTRL_R>;
#[doc = "Register PWR_CTRL_R `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_CTRL_R {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SD_BUS_PWR_VDD1`"]
pub type SD_BUS_PWR_VDD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SD_BUS_PWR_VDD1`"]
pub struct SD_BUS_PWR_VDD1_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_BUS_PWR_VDD1_W<'a> {
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
#[doc = "Reader of field `SD_BUS_VOL_VDD1`"]
pub type SD_BUS_VOL_VDD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SD_BUS_VOL_VDD1`"]
pub struct SD_BUS_VOL_VDD1_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_BUS_VOL_VDD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u8) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
    #[inline(always)]
    pub fn sd_bus_pwr_vdd1(&self) -> SD_BUS_PWR_VDD1_R {
        SD_BUS_PWR_VDD1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
    #[inline(always)]
    pub fn sd_bus_vol_vdd1(&self) -> SD_BUS_VOL_VDD1_R {
        SD_BUS_VOL_VDD1_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
    #[inline(always)]
    pub fn sd_bus_pwr_vdd1(&mut self) -> SD_BUS_PWR_VDD1_W {
        SD_BUS_PWR_VDD1_W { w: self }
    }
    #[doc = "Bits 1:3 - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
    #[inline(always)]
    pub fn sd_bus_vol_vdd1(&mut self) -> SD_BUS_VOL_VDD1_W {
        SD_BUS_VOL_VDD1_W { w: self }
    }
}
