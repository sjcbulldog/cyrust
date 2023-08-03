#[doc = "Reader of register ANA_PWR_CFG"]
pub type R = crate::R<u32, super::ANA_PWR_CFG>;
#[doc = "Writer for register ANA_PWR_CFG"]
pub type W = crate::W<u32, super::ANA_PWR_CFG>;
#[doc = "Register ANA_PWR_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_PWR_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWR_UP_DELAY`"]
pub type PWR_UP_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWR_UP_DELAY`"]
pub struct PWR_UP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_UP_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DUTY_CYCLE_SAR_ACT_EN`"]
pub type DUTY_CYCLE_SAR_ACT_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DUTY_CYCLE_SAR_ACT_EN`"]
pub struct DUTY_CYCLE_SAR_ACT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CYCLE_SAR_ACT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Power up time for analog blocks. Fastest power up time is achieved with a setting of 0. Additional time can be added to allow for analog settling. The power up time is in clk_dpslp cycles. This field is only applicable when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn pwr_up_delay(&self) -> PWR_UP_DELAY_R {
        PWR_UP_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Enable for powering down (duty cycling) a SAR in chip active mode. This is a risk mitigation bit for power reduction in chip active mode. In order for this field affect SAR operation, the SAR must be configured for deepsleep clocking (SAR_CLOCK_SEL.CLOCK_SEL set to 1), and the SAR must be set for Timer-based hardware triggering (either by following the guidlines in SAR_OVR_CTRL.HW_TR_TIMER_SEL or SAR_SIMULT_CTRL.SIMULT_HW_TIMER_SEL). If this bit is set for a given SAR, the Timer is the only valid trigger source. Non-timer based Hardware (DSI) triggers cannot be used nor can Firmware based triggers (FW,Continuous,Injection). Furthermore, trigger collision functionality will be limited to interrupt generation only. -0: Legacy (SAR not duty cycled in chip active mode -1: SAR duty cycled in chip active mode <0>: Active Mode Duty Cycle enable for SAR0 <1>: Active Mode Duty Cycle enable for SAR1 <2>: Active Mode Duty Cycle enable for SAR2 <3>: Active Mode Duty Cycle enable for SAR3 This field is only applicable when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn duty_cycle_sar_act_en(&self) -> DUTY_CYCLE_SAR_ACT_EN_R {
        DUTY_CYCLE_SAR_ACT_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Power up time for analog blocks. Fastest power up time is achieved with a setting of 0. Additional time can be added to allow for analog settling. The power up time is in clk_dpslp cycles. This field is only applicable when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn pwr_up_delay(&mut self) -> PWR_UP_DELAY_W {
        PWR_UP_DELAY_W { w: self }
    }
    #[doc = "Bits 8:11 - Enable for powering down (duty cycling) a SAR in chip active mode. This is a risk mitigation bit for power reduction in chip active mode. In order for this field affect SAR operation, the SAR must be configured for deepsleep clocking (SAR_CLOCK_SEL.CLOCK_SEL set to 1), and the SAR must be set for Timer-based hardware triggering (either by following the guidlines in SAR_OVR_CTRL.HW_TR_TIMER_SEL or SAR_SIMULT_CTRL.SIMULT_HW_TIMER_SEL). If this bit is set for a given SAR, the Timer is the only valid trigger source. Non-timer based Hardware (DSI) triggers cannot be used nor can Firmware based triggers (FW,Continuous,Injection). Furthermore, trigger collision functionality will be limited to interrupt generation only. -0: Legacy (SAR not duty cycled in chip active mode -1: SAR duty cycled in chip active mode <0>: Active Mode Duty Cycle enable for SAR0 <1>: Active Mode Duty Cycle enable for SAR1 <2>: Active Mode Duty Cycle enable for SAR2 <3>: Active Mode Duty Cycle enable for SAR3 This field is only applicable when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn duty_cycle_sar_act_en(&mut self) -> DUTY_CYCLE_SAR_ACT_EN_W {
        DUTY_CYCLE_SAR_ACT_EN_W { w: self }
    }
}
