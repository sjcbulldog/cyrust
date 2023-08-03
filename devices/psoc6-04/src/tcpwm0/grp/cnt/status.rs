#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `DOWN`"]
pub type DOWN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TR_CAPTURE0`"]
pub type TR_CAPTURE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TR_COUNT`"]
pub type TR_COUNT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TR_RELOAD`"]
pub type TR_RELOAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `TR_STOP`"]
pub type TR_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TR_START`"]
pub type TR_START_R = crate::R<bool, bool>;
#[doc = "Reader of field `TR_CAPTURE1`"]
pub type TR_CAPTURE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINE_OUT`"]
pub type LINE_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINE_COMPL_OUT`"]
pub type LINE_COMPL_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RUNNING`"]
pub type RUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `DT_CNT_L`"]
pub type DT_CNT_L_R = crate::R<u8, u8>;
#[doc = "Reader of field `DT_CNT_H`"]
pub type DT_CNT_H_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates the actual level of the selected capture 0 trigger."]
    #[inline(always)]
    pub fn tr_capture0(&self) -> TR_CAPTURE0_R {
        TR_CAPTURE0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates the actual level of the selected count trigger."]
    #[inline(always)]
    pub fn tr_count(&self) -> TR_COUNT_R {
        TR_COUNT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates the actual level of the selected reload trigger."]
    #[inline(always)]
    pub fn tr_reload(&self) -> TR_RELOAD_R {
        TR_RELOAD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates the actual level of the selected stop trigger."]
    #[inline(always)]
    pub fn tr_stop(&self) -> TR_STOP_R {
        TR_STOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates the actual level of the selected start trigger."]
    #[inline(always)]
    pub fn tr_start(&self) -> TR_START_R {
        TR_START_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates the actual level of the selected capture 1 trigger."]
    #[inline(always)]
    pub fn tr_capture1(&self) -> TR_CAPTURE1_R {
        TR_CAPTURE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates the actual level of the PWM line output signal."]
    #[inline(always)]
    pub fn line_out(&self) -> LINE_OUT_R {
        LINE_OUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Indicates the actual level of the complementary PWM line output signal."]
    #[inline(always)]
    pub fn line_compl_out(&self) -> LINE_COMPL_OUT_R {
        LINE_COMPL_OUT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - When '0', the counter is NOT running. When '1', the counter is running. This field is used to indicate that the counter is running after a start/reload event and that the counter is stopped after a stop event. When a running counter operation is paused in debug state (see CTRL.DBG_PAUSE) then the RUNNING bit is still '1'."]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion (8bit dead time counter or low byte of 16-bit dead time counter). In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
    #[inline(always)]
    pub fn dt_cnt_l(&self) -> DT_CNT_L_R {
        DT_CNT_L_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - High byte of 16-bit dead time counter. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8bit wide and the only the field DT_CNT_L is used as dead time counter."]
    #[inline(always)]
    pub fn dt_cnt_h(&self) -> DT_CNT_H_R {
        DT_CNT_H_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
