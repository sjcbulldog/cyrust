#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLED_A {
    #[doc = "0: N/A"]
    DISABLED = 0,
    #[doc = "1: N/A"]
    ENABLED = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, ENABLED_A>;
impl ENABLED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DISABLED,
            true => ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLED_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLED_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
