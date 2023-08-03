#[doc = "Reader of register DIV_24_5_CTL[%s]"]
pub type R = crate::R<u32, super::DIV_24_5_CTL>;
#[doc = "Writer for register DIV_24_5_CTL[%s]"]
pub type W = crate::W<u32, super::DIV_24_5_CTL>;
#[doc = "Register DIV_24_5_CTL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DIV_24_5_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRAC5_DIV`"]
pub type FRAC5_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRAC5_DIV`"]
pub struct FRAC5_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC5_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `INT24_DIV`"]
pub type INT24_DIV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INT24_DIV`"]
pub struct INT24_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INT24_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn frac5_div(&self) -> FRAC5_DIV_R {
        FRAC5_DIV_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - Integer division by (1+INT24_DIV). Allows for integer divisions in the range \\[1, 16,777,216\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 16,777,216 31/32\\]
in 1/32 increments. For the generation of a divided clock, the integer division range is restricted to \\[2, 16,777,216 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 16,777,216\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int24_div(&self) -> INT24_DIV_R {
        INT24_DIV_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:7 - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn frac5_div(&mut self) -> FRAC5_DIV_W {
        FRAC5_DIV_W { w: self }
    }
    #[doc = "Bits 8:31 - Integer division by (1+INT24_DIV). Allows for integer divisions in the range \\[1, 16,777,216\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 16,777,216 31/32\\]
in 1/32 increments. For the generation of a divided clock, the integer division range is restricted to \\[2, 16,777,216 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 16,777,216\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int24_div(&mut self) -> INT24_DIV_W {
        INT24_DIV_W { w: self }
    }
}
