#[doc = "Reader of register SRSS_INTR_MASK"]
pub type R = crate::R<u32, super::SRSS_INTR_MASK>;
#[doc = "Writer for register SRSS_INTR_MASK"]
pub type W = crate::W<u32, super::SRSS_INTR_MASK>;
#[doc = "Register SRSS_INTR_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::SRSS_INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_MATCH`"]
pub type WDT_MATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_MATCH`"]
pub struct WDT_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_MATCH_W<'a> {
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
#[doc = "Reader of field `HVLVD1`"]
pub type HVLVD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HVLVD1`"]
pub struct HVLVD1_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLVD1_W<'a> {
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
#[doc = "Reader of field `CLK_CAL`"]
pub type CLK_CAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_CAL`"]
pub struct CLK_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_CAL_W<'a> {
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
    #[doc = "Bit 0 - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WDT_MATCH_R {
        WDT_MATCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1(&self) -> HVLVD1_R {
        HVLVD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask for clock calibration done"]
    #[inline(always)]
    pub fn clk_cal(&self) -> CLK_CAL_R {
        CLK_CAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for watchdog timer. Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts. When WDT resets the chip, it also internally pends an interrupt that survives the reset. To prevent unintended ISR execution, clear SRSS_INTR.WDT_MATCH before setting this bit."]
    #[inline(always)]
    pub fn wdt_match(&mut self) -> WDT_MATCH_W {
        WDT_MATCH_W { w: self }
    }
    #[doc = "Bit 1 - Mask for low voltage detector HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1(&mut self) -> HVLVD1_W {
        HVLVD1_W { w: self }
    }
    #[doc = "Bit 5 - Mask for clock calibration done"]
    #[inline(always)]
    pub fn clk_cal(&mut self) -> CLK_CAL_W {
        CLK_CAL_W { w: self }
    }
}
