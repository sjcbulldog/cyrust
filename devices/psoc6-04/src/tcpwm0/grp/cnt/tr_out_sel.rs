#[doc = "Reader of register TR_OUT_SEL"]
pub type R = crate::R<u32, super::TR_OUT_SEL>;
#[doc = "Writer for register TR_OUT_SEL"]
pub type W = crate::W<u32, super::TR_OUT_SEL>;
#[doc = "Register TR_OUT_SEL `reset()`'s with value 0x32"]
impl crate::ResetValue for super::TR_OUT_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x32
    }
}
#[doc = "Selects one of the internal events to generate the output trigger 0. Default setting selects the terminal count event.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT0_A {
    #[doc = "0: Overflow event"]
    OVERFLOW = 0,
    #[doc = "1: Underflow event"]
    UNDERFLOW = 1,
    #[doc = "2: Terminal count event (default selection)"]
    TC = 2,
    #[doc = "3: Compare match 0 event"]
    CC0_MATCH = 3,
    #[doc = "4: Compare match 1 event"]
    CC1_MATCH = 4,
    #[doc = "5: PWM output signal 'line_out'"]
    LINE_OUT = 5,
    #[doc = "6: N/A"]
    RSVD6 = 6,
    #[doc = "7: Output trigger disabled."]
    DISABLED = 7,
}
impl From<OUT0_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUT0`"]
pub type OUT0_R = crate::R<u8, OUT0_A>;
impl OUT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT0_A {
        match self.bits {
            0 => OUT0_A::OVERFLOW,
            1 => OUT0_A::UNDERFLOW,
            2 => OUT0_A::TC,
            3 => OUT0_A::CC0_MATCH,
            4 => OUT0_A::CC1_MATCH,
            5 => OUT0_A::LINE_OUT,
            6 => OUT0_A::RSVD6,
            7 => OUT0_A::DISABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OUT0_A::OVERFLOW
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == OUT0_A::UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == OUT0_A::TC
    }
    #[doc = "Checks if the value of the field is `CC0_MATCH`"]
    #[inline(always)]
    pub fn is_cc0_match(&self) -> bool {
        *self == OUT0_A::CC0_MATCH
    }
    #[doc = "Checks if the value of the field is `CC1_MATCH`"]
    #[inline(always)]
    pub fn is_cc1_match(&self) -> bool {
        *self == OUT0_A::CC1_MATCH
    }
    #[doc = "Checks if the value of the field is `LINE_OUT`"]
    #[inline(always)]
    pub fn is_line_out(&self) -> bool {
        *self == OUT0_A::LINE_OUT
    }
    #[doc = "Checks if the value of the field is `RSVD6`"]
    #[inline(always)]
    pub fn is_rsvd6(&self) -> bool {
        *self == OUT0_A::RSVD6
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUT0_A::DISABLED
    }
}
#[doc = "Write proxy for field `OUT0`"]
pub struct OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut W {
        self.variant(OUT0_A::OVERFLOW)
    }
    #[doc = "Underflow event"]
    #[inline(always)]
    pub fn underflow(self) -> &'a mut W {
        self.variant(OUT0_A::UNDERFLOW)
    }
    #[doc = "Terminal count event (default selection)"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(OUT0_A::TC)
    }
    #[doc = "Compare match 0 event"]
    #[inline(always)]
    pub fn cc0_match(self) -> &'a mut W {
        self.variant(OUT0_A::CC0_MATCH)
    }
    #[doc = "Compare match 1 event"]
    #[inline(always)]
    pub fn cc1_match(self) -> &'a mut W {
        self.variant(OUT0_A::CC1_MATCH)
    }
    #[doc = "PWM output signal 'line_out'"]
    #[inline(always)]
    pub fn line_out(self) -> &'a mut W {
        self.variant(OUT0_A::LINE_OUT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd6(self) -> &'a mut W {
        self.variant(OUT0_A::RSVD6)
    }
    #[doc = "Output trigger disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OUT0_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Selects one of the internal events to generate the output trigger 1. Default setting selects the compare match 0 event.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT1_A {
    #[doc = "0: Overflow event"]
    OVERFLOW = 0,
    #[doc = "1: Underflow event"]
    UNDERFLOW = 1,
    #[doc = "2: Terminal count event"]
    TC = 2,
    #[doc = "3: Compare match 0 event (default selection)"]
    CC0_MATCH = 3,
    #[doc = "4: Compare match 1 event"]
    CC1_MATCH = 4,
    #[doc = "5: PWM output signal 'line_out'"]
    LINE_OUT = 5,
    #[doc = "6: N/A"]
    RSVD6 = 6,
    #[doc = "7: Output trigger disabled."]
    DISABLED = 7,
}
impl From<OUT1_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUT1`"]
pub type OUT1_R = crate::R<u8, OUT1_A>;
impl OUT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT1_A {
        match self.bits {
            0 => OUT1_A::OVERFLOW,
            1 => OUT1_A::UNDERFLOW,
            2 => OUT1_A::TC,
            3 => OUT1_A::CC0_MATCH,
            4 => OUT1_A::CC1_MATCH,
            5 => OUT1_A::LINE_OUT,
            6 => OUT1_A::RSVD6,
            7 => OUT1_A::DISABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OUT1_A::OVERFLOW
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == OUT1_A::UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == OUT1_A::TC
    }
    #[doc = "Checks if the value of the field is `CC0_MATCH`"]
    #[inline(always)]
    pub fn is_cc0_match(&self) -> bool {
        *self == OUT1_A::CC0_MATCH
    }
    #[doc = "Checks if the value of the field is `CC1_MATCH`"]
    #[inline(always)]
    pub fn is_cc1_match(&self) -> bool {
        *self == OUT1_A::CC1_MATCH
    }
    #[doc = "Checks if the value of the field is `LINE_OUT`"]
    #[inline(always)]
    pub fn is_line_out(&self) -> bool {
        *self == OUT1_A::LINE_OUT
    }
    #[doc = "Checks if the value of the field is `RSVD6`"]
    #[inline(always)]
    pub fn is_rsvd6(&self) -> bool {
        *self == OUT1_A::RSVD6
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUT1_A::DISABLED
    }
}
#[doc = "Write proxy for field `OUT1`"]
pub struct OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut W {
        self.variant(OUT1_A::OVERFLOW)
    }
    #[doc = "Underflow event"]
    #[inline(always)]
    pub fn underflow(self) -> &'a mut W {
        self.variant(OUT1_A::UNDERFLOW)
    }
    #[doc = "Terminal count event"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(OUT1_A::TC)
    }
    #[doc = "Compare match 0 event (default selection)"]
    #[inline(always)]
    pub fn cc0_match(self) -> &'a mut W {
        self.variant(OUT1_A::CC0_MATCH)
    }
    #[doc = "Compare match 1 event"]
    #[inline(always)]
    pub fn cc1_match(self) -> &'a mut W {
        self.variant(OUT1_A::CC1_MATCH)
    }
    #[doc = "PWM output signal 'line_out'"]
    #[inline(always)]
    pub fn line_out(self) -> &'a mut W {
        self.variant(OUT1_A::LINE_OUT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd6(self) -> &'a mut W {
        self.variant(OUT1_A::RSVD6)
    }
    #[doc = "Output trigger disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OUT1_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects one of the internal events to generate the output trigger 0. Default setting selects the terminal count event."]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Selects one of the internal events to generate the output trigger 1. Default setting selects the compare match 0 event."]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects one of the internal events to generate the output trigger 0. Default setting selects the terminal count event."]
    #[inline(always)]
    pub fn out0(&mut self) -> OUT0_W {
        OUT0_W { w: self }
    }
    #[doc = "Bits 4:6 - Selects one of the internal events to generate the output trigger 1. Default setting selects the compare match 0 event."]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W {
        OUT1_W { w: self }
    }
}
