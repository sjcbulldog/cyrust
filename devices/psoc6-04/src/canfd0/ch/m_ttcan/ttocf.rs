#[doc = "Reader of register TTOCF"]
pub type R = crate::R<u32, super::TTOCF>;
#[doc = "Writer for register TTOCF"]
pub type W = crate::W<u32, super::TTOCF>;
#[doc = "Register TTOCF `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::TTOCF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `OM`"]
pub type OM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OM`"]
pub struct OM_W<'a> {
    w: &'a mut W,
}
impl<'a> OM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `GEN`"]
pub type GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEN`"]
pub struct GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_W<'a> {
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
#[doc = "Reader of field `TM`"]
pub type TM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TM`"]
pub struct TM_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_W<'a> {
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
#[doc = "Reader of field `LDSDL`"]
pub type LDSDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LDSDL`"]
pub struct LDSDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDSDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `IRTO`"]
pub type IRTO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRTO`"]
pub struct IRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> IRTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EECS`"]
pub type EECS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EECS`"]
pub struct EECS_W<'a> {
    w: &'a mut W,
}
impl<'a> EECS_W<'a> {
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
#[doc = "Reader of field `AWL`"]
pub type AWL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AWL`"]
pub struct AWL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `EGTF`"]
pub type EGTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EGTF`"]
pub struct EGTF_W<'a> {
    w: &'a mut W,
}
impl<'a> EGTF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `ECC`"]
pub type ECC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECC`"]
pub struct ECC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `EVTP`"]
pub type EVTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVTP`"]
pub struct EVTP_W<'a> {
    w: &'a mut W,
}
impl<'a> EVTP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation Mode 00= Event-driven CAN communication, default 01= TTCAN level 1 10= TTCAN level 2 11= TTCAN level 0"]
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Gap Enable 0= Strictly time-triggered operation 1= External event-synchronized time-triggered operation"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Time Master 0= Time Master function disabled 1= Potential Time Master"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit The Synchronization Deviation Limit SDL is configured by its dual logarithm LDSDL with SDL = 2(LDSDL + 5). It should not exceed the clock tolerance given by the CAN bit timing configuration. 0x0-7 LD of Synchronization Deviation Limit (SDL <= 32...4096)"]
    #[inline(always)]
    pub fn ldsdl(&self) -> LDSDL_R {
        LDSDL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset 0x00-7F Positive offset, range from 0 to 127"]
    #[inline(always)]
    pub fn irto(&self) -> IRTO_R {
        IRTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization If enabled, TUR configuration (TURCF.NCL only) may be updated during TTCAN operation. 0= External clock synchronization in TTCAN Level 0,2 disabled 1= External clock synchronization in TTCAN Level 0,2 enabled"]
    #[inline(always)]
    pub fn eecs(&self) -> EECS_R {
        EECS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit The application watchdog can be disabled by programming AWL to 0x00. 0x00-FF Maximum time after which the application has to serve the application watchdog. The application watchdog is incremented once each 256 NTUs."]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Global Time Filtering 0= Global time filtering in TTCAN Level 0,2 is disabled 1= Global time filtering in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub fn egtf(&self) -> EGTF_R {
        EGTF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable Clock Calibration 0= Automatic clock calibration in TTCAN Level 0,2 is disabled 1= Automatic clock calibration in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Event Trigger Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub fn evtp(&self) -> EVTP_R {
        EVTP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation Mode 00= Event-driven CAN communication, default 01= TTCAN level 1 10= TTCAN level 2 11= TTCAN level 0"]
    #[inline(always)]
    pub fn om(&mut self) -> OM_W {
        OM_W { w: self }
    }
    #[doc = "Bit 3 - Gap Enable 0= Strictly time-triggered operation 1= External event-synchronized time-triggered operation"]
    #[inline(always)]
    pub fn gen(&mut self) -> GEN_W {
        GEN_W { w: self }
    }
    #[doc = "Bit 4 - Time Master 0= Time Master function disabled 1= Potential Time Master"]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit The Synchronization Deviation Limit SDL is configured by its dual logarithm LDSDL with SDL = 2(LDSDL + 5). It should not exceed the clock tolerance given by the CAN bit timing configuration. 0x0-7 LD of Synchronization Deviation Limit (SDL <= 32...4096)"]
    #[inline(always)]
    pub fn ldsdl(&mut self) -> LDSDL_W {
        LDSDL_W { w: self }
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset 0x00-7F Positive offset, range from 0 to 127"]
    #[inline(always)]
    pub fn irto(&mut self) -> IRTO_W {
        IRTO_W { w: self }
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization If enabled, TUR configuration (TURCF.NCL only) may be updated during TTCAN operation. 0= External clock synchronization in TTCAN Level 0,2 disabled 1= External clock synchronization in TTCAN Level 0,2 enabled"]
    #[inline(always)]
    pub fn eecs(&mut self) -> EECS_W {
        EECS_W { w: self }
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit The application watchdog can be disabled by programming AWL to 0x00. 0x00-FF Maximum time after which the application has to serve the application watchdog. The application watchdog is incremented once each 256 NTUs."]
    #[inline(always)]
    pub fn awl(&mut self) -> AWL_W {
        AWL_W { w: self }
    }
    #[doc = "Bit 24 - Enable Global Time Filtering 0= Global time filtering in TTCAN Level 0,2 is disabled 1= Global time filtering in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub fn egtf(&mut self) -> EGTF_W {
        EGTF_W { w: self }
    }
    #[doc = "Bit 25 - Enable Clock Calibration 0= Automatic clock calibration in TTCAN Level 0,2 is disabled 1= Automatic clock calibration in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub fn ecc(&mut self) -> ECC_W {
        ECC_W { w: self }
    }
    #[doc = "Bit 26 - Event Trigger Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub fn evtp(&mut self) -> EVTP_W {
        EVTP_W { w: self }
    }
}
