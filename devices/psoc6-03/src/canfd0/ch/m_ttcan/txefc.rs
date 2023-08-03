#[doc = "Reader of register TXEFC"]
pub type R = crate::R<u32, super::TXEFC>;
#[doc = "Writer for register TXEFC"]
pub type W = crate::W<u32, super::TXEFC>;
#[doc = "Register TXEFC `reset()`'s with value 0"]
impl crate::ResetValue for super::TXEFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFSA`"]
pub type EFSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFSA`"]
pub struct EFSA_W<'a> {
    w: &'a mut W,
}
impl<'a> EFSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `EFS`"]
pub type EFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFS`"]
pub struct EFS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `EFWM`"]
pub type EFWM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFWM`"]
pub struct EFWM_W<'a> {
    w: &'a mut W,
}
impl<'a> EFWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn efsa(&self) -> EFSA_R {
        EFSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements 32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS-1"]
    #[inline(always)]
    pub fn efs(&self) -> EFS_R {
        EFS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) 32= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn efwm(&self) -> EFWM_R {
        EFWM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn efsa(&mut self) -> EFSA_W {
        EFSA_W { w: self }
    }
    #[doc = "Bits 16:21 - Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements 32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS-1"]
    #[inline(always)]
    pub fn efs(&mut self) -> EFS_W {
        EFS_W { w: self }
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) 32= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn efwm(&mut self) -> EFWM_W {
        EFWM_W { w: self }
    }
}
