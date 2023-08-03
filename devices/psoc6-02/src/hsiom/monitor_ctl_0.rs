#[doc = "Reader of register MONITOR_CTL_0"]
pub type R = crate::R<u32, super::MONITOR_CTL_0>;
#[doc = "Writer for register MONITOR_CTL_0"]
pub type W = crate::W<u32, super::MONITOR_CTL_0>;
#[doc = "Register MONITOR_CTL_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MONITOR_CTL_0 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `MONITOR_EN`"]
pub type MONITOR_EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MONITOR_EN`"]
pub struct MONITOR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MONITOR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn monitor_en(&self) -> MONITOR_EN_R {
        MONITOR_EN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn monitor_en(&mut self) -> MONITOR_EN_W {
        MONITOR_EN_W { w: self }
    }
}
