#[doc = "Reader of register SEQ_PROGRAM_CTL_1"]
pub type R = crate::R<u32, super::SEQ_PROGRAM_CTL_1>;
#[doc = "Writer for register SEQ_PROGRAM_CTL_1"]
pub type W = crate::W<u32, super::SEQ_PROGRAM_CTL_1>;
#[doc = "Register SEQ_PROGRAM_CTL_1 `reset()`'s with value 0x0022_0020"]
impl crate::ResetValue for super::SEQ_PROGRAM_CTL_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0022_0020
    }
}
#[doc = "Reader of field `CYCLES`"]
pub type CYCLES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CYCLES`"]
pub struct CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `STROBE_A`"]
pub type STROBE_A_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_A`"]
pub struct STROBE_A_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_A_W<'a> {
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
#[doc = "Reader of field `STROBE_B`"]
pub type STROBE_B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_B`"]
pub struct STROBE_B_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_B_W<'a> {
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
#[doc = "Reader of field `STROBE_C`"]
pub type STROBE_C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_C`"]
pub struct STROBE_C_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_C_W<'a> {
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
#[doc = "Reader of field `STROBE_D`"]
pub type STROBE_D_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_D`"]
pub struct STROBE_D_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_D_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `STROBE_E`"]
pub type STROBE_E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_E`"]
pub struct STROBE_E_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `STROBE_F`"]
pub type STROBE_F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_F`"]
pub struct STROBE_F_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `STROBE_G`"]
pub type STROBE_G_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_G`"]
pub struct STROBE_G_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_G_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
    #[doc = "Bits 0:9 - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub fn strobe_a(&self) -> STROBE_A_R {
        STROBE_A_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&self) -> STROBE_B_R {
        STROBE_B_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&self) -> STROBE_C_R {
        STROBE_C_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&self) -> STROBE_D_R {
        STROBE_D_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&self) -> STROBE_E_R {
        STROBE_E_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&self) -> STROBE_F_R {
        STROBE_F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&self) -> STROBE_G_R {
        STROBE_G_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 31 - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
    #[inline(always)]
    pub fn cycles(&mut self) -> CYCLES_W {
        CYCLES_W { w: self }
    }
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub fn strobe_a(&mut self) -> STROBE_A_W {
        STROBE_A_W { w: self }
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&mut self) -> STROBE_B_W {
        STROBE_B_W { w: self }
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&mut self) -> STROBE_C_W {
        STROBE_C_W { w: self }
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&mut self) -> STROBE_D_W {
        STROBE_D_W { w: self }
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&mut self) -> STROBE_E_W {
        STROBE_E_W { w: self }
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&mut self) -> STROBE_F_W {
        STROBE_F_W { w: self }
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&mut self) -> STROBE_G_W {
        STROBE_G_W { w: self }
    }
    #[doc = "Bit 31 - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
}
