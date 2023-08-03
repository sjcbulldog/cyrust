#[doc = "Reader of register RED_CTL45"]
pub type R = crate::R<u32, super::RED_CTL45>;
#[doc = "Writer for register RED_CTL45"]
pub type W = crate::W<u32, super::RED_CTL45>;
#[doc = "Register RED_CTL45 `reset()`'s with value 0"]
impl crate::ResetValue for super::RED_CTL45 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `RED_ADDR_4`"]
pub type RED_ADDR_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RED_ADDR_4`"]
pub struct RED_ADDR_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RED_EN_4`"]
pub type RED_EN_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_EN_4`"]
pub struct RED_EN_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_4_W<'a> {
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
#[doc = "Reader of field `RED_ADDR_5`"]
pub type RED_ADDR_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RED_ADDR_5`"]
pub struct RED_ADDR_5_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RED_EN_5`"]
pub type RED_EN_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_EN_5`"]
pub struct RED_EN_5_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_5_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 4"]
    #[inline(always)]
    pub fn red_addr_4(&self) -> RED_ADDR_4_R {
        RED_ADDR_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 4"]
    #[inline(always)]
    pub fn red_en_4(&self) -> RED_EN_4_R {
        RED_EN_4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 5"]
    #[inline(always)]
    pub fn red_addr_5(&self) -> RED_ADDR_5_R {
        RED_ADDR_5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 5"]
    #[inline(always)]
    pub fn red_en_5(&self) -> RED_EN_5_R {
        RED_EN_5_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 4"]
    #[inline(always)]
    pub fn red_addr_4(&mut self) -> RED_ADDR_4_W {
        RED_ADDR_4_W { w: self }
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 4"]
    #[inline(always)]
    pub fn red_en_4(&mut self) -> RED_EN_4_W {
        RED_EN_4_W { w: self }
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 5"]
    #[inline(always)]
    pub fn red_addr_5(&mut self) -> RED_ADDR_5_W {
        RED_ADDR_5_W { w: self }
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 5"]
    #[inline(always)]
    pub fn red_en_5(&mut self) -> RED_EN_5_W {
        RED_EN_5_W { w: self }
    }
}
