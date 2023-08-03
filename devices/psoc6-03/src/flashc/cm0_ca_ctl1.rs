#[doc = "Reader of register CM0_CA_CTL1"]
pub type R = crate::R<u32, super::CM0_CA_CTL1>;
#[doc = "Writer for register CM0_CA_CTL1"]
pub type W = crate::W<u32, super::CM0_CA_CTL1>;
#[doc = "Register CM0_CA_CTL1 `reset()`'s with value 0xfa05_0003"]
impl crate::ResetValue for super::CM0_CA_CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfa05_0003
    }
}
#[doc = "Specifies power mode for CM0 cache. The following sequnece should be followed for truning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: Power OFF the CM0 cache SRAM."]
    OFF = 0,
    #[doc = "1: Undefined"]
    RSVD = 1,
    #[doc = "2: Put CM0 cache SRAM in retained mode."]
    RETAINED = 2,
    #[doc = "3: Enable/Turn ON the CM0 cache SRAM."]
    ENABLED = 3,
}
impl From<PWR_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWR_MODE`"]
pub type PWR_MODE_R = crate::R<u8, PWR_MODE_A>;
impl PWR_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWR_MODE_A {
        match self.bits {
            0 => PWR_MODE_A::OFF,
            1 => PWR_MODE_A::RSVD,
            2 => PWR_MODE_A::RETAINED,
            3 => PWR_MODE_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWR_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == PWR_MODE_A::RSVD
    }
    #[doc = "Checks if the value of the field is `RETAINED`"]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PWR_MODE_A::RETAINED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWR_MODE_A::ENABLED
    }
}
#[doc = "Write proxy for field `PWR_MODE`"]
pub struct PWR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWR_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Power OFF the CM0 cache SRAM."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PWR_MODE_A::OFF)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(PWR_MODE_A::RSVD)
    }
    #[doc = "Put CM0 cache SRAM in retained mode."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut W {
        self.variant(PWR_MODE_A::RETAINED)
    }
    #[doc = "Enable/Turn ON the CM0 cache SRAM."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWR_MODE_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `VECTKEYSTAT`"]
pub type VECTKEYSTAT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - Specifies power mode for CM0 cache. The following sequnece should be followed for truning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache."]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:31 - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
    #[inline(always)]
    pub fn vectkeystat(&self) -> VECTKEYSTAT_R {
        VECTKEYSTAT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Specifies power mode for CM0 cache. The following sequnece should be followed for truning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache."]
    #[inline(always)]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W {
        PWR_MODE_W { w: self }
    }
}
