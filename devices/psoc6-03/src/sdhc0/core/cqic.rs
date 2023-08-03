#[doc = "Reader of register CQIC"]
pub type R = crate::R<u32, super::CQIC>;
#[doc = "Writer for register CQIC"]
pub type W = crate::W<u32, super::CQIC>;
#[doc = "Register CQIC `reset()`'s with value 0"]
impl crate::ResetValue for super::CQIC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOUT_VAL`"]
pub type TOUT_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOUT_VAL`"]
pub struct TOUT_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Write proxy for field `TOUT_VAL_WEN`"]
pub struct TOUT_VAL_WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_VAL_WEN_W<'a> {
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
#[doc = "Write proxy for field `INTC_TH`"]
pub struct INTC_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> INTC_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `INTC_TH_WEN`"]
pub struct INTC_TH_WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTC_TH_WEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `INTC_RST`"]
pub struct INTC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> INTC_RST_W<'a> {
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
#[doc = "Reader of field `INTC_STAT`"]
pub type INTC_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTC_EN`"]
pub type INTC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTC_EN`"]
pub struct INTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Interrupt Coalescing Timeout Value Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when the first data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field, it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. - 0x0: Timer is disabled. Timeout-based interrupt is not generated - 0x1: Timeout on 01x1024 cycles of timer clock frequency - 0x2: Timeout on 02x1024 cycles of timer clock frequency - ........ - 0x7f: Timeout on 127x1024 cycles of timer clock frequency In order to write to this field, the TOUT_VAL_WEN bit must be set at the same write operation."]
    #[inline(always)]
    pub fn tout_val(&self) -> TOUT_VAL_R {
        TOUT_VAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 20 - Interrupt Coalescing Status Bit This bit indicates to the software whether any tasks (with INT=0) have completed and counted towards interrupt coalescing (that is, this is set if and only if INTC counter > 0). Values: - 0x1 (INTC_ATLEAST1_COMP): At least one INT0 task completion has been counted (INTC counter > 0) - 0x0 (INTC_NO_TASK_COMP): INT0 Task completions have not occurred since last counter reset (INTC counter == 0)"]
    #[inline(always)]
    pub fn intc_stat(&self) -> INTC_STAT_R {
        INTC_STAT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable Bit Values: - 0x1 (ENABLE_INT_COALESCING): Interrupt coalescing mechanism is active. Interrupts are counted and timed, and coalesced interrupts are generated - 0x0 (DISABLE_INT_COALESCING): Interrupt coalescing mechanism is disabled (Default)."]
    #[inline(always)]
    pub fn intc_en(&self) -> INTC_EN_R {
        INTC_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Interrupt Coalescing Timeout Value Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when the first data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field, it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. - 0x0: Timer is disabled. Timeout-based interrupt is not generated - 0x1: Timeout on 01x1024 cycles of timer clock frequency - 0x2: Timeout on 02x1024 cycles of timer clock frequency - ........ - 0x7f: Timeout on 127x1024 cycles of timer clock frequency In order to write to this field, the TOUT_VAL_WEN bit must be set at the same write operation."]
    #[inline(always)]
    pub fn tout_val(&mut self) -> TOUT_VAL_W {
        TOUT_VAL_W { w: self }
    }
    #[doc = "Bit 7 - When software writes 1 to this bit, the value TOUT_VAL is updated with the contents written on the same cycle. Values: - 0x1 (WEN_SET): Sets TOUT_VAL_WEN - 0x0 (WEN_CLR): clears TOUT_VAL_WEN"]
    #[inline(always)]
    pub fn tout_val_wen(&mut self) -> TOUT_VAL_WEN_W {
        TOUT_VAL_WEN_W { w: self }
    }
    #[doc = "Bits 8:12 - Interrupt Coalescing Counter Threshold filed Software uses this field to configure the number of task completions (only tasks with INT=0 in the Task Descriptor), which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted by CQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in INTC_TH, and generates interrupt. - 0x0: Interrupt coalescing feature disabled - 0x1: Interrupt coalescing interrupt generated after 1 task when INT=0 completes - 0x2: Interrupt coalescing interrupt generated after 2 tasks when INT=0 completes - ........ - 0x1f: Interrupt coalescing interrupt generated after 31 tasks when INT=0 completes To write to this field, the INTC_TH_WEN bit must be set during the same write operation."]
    #[inline(always)]
    pub fn intc_th(&mut self) -> INTC_TH_W {
        INTC_TH_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt Coalescing Counter Threshold Write Enable When software writes 1 to this bit, the value INTC_TH is updated with the contents written on the same cycle. Values: - 0x1 (WEN_SET): Sets INTC_TH_WEN - 0x0 (WEN_CLR): Clears INTC_TH_WEN"]
    #[inline(always)]
    pub fn intc_th_wen(&mut self) -> INTC_TH_WEN_W {
        INTC_TH_WEN_W { w: self }
    }
    #[doc = "Bit 16 - Counter and Timer Reset When host driver writes 1, the interrupt coalescing timer and counter are reset. Values: - 0x1 (ASSERT_INTC_RESET): Interrupt coalescing timer and counter are reset - 0x0 (NO_EFFECT): No Effect"]
    #[inline(always)]
    pub fn intc_rst(&mut self) -> INTC_RST_W {
        INTC_RST_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable Bit Values: - 0x1 (ENABLE_INT_COALESCING): Interrupt coalescing mechanism is active. Interrupts are counted and timed, and coalesced interrupts are generated - 0x0 (DISABLE_INT_COALESCING): Interrupt coalescing mechanism is disabled (Default)."]
    #[inline(always)]
    pub fn intc_en(&mut self) -> INTC_EN_W {
        INTC_EN_W { w: self }
    }
}
