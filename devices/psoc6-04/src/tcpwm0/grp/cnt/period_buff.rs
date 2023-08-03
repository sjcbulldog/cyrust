#[doc = "Reader of register PERIOD_BUFF"]
pub type R = crate::R<u32, super::PERIOD_BUFF>;
#[doc = "Writer for register PERIOD_BUFF"]
pub type W = crate::W<u32, super::PERIOD_BUFF>;
#[doc = "Register PERIOD_BUFF `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PERIOD_BUFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PERIOD`"]
pub type PERIOD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERIOD`"]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register. In PWM_PR mode PEROD_BUFF defines the LFSR polynomial. Each bit represents a tap of the shift register which can be feed back to the MSB via an XOR tree. Examples for GRP_CNT_WIDTH = 16: - Maximum length 16bit LFSR - polynomial x^16 + x^14 + x^13 + x^11 + 1 - taps 0,2,3,5 -> PERIOD = 0x002d - period is 2^16-1 = 65535 cycles - Maximum length 8bit LFSR: - polynomial x^8 + x^6 + x^5 + x^4 + 1 - taps 8,10,11,12 (realized in 8 MSBs of 16bit LFSR) - period is 2^8-1 = 255 cycles In SR mode PERIOD_BUFF defines which tap of the shift register generates the PWM output signals. For a delay of n cycles (from capture event to PWM output) the bit CNT_WIDTH-n should be set to '1'. For a shift register function only one tap should be use, i.e. a one-hot value must be written to PERIOD_BUFF. If multiple bits in PERIOD_BUFF are set then the taps are XOR combined."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register. In PWM_PR mode PEROD_BUFF defines the LFSR polynomial. Each bit represents a tap of the shift register which can be feed back to the MSB via an XOR tree. Examples for GRP_CNT_WIDTH = 16: - Maximum length 16bit LFSR - polynomial x^16 + x^14 + x^13 + x^11 + 1 - taps 0,2,3,5 -> PERIOD = 0x002d - period is 2^16-1 = 65535 cycles - Maximum length 8bit LFSR: - polynomial x^8 + x^6 + x^5 + x^4 + 1 - taps 8,10,11,12 (realized in 8 MSBs of 16bit LFSR) - period is 2^8-1 = 255 cycles In SR mode PERIOD_BUFF defines which tap of the shift register generates the PWM output signals. For a delay of n cycles (from capture event to PWM output) the bit CNT_WIDTH-n should be set to '1'. For a shift register function only one tap should be use, i.e. a one-hot value must be written to PERIOD_BUFF. If multiple bits in PERIOD_BUFF are set then the taps are XOR combined."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
}
