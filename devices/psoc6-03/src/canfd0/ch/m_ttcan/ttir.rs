#[doc = "Reader of register TTIR"]
pub type R = crate::R<u32, super::TTIR>;
#[doc = "Writer for register TTIR"]
pub type W = crate::W<u32, super::TTIR>;
#[doc = "Register TTIR `reset()`'s with value 0"]
impl crate::ResetValue for super::TTIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SBC`"]
pub type SBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBC`"]
pub struct SBC_W<'a> {
    w: &'a mut W,
}
impl<'a> SBC_W<'a> {
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
#[doc = "Reader of field `SMC`"]
pub type SMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMC`"]
pub struct SMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_W<'a> {
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
#[doc = "Reader of field `CSM_`"]
pub type CSM__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSM_`"]
pub struct CSM__W<'a> {
    w: &'a mut W,
}
impl<'a> CSM__W<'a> {
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
#[doc = "Reader of field `SOG`"]
pub type SOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOG`"]
pub struct SOG_W<'a> {
    w: &'a mut W,
}
impl<'a> SOG_W<'a> {
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
#[doc = "Reader of field `RTMI`"]
pub type RTMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTMI`"]
pub struct RTMI_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMI_W<'a> {
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
#[doc = "Reader of field `TTMI`"]
pub type TTMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TTMI`"]
pub struct TTMI_W<'a> {
    w: &'a mut W,
}
impl<'a> TTMI_W<'a> {
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
#[doc = "Reader of field `SWE`"]
pub type SWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWE`"]
pub struct SWE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GTW`"]
pub type GTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTW`"]
pub struct GTW_W<'a> {
    w: &'a mut W,
}
impl<'a> GTW_W<'a> {
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
#[doc = "Reader of field `GTD`"]
pub type GTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTD`"]
pub struct GTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GTD_W<'a> {
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
#[doc = "Reader of field `GTE`"]
pub type GTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTE`"]
pub struct GTE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTE_W<'a> {
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
#[doc = "Reader of field `TXU`"]
pub type TXU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXU`"]
pub struct TXU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXU_W<'a> {
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
#[doc = "Reader of field `TXO`"]
pub type TXO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXO`"]
pub struct TXO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXO_W<'a> {
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
#[doc = "Reader of field `SE1`"]
pub type SE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE1`"]
pub struct SE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1_W<'a> {
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
#[doc = "Reader of field `SE2`"]
pub type SE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE2`"]
pub struct SE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2_W<'a> {
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
#[doc = "Reader of field `ELC`"]
pub type ELC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELC`"]
pub struct ELC_W<'a> {
    w: &'a mut W,
}
impl<'a> ELC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `IWT`"]
pub type IWT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWT`"]
pub struct IWT_W<'a> {
    w: &'a mut W,
}
impl<'a> IWT_W<'a> {
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
#[doc = "Reader of field `WT`"]
pub type WT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WT`"]
pub struct WT_W<'a> {
    w: &'a mut W,
}
impl<'a> WT_W<'a> {
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
#[doc = "Reader of field `AW`"]
pub type AW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AW`"]
pub struct AW_W<'a> {
    w: &'a mut W,
}
impl<'a> AW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CER`"]
pub type CER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CER`"]
pub struct CER_W<'a> {
    w: &'a mut W,
}
impl<'a> CER_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle 0= No Basic Cycle started since bit has been reset 1= Basic Cycle started"]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle 0= No Matrix Cycle started since bit has been reset 1= Matrix Cycle started"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode 0= No change in master to slave relation or schedule synchronization 1= Master to slave relation or schedule synchronization changed, also set when TTOST.SPL is reset"]
    #[inline(always)]
    pub fn csm_(&self) -> CSM__R {
        CSM__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Gap 0= No reference message seen with Next_is_Gap bit set 1= Reference message with Next_is_Gap bit set becomes valid"]
    #[inline(always)]
    pub fn sog(&self) -> SOG_R {
        SOG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Set when time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Time mark not reached 1= Time mark reached"]
    #[inline(always)]
    pub fn rtmi(&self) -> RTMI_R {
        RTMI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Internal time mark events are configured by trigger memory element TMIN (see Section 2.4.7). Set when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Gap or In_Schedule. 0= Time mark not reached 1= Time mark reached (Level 0: cycle time TTOCF.IRTO * 0x200)"]
    #[inline(always)]
    pub fn ttmi(&self) -> TTMI_R {
        TTMI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event 0= No rising/falling edge at stop watch trigger pin m_ttcan_swt detected 1= Rising/falling edge at stop watch trigger pin m_ttcan_swt detected"]
    #[inline(always)]
    pub fn swe(&self) -> SWE_R {
        SWE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap 0= No global time wrap occurred 1= Global time wrap from 0xFFFF to 0x0000 occurred"]
    #[inline(always)]
    pub fn gtw(&self) -> GTW_R {
        GTW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity 0= No discontinuity of global time 1= Discontinuity of global time"]
    #[inline(always)]
    pub fn gtd(&self) -> GTD_R {
        GTD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Time Error Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL, TTCAN Level 0,2 only. 0= Synchronization deviation within limit 1= Synchronization deviation exceeded limit"]
    #[inline(always)]
    pub fn gte(&self) -> GTE_R {
        GTE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow 0= Number of Tx Trigger as expected 1= Less Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub fn txu(&self) -> TXU_R {
        TXU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow 0= Number of Tx Trigger as expected 1= More Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1 0= No scheduling error 1 1= Scheduling error 1 occurred"]
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2 0= No scheduling error 2 1= Scheduling error 2 occurred"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Error Level Changed Not set when error level changed during initialization. 0= No change in error level 1= Error level changed"]
    #[inline(always)]
    pub fn elc(&self) -> ELC_R {
        ELC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger The initialization is restarted by resetting IWT. 0= No missing reference message during system startup 1= No system startup due to missing reference message"]
    #[inline(always)]
    pub fn iwt(&self) -> IWT_R {
        IWT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger 0= No missing reference message 1= Missing reference message (Level 0: cycle time 0xFF00)"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog 0= Application watchdog served in time 1= Application watchdog not served in time"]
    #[inline(always)]
    pub fn aw(&self) -> AW_R {
        AW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configuration Error Trigger out of order. 0= No error found in trigger list 1= Error found in trigger list"]
    #[inline(always)]
    pub fn cer(&self) -> CER_R {
        CER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle 0= No Basic Cycle started since bit has been reset 1= Basic Cycle started"]
    #[inline(always)]
    pub fn sbc(&mut self) -> SBC_W {
        SBC_W { w: self }
    }
    #[doc = "Bit 1 - Start of Matrix Cycle 0= No Matrix Cycle started since bit has been reset 1= Matrix Cycle started"]
    #[inline(always)]
    pub fn smc(&mut self) -> SMC_W {
        SMC_W { w: self }
    }
    #[doc = "Bit 2 - Change of Synchronization Mode 0= No change in master to slave relation or schedule synchronization 1= Master to slave relation or schedule synchronization changed, also set when TTOST.SPL is reset"]
    #[inline(always)]
    pub fn csm_(&mut self) -> CSM__W {
        CSM__W { w: self }
    }
    #[doc = "Bit 3 - Start of Gap 0= No reference message seen with Next_is_Gap bit set 1= Reference message with Next_is_Gap bit set becomes valid"]
    #[inline(always)]
    pub fn sog(&mut self) -> SOG_W {
        SOG_W { w: self }
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Set when time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Time mark not reached 1= Time mark reached"]
    #[inline(always)]
    pub fn rtmi(&mut self) -> RTMI_W {
        RTMI_W { w: self }
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Internal time mark events are configured by trigger memory element TMIN (see Section 2.4.7). Set when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Gap or In_Schedule. 0= Time mark not reached 1= Time mark reached (Level 0: cycle time TTOCF.IRTO * 0x200)"]
    #[inline(always)]
    pub fn ttmi(&mut self) -> TTMI_W {
        TTMI_W { w: self }
    }
    #[doc = "Bit 6 - Stop Watch Event 0= No rising/falling edge at stop watch trigger pin m_ttcan_swt detected 1= Rising/falling edge at stop watch trigger pin m_ttcan_swt detected"]
    #[inline(always)]
    pub fn swe(&mut self) -> SWE_W {
        SWE_W { w: self }
    }
    #[doc = "Bit 7 - Global Time Wrap 0= No global time wrap occurred 1= Global time wrap from 0xFFFF to 0x0000 occurred"]
    #[inline(always)]
    pub fn gtw(&mut self) -> GTW_W {
        GTW_W { w: self }
    }
    #[doc = "Bit 8 - Global Time Discontinuity 0= No discontinuity of global time 1= Discontinuity of global time"]
    #[inline(always)]
    pub fn gtd(&mut self) -> GTD_W {
        GTD_W { w: self }
    }
    #[doc = "Bit 9 - Global Time Error Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL, TTCAN Level 0,2 only. 0= Synchronization deviation within limit 1= Synchronization deviation exceeded limit"]
    #[inline(always)]
    pub fn gte(&mut self) -> GTE_W {
        GTE_W { w: self }
    }
    #[doc = "Bit 10 - Tx Count Underflow 0= Number of Tx Trigger as expected 1= Less Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub fn txu(&mut self) -> TXU_W {
        TXU_W { w: self }
    }
    #[doc = "Bit 11 - Tx Count Overflow 0= Number of Tx Trigger as expected 1= More Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub fn txo(&mut self) -> TXO_W {
        TXO_W { w: self }
    }
    #[doc = "Bit 12 - Scheduling Error 1 0= No scheduling error 1 1= Scheduling error 1 occurred"]
    #[inline(always)]
    pub fn se1(&mut self) -> SE1_W {
        SE1_W { w: self }
    }
    #[doc = "Bit 13 - Scheduling Error 2 0= No scheduling error 2 1= Scheduling error 2 occurred"]
    #[inline(always)]
    pub fn se2(&mut self) -> SE2_W {
        SE2_W { w: self }
    }
    #[doc = "Bit 14 - Error Level Changed Not set when error level changed during initialization. 0= No change in error level 1= Error level changed"]
    #[inline(always)]
    pub fn elc(&mut self) -> ELC_W {
        ELC_W { w: self }
    }
    #[doc = "Bit 15 - Initialization Watch Trigger The initialization is restarted by resetting IWT. 0= No missing reference message during system startup 1= No system startup due to missing reference message"]
    #[inline(always)]
    pub fn iwt(&mut self) -> IWT_W {
        IWT_W { w: self }
    }
    #[doc = "Bit 16 - Watch Trigger 0= No missing reference message 1= Missing reference message (Level 0: cycle time 0xFF00)"]
    #[inline(always)]
    pub fn wt(&mut self) -> WT_W {
        WT_W { w: self }
    }
    #[doc = "Bit 17 - Application Watchdog 0= Application watchdog served in time 1= Application watchdog not served in time"]
    #[inline(always)]
    pub fn aw(&mut self) -> AW_W {
        AW_W { w: self }
    }
    #[doc = "Bit 18 - Configuration Error Trigger out of order. 0= No error found in trigger list 1= Error found in trigger list"]
    #[inline(always)]
    pub fn cer(&mut self) -> CER_W {
        CER_W { w: self }
    }
}
