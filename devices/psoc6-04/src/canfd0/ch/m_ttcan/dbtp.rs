#[doc = "Reader of register DBTP"]
pub type R = crate::R<u32, super::DBTP>;
#[doc = "Writer for register DBTP"]
pub type W = crate::W<u32, super::DBTP>;
#[doc = "Register DBTP `reset()`'s with value 0x0a33"]
impl crate::ResetValue for super::DBTP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a33
    }
}
#[doc = "Reader of field `DSJW`"]
pub type DSJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSJW`"]
pub struct DSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> DSJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DTSEG2`"]
pub type DTSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTSEG2`"]
pub struct DTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DTSEG1`"]
pub type DTSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTSEG1`"]
pub struct DTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DBRP`"]
pub type DBRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBRP`"]
pub struct DBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TDC`"]
pub type TDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDC`"]
pub struct TDC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data (Re)Synchronization Jump Width 0x0-0xF Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point 0x0-0xF Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point 0x00-0x1F Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler 0x00-0x1F The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transmitter Delay Compensation 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data (Re)Synchronization Jump Width 0x0-0xF Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn dsjw(&mut self) -> DSJW_W {
        DSJW_W { w: self }
    }
    #[doc = "Bits 4:7 - Data time segment after sample point 0x0-0xF Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn dtseg2(&mut self) -> DTSEG2_W {
        DTSEG2_W { w: self }
    }
    #[doc = "Bits 8:12 - Data time segment before sample point 0x00-0x1F Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn dtseg1(&mut self) -> DTSEG1_W {
        DTSEG1_W { w: self }
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler 0x00-0x1F The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn dbrp(&mut self) -> DBRP_W {
        DBRP_W { w: self }
    }
    #[doc = "Bit 23 - Transmitter Delay Compensation 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled"]
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W {
        TDC_W { w: self }
    }
}
