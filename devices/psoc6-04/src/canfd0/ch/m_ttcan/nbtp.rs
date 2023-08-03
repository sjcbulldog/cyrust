#[doc = "Reader of register NBTP"]
pub type R = crate::R<u32, super::NBTP>;
#[doc = "Writer for register NBTP"]
pub type W = crate::W<u32, super::NBTP>;
#[doc = "Register NBTP `reset()`'s with value 0x0600_0a03"]
impl crate::ResetValue for super::NBTP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0600_0a03
    }
}
#[doc = "Reader of field `NTSEG2`"]
pub type NTSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NTSEG2`"]
pub struct NTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> NTSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `NTSEG1`"]
pub type NTSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NTSEG1`"]
pub struct NTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> NTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NBRP`"]
pub type NBRP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NBRP`"]
pub struct NBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> NBRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `NSJW`"]
pub type NSJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NSJW`"]
pub struct NSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> NSJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point 0x01-0x7F Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point 0x01-0xFF Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler 0x000-0x1FFThe value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Nominal (Re)Synchronization Jump Width 0x00-0x7F Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point 0x01-0x7F Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn ntseg2(&mut self) -> NTSEG2_W {
        NTSEG2_W { w: self }
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point 0x01-0xFF Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn ntseg1(&mut self) -> NTSEG1_W {
        NTSEG1_W { w: self }
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler 0x000-0x1FFThe value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn nbrp(&mut self) -> NBRP_W {
        NBRP_W { w: self }
    }
    #[doc = "Bits 25:31 - Nominal (Re)Synchronization Jump Width 0x00-0x7F Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn nsjw(&mut self) -> NSJW_W {
        NSJW_W { w: self }
    }
}
