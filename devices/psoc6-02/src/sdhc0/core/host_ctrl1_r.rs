#[doc = "Reader of register HOST_CTRL1_R"]
pub type R = crate::R<u8, super::HOST_CTRL1_R>;
#[doc = "Writer for register HOST_CTRL1_R"]
pub type W = crate::W<u8, super::HOST_CTRL1_R>;
#[doc = "Register HOST_CTRL1_R `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_CTRL1_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `LED_CTRL`"]
pub type LED_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LED_CTRL`"]
pub struct LED_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_CTRL_W<'a> {
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
#[doc = "Reader of field `DAT_XFER_WIDTH`"]
pub type DAT_XFER_WIDTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_XFER_WIDTH`"]
pub struct DAT_XFER_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_XFER_WIDTH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `HIGH_SPEED_EN`"]
pub type HIGH_SPEED_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIGH_SPEED_EN`"]
pub struct HIGH_SPEED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_SPEED_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMA_SEL`"]
pub type DMA_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_SEL`"]
pub struct DMA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u8) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `EXT_DAT_XFER`"]
pub type EXT_DAT_XFER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT_DAT_XFER`"]
pub struct EXT_DAT_XFER_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_DAT_XFER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CARD_DETECT_TEST_LVL`"]
pub type CARD_DETECT_TEST_LVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_DETECT_TEST_LVL`"]
pub struct CARD_DETECT_TEST_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_DETECT_TEST_LVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CARD_DETECT_SIG_SEL`"]
pub type CARD_DETECT_SIG_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_DETECT_SIG_SEL`"]
pub struct CARD_DETECT_SIG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_DETECT_SIG_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LED Control This bit is used to caution the user not to remove the card while the SD card is being accessed. The value is reflected on the led_ctrl ouput. Values: - 0x0 (OFF): LED off - 0x1 (ON): LED on"]
    #[inline(always)]
    pub fn led_ctrl(&self) -> LED_CTRL_R {
        LED_CTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. Values: - 0x1 (FOUR_BIT): 4-bit mode - 0x0 (ONE_BIT): 1-bit mode"]
    #[inline(always)]
    pub fn dat_xfer_width(&self) -> DAT_XFER_WIDTH_R {
        DAT_XFER_WIDTH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable (SD/eMMC Mode only) Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDHC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of card clock irrespective of this bit. Values: - 0x1 (HIGH_SPEED): High Speed mode - 0x0 (NORMAL_SPEED): Normal Speed mode"]
    #[inline(always)]
    pub fn high_speed_en(&self) -> HIGH_SPEED_EN_R {
        HIGH_SPEED_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    pub fn dma_sel(&self) -> DMA_SEL_R {
        DMA_SEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: - 0x1 (EIGHT_BIT): 8-bit Bus Width - 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
    #[inline(always)]
    pub fn ext_dat_xfer(&self) -> EXT_DAT_XFER_R {
        EXT_DAT_XFER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates whether a card inserted or not. Values: - 0x1 (CARD_INSERTED): Card Inserted - 0x0 (No_CARD): No Card"]
    #[inline(always)]
    pub fn card_detect_test_lvl(&self) -> CARD_DETECT_TEST_LVL_R {
        CARD_DETECT_TEST_LVL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection This bit selects a source for card detection. When the source for the card detection is switched, the interrupt must be disabled during the switching period. Values: - 0x1 (CARD_DT_TEST_LEVEL): Card Detect Test Level is selected (for test purpose) - 0x0 (card_detect_n): card_detect_n signal is selected (for normal use)"]
    #[inline(always)]
    pub fn card_detect_sig_sel(&self) -> CARD_DETECT_SIG_SEL_R {
        CARD_DETECT_SIG_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control This bit is used to caution the user not to remove the card while the SD card is being accessed. The value is reflected on the led_ctrl ouput. Values: - 0x0 (OFF): LED off - 0x1 (ON): LED on"]
    #[inline(always)]
    pub fn led_ctrl(&mut self) -> LED_CTRL_W {
        LED_CTRL_W { w: self }
    }
    #[doc = "Bit 1 - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. Values: - 0x1 (FOUR_BIT): 4-bit mode - 0x0 (ONE_BIT): 1-bit mode"]
    #[inline(always)]
    pub fn dat_xfer_width(&mut self) -> DAT_XFER_WIDTH_W {
        DAT_XFER_WIDTH_W { w: self }
    }
    #[doc = "Bit 2 - High Speed Enable (SD/eMMC Mode only) Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDHC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of card clock irrespective of this bit. Values: - 0x1 (HIGH_SPEED): High Speed mode - 0x0 (NORMAL_SPEED): Normal Speed mode"]
    #[inline(always)]
    pub fn high_speed_en(&mut self) -> HIGH_SPEED_EN_W {
        HIGH_SPEED_EN_W { w: self }
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    pub fn dma_sel(&mut self) -> DMA_SEL_W {
        DMA_SEL_W { w: self }
    }
    #[doc = "Bit 5 - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: - 0x1 (EIGHT_BIT): 8-bit Bus Width - 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
    #[inline(always)]
    pub fn ext_dat_xfer(&mut self) -> EXT_DAT_XFER_W {
        EXT_DAT_XFER_W { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates whether a card inserted or not. Values: - 0x1 (CARD_INSERTED): Card Inserted - 0x0 (No_CARD): No Card"]
    #[inline(always)]
    pub fn card_detect_test_lvl(&mut self) -> CARD_DETECT_TEST_LVL_W {
        CARD_DETECT_TEST_LVL_W { w: self }
    }
    #[doc = "Bit 7 - Card Detect Signal Selection This bit selects a source for card detection. When the source for the card detection is switched, the interrupt must be disabled during the switching period. Values: - 0x1 (CARD_DT_TEST_LEVEL): Card Detect Test Level is selected (for test purpose) - 0x0 (card_detect_n): card_detect_n signal is selected (for normal use)"]
    #[inline(always)]
    pub fn card_detect_sig_sel(&mut self) -> CARD_DETECT_SIG_SEL_W {
        CARD_DETECT_SIG_SEL_W { w: self }
    }
}
