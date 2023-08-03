#[doc = "Reader of register TTOCN"]
pub type R = crate::R<u32, super::TTOCN>;
#[doc = "Writer for register TTOCN"]
pub type W = crate::W<u32, super::TTOCN>;
#[doc = "Register TTOCN `reset()`'s with value 0"]
impl crate::ResetValue for super::TTOCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SGT`"]
pub type SGT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SGT`"]
pub struct SGT_W<'a> {
    w: &'a mut W,
}
impl<'a> SGT_W<'a> {
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
#[doc = "Reader of field `ECS`"]
pub type ECS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECS`"]
pub struct ECS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECS_W<'a> {
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
#[doc = "Reader of field `SWP`"]
pub type SWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWP`"]
pub struct SWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWP_W<'a> {
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
#[doc = "Reader of field `SWS`"]
pub type SWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWS`"]
pub struct SWS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `RTIE`"]
pub type RTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTIE`"]
pub struct RTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIE_W<'a> {
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
#[doc = "Reader of field `TMC`"]
pub type TMC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMC`"]
pub struct TMC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `TTIE`"]
pub type TTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TTIE`"]
pub struct TTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TTIE_W<'a> {
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
#[doc = "Reader of field `GCS`"]
pub type GCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCS`"]
pub struct GCS_W<'a> {
    w: &'a mut W,
}
impl<'a> GCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `FGP`"]
pub type FGP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FGP`"]
pub struct FGP_W<'a> {
    w: &'a mut W,
}
impl<'a> FGP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TMG`"]
pub type TMG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMG`"]
pub struct TMG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `NIG`"]
pub type NIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIG`"]
pub struct NIG_W<'a> {
    w: &'a mut W,
}
impl<'a> NIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ESCN`"]
pub type ESCN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESCN`"]
pub struct ESCN_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `LCKC`"]
pub type LCKC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Set Global time Writing a '1' to SGT sets TTOST.WGDT if the node is the actual Time Master. SGT is reset after one Host clock period. The global time preset takes effect when the node transmits the next reference message with the Master_Ref_Mark modified by the preset value written to TTGTP."]
    #[inline(always)]
    pub fn sgt(&self) -> SGT_R {
        SGT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Clock Synchronization Writing a '1' to ECS sets TTOST.WECS if the node is the actual Time Master. ECS is reset after one Host clock period. The external clock synchronization takes effect at the start of the next basic cycle."]
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop Watch Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Stop Watch Source 00= Stop Watch disabled 01= Actual value of cycle time is copied to TTCPT.SWV 10= Actual value of local time is copied to TTCPT.SWV 11= Actual value of global time is copied to TTCPT.SWV"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable Register time mark interrupts are configured by register TTTMK. A register time mark interrupt pulse with the length of one NTU is generated when the time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Register Time Mark Interrupt output m_ttcan_rtp disabled 1= Register Time Mark Interrupt output m_ttcan_rtp enabled"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare 00= No Register Time Mark Interrupt generated 01= Register Time Mark Interrupt if Time Mark = cycle time 10= Register Time Mark Interrupt if Time Mark = local time 11= Register Time Mark Interrupt if Time Mark = global time"]
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable External time mark events are configured by trigger memory element TMEX (see Section 2.4.7). A trigger time mark interrupt pulse is generated when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Schedule or In_Gap. 0= Trigger Time Mark Interrupt output m_ttcan_tmp disabled 1= Trigger Time Mark Interrupt output m_ttcan_tmp enabled"]
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Gap Control Select 0= Gap control independent from m_ttcan_evt 1= Gap control by input pin m_ttcan_evt"]
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Finish Gap Set by the CPU, reset by each reference message 0= No reference message requested 1= Application requested start of reference message"]
    #[inline(always)]
    pub fn fgp(&self) -> FGP_R {
        FGP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Time Mark Gap 0= Reset by each reference message 1= Next reference message started when Register Time Mark interrupt TTIR.RTMI is activated"]
    #[inline(always)]
    pub fn tmg(&self) -> TMG_R {
        TMG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Next is Gap This bit can only be set when the M_TTCAN is the actual Time Master and when it is configured for external event-synchronized time-triggered operation (TTOCF.GEN = '1') 0= No action, reset by reception of any reference message 1= Transmit next reference message with Next_is_Gap = '1'"]
    #[inline(always)]
    pub fn nig(&self) -> NIG_R {
        NIG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Synchronization Control If enabled the M_TTCAN synchronizes its cycle time phase to an external event signaled by a rising edge at pin m_ttcan_evt (see Section 4.11). 0= External synchronization disabled 1= External synchronization enabled"]
    #[inline(always)]
    pub fn escn(&self) -> ESCN_R {
        ESCN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TT Operation Control Register Locked Set by a write access to register TTOCN. Reset when the updated configuration has been synchronized into the CAN clock domain. 0= Write access to TTOCN enabled 1= Write access to TTOCN locked"]
    #[inline(always)]
    pub fn lckc(&self) -> LCKC_R {
        LCKC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set Global time Writing a '1' to SGT sets TTOST.WGDT if the node is the actual Time Master. SGT is reset after one Host clock period. The global time preset takes effect when the node transmits the next reference message with the Master_Ref_Mark modified by the preset value written to TTGTP."]
    #[inline(always)]
    pub fn sgt(&mut self) -> SGT_W {
        SGT_W { w: self }
    }
    #[doc = "Bit 1 - External Clock Synchronization Writing a '1' to ECS sets TTOST.WECS if the node is the actual Time Master. ECS is reset after one Host clock period. The external clock synchronization takes effect at the start of the next basic cycle."]
    #[inline(always)]
    pub fn ecs(&mut self) -> ECS_W {
        ECS_W { w: self }
    }
    #[doc = "Bit 2 - Stop Watch Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub fn swp(&mut self) -> SWP_W {
        SWP_W { w: self }
    }
    #[doc = "Bits 3:4 - Stop Watch Source 00= Stop Watch disabled 01= Actual value of cycle time is copied to TTCPT.SWV 10= Actual value of local time is copied to TTCPT.SWV 11= Actual value of global time is copied to TTCPT.SWV"]
    #[inline(always)]
    pub fn sws(&mut self) -> SWS_W {
        SWS_W { w: self }
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable Register time mark interrupts are configured by register TTTMK. A register time mark interrupt pulse with the length of one NTU is generated when the time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Register Time Mark Interrupt output m_ttcan_rtp disabled 1= Register Time Mark Interrupt output m_ttcan_rtp enabled"]
    #[inline(always)]
    pub fn rtie(&mut self) -> RTIE_W {
        RTIE_W { w: self }
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare 00= No Register Time Mark Interrupt generated 01= Register Time Mark Interrupt if Time Mark = cycle time 10= Register Time Mark Interrupt if Time Mark = local time 11= Register Time Mark Interrupt if Time Mark = global time"]
    #[inline(always)]
    pub fn tmc(&mut self) -> TMC_W {
        TMC_W { w: self }
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable External time mark events are configured by trigger memory element TMEX (see Section 2.4.7). A trigger time mark interrupt pulse is generated when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Schedule or In_Gap. 0= Trigger Time Mark Interrupt output m_ttcan_tmp disabled 1= Trigger Time Mark Interrupt output m_ttcan_tmp enabled"]
    #[inline(always)]
    pub fn ttie(&mut self) -> TTIE_W {
        TTIE_W { w: self }
    }
    #[doc = "Bit 9 - Gap Control Select 0= Gap control independent from m_ttcan_evt 1= Gap control by input pin m_ttcan_evt"]
    #[inline(always)]
    pub fn gcs(&mut self) -> GCS_W {
        GCS_W { w: self }
    }
    #[doc = "Bit 10 - Finish Gap Set by the CPU, reset by each reference message 0= No reference message requested 1= Application requested start of reference message"]
    #[inline(always)]
    pub fn fgp(&mut self) -> FGP_W {
        FGP_W { w: self }
    }
    #[doc = "Bit 11 - Time Mark Gap 0= Reset by each reference message 1= Next reference message started when Register Time Mark interrupt TTIR.RTMI is activated"]
    #[inline(always)]
    pub fn tmg(&mut self) -> TMG_W {
        TMG_W { w: self }
    }
    #[doc = "Bit 12 - Next is Gap This bit can only be set when the M_TTCAN is the actual Time Master and when it is configured for external event-synchronized time-triggered operation (TTOCF.GEN = '1') 0= No action, reset by reception of any reference message 1= Transmit next reference message with Next_is_Gap = '1'"]
    #[inline(always)]
    pub fn nig(&mut self) -> NIG_W {
        NIG_W { w: self }
    }
    #[doc = "Bit 13 - External Synchronization Control If enabled the M_TTCAN synchronizes its cycle time phase to an external event signaled by a rising edge at pin m_ttcan_evt (see Section 4.11). 0= External synchronization disabled 1= External synchronization enabled"]
    #[inline(always)]
    pub fn escn(&mut self) -> ESCN_W {
        ESCN_W { w: self }
    }
}
