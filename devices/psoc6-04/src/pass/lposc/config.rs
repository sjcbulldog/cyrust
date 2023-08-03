#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPOSC functionality while in DEEPSLEEP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEEPSLEEP_MODE_A {
    #[doc = "0: LPOSC enabled by TIMER trigger"]
    DUTYCYCLED = 0,
    #[doc = "1: LPOSC always on in deepsleep"]
    ALWAYS_ON = 1,
}
impl From<DEEPSLEEP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DEEPSLEEP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEEPSLEEP_MODE`"]
pub type DEEPSLEEP_MODE_R = crate::R<bool, DEEPSLEEP_MODE_A>;
impl DEEPSLEEP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEPSLEEP_MODE_A {
        match self.bits {
            false => DEEPSLEEP_MODE_A::DUTYCYCLED,
            true => DEEPSLEEP_MODE_A::ALWAYS_ON,
        }
    }
    #[doc = "Checks if the value of the field is `DUTYCYCLED`"]
    #[inline(always)]
    pub fn is_dutycycled(&self) -> bool {
        *self == DEEPSLEEP_MODE_A::DUTYCYCLED
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ON`"]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        *self == DEEPSLEEP_MODE_A::ALWAYS_ON
    }
}
#[doc = "Write proxy for field `DEEPSLEEP_MODE`"]
pub struct DEEPSLEEP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPSLEEP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEEPSLEEP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPOSC enabled by TIMER trigger"]
    #[inline(always)]
    pub fn dutycycled(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::DUTYCYCLED)
    }
    #[doc = "LPOSC always on in deepsleep"]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::ALWAYS_ON)
    }
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
impl R {
    #[doc = "Bit 0 - LPOSC functionality while in DEEPSLEEP"]
    #[inline(always)]
    pub fn deepsleep_mode(&self) -> DEEPSLEEP_MODE_R {
        DEEPSLEEP_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPOSC functionality while in DEEPSLEEP"]
    #[inline(always)]
    pub fn deepsleep_mode(&mut self) -> DEEPSLEEP_MODE_W {
        DEEPSLEEP_MODE_W { w: self }
    }
}
