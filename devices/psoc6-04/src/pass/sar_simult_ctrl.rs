#[doc = "Reader of register SAR_SIMULT_CTRL"]
pub type R = crate::R<u32, super::SAR_SIMULT_CTRL>;
#[doc = "Writer for register SAR_SIMULT_CTRL"]
pub type W = crate::W<u32, super::SAR_SIMULT_CTRL>;
#[doc = "Register SAR_SIMULT_CTRL `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::SAR_SIMULT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0000
    }
}
#[doc = "Reader of field `SIMULT_HW_TR_EN`"]
pub type SIMULT_HW_TR_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMULT_HW_TR_EN`"]
pub struct SIMULT_HW_TR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMULT_HW_TR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Source for Simult Hardware trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIMULT_HW_TR_SRC_A {
    #[doc = "0: SAR 0 HW Trigger Input"]
    SAR_TR_IN_0 = 0,
    #[doc = "1: SAR 1 HW Trigger Input"]
    SAR_TR_IN_1 = 1,
    #[doc = "2: SAR 2 HW Trigger Input"]
    SAR_TR_IN_2 = 2,
    #[doc = "3: SAR 3 HW Trigger Input"]
    SAR_TR_IN_3 = 3,
}
impl From<SIMULT_HW_TR_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SIMULT_HW_TR_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIMULT_HW_TR_SRC`"]
pub type SIMULT_HW_TR_SRC_R = crate::R<u8, SIMULT_HW_TR_SRC_A>;
impl SIMULT_HW_TR_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIMULT_HW_TR_SRC_A {
        match self.bits {
            0 => SIMULT_HW_TR_SRC_A::SAR_TR_IN_0,
            1 => SIMULT_HW_TR_SRC_A::SAR_TR_IN_1,
            2 => SIMULT_HW_TR_SRC_A::SAR_TR_IN_2,
            3 => SIMULT_HW_TR_SRC_A::SAR_TR_IN_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAR_TR_IN_0`"]
    #[inline(always)]
    pub fn is_sar_tr_in_0(&self) -> bool {
        *self == SIMULT_HW_TR_SRC_A::SAR_TR_IN_0
    }
    #[doc = "Checks if the value of the field is `SAR_TR_IN_1`"]
    #[inline(always)]
    pub fn is_sar_tr_in_1(&self) -> bool {
        *self == SIMULT_HW_TR_SRC_A::SAR_TR_IN_1
    }
    #[doc = "Checks if the value of the field is `SAR_TR_IN_2`"]
    #[inline(always)]
    pub fn is_sar_tr_in_2(&self) -> bool {
        *self == SIMULT_HW_TR_SRC_A::SAR_TR_IN_2
    }
    #[doc = "Checks if the value of the field is `SAR_TR_IN_3`"]
    #[inline(always)]
    pub fn is_sar_tr_in_3(&self) -> bool {
        *self == SIMULT_HW_TR_SRC_A::SAR_TR_IN_3
    }
}
#[doc = "Write proxy for field `SIMULT_HW_TR_SRC`"]
pub struct SIMULT_HW_TR_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMULT_HW_TR_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIMULT_HW_TR_SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SAR 0 HW Trigger Input"]
    #[inline(always)]
    pub fn sar_tr_in_0(self) -> &'a mut W {
        self.variant(SIMULT_HW_TR_SRC_A::SAR_TR_IN_0)
    }
    #[doc = "SAR 1 HW Trigger Input"]
    #[inline(always)]
    pub fn sar_tr_in_1(self) -> &'a mut W {
        self.variant(SIMULT_HW_TR_SRC_A::SAR_TR_IN_1)
    }
    #[doc = "SAR 2 HW Trigger Input"]
    #[inline(always)]
    pub fn sar_tr_in_2(self) -> &'a mut W {
        self.variant(SIMULT_HW_TR_SRC_A::SAR_TR_IN_2)
    }
    #[doc = "SAR 3 HW Trigger Input"]
    #[inline(always)]
    pub fn sar_tr_in_3(self) -> &'a mut W {
        self.variant(SIMULT_HW_TR_SRC_A::SAR_TR_IN_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIMULT_HW_TR_TIMER_SEL`"]
pub type SIMULT_HW_TR_TIMER_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMULT_HW_TR_TIMER_SEL`"]
pub struct SIMULT_HW_TR_TIMER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMULT_HW_TR_TIMER_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SIMULT_HW_TR_LEVEL`"]
pub type SIMULT_HW_TR_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMULT_HW_TR_LEVEL`"]
pub struct SIMULT_HW_TR_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMULT_HW_TR_LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SIMULT_HW_SYNC_TR`"]
pub type SIMULT_HW_SYNC_TR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMULT_HW_SYNC_TR`"]
pub struct SIMULT_HW_SYNC_TR_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMULT_HW_SYNC_TR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SIMULT_TR_SCAN_CNT_SEL`"]
pub type SIMULT_TR_SCAN_CNT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMULT_TR_SCAN_CNT_SEL`"]
pub struct SIMULT_TR_SCAN_CNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMULT_TR_SCAN_CNT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SIMULT_EOS_INTR_SCAN_CNT_SEL`"]
pub type SIMULT_EOS_INTR_SCAN_CNT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMULT_EOS_INTR_SCAN_CNT_SEL`"]
pub struct SIMULT_EOS_INTR_SCAN_CNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMULT_EOS_INTR_SCAN_CNT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SAR simultaneous hardware triggering enable (one bit per SAR) -0: disabled -1: SAR trigger override enabled (SAR trigger set by SAR_OVR_CTRL register) <0>: Simultaneuous sampling enable for SAR0 <1>: Simultaneuous sampling enable for SAR1 <2>: Simultaneuous sampling enable for SAR2 <3>: Simultaneuous sampling enable for SAR3 Simultaneous sampling requires at least two bits in this field to be set. If less than two bits are set, this register will not affect SAR operation."]
    #[inline(always)]
    pub fn simult_hw_tr_en(&self) -> SIMULT_HW_TR_EN_R {
        SIMULT_HW_TR_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Source for Simult Hardware trigger"]
    #[inline(always)]
    pub fn simult_hw_tr_src(&self) -> SIMULT_HW_TR_SRC_R {
        SIMULT_HW_TR_SRC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - SAR hardware trigger source select -0: SIMULT_HW_TR_SRC -1: Timer trigger"]
    #[inline(always)]
    pub fn simult_hw_tr_timer_sel(&self) -> SIMULT_HW_TR_TIMER_SEL_R {
        SIMULT_HW_TR_TIMER_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 18 - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans. This field cannot be set when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn simult_hw_tr_level(&self) -> SIMULT_HW_TR_LEVEL_R {
        SIMULT_HW_TR_LEVEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - - 0: bypass clock domain synchronization of the Simult trigger signal. - 1: synchronize the Simult trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub fn simult_hw_sync_tr(&self) -> SIMULT_HW_SYNC_TR_R {
        SIMULT_HW_SYNC_TR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Simultaneous trigger sample select -0: Disabled -1: Enabled, SAR takes SAR_TR_SCAN_CNT per trigger (valid for both Firmware and Edge Senstive Hardware triggering, but ignored for Level Sensitive Hardware triggering and CONTINUOUS triggering) This feature cannot be enabled if either SAR is configured for Non-Tailgating Injection (SAR.INJ_CHAN_CONFIG.INJ_TAILGATING=0 while SAR.INJ_CHAN_CONFIG.INJ_START_EN=1)"]
    #[inline(always)]
    pub fn simult_tr_scan_cnt_sel(&self) -> SIMULT_TR_SCAN_CNT_SEL_R {
        SIMULT_TR_SCAN_CNT_SEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Simultaneous SAR EOS interrupt source select. This feature is not available for FW or Continuous triggering. -0: Legacy (SAR EOS is the source of the SAR EOS interrupt) -1: Enabled, SAR EOS interrupt only occurs for the EOS when sample=SAR_TR_SCAN_CNT.SCAN_CNT."]
    #[inline(always)]
    pub fn simult_eos_intr_scan_cnt_sel(&self) -> SIMULT_EOS_INTR_SCAN_CNT_SEL_R {
        SIMULT_EOS_INTR_SCAN_CNT_SEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SAR simultaneous hardware triggering enable (one bit per SAR) -0: disabled -1: SAR trigger override enabled (SAR trigger set by SAR_OVR_CTRL register) <0>: Simultaneuous sampling enable for SAR0 <1>: Simultaneuous sampling enable for SAR1 <2>: Simultaneuous sampling enable for SAR2 <3>: Simultaneuous sampling enable for SAR3 Simultaneous sampling requires at least two bits in this field to be set. If less than two bits are set, this register will not affect SAR operation."]
    #[inline(always)]
    pub fn simult_hw_tr_en(&mut self) -> SIMULT_HW_TR_EN_W {
        SIMULT_HW_TR_EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Source for Simult Hardware trigger"]
    #[inline(always)]
    pub fn simult_hw_tr_src(&mut self) -> SIMULT_HW_TR_SRC_W {
        SIMULT_HW_TR_SRC_W { w: self }
    }
    #[doc = "Bit 8 - SAR hardware trigger source select -0: SIMULT_HW_TR_SRC -1: Timer trigger"]
    #[inline(always)]
    pub fn simult_hw_tr_timer_sel(&mut self) -> SIMULT_HW_TR_TIMER_SEL_W {
        SIMULT_HW_TR_TIMER_SEL_W { w: self }
    }
    #[doc = "Bit 18 - - 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans. This field cannot be set when SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn simult_hw_tr_level(&mut self) -> SIMULT_HW_TR_LEVEL_W {
        SIMULT_HW_TR_LEVEL_W { w: self }
    }
    #[doc = "Bit 19 - - 0: bypass clock domain synchronization of the Simult trigger signal. - 1: synchronize the Simult trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub fn simult_hw_sync_tr(&mut self) -> SIMULT_HW_SYNC_TR_W {
        SIMULT_HW_SYNC_TR_W { w: self }
    }
    #[doc = "Bit 20 - Simultaneous trigger sample select -0: Disabled -1: Enabled, SAR takes SAR_TR_SCAN_CNT per trigger (valid for both Firmware and Edge Senstive Hardware triggering, but ignored for Level Sensitive Hardware triggering and CONTINUOUS triggering) This feature cannot be enabled if either SAR is configured for Non-Tailgating Injection (SAR.INJ_CHAN_CONFIG.INJ_TAILGATING=0 while SAR.INJ_CHAN_CONFIG.INJ_START_EN=1)"]
    #[inline(always)]
    pub fn simult_tr_scan_cnt_sel(&mut self) -> SIMULT_TR_SCAN_CNT_SEL_W {
        SIMULT_TR_SCAN_CNT_SEL_W { w: self }
    }
    #[doc = "Bit 21 - Simultaneous SAR EOS interrupt source select. This feature is not available for FW or Continuous triggering. -0: Legacy (SAR EOS is the source of the SAR EOS interrupt) -1: Enabled, SAR EOS interrupt only occurs for the EOS when sample=SAR_TR_SCAN_CNT.SCAN_CNT."]
    #[inline(always)]
    pub fn simult_eos_intr_scan_cnt_sel(&mut self) -> SIMULT_EOS_INTR_SCAN_CNT_SEL_W {
        SIMULT_EOS_INTR_SCAN_CNT_SEL_W { w: self }
    }
}
