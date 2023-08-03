#[doc = "Reader of register ROM_CTL"]
pub type R = crate::R<u32, super::ROM_CTL>;
#[doc = "Writer for register ROM_CTL"]
pub type W = crate::W<u32, super::ROM_CTL>;
#[doc = "Register ROM_CTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::ROM_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
#[doc = "Reader of field `SLOW_WS`"]
pub type SLOW_WS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLOW_WS`"]
pub struct SLOW_WS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW_WS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FAST_WS`"]
pub type FAST_WS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FAST_WS`"]
pub struct FAST_WS_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_WS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. ROM_CTL.SLOW_WS = '0' when clk_hf <=100 MHz. ROM_CTL.SLOW_WS = '1' when 100MHz < clk_hf <=clk_hf_max. Note: clk_hf_max depends on the target device. Refer datasheet."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SLOW_WS_R {
        SLOW_WS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles. ROM_CTL.FAST_WS = '0' when clk_hf <= clk_hf_max."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FAST_WS_R {
        FAST_WS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. ROM_CTL.SLOW_WS = '0' when clk_hf <=100 MHz. ROM_CTL.SLOW_WS = '1' when 100MHz < clk_hf <=clk_hf_max. Note: clk_hf_max depends on the target device. Refer datasheet."]
    #[inline(always)]
    pub fn slow_ws(&mut self) -> SLOW_WS_W {
        SLOW_WS_W { w: self }
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles. ROM_CTL.FAST_WS = '0' when clk_hf <= clk_hf_max."]
    #[inline(always)]
    pub fn fast_ws(&mut self) -> FAST_WS_W {
        FAST_WS_W { w: self }
    }
}
