#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0xf0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf0
    }
}
#[doc = "Reader of field `AUTO_RELOAD_CC0`"]
pub type AUTO_RELOAD_CC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_RELOAD_CC0`"]
pub struct AUTO_RELOAD_CC0_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_RELOAD_CC0_W<'a> {
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
#[doc = "Reader of field `AUTO_RELOAD_CC1`"]
pub type AUTO_RELOAD_CC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_RELOAD_CC1`"]
pub struct AUTO_RELOAD_CC1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_RELOAD_CC1_W<'a> {
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
#[doc = "Reader of field `AUTO_RELOAD_PERIOD`"]
pub type AUTO_RELOAD_PERIOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_RELOAD_PERIOD`"]
pub struct AUTO_RELOAD_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_RELOAD_PERIOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AUTO_RELOAD_LINE_SEL`"]
pub type AUTO_RELOAD_LINE_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_RELOAD_LINE_SEL`"]
pub struct AUTO_RELOAD_LINE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_RELOAD_LINE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CC0_MATCH_UP_EN`"]
pub type CC0_MATCH_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC0_MATCH_UP_EN`"]
pub struct CC0_MATCH_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC0_MATCH_UP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CC0_MATCH_DOWN_EN`"]
pub type CC0_MATCH_DOWN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC0_MATCH_DOWN_EN`"]
pub struct CC0_MATCH_DOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC0_MATCH_DOWN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CC1_MATCH_UP_EN`"]
pub type CC1_MATCH_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC1_MATCH_UP_EN`"]
pub struct CC1_MATCH_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1_MATCH_UP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CC1_MATCH_DOWN_EN`"]
pub type CC1_MATCH_DOWN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC1_MATCH_DOWN_EN`"]
pub struct CC1_MATCH_DOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1_MATCH_DOWN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PWM_IMM_KILL`"]
pub type PWM_IMM_KILL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_IMM_KILL`"]
pub struct PWM_IMM_KILL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_IMM_KILL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PWM_STOP_ON_KILL`"]
pub type PWM_STOP_ON_KILL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_STOP_ON_KILL`"]
pub struct PWM_STOP_ON_KILL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_STOP_ON_KILL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PWM_SYNC_KILL`"]
pub type PWM_SYNC_KILL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_SYNC_KILL`"]
pub struct PWM_SYNC_KILL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SYNC_KILL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Specifies the behavior of the PWM outputs 'line_out' and 'line_compl_out' while the TCPWM counter is disabled (CTL.ENABLED='0') or stopped. Note: The output signal of this selection can be further modified by the immediate kill logic and line_out polarity settings (CTRL.QUAD_ENCODING_MODE).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_DISABLE_MODE_A {
    #[doc = "0: The behavior is the same is in previous mxtcpwm (version 1).\n\nWhen the counter is disabled the PWM outputs 'line_out' and 'line_compl_out' are NOT driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance).\nNote: This is realized by driving the TCPWM output 'line_out_en' to 0.\n\nWhen the counter is stopped upon a stop event the PWM outputs are deactivated (to the polarity defined by CTL.QUAD_ENCODING_MODE)."]
    Z = 0,
    #[doc = "1: When the counter is disabled the PWM outputs 'line_out' and 'line_compl_out' are driven by the TCPWM.\nWhen the counter is disabled or stopped upon a stop event the PWM outputs are retained (keep their previous levels). \nWhile the counter is disabled or stopped the PWM outputs can be changed via LINE_SEL (when parameter GRP_SMC_PRESENT = 1)."]
    RETAIN = 1,
    #[doc = "2: When the counter is disabled the PWM outputs 'line_out' and 'line_compl_out' are driven by the TCPWM. \nWhen the counter is disabled or stopped upon a stop event the PWM output 'line_out' is driven as a fixed '0' and the PWM output 'line_compl_out' is driven as a fixed '1'."]
    L = 2,
    #[doc = "3: When the counter is disabled the PWM outputs 'line_out' and 'line_compl_out' are driven by the TCPWM. \nWhen the counter is disabled or stopped upon a stop event the PWM output 'line_out' is driven as a fixed '1' and the PWM output 'line_compl_out' is driven as a fixed '0'."]
    H = 3,
}
impl From<PWM_DISABLE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_DISABLE_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM_DISABLE_MODE`"]
pub type PWM_DISABLE_MODE_R = crate::R<u8, PWM_DISABLE_MODE_A>;
impl PWM_DISABLE_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_DISABLE_MODE_A {
        match self.bits {
            0 => PWM_DISABLE_MODE_A::Z,
            1 => PWM_DISABLE_MODE_A::RETAIN,
            2 => PWM_DISABLE_MODE_A::L,
            3 => PWM_DISABLE_MODE_A::H,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Z`"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == PWM_DISABLE_MODE_A::Z
    }
    #[doc = "Checks if the value of the field is `RETAIN`"]
    #[inline(always)]
    pub fn is_retain(&self) -> bool {
        *self == PWM_DISABLE_MODE_A::RETAIN
    }
    #[doc = "Checks if the value of the field is `L`"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == PWM_DISABLE_MODE_A::L
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == PWM_DISABLE_MODE_A::H
    }
}
#[doc = "Write proxy for field `PWM_DISABLE_MODE`"]
pub struct PWM_DISABLE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_DISABLE_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_DISABLE_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The behavior is the same is in previous mxtcpwm (version 1). When the counter is disabled the PWM outputs 'line_out' and 'line_compl_out' are NOT driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance). Note: This is realized by driving the TCPWM output 'line_out_en' to 0. When the counter is stopped upon a stop event the PWM outputs are deactivated (to the polarity defined by CTL.QUAD_ENCODING_MODE)."]
    #[inline(always)]
    pub fn z(self) -> &'a mut W {
        self.variant(PWM_DISABLE_MODE_A::Z)
    }
    #[doc = "When the counter is disabled the PWM outputs 'line_out' and 'line_compl_out' are driven by the TCPWM. When the counter is disabled or stopped upon a stop event the PWM outputs are retained (keep their previous levels). While the counter is disabled or stopped the PWM outputs can be changed via LINE_SEL (when parameter GRP_SMC_PRESENT = 1)."]
    #[inline(always)]
    pub fn retain(self) -> &'a mut W {
        self.variant(PWM_DISABLE_MODE_A::RETAIN)
    }
    #[doc = "When the counter is disabled the PWM outputs 'line_out' and 'line_compl_out' are driven by the TCPWM. When the counter is disabled or stopped upon a stop event the PWM output 'line_out' is driven as a fixed '0' and the PWM output 'line_compl_out' is driven as a fixed '1'."]
    #[inline(always)]
    pub fn l(self) -> &'a mut W {
        self.variant(PWM_DISABLE_MODE_A::L)
    }
    #[doc = "When the counter is disabled the PWM outputs 'line_out' and 'line_compl_out' are driven by the TCPWM. When the counter is disabled or stopped upon a stop event the PWM output 'line_out' is driven as a fixed '1' and the PWM output 'line_compl_out' is driven as a fixed '0'."]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(PWM_DISABLE_MODE_A::H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Determines counter direction. In QUAD mode this field acts as QUAD_RANGE_MODE field selecting between different counter range, reload value and compare / capture behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UP_DOWN_MODE_A {
    #[doc = "0: Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    COUNT_UP = 0,
    #[doc = "1: Count down (to '0'). An underflow event is generated when  the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_DOWN = 1,
    #[doc = "2: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_UPDN1 = 2,
    #[doc = "3: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    COUNT_UPDN2 = 3,
}
impl From<UP_DOWN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UP_DOWN_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UP_DOWN_MODE`"]
pub type UP_DOWN_MODE_R = crate::R<u8, UP_DOWN_MODE_A>;
impl UP_DOWN_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_DOWN_MODE_A {
        match self.bits {
            0 => UP_DOWN_MODE_A::COUNT_UP,
            1 => UP_DOWN_MODE_A::COUNT_DOWN,
            2 => UP_DOWN_MODE_A::COUNT_UPDN1,
            3 => UP_DOWN_MODE_A::COUNT_UPDN2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COUNT_UP`"]
    #[inline(always)]
    pub fn is_count_up(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UP
    }
    #[doc = "Checks if the value of the field is `COUNT_DOWN`"]
    #[inline(always)]
    pub fn is_count_down(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_DOWN
    }
    #[doc = "Checks if the value of the field is `COUNT_UPDN1`"]
    #[inline(always)]
    pub fn is_count_updn1(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UPDN1
    }
    #[doc = "Checks if the value of the field is `COUNT_UPDN2`"]
    #[inline(always)]
    pub fn is_count_updn2(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UPDN2
    }
}
#[doc = "Write proxy for field `UP_DOWN_MODE`"]
pub struct UP_DOWN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_DOWN_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UP_DOWN_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    #[inline(always)]
    pub fn count_up(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UP)
    }
    #[doc = "Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_down(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_DOWN)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_updn1(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN1)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    #[inline(always)]
    pub fn count_updn2(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ONE_SHOT`"]
pub type ONE_SHOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONE_SHOT`"]
pub struct ONE_SHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_SHOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "In QUAD mode this field selects the quadrature encoding mode (X1/X2/X4) or the Up / Down rotary counting mode. In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUAD_ENCODING_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUAD_ENCODING_MODE\\[1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QUAD_ENCODING_MODE_A {
    #[doc = "0: X1 encoding (QUAD mode)\nThis encoding is identical with an up / down counting functionality of the following way: Rising edges of input phiA increment or decrement the counter depending on the state of input phiB (direction input)."]
    X1 = 0,
    #[doc = "1: X2 encoding (QUAD mode)"]
    X2 = 1,
    #[doc = "2: X4 encoding (QUAD mode)"]
    X4 = 2,
    #[doc = "3: Up / Down rotary counting mode. Input phiA increments the counter, input phiB decrements the counter. The trigger edge detection settings apply."]
    UP_DOWN = 3,
}
impl From<QUAD_ENCODING_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: QUAD_ENCODING_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `QUAD_ENCODING_MODE`"]
pub type QUAD_ENCODING_MODE_R = crate::R<u8, QUAD_ENCODING_MODE_A>;
impl QUAD_ENCODING_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUAD_ENCODING_MODE_A {
        match self.bits {
            0 => QUAD_ENCODING_MODE_A::X1,
            1 => QUAD_ENCODING_MODE_A::X2,
            2 => QUAD_ENCODING_MODE_A::X4,
            3 => QUAD_ENCODING_MODE_A::UP_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == QUAD_ENCODING_MODE_A::X1
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == QUAD_ENCODING_MODE_A::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == QUAD_ENCODING_MODE_A::X4
    }
    #[doc = "Checks if the value of the field is `UP_DOWN`"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == QUAD_ENCODING_MODE_A::UP_DOWN
    }
}
#[doc = "Write proxy for field `QUAD_ENCODING_MODE`"]
pub struct QUAD_ENCODING_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QUAD_ENCODING_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUAD_ENCODING_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "X1 encoding (QUAD mode) This encoding is identical with an up / down counting functionality of the following way: Rising edges of input phiA increment or decrement the counter depending on the state of input phiB (direction input)."]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(QUAD_ENCODING_MODE_A::X1)
    }
    #[doc = "X2 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(QUAD_ENCODING_MODE_A::X2)
    }
    #[doc = "X4 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(QUAD_ENCODING_MODE_A::X4)
    }
    #[doc = "Up / Down rotary counting mode. Input phiA increments the counter, input phiB decrements the counter. The trigger edge detection settings apply."]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut W {
        self.variant(QUAD_ENCODING_MODE_A::UP_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Counter mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Timer mode"]
    TIMER = 0,
    #[doc = "1: N/A"]
    RSVD1 = 1,
    #[doc = "2: Capture mode"]
    CAPTURE = 2,
    #[doc = "3: Quadrature mode\n\nDifferent encoding modes can be selected by QUAD_ENCODING_MODE including up/down count functionality.\nDifferent counter range, reload value and capture behavior can be selected by QUAD_RANGE_MODE (overloaded field UP_DOWN_MODE)."]
    QUAD = 3,
    #[doc = "4: Pulse width modulation (PWM) mode"]
    PWM = 4,
    #[doc = "5: PWM with deadtime insertion mode"]
    PWM_DT = 5,
    #[doc = "6: Pseudo random pulse width modulation"]
    PWM_PR = 6,
    #[doc = "7: Shift register mode."]
    SR = 7,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::TIMER,
            1 => MODE_A::RSVD1,
            2 => MODE_A::CAPTURE,
            3 => MODE_A::QUAD,
            4 => MODE_A::PWM,
            5 => MODE_A::PWM_DT,
            6 => MODE_A::PWM_PR,
            7 => MODE_A::SR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == MODE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `RSVD1`"]
    #[inline(always)]
    pub fn is_rsvd1(&self) -> bool {
        *self == MODE_A::RSVD1
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == MODE_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == MODE_A::QUAD
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_A::PWM
    }
    #[doc = "Checks if the value of the field is `PWM_DT`"]
    #[inline(always)]
    pub fn is_pwm_dt(&self) -> bool {
        *self == MODE_A::PWM_DT
    }
    #[doc = "Checks if the value of the field is `PWM_PR`"]
    #[inline(always)]
    pub fn is_pwm_pr(&self) -> bool {
        *self == MODE_A::PWM_PR
    }
    #[doc = "Checks if the value of the field is `SR`"]
    #[inline(always)]
    pub fn is_sr(&self) -> bool {
        *self == MODE_A::SR
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(MODE_A::TIMER)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd1(self) -> &'a mut W {
        self.variant(MODE_A::RSVD1)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(MODE_A::CAPTURE)
    }
    #[doc = "Quadrature mode Different encoding modes can be selected by QUAD_ENCODING_MODE including up/down count functionality. Different counter range, reload value and capture behavior can be selected by QUAD_RANGE_MODE (overloaded field UP_DOWN_MODE)."]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(MODE_A::QUAD)
    }
    #[doc = "Pulse width modulation (PWM) mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODE_A::PWM)
    }
    #[doc = "PWM with deadtime insertion mode"]
    #[inline(always)]
    pub fn pwm_dt(self) -> &'a mut W {
        self.variant(MODE_A::PWM_DT)
    }
    #[doc = "Pseudo random pulse width modulation"]
    #[inline(always)]
    pub fn pwm_pr(self) -> &'a mut W {
        self.variant(MODE_A::PWM_PR)
    }
    #[doc = "Shift register mode."]
    #[inline(always)]
    pub fn sr(self) -> &'a mut W {
        self.variant(MODE_A::SR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `DBG_FREEZE_EN`"]
pub type DBG_FREEZE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_FREEZE_EN`"]
pub struct DBG_FREEZE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_FREEZE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
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
    #[doc = "Bit 0 - Specifies switching of the CC0 and buffered CC0 values. This field has a function in TIMER, QUAD (QUAD_RANGE0_CMP, QUAD_RANGE1_CMP range modes), SR, PWM, PWM_DT and PWM_PR modes. Timer, QUAD, SR modes: '0': never switch. '1': switch on a compare match 0 event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_cc0(&self) -> AUTO_RELOAD_CC0_R {
        AUTO_RELOAD_CC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Specifies switching of the CC1 and buffered CC1 values. This field has a function in TIMER, QUAD (QUAD_RANGE0_CMP, QUAD_RANGE1_CMP range modes), SR, PWM, PWM_DT and PWM_PR modes. Timer, QUAD, SR modes: '0': never switch. '1': switch on a compare match 1 event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_cc1(&self) -> AUTO_RELOAD_CC1_R {
        AUTO_RELOAD_CC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM and PWM_DT modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event. In QUAD mode, QUAD_RANGE0_CMP range mode this field is used to select the index / wrap-around capture function. '0': Captures on index (reload) event. The counter value is copied to the PERIOD register on an index (reload) event. '1': Captures when COUNTER equals 0 or 0xffff. The counter value is copied to the PERIOD register when COUNTER equals 0 or 0xffff."]
    #[inline(always)]
    pub fn auto_reload_period(&self) -> AUTO_RELOAD_PERIOD_R {
        AUTO_RELOAD_PERIOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Specifies switching of the LINE_SEL and LINE_BUFF_SEL values. This field has a function in PWM and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_line_sel(&self) -> AUTO_RELOAD_LINE_SEL_R {
        AUTO_RELOAD_LINE_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables / disables the compare match 0 event generation (COUNTER equals CC0 register) when counting up (STATUS.DOWN = 0) in CNT_UPDN1/2 mode. '0': compare match 0 event generation disabled when counting up '1': compare match 0 event generation enabled when counting up This field has a function in PWM and PWM_DT modes only."]
    #[inline(always)]
    pub fn cc0_match_up_en(&self) -> CC0_MATCH_UP_EN_R {
        CC0_MATCH_UP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables / disables the compare match 0 event generation (COUNTER equals CC0 register) when counting down (STATUS.DOWN = 1) in CNT_UPDN1/2 mode. '0': compare match 0 event generation disabled when counting down '1': compare match 0 event generation enabled when counting down This field has a function in PWM and PWM_DT modes only."]
    #[inline(always)]
    pub fn cc0_match_down_en(&self) -> CC0_MATCH_DOWN_EN_R {
        CC0_MATCH_DOWN_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables / disables the compare match 1 event generation (COUNTER equals CC0 register) when counting up (STATUS.DOWN = 0) in CNT_UPDN1/2 mode. '0': compare match 1 event generation disabled when counting up '1': compare match 1 event generation enabled when counting up This field has a function in PWM and PWM_DT modes only."]
    #[inline(always)]
    pub fn cc1_match_up_en(&self) -> CC1_MATCH_UP_EN_R {
        CC1_MATCH_UP_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables / disables the compare match 1 event generation (COUNTER equals CC0 register) when counting down (STATUS.DOWN = 1) in CNT_UPDN1/2 mode. '0': compare match 1 event generation disabled when counting down '1': compare match 1 event generation enabled when counting down This field has a function in PWM and PWM_DT modes only."]
    #[inline(always)]
    pub fn cc1_match_down_en(&self) -> CC1_MATCH_DOWN_EN_R {
        CC1_MATCH_DOWN_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Specifies whether the kill event immediately deactivates the 'dt_line_out' and 'dt_line_compl_out' signals or with the next module clock ('active count' pre-scaled 'clk_counter'). '0': synchronous kill activation. Deactivates the 'dt_line_out' and 'dt_line_compl_out' signals with the next module clock ('active count' pre-scaled 'clk_counter'). '1': immediate kill activation. Immediately deactivates the 'dt_line_out' and 'dt_line_compl_out' signals. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn pwm_imm_kill(&self) -> PWM_IMM_KILL_R {
        PWM_IMM_KILL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn pwm_stop_on_kill(&self) -> PWM_STOP_ON_KILL_R {
        PWM_STOP_ON_KILL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub fn pwm_sync_kill(&self) -> PWM_SYNC_KILL_R {
        PWM_SYNC_KILL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Specifies the behavior of the PWM outputs 'line_out' and 'line_compl_out' while the TCPWM counter is disabled (CTL.ENABLED='0') or stopped. Note: The output signal of this selection can be further modified by the immediate kill logic and line_out polarity settings (CTRL.QUAD_ENCODING_MODE)."]
    #[inline(always)]
    pub fn pwm_disable_mode(&self) -> PWM_DISABLE_MODE_R {
        PWM_DISABLE_MODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Determines counter direction. In QUAD mode this field acts as QUAD_RANGE_MODE field selecting between different counter range, reload value and compare / capture behavior."]
    #[inline(always)]
    pub fn up_down_mode(&self) -> UP_DOWN_MODE_R {
        UP_DOWN_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub fn one_shot(&self) -> ONE_SHOT_R {
        ONE_SHOT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - In QUAD mode this field selects the quadrature encoding mode (X1/X2/X4) or the Up / Down rotary counting mode. In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUAD_ENCODING_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUAD_ENCODING_MODE\\[1\\]."]
    #[inline(always)]
    pub fn quad_encoding_mode(&self) -> QUAD_ENCODING_MODE_R {
        QUAD_ENCODING_MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Specifies the counter behavior in debug mode. '0': The counter operation continues in debug mode. '1': The counter operation freezes in debug mode."]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DBG_FREEZE_EN_R {
        DBG_FREEZE_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Counter enable. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_IN_SEL, TR_IN_EDGE_SEL, TR_PWM_CTRL and TR_OUT_SEL register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_out0' and tr_out1'). - the counter's line outputs ('line_out' and 'line_compl_out')."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies switching of the CC0 and buffered CC0 values. This field has a function in TIMER, QUAD (QUAD_RANGE0_CMP, QUAD_RANGE1_CMP range modes), SR, PWM, PWM_DT and PWM_PR modes. Timer, QUAD, SR modes: '0': never switch. '1': switch on a compare match 0 event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_cc0(&mut self) -> AUTO_RELOAD_CC0_W {
        AUTO_RELOAD_CC0_W { w: self }
    }
    #[doc = "Bit 1 - Specifies switching of the CC1 and buffered CC1 values. This field has a function in TIMER, QUAD (QUAD_RANGE0_CMP, QUAD_RANGE1_CMP range modes), SR, PWM, PWM_DT and PWM_PR modes. Timer, QUAD, SR modes: '0': never switch. '1': switch on a compare match 1 event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_cc1(&mut self) -> AUTO_RELOAD_CC1_W {
        AUTO_RELOAD_CC1_W { w: self }
    }
    #[doc = "Bit 2 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM and PWM_DT modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event. In QUAD mode, QUAD_RANGE0_CMP range mode this field is used to select the index / wrap-around capture function. '0': Captures on index (reload) event. The counter value is copied to the PERIOD register on an index (reload) event. '1': Captures when COUNTER equals 0 or 0xffff. The counter value is copied to the PERIOD register when COUNTER equals 0 or 0xffff."]
    #[inline(always)]
    pub fn auto_reload_period(&mut self) -> AUTO_RELOAD_PERIOD_W {
        AUTO_RELOAD_PERIOD_W { w: self }
    }
    #[doc = "Bit 3 - Specifies switching of the LINE_SEL and LINE_BUFF_SEL values. This field has a function in PWM and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_line_sel(&mut self) -> AUTO_RELOAD_LINE_SEL_W {
        AUTO_RELOAD_LINE_SEL_W { w: self }
    }
    #[doc = "Bit 4 - Enables / disables the compare match 0 event generation (COUNTER equals CC0 register) when counting up (STATUS.DOWN = 0) in CNT_UPDN1/2 mode. '0': compare match 0 event generation disabled when counting up '1': compare match 0 event generation enabled when counting up This field has a function in PWM and PWM_DT modes only."]
    #[inline(always)]
    pub fn cc0_match_up_en(&mut self) -> CC0_MATCH_UP_EN_W {
        CC0_MATCH_UP_EN_W { w: self }
    }
    #[doc = "Bit 5 - Enables / disables the compare match 0 event generation (COUNTER equals CC0 register) when counting down (STATUS.DOWN = 1) in CNT_UPDN1/2 mode. '0': compare match 0 event generation disabled when counting down '1': compare match 0 event generation enabled when counting down This field has a function in PWM and PWM_DT modes only."]
    #[inline(always)]
    pub fn cc0_match_down_en(&mut self) -> CC0_MATCH_DOWN_EN_W {
        CC0_MATCH_DOWN_EN_W { w: self }
    }
    #[doc = "Bit 6 - Enables / disables the compare match 1 event generation (COUNTER equals CC0 register) when counting up (STATUS.DOWN = 0) in CNT_UPDN1/2 mode. '0': compare match 1 event generation disabled when counting up '1': compare match 1 event generation enabled when counting up This field has a function in PWM and PWM_DT modes only."]
    #[inline(always)]
    pub fn cc1_match_up_en(&mut self) -> CC1_MATCH_UP_EN_W {
        CC1_MATCH_UP_EN_W { w: self }
    }
    #[doc = "Bit 7 - Enables / disables the compare match 1 event generation (COUNTER equals CC0 register) when counting down (STATUS.DOWN = 1) in CNT_UPDN1/2 mode. '0': compare match 1 event generation disabled when counting down '1': compare match 1 event generation enabled when counting down This field has a function in PWM and PWM_DT modes only."]
    #[inline(always)]
    pub fn cc1_match_down_en(&mut self) -> CC1_MATCH_DOWN_EN_W {
        CC1_MATCH_DOWN_EN_W { w: self }
    }
    #[doc = "Bit 8 - Specifies whether the kill event immediately deactivates the 'dt_line_out' and 'dt_line_compl_out' signals or with the next module clock ('active count' pre-scaled 'clk_counter'). '0': synchronous kill activation. Deactivates the 'dt_line_out' and 'dt_line_compl_out' signals with the next module clock ('active count' pre-scaled 'clk_counter'). '1': immediate kill activation. Immediately deactivates the 'dt_line_out' and 'dt_line_compl_out' signals. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn pwm_imm_kill(&mut self) -> PWM_IMM_KILL_W {
        PWM_IMM_KILL_W { w: self }
    }
    #[doc = "Bit 9 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn pwm_stop_on_kill(&mut self) -> PWM_STOP_ON_KILL_W {
        PWM_STOP_ON_KILL_W { w: self }
    }
    #[doc = "Bit 10 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub fn pwm_sync_kill(&mut self) -> PWM_SYNC_KILL_W {
        PWM_SYNC_KILL_W { w: self }
    }
    #[doc = "Bits 12:13 - Specifies the behavior of the PWM outputs 'line_out' and 'line_compl_out' while the TCPWM counter is disabled (CTL.ENABLED='0') or stopped. Note: The output signal of this selection can be further modified by the immediate kill logic and line_out polarity settings (CTRL.QUAD_ENCODING_MODE)."]
    #[inline(always)]
    pub fn pwm_disable_mode(&mut self) -> PWM_DISABLE_MODE_W {
        PWM_DISABLE_MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Determines counter direction. In QUAD mode this field acts as QUAD_RANGE_MODE field selecting between different counter range, reload value and compare / capture behavior."]
    #[inline(always)]
    pub fn up_down_mode(&mut self) -> UP_DOWN_MODE_W {
        UP_DOWN_MODE_W { w: self }
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub fn one_shot(&mut self) -> ONE_SHOT_W {
        ONE_SHOT_W { w: self }
    }
    #[doc = "Bits 20:21 - In QUAD mode this field selects the quadrature encoding mode (X1/X2/X4) or the Up / Down rotary counting mode. In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUAD_ENCODING_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUAD_ENCODING_MODE\\[1\\]."]
    #[inline(always)]
    pub fn quad_encoding_mode(&mut self) -> QUAD_ENCODING_MODE_W {
        QUAD_ENCODING_MODE_W { w: self }
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 30 - Specifies the counter behavior in debug mode. '0': The counter operation continues in debug mode. '1': The counter operation freezes in debug mode."]
    #[inline(always)]
    pub fn dbg_freeze_en(&mut self) -> DBG_FREEZE_EN_W {
        DBG_FREEZE_EN_W { w: self }
    }
    #[doc = "Bit 31 - Counter enable. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_IN_SEL, TR_IN_EDGE_SEL, TR_PWM_CTRL and TR_OUT_SEL register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_out0' and tr_out1'). - the counter's line outputs ('line_out' and 'line_compl_out')."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
