#[doc = "Reader of register SAR_OVR_CTRL"]
pub type R = crate::R<u32, super::SAR_OVR_CTRL>;
#[doc = "Writer for register SAR_OVR_CTRL"]
pub type W = crate::W<u32, super::SAR_OVR_CTRL>;
#[doc = "Register SAR_OVR_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_OVR_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HW_TR_TIMER_SEL`"]
pub type HW_TR_TIMER_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HW_TR_TIMER_SEL`"]
pub struct HW_TR_TIMER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_TR_TIMER_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TR_SCAN_CNT_SEL`"]
pub type TR_SCAN_CNT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TR_SCAN_CNT_SEL`"]
pub struct TR_SCAN_CNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_SCAN_CNT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EOS_INTR_SCAN_CNT_SEL`"]
pub type EOS_INTR_SCAN_CNT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EOS_INTR_SCAN_CNT_SEL`"]
pub struct EOS_INTR_SCAN_CNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EOS_INTR_SCAN_CNT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SAR hardware trigger source select (one bit per SAR). SAR must be configured for hardware triggering (SAR.SAMPLE_CTRL.DSI_TRIGGER_EN must be set to 1). -0: Legacy (tr_sar_in_<N>) -1: Timer trigger <0>: HW Trigger source for SAR0 <1>: HW Trigger source for SAR1 <2>: HW Trigger source for SAR2 <3>: HW Trigger source for SAR3"]
    #[inline(always)]
    pub fn hw_tr_timer_sel(&self) -> HW_TR_TIMER_SEL_R {
        HW_TR_TIMER_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SAR trigger sample select (one bit per SAR). -0: Disabled -1: Enabled, SAR takes SAR_TR_SCAN_CNT per trigger (valid for both Firmware and Edge Senstive Hardware triggering, but ignored for Level Sensitive Hardware triggering and CONTINUOUS triggering). This feature cannot be enabled if the SAR is configured for Non-Tailgating Injection (SAR.INJ_CHAN_CONFIG.INJ_TAILGATING=0 while SAR.INJ_CHAN_CONFIG.INJ_START_EN=1) <0>: trigger sample select for SAR0 <1>: trigger sample select for SAR1 <2>: trigger sample select for SAR2 <3>: trigger sample select for SAR3"]
    #[inline(always)]
    pub fn tr_scan_cnt_sel(&self) -> TR_SCAN_CNT_SEL_R {
        TR_SCAN_CNT_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SAR EOS interrupt source select (one bit per SAR). This feature is not available for FW or Continuous triggering. -0: Legacy (SAR EOS is the source of the SAR EOS interrupt) -1: Enabled, SAR EOS interrupt only occurs for the EOS when sample=SAR_TR_SCAN_CNT.SCAN_CNT. <0>: EOS interrupt sample count select for SAR0 <1>: EOS interrupt sample count select for SAR1 <2>: EOS interrupt sample count select for SAR2 <3>: EOS interrupt sample count select for SAR3"]
    #[inline(always)]
    pub fn eos_intr_scan_cnt_sel(&self) -> EOS_INTR_SCAN_CNT_SEL_R {
        EOS_INTR_SCAN_CNT_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SAR hardware trigger source select (one bit per SAR). SAR must be configured for hardware triggering (SAR.SAMPLE_CTRL.DSI_TRIGGER_EN must be set to 1). -0: Legacy (tr_sar_in_<N>) -1: Timer trigger <0>: HW Trigger source for SAR0 <1>: HW Trigger source for SAR1 <2>: HW Trigger source for SAR2 <3>: HW Trigger source for SAR3"]
    #[inline(always)]
    pub fn hw_tr_timer_sel(&mut self) -> HW_TR_TIMER_SEL_W {
        HW_TR_TIMER_SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - SAR trigger sample select (one bit per SAR). -0: Disabled -1: Enabled, SAR takes SAR_TR_SCAN_CNT per trigger (valid for both Firmware and Edge Senstive Hardware triggering, but ignored for Level Sensitive Hardware triggering and CONTINUOUS triggering). This feature cannot be enabled if the SAR is configured for Non-Tailgating Injection (SAR.INJ_CHAN_CONFIG.INJ_TAILGATING=0 while SAR.INJ_CHAN_CONFIG.INJ_START_EN=1) <0>: trigger sample select for SAR0 <1>: trigger sample select for SAR1 <2>: trigger sample select for SAR2 <3>: trigger sample select for SAR3"]
    #[inline(always)]
    pub fn tr_scan_cnt_sel(&mut self) -> TR_SCAN_CNT_SEL_W {
        TR_SCAN_CNT_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - SAR EOS interrupt source select (one bit per SAR). This feature is not available for FW or Continuous triggering. -0: Legacy (SAR EOS is the source of the SAR EOS interrupt) -1: Enabled, SAR EOS interrupt only occurs for the EOS when sample=SAR_TR_SCAN_CNT.SCAN_CNT. <0>: EOS interrupt sample count select for SAR0 <1>: EOS interrupt sample count select for SAR1 <2>: EOS interrupt sample count select for SAR2 <3>: EOS interrupt sample count select for SAR3"]
    #[inline(always)]
    pub fn eos_intr_scan_cnt_sel(&mut self) -> EOS_INTR_SCAN_CNT_SEL_W {
        EOS_INTR_SCAN_CNT_SEL_W { w: self }
    }
}
