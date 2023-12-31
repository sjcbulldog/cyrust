#[doc = "Reader of register XIDFC"]
pub type R = crate::R<u32, super::XIDFC>;
#[doc = "Writer for register XIDFC"]
pub type W = crate::W<u32, super::XIDFC>;
#[doc = "Register XIDFC `reset()`'s with value 0"]
impl crate::ResetValue for super::XIDFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLESA`"]
pub type FLESA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLESA`"]
pub struct FLESA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLESA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `LSE`"]
pub type LSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSE`"]
pub struct LSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements 64= Values greater than 64 are interpreted as 64"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn flesa(&mut self) -> FLESA_W {
        FLESA_W { w: self }
    }
    #[doc = "Bits 16:22 - List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements 64= Values greater than 64 are interpreted as 64"]
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W {
        LSE_W { w: self }
    }
}
