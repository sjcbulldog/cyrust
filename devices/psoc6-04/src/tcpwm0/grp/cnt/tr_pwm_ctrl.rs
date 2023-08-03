#[doc = "Reader of register TR_PWM_CTRL"]
pub type R = crate::R<u32, super::TR_PWM_CTRL>;
#[doc = "Writer for register TR_PWM_CTRL"]
pub type W = crate::W<u32, super::TR_PWM_CTRL>;
#[doc = "Register TR_PWM_CTRL `reset()`'s with value 0xff"]
impl crate::ResetValue for super::TR_PWM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Determines the effect of a compare match 0 event (COUNTER equals CC0 register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC0 register should be set to '0'. For a 100 percent duty cycle, the counter CC0 register should be set to larger than the counter PERIOD register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC0_MATCH_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<CC0_MATCH_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CC0_MATCH_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CC0_MATCH_MODE`"]
pub type CC0_MATCH_MODE_R = crate::R<u8, CC0_MATCH_MODE_A>;
impl CC0_MATCH_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC0_MATCH_MODE_A {
        match self.bits {
            0 => CC0_MATCH_MODE_A::SET,
            1 => CC0_MATCH_MODE_A::CLEAR,
            2 => CC0_MATCH_MODE_A::INVERT,
            3 => CC0_MATCH_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CC0_MATCH_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CC0_MATCH_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == CC0_MATCH_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CC0_MATCH_MODE_A::NO_CHANGE
    }
}
#[doc = "Write proxy for field `CC0_MATCH_MODE`"]
pub struct CC0_MATCH_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC0_MATCH_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC0_MATCH_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CC0_MATCH_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC0_MATCH_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(CC0_MATCH_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CC0_MATCH_MODE_A::NO_CHANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVERFLOW_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<OVERFLOW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OVERFLOW_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OVERFLOW_MODE`"]
pub type OVERFLOW_MODE_R = crate::R<u8, OVERFLOW_MODE_A>;
impl OVERFLOW_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_MODE_A {
        match self.bits {
            0 => OVERFLOW_MODE_A::SET,
            1 => OVERFLOW_MODE_A::CLEAR,
            2 => OVERFLOW_MODE_A::INVERT,
            3 => OVERFLOW_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OVERFLOW_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == OVERFLOW_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == OVERFLOW_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == OVERFLOW_MODE_A::NO_CHANGE
    }
}
#[doc = "Write proxy for field `OVERFLOW_MODE`"]
pub struct OVERFLOW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::NO_CHANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UNDERFLOW_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<UNDERFLOW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UNDERFLOW_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UNDERFLOW_MODE`"]
pub type UNDERFLOW_MODE_R = crate::R<u8, UNDERFLOW_MODE_A>;
impl UNDERFLOW_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFLOW_MODE_A {
        match self.bits {
            0 => UNDERFLOW_MODE_A::SET,
            1 => UNDERFLOW_MODE_A::CLEAR,
            2 => UNDERFLOW_MODE_A::INVERT,
            3 => UNDERFLOW_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UNDERFLOW_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UNDERFLOW_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == UNDERFLOW_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == UNDERFLOW_MODE_A::NO_CHANGE
    }
}
#[doc = "Write proxy for field `UNDERFLOW_MODE`"]
pub struct UNDERFLOW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDERFLOW_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::NO_CHANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Determines the effect of a compare match 1 event (COUNTER equals CC1 register) on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1_MATCH_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<CC1_MATCH_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1_MATCH_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CC1_MATCH_MODE`"]
pub type CC1_MATCH_MODE_R = crate::R<u8, CC1_MATCH_MODE_A>;
impl CC1_MATCH_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1_MATCH_MODE_A {
        match self.bits {
            0 => CC1_MATCH_MODE_A::SET,
            1 => CC1_MATCH_MODE_A::CLEAR,
            2 => CC1_MATCH_MODE_A::INVERT,
            3 => CC1_MATCH_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CC1_MATCH_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CC1_MATCH_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == CC1_MATCH_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CC1_MATCH_MODE_A::NO_CHANGE
    }
}
#[doc = "Write proxy for field `CC1_MATCH_MODE`"]
pub struct CC1_MATCH_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1_MATCH_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1_MATCH_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CC1_MATCH_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1_MATCH_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(CC1_MATCH_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CC1_MATCH_MODE_A::NO_CHANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Determines the effect of a compare match 0 event (COUNTER equals CC0 register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC0 register should be set to '0'. For a 100 percent duty cycle, the counter CC0 register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub fn cc0_match_mode(&self) -> CC0_MATCH_MODE_R {
        CC0_MATCH_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn overflow_mode(&self) -> OVERFLOW_MODE_R {
        OVERFLOW_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    pub fn underflow_mode(&self) -> UNDERFLOW_MODE_R {
        UNDERFLOW_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Determines the effect of a compare match 1 event (COUNTER equals CC1 register) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn cc1_match_mode(&self) -> CC1_MATCH_MODE_R {
        CC1_MATCH_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines the effect of a compare match 0 event (COUNTER equals CC0 register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC0 register should be set to '0'. For a 100 percent duty cycle, the counter CC0 register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub fn cc0_match_mode(&mut self) -> CC0_MATCH_MODE_W {
        CC0_MATCH_MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn overflow_mode(&mut self) -> OVERFLOW_MODE_W {
        OVERFLOW_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    pub fn underflow_mode(&mut self) -> UNDERFLOW_MODE_W {
        UNDERFLOW_MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Determines the effect of a compare match 1 event (COUNTER equals CC1 register) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn cc1_match_mode(&mut self) -> CC1_MATCH_MODE_W {
        CC1_MATCH_MODE_W { w: self }
    }
}
