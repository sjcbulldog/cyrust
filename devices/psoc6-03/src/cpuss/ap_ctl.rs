#[doc = "Reader of register AP_CTL"]
pub type R = crate::R<u32, super::AP_CTL>;
#[doc = "Writer for register AP_CTL"]
pub type W = crate::W<u32, super::AP_CTL>;
#[doc = "Register AP_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::AP_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CM0_ENABLE`"]
pub type CM0_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CM0_ENABLE`"]
pub struct CM0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CM0_ENABLE_W<'a> {
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
#[doc = "Reader of field `CM4_ENABLE`"]
pub type CM4_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CM4_ENABLE`"]
pub struct CM4_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CM4_ENABLE_W<'a> {
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
#[doc = "Reader of field `SYS_ENABLE`"]
pub type SYS_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_ENABLE`"]
pub struct SYS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_ENABLE_W<'a> {
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
#[doc = "Reader of field `CM0_DISABLE`"]
pub type CM0_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CM0_DISABLE`"]
pub struct CM0_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CM0_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CM4_DISABLE`"]
pub type CM4_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CM4_DISABLE`"]
pub struct CM4_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CM4_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SYS_DISABLE`"]
pub type SYS_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYS_DISABLE`"]
pub struct SYS_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_DISABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm0_enable(&self) -> CM0_ENABLE_R {
        CM0_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm4_enable(&self) -> CM4_ENABLE_R {
        CM4_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn sys_enable(&self) -> SYS_ENABLE_R {
        SYS_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm0_disable(&self) -> CM0_DISABLE_R {
        CM0_DISABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm4_disable(&self) -> CM4_DISABLE_R {
        CM4_DISABLE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub fn sys_disable(&self) -> SYS_DISABLE_R {
        SYS_DISABLE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm0_enable(&mut self) -> CM0_ENABLE_W {
        CM0_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm4_enable(&mut self) -> CM4_ENABLE_W {
        CM4_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn sys_enable(&mut self) -> SYS_ENABLE_W {
        SYS_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm0_disable(&mut self) -> CM0_DISABLE_W {
        CM0_DISABLE_W { w: self }
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm4_disable(&mut self) -> CM4_DISABLE_W {
        CM4_DISABLE_W { w: self }
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub fn sys_disable(&mut self) -> SYS_DISABLE_W {
        SYS_DISABLE_W { w: self }
    }
}
