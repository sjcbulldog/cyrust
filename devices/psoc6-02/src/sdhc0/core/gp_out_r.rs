#[doc = "Reader of register GP_OUT_R"]
pub type R = crate::R<u32, super::GP_OUT_R>;
#[doc = "Writer for register GP_OUT_R"]
pub type W = crate::W<u32, super::GP_OUT_R>;
#[doc = "Register GP_OUT_R `reset()`'s with value 0"]
impl crate::ResetValue for super::GP_OUT_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `CARD_DETECT_EN`"]
pub type CARD_DETECT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_DETECT_EN`"]
pub struct CARD_DETECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_DETECT_EN_W<'a> {
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
#[doc = "Reader of field `CARD_MECH_WRITE_PROT_EN`"]
pub type CARD_MECH_WRITE_PROT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_MECH_WRITE_PROT_EN`"]
pub struct CARD_MECH_WRITE_PROT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_MECH_WRITE_PROT_EN_W<'a> {
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
#[doc = "Reader of field `LED_CTRL_OE`"]
pub type LED_CTRL_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LED_CTRL_OE`"]
pub struct LED_CTRL_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_CTRL_OE_W<'a> {
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
#[doc = "Reader of field `CARD_CLOCK_OE`"]
pub type CARD_CLOCK_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_CLOCK_OE`"]
pub struct CARD_CLOCK_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_CLOCK_OE_W<'a> {
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
#[doc = "Reader of field `CARD_IF_PWR_EN_OE`"]
pub type CARD_IF_PWR_EN_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_IF_PWR_EN_OE`"]
pub struct CARD_IF_PWR_EN_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_IF_PWR_EN_OE_W<'a> {
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
#[doc = "Reader of field `IO_VOLT_SEL_OE`"]
pub type IO_VOLT_SEL_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO_VOLT_SEL_OE`"]
pub struct IO_VOLT_SEL_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_VOLT_SEL_OE_W<'a> {
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
#[doc = "Reader of field `CARD_CLOCK_OUT_DLY`"]
pub type CARD_CLOCK_OUT_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CARD_CLOCK_OUT_DLY`"]
pub struct CARD_CLOCK_OUT_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_CLOCK_OUT_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `CARD_CLOCK_IN_DLY`"]
pub type CARD_CLOCK_IN_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CARD_CLOCK_IN_DLY`"]
pub struct CARD_CLOCK_IN_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_CLOCK_IN_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
    #[inline(always)]
    pub fn card_detect_en(&self) -> CARD_DETECT_EN_R {
        CARD_DETECT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
    #[inline(always)]
    pub fn card_mech_write_prot_en(&self) -> CARD_MECH_WRITE_PROT_EN_R {
        CARD_MECH_WRITE_PROT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
    #[inline(always)]
    pub fn led_ctrl_oe(&self) -> LED_CTRL_OE_R {
        LED_CTRL_OE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
    #[inline(always)]
    pub fn card_clock_oe(&self) -> CARD_CLOCK_OE_R {
        CARD_CLOCK_OE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
    #[inline(always)]
    pub fn card_if_pwr_en_oe(&self) -> CARD_IF_PWR_EN_OE_R {
        CARD_IF_PWR_EN_OE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
    #[inline(always)]
    pub fn io_volt_sel_oe(&self) -> IO_VOLT_SEL_OE_R {
        IO_VOLT_SEL_OE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn card_clock_out_dly(&self) -> CARD_CLOCK_OUT_DLY_R {
        CARD_CLOCK_OUT_DLY_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
    #[inline(always)]
    pub fn card_clock_in_dly(&self) -> CARD_CLOCK_IN_DLY_R {
        CARD_CLOCK_IN_DLY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
    #[inline(always)]
    pub fn card_detect_en(&mut self) -> CARD_DETECT_EN_W {
        CARD_DETECT_EN_W { w: self }
    }
    #[doc = "Bit 1 - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
    #[inline(always)]
    pub fn card_mech_write_prot_en(&mut self) -> CARD_MECH_WRITE_PROT_EN_W {
        CARD_MECH_WRITE_PROT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
    #[inline(always)]
    pub fn led_ctrl_oe(&mut self) -> LED_CTRL_OE_W {
        LED_CTRL_OE_W { w: self }
    }
    #[doc = "Bit 3 - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
    #[inline(always)]
    pub fn card_clock_oe(&mut self) -> CARD_CLOCK_OE_W {
        CARD_CLOCK_OE_W { w: self }
    }
    #[doc = "Bit 4 - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
    #[inline(always)]
    pub fn card_if_pwr_en_oe(&mut self) -> CARD_IF_PWR_EN_OE_W {
        CARD_IF_PWR_EN_OE_W { w: self }
    }
    #[doc = "Bit 5 - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
    #[inline(always)]
    pub fn io_volt_sel_oe(&mut self) -> IO_VOLT_SEL_OE_W {
        IO_VOLT_SEL_OE_W { w: self }
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn card_clock_out_dly(&mut self) -> CARD_CLOCK_OUT_DLY_W {
        CARD_CLOCK_OUT_DLY_W { w: self }
    }
    #[doc = "Bits 8:9 - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
    #[inline(always)]
    pub fn card_clock_in_dly(&mut self) -> CARD_CLOCK_IN_DLY_W {
        CARD_CLOCK_IN_DLY_W { w: self }
    }
}
