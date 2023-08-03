#[doc = "Reader of register TS_CTL"]
pub type R = crate::R<u32, super::TS_CTL>;
#[doc = "Writer for register TS_CTL"]
pub type W = crate::W<u32, super::TS_CTL>;
#[doc = "Register TS_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TS_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
    #[doc = "Bits 0:15 - Time Stamp counter prescale value. When enabled divide the Host clock (HCLK) by PRESCALE+1 to create Time Stamp clock ticks."]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Counter enable bit 0 = Count disabled. Stop counting up and keep the counter value 1 = Count enabled. Start counting up from the current value"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Stamp counter prescale value. When enabled divide the Host clock (HCLK) by PRESCALE+1 to create Time Stamp clock ticks."]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 31 - Counter enable bit 0 = Count disabled. Stop counting up and keep the counter value 1 = Count enabled. Start counting up from the current value"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
