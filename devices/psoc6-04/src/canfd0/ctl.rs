#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STOP_REQ`"]
pub type STOP_REQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STOP_REQ`"]
pub struct STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_REQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MRAM_OFF`"]
pub type MRAM_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MRAM_OFF`"]
pub struct MRAM_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAM_OFF_W<'a> {
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
    #[doc = "Bits 0:7 - Clock Stop Request for each TTCAN IP . The m_ttcan_clkstop_req of each TTCAN IP is directly driven by these bits."]
    #[inline(always)]
    pub fn stop_req(&self) -> STOP_REQ_R {
        STOP_REQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - MRAM off 0= Default MRAM on (with MRAM retained in DeepSleep). 1= Switch MRAM off (not retained) to save power. Before setting this bit all the CAN channels have to be powered down using the STOP_REQ/ACK bits. When the MRAM is off any access attempt to it is considered an address error (as if MRAM_SIZE=0). After switching the MRAM on again software needs to allow for a certain power up time before MRAM can be used, i.e. before STOP_REQ can be de-asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register. To meet S8 platform requirements, MRAM_OFF should be set to 0 prior to transitioning to Hibernate mode."]
    #[inline(always)]
    pub fn mram_off(&self) -> MRAM_OFF_R {
        MRAM_OFF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Stop Request for each TTCAN IP . The m_ttcan_clkstop_req of each TTCAN IP is directly driven by these bits."]
    #[inline(always)]
    pub fn stop_req(&mut self) -> STOP_REQ_W {
        STOP_REQ_W { w: self }
    }
    #[doc = "Bit 31 - MRAM off 0= Default MRAM on (with MRAM retained in DeepSleep). 1= Switch MRAM off (not retained) to save power. Before setting this bit all the CAN channels have to be powered down using the STOP_REQ/ACK bits. When the MRAM is off any access attempt to it is considered an address error (as if MRAM_SIZE=0). After switching the MRAM on again software needs to allow for a certain power up time before MRAM can be used, i.e. before STOP_REQ can be de-asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register. To meet S8 platform requirements, MRAM_OFF should be set to 0 prior to transitioning to Hibernate mode."]
    #[inline(always)]
    pub fn mram_off(&mut self) -> MRAM_OFF_W {
        MRAM_OFF_W { w: self }
    }
}
