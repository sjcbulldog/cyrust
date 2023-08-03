#[doc = "Reader of register RXF0C"]
pub type R = crate::R<u32, super::RXF0C>;
#[doc = "Writer for register RXF0C"]
pub type W = crate::W<u32, super::RXF0C>;
#[doc = "Register RXF0C `reset()`'s with value 0"]
impl crate::ResetValue for super::RXF0C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F0SA`"]
pub type F0SA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `F0SA`"]
pub struct F0SA_W<'a> {
    w: &'a mut W,
}
impl<'a> F0SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `F0S`"]
pub type F0S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0S`"]
pub struct F0S_W<'a> {
    w: &'a mut W,
}
impl<'a> F0S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `F0WM`"]
pub type F0WM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0WM`"]
pub struct F0WM_W<'a> {
    w: &'a mut W,
}
impl<'a> F0WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `F0OM`"]
pub type F0OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `F0OM`"]
pub struct F0OM_W<'a> {
    w: &'a mut W,
}
impl<'a> F0OM_W<'a> {
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
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) 64= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn f0sa(&mut self) -> F0SA_W {
        F0SA_W { w: self }
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
    #[inline(always)]
    pub fn f0s(&mut self) -> F0S_W {
        F0S_W { w: self }
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) 64= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn f0wm(&mut self) -> F0WM_W {
        F0WM_W { w: self }
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
    #[inline(always)]
    pub fn f0om(&mut self) -> F0OM_W {
        F0OM_W { w: self }
    }
}
