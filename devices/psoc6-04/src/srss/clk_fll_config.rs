#[doc = "Reader of register CLK_FLL_CONFIG"]
pub type R = crate::R<u32, super::CLK_FLL_CONFIG>;
#[doc = "Writer for register CLK_FLL_CONFIG"]
pub type W = crate::W<u32, super::CLK_FLL_CONFIG>;
#[doc = "Register CLK_FLL_CONFIG `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::CLK_FLL_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Reader of field `FLL_MULT`"]
pub type FLL_MULT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLL_MULT`"]
pub struct FLL_MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLL_MULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
#[doc = "Reader of field `FLL_OUTPUT_DIV`"]
pub type FLL_OUTPUT_DIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLL_OUTPUT_DIV`"]
pub struct FLL_OUTPUT_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FLL_OUTPUT_DIV_W<'a> {
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
#[doc = "Reader of field `FLL_ENABLE`"]
pub type FLL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLL_ENABLE`"]
pub struct FLL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLL_ENABLE_W<'a> {
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
    #[doc = "Bits 0:17 - Multiplier to determine CCO frequency in multiples of the frequency of the selected reference clock (Fref). Ffll = (FLL_MULT) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV+1)"]
    #[inline(always)]
    pub fn fll_mult(&self) -> FLL_MULT_R {
        FLL_MULT_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 24 - Control bits for Output divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: no division 1: divide by 2"]
    #[inline(always)]
    pub fn fll_output_div(&self) -> FLL_OUTPUT_DIV_R {
        FLL_OUTPUT_DIV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master enable for FLL. The FLL requires firmware sequencing when enabling, disabling, and entering/exiting DEEPSLEEP. To enable the FLL, first enable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=1 and wait until CLK_FLL_STATUS.CCO_READY==1. Next, ensure the reference clock has stabilized and CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF. Next, write FLL_ENABLE=1 and wait until CLK_FLL_STATUS.LOCKED==1. Finally, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. It takes seven reference clock cycles plus four FLL output cycles to switch to the FLL output. Do not disable the FLL before this time completes. To disable the FLL, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF and (optionally) read the same register to ensure the write completes. Then, wait at least seven FLL reference clock cycles before disabling it with FLL_ENABLE=0. Lastly, disable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=0. Before entering DEEPSLEEP, either disable the FLL using above sequence or use the following procedure to deselect/select it before/after DEEPSLEEP. Before entering DEEPSLEEP, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF to change the FLL to use its reference clock. After DEEPSLEEP wakeup, wait until CLK_FLL_STATUS.LOCKED==1 and then write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn fll_enable(&self) -> FLL_ENABLE_R {
        FLL_ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Multiplier to determine CCO frequency in multiples of the frequency of the selected reference clock (Fref). Ffll = (FLL_MULT) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV+1)"]
    #[inline(always)]
    pub fn fll_mult(&mut self) -> FLL_MULT_W {
        FLL_MULT_W { w: self }
    }
    #[doc = "Bit 24 - Control bits for Output divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: no division 1: divide by 2"]
    #[inline(always)]
    pub fn fll_output_div(&mut self) -> FLL_OUTPUT_DIV_W {
        FLL_OUTPUT_DIV_W { w: self }
    }
    #[doc = "Bit 31 - Master enable for FLL. The FLL requires firmware sequencing when enabling, disabling, and entering/exiting DEEPSLEEP. To enable the FLL, first enable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=1 and wait until CLK_FLL_STATUS.CCO_READY==1. Next, ensure the reference clock has stabilized and CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF. Next, write FLL_ENABLE=1 and wait until CLK_FLL_STATUS.LOCKED==1. Finally, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. It takes seven reference clock cycles plus four FLL output cycles to switch to the FLL output. Do not disable the FLL before this time completes. To disable the FLL, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF and (optionally) read the same register to ensure the write completes. Then, wait at least seven FLL reference clock cycles before disabling it with FLL_ENABLE=0. Lastly, disable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=0. Before entering DEEPSLEEP, either disable the FLL using above sequence or use the following procedure to deselect/select it before/after DEEPSLEEP. Before entering DEEPSLEEP, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF to change the FLL to use its reference clock. After DEEPSLEEP wakeup, wait until CLK_FLL_STATUS.LOCKED==1 and then write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT to switch to the FLL output. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn fll_enable(&mut self) -> FLL_ENABLE_W {
        FLL_ENABLE_W { w: self }
    }
}
