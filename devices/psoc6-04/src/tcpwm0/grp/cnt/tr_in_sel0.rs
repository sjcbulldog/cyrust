#[doc = "Reader of register TR_IN_SEL0"]
pub type R = crate::R<u32, super::TR_IN_SEL0>;
#[doc = "Writer for register TR_IN_SEL0"]
pub type W = crate::W<u32, super::TR_IN_SEL0>;
#[doc = "Register TR_IN_SEL0 `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::TR_IN_SEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `CAPTURE0_SEL`"]
pub type CAPTURE0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPTURE0_SEL`"]
pub struct CAPTURE0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `COUNT_SEL`"]
pub type COUNT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COUNT_SEL`"]
pub struct COUNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RELOAD_SEL`"]
pub type RELOAD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RELOAD_SEL`"]
pub struct RELOAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `STOP_SEL`"]
pub type STOP_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STOP_SEL`"]
pub struct STOP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects one of the up to 256 input triggers as a capture0 trigger. Input trigger 0 is always '0' and input trigger 1 is always '1'. If existing, the one-to-one trigger inputs 'tr_one_cnt_in' (different to each counter) are selected by setting 2 and above. The settings above are used for the general purpose trigger inputs 'tr_all_cnt_in' connected to all counters selected. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn capture0_sel(&self) -> CAPTURE0_SEL_R {
        CAPTURE0_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Selects one of the 256 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub fn count_sel(&self) -> COUNT_SEL_R {
        COUNT_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Selects one of the 256 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In these modes, it will update the counter with 0x8000 (counter midpoint) or 0x0000 depending on the QUAD_RANGE_MODE."]
    #[inline(always)]
    pub fn reload_sel(&self) -> RELOAD_SEL_R {
        RELOAD_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Selects one of the 256 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn stop_sel(&self) -> STOP_SEL_R {
        STOP_SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects one of the up to 256 input triggers as a capture0 trigger. Input trigger 0 is always '0' and input trigger 1 is always '1'. If existing, the one-to-one trigger inputs 'tr_one_cnt_in' (different to each counter) are selected by setting 2 and above. The settings above are used for the general purpose trigger inputs 'tr_all_cnt_in' connected to all counters selected. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn capture0_sel(&mut self) -> CAPTURE0_SEL_W {
        CAPTURE0_SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - Selects one of the 256 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub fn count_sel(&mut self) -> COUNT_SEL_W {
        COUNT_SEL_W { w: self }
    }
    #[doc = "Bits 16:23 - Selects one of the 256 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In these modes, it will update the counter with 0x8000 (counter midpoint) or 0x0000 depending on the QUAD_RANGE_MODE."]
    #[inline(always)]
    pub fn reload_sel(&mut self) -> RELOAD_SEL_W {
        RELOAD_SEL_W { w: self }
    }
    #[doc = "Bits 24:31 - Selects one of the 256 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn stop_sel(&mut self) -> STOP_SEL_W {
        STOP_SEL_W { w: self }
    }
}
