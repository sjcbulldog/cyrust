#[doc = "Reader of register RXFTOP_CTL"]
pub type R = crate::R<u32, super::RXFTOP_CTL>;
#[doc = "Writer for register RXFTOP_CTL"]
pub type W = crate::W<u32, super::RXFTOP_CTL>;
#[doc = "Register RXFTOP_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFTOP_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F0TPE`"]
pub type F0TPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `F0TPE`"]
pub struct F0TPE_W<'a> {
    w: &'a mut W,
}
impl<'a> F0TPE_W<'a> {
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
#[doc = "Reader of field `F1TPE`"]
pub type F1TPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `F1TPE`"]
pub struct F1TPE_W<'a> {
    w: &'a mut W,
}
impl<'a> F1TPE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - FIFO 0 Top Pointer Enable. This enables the FIFO top pointer logic to set the FIFO Top Address (FnTA) and message word counter. This logic is also disabled when the IP is being reconfigured (CCCR.CCE=1). When this logic is disabled a Read from RXFTOP0_DATA is undefined."]
    #[inline(always)]
    pub fn f0tpe(&self) -> F0TPE_R {
        F0TPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO 1 Top Pointer Enable."]
    #[inline(always)]
    pub fn f1tpe(&self) -> F1TPE_R {
        F1TPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO 0 Top Pointer Enable. This enables the FIFO top pointer logic to set the FIFO Top Address (FnTA) and message word counter. This logic is also disabled when the IP is being reconfigured (CCCR.CCE=1). When this logic is disabled a Read from RXFTOP0_DATA is undefined."]
    #[inline(always)]
    pub fn f0tpe(&mut self) -> F0TPE_W {
        F0TPE_W { w: self }
    }
    #[doc = "Bit 1 - FIFO 1 Top Pointer Enable."]
    #[inline(always)]
    pub fn f1tpe(&mut self) -> F1TPE_W {
        F1TPE_W { w: self }
    }
}
