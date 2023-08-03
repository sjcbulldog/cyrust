#[doc = "Reader of register LINE_SEL"]
pub type R = crate::R<u32, super::LINE_SEL>;
#[doc = "Writer for register LINE_SEL"]
pub type W = crate::W<u32, super::LINE_SEL>;
#[doc = "Register LINE_SEL `reset()`'s with value 0x32"]
impl crate::ResetValue for super::LINE_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x32
    }
}
#[doc = "Selects the source for the output signal 'line_out'. Default setting is the PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[0\\]).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT_SEL_A {
    #[doc = "0: fixed '0'"]
    L = 0,
    #[doc = "1: fixed '1'"]
    H = 1,
    #[doc = "2: PWM signal 'line'"]
    PWM = 2,
    #[doc = "3: inverted PWM signal 'line'"]
    PWM_INV = 3,
    #[doc = "4: The output 'line_out' is not driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance).\n\nNote: This is realized by driving the output 'line_out_en' to 0."]
    Z = 4,
    #[doc = "5: N/A"]
    RSVD5 = 5,
    #[doc = "6: N/A"]
    RSVD6 = 6,
    #[doc = "7: N/A"]
    RSVD7 = 7,
}
impl From<OUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUT_SEL`"]
pub type OUT_SEL_R = crate::R<u8, OUT_SEL_A>;
impl OUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT_SEL_A {
        match self.bits {
            0 => OUT_SEL_A::L,
            1 => OUT_SEL_A::H,
            2 => OUT_SEL_A::PWM,
            3 => OUT_SEL_A::PWM_INV,
            4 => OUT_SEL_A::Z,
            5 => OUT_SEL_A::RSVD5,
            6 => OUT_SEL_A::RSVD6,
            7 => OUT_SEL_A::RSVD7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L`"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == OUT_SEL_A::L
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == OUT_SEL_A::H
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == OUT_SEL_A::PWM
    }
    #[doc = "Checks if the value of the field is `PWM_INV`"]
    #[inline(always)]
    pub fn is_pwm_inv(&self) -> bool {
        *self == OUT_SEL_A::PWM_INV
    }
    #[doc = "Checks if the value of the field is `Z`"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == OUT_SEL_A::Z
    }
    #[doc = "Checks if the value of the field is `RSVD5`"]
    #[inline(always)]
    pub fn is_rsvd5(&self) -> bool {
        *self == OUT_SEL_A::RSVD5
    }
    #[doc = "Checks if the value of the field is `RSVD6`"]
    #[inline(always)]
    pub fn is_rsvd6(&self) -> bool {
        *self == OUT_SEL_A::RSVD6
    }
    #[doc = "Checks if the value of the field is `RSVD7`"]
    #[inline(always)]
    pub fn is_rsvd7(&self) -> bool {
        *self == OUT_SEL_A::RSVD7
    }
}
#[doc = "Write proxy for field `OUT_SEL`"]
pub struct OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "fixed '0'"]
    #[inline(always)]
    pub fn l(self) -> &'a mut W {
        self.variant(OUT_SEL_A::L)
    }
    #[doc = "fixed '1'"]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(OUT_SEL_A::H)
    }
    #[doc = "PWM signal 'line'"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(OUT_SEL_A::PWM)
    }
    #[doc = "inverted PWM signal 'line'"]
    #[inline(always)]
    pub fn pwm_inv(self) -> &'a mut W {
        self.variant(OUT_SEL_A::PWM_INV)
    }
    #[doc = "The output 'line_out' is not driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance). Note: This is realized by driving the output 'line_out_en' to 0."]
    #[inline(always)]
    pub fn z(self) -> &'a mut W {
        self.variant(OUT_SEL_A::Z)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd5(self) -> &'a mut W {
        self.variant(OUT_SEL_A::RSVD5)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd6(self) -> &'a mut W {
        self.variant(OUT_SEL_A::RSVD6)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd7(self) -> &'a mut W {
        self.variant(OUT_SEL_A::RSVD7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Selects the source for the output signal 'line_compl_out'. Default setting is the inverted PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_compl_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[1\\]).\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMPL_OUT_SEL_A {
    #[doc = "0: fixed '0'"]
    L = 0,
    #[doc = "1: fixed '1'"]
    H = 1,
    #[doc = "2: PWM signal 'line'"]
    PWM = 2,
    #[doc = "3: inverted PWM signal 'line'"]
    PWM_INV = 3,
    #[doc = "4: The output 'line_compl_out' is not driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance).\n\nNote: This is realized by driving the output 'line_compl_out_en' to 0."]
    Z = 4,
    #[doc = "5: N/A"]
    RSVD5 = 5,
    #[doc = "6: N/A"]
    RSVD6 = 6,
    #[doc = "7: N/A"]
    RSVD7 = 7,
}
impl From<COMPL_OUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPL_OUT_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMPL_OUT_SEL`"]
pub type COMPL_OUT_SEL_R = crate::R<u8, COMPL_OUT_SEL_A>;
impl COMPL_OUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPL_OUT_SEL_A {
        match self.bits {
            0 => COMPL_OUT_SEL_A::L,
            1 => COMPL_OUT_SEL_A::H,
            2 => COMPL_OUT_SEL_A::PWM,
            3 => COMPL_OUT_SEL_A::PWM_INV,
            4 => COMPL_OUT_SEL_A::Z,
            5 => COMPL_OUT_SEL_A::RSVD5,
            6 => COMPL_OUT_SEL_A::RSVD6,
            7 => COMPL_OUT_SEL_A::RSVD7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L`"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == COMPL_OUT_SEL_A::L
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == COMPL_OUT_SEL_A::H
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == COMPL_OUT_SEL_A::PWM
    }
    #[doc = "Checks if the value of the field is `PWM_INV`"]
    #[inline(always)]
    pub fn is_pwm_inv(&self) -> bool {
        *self == COMPL_OUT_SEL_A::PWM_INV
    }
    #[doc = "Checks if the value of the field is `Z`"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == COMPL_OUT_SEL_A::Z
    }
    #[doc = "Checks if the value of the field is `RSVD5`"]
    #[inline(always)]
    pub fn is_rsvd5(&self) -> bool {
        *self == COMPL_OUT_SEL_A::RSVD5
    }
    #[doc = "Checks if the value of the field is `RSVD6`"]
    #[inline(always)]
    pub fn is_rsvd6(&self) -> bool {
        *self == COMPL_OUT_SEL_A::RSVD6
    }
    #[doc = "Checks if the value of the field is `RSVD7`"]
    #[inline(always)]
    pub fn is_rsvd7(&self) -> bool {
        *self == COMPL_OUT_SEL_A::RSVD7
    }
}
#[doc = "Write proxy for field `COMPL_OUT_SEL`"]
pub struct COMPL_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPL_OUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPL_OUT_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "fixed '0'"]
    #[inline(always)]
    pub fn l(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::L)
    }
    #[doc = "fixed '1'"]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::H)
    }
    #[doc = "PWM signal 'line'"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::PWM)
    }
    #[doc = "inverted PWM signal 'line'"]
    #[inline(always)]
    pub fn pwm_inv(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::PWM_INV)
    }
    #[doc = "The output 'line_compl_out' is not driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance). Note: This is realized by driving the output 'line_compl_out_en' to 0."]
    #[inline(always)]
    pub fn z(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::Z)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd5(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::RSVD5)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd6(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::RSVD6)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd7(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::RSVD7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the source for the output signal 'line_out'. Default setting is the PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[0\\])."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Selects the source for the output signal 'line_compl_out'. Default setting is the inverted PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_compl_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[1\\])."]
    #[inline(always)]
    pub fn compl_out_sel(&self) -> COMPL_OUT_SEL_R {
        COMPL_OUT_SEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the source for the output signal 'line_out'. Default setting is the PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[0\\])."]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OUT_SEL_W {
        OUT_SEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Selects the source for the output signal 'line_compl_out'. Default setting is the inverted PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_compl_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[1\\])."]
    #[inline(always)]
    pub fn compl_out_sel(&mut self) -> COMPL_OUT_SEL_W {
        COMPL_OUT_SEL_W { w: self }
    }
}
