#[doc = "Reader of register SL_CTL"]
pub type R = crate::R<u32, super::SL_CTL>;
#[doc = "Writer for register SL_CTL"]
pub type W = crate::W<u32, super::SL_CTL>;
#[doc = "Register SL_CTL `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::SL_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `ENABLED_0`"]
pub type ENABLED_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_0`"]
pub struct ENABLED_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_0_W<'a> {
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
#[doc = "Reader of field `ENABLED_1`"]
pub type ENABLED_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_1`"]
pub struct ENABLED_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_1_W<'a> {
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
#[doc = "Reader of field `ENABLED_2`"]
pub type ENABLED_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_2`"]
pub struct ENABLED_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_2_W<'a> {
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
#[doc = "Reader of field `ENABLED_3`"]
pub type ENABLED_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_3`"]
pub struct ENABLED_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_3_W<'a> {
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
#[doc = "Reader of field `ENABLED_4`"]
pub type ENABLED_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_4`"]
pub struct ENABLED_4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_4_W<'a> {
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
#[doc = "Reader of field `ENABLED_5`"]
pub type ENABLED_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_5`"]
pub struct ENABLED_5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_5_W<'a> {
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
#[doc = "Reader of field `ENABLED_6`"]
pub type ENABLED_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_6`"]
pub struct ENABLED_6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_6_W<'a> {
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
#[doc = "Reader of field `ENABLED_7`"]
pub type ENABLED_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_7`"]
pub struct ENABLED_7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_7_W<'a> {
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
#[doc = "Reader of field `ENABLED_8`"]
pub type ENABLED_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_8`"]
pub struct ENABLED_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_8_W<'a> {
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
#[doc = "Reader of field `ENABLED_9`"]
pub type ENABLED_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_9`"]
pub struct ENABLED_9_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_9_W<'a> {
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
#[doc = "Reader of field `ENABLED_10`"]
pub type ENABLED_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_10`"]
pub struct ENABLED_10_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_10_W<'a> {
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
#[doc = "Reader of field `ENABLED_11`"]
pub type ENABLED_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_11`"]
pub struct ENABLED_11_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ENABLED_12`"]
pub type ENABLED_12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_12`"]
pub struct ENABLED_12_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ENABLED_13`"]
pub type ENABLED_13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_13`"]
pub struct ENABLED_13_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ENABLED_14`"]
pub type ENABLED_14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_14`"]
pub struct ENABLED_14_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ENABLED_15`"]
pub type ENABLED_15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED_15`"]
pub struct ENABLED_15_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DISABLED_0`"]
pub type DISABLED_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_0`"]
pub struct DISABLED_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_0_W<'a> {
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
#[doc = "Reader of field `DISABLED_1`"]
pub type DISABLED_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_1`"]
pub struct DISABLED_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_1_W<'a> {
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
#[doc = "Reader of field `DISABLED_2`"]
pub type DISABLED_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_2`"]
pub struct DISABLED_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_2_W<'a> {
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
#[doc = "Reader of field `DISABLED_3`"]
pub type DISABLED_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_3`"]
pub struct DISABLED_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_3_W<'a> {
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
#[doc = "Reader of field `DISABLED_4`"]
pub type DISABLED_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_4`"]
pub struct DISABLED_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_4_W<'a> {
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
#[doc = "Reader of field `DISABLED_5`"]
pub type DISABLED_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_5`"]
pub struct DISABLED_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_5_W<'a> {
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
#[doc = "Reader of field `DISABLED_6`"]
pub type DISABLED_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_6`"]
pub struct DISABLED_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_6_W<'a> {
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
#[doc = "Reader of field `DISABLED_7`"]
pub type DISABLED_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_7`"]
pub struct DISABLED_7_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `DISABLED_8`"]
pub type DISABLED_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_8`"]
pub struct DISABLED_8_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DISABLED_9`"]
pub type DISABLED_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_9`"]
pub struct DISABLED_9_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DISABLED_10`"]
pub type DISABLED_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_10`"]
pub struct DISABLED_10_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DISABLED_11`"]
pub type DISABLED_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_11`"]
pub struct DISABLED_11_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `DISABLED_12`"]
pub type DISABLED_12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_12`"]
pub struct DISABLED_12_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DISABLED_13`"]
pub type DISABLED_13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_13`"]
pub struct DISABLED_13_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `DISABLED_14`"]
pub type DISABLED_14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_14`"]
pub struct DISABLED_14_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_14_W<'a> {
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
#[doc = "Reader of field `DISABLED_15`"]
pub type DISABLED_15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_15`"]
pub struct DISABLED_15_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_15_W<'a> {
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
    #[doc = "Bit 0 - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn enabled_0(&self) -> ENABLED_0_R {
        ENABLED_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn enabled_1(&self) -> ENABLED_1_R {
        ENABLED_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn enabled_2(&self) -> ENABLED_2_R {
        ENABLED_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn enabled_3(&self) -> ENABLED_3_R {
        ENABLED_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn enabled_4(&self) -> ENABLED_4_R {
        ENABLED_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn enabled_5(&self) -> ENABLED_5_R {
        ENABLED_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn enabled_6(&self) -> ENABLED_6_R {
        ENABLED_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn enabled_7(&self) -> ENABLED_7_R {
        ENABLED_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn enabled_8(&self) -> ENABLED_8_R {
        ENABLED_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn enabled_9(&self) -> ENABLED_9_R {
        ENABLED_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn enabled_10(&self) -> ENABLED_10_R {
        ENABLED_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn enabled_11(&self) -> ENABLED_11_R {
        ENABLED_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn enabled_12(&self) -> ENABLED_12_R {
        ENABLED_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn enabled_13(&self) -> ENABLED_13_R {
        ENABLED_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn enabled_14(&self) -> ENABLED_14_R {
        ENABLED_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn enabled_15(&self) -> ENABLED_15_R {
        ENABLED_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
    #[inline(always)]
    pub fn disabled_0(&self) -> DISABLED_0_R {
        DISABLED_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn disabled_1(&self) -> DISABLED_1_R {
        DISABLED_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn disabled_2(&self) -> DISABLED_2_R {
        DISABLED_2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn disabled_3(&self) -> DISABLED_3_R {
        DISABLED_3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn disabled_4(&self) -> DISABLED_4_R {
        DISABLED_4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn disabled_5(&self) -> DISABLED_5_R {
        DISABLED_5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    pub fn disabled_6(&self) -> DISABLED_6_R {
        DISABLED_6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn disabled_7(&self) -> DISABLED_7_R {
        DISABLED_7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn disabled_8(&self) -> DISABLED_8_R {
        DISABLED_8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn disabled_9(&self) -> DISABLED_9_R {
        DISABLED_9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn disabled_10(&self) -> DISABLED_10_R {
        DISABLED_10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - N/A"]
    #[inline(always)]
    pub fn disabled_11(&self) -> DISABLED_11_R {
        DISABLED_11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    pub fn disabled_12(&self) -> DISABLED_12_R {
        DISABLED_12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    pub fn disabled_13(&self) -> DISABLED_13_R {
        DISABLED_13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn disabled_14(&self) -> DISABLED_14_R {
        DISABLED_14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn disabled_15(&self) -> DISABLED_15_R {
        DISABLED_15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn enabled_0(&mut self) -> ENABLED_0_W {
        ENABLED_0_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn enabled_1(&mut self) -> ENABLED_1_W {
        ENABLED_1_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn enabled_2(&mut self) -> ENABLED_2_W {
        ENABLED_2_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn enabled_3(&mut self) -> ENABLED_3_W {
        ENABLED_3_W { w: self }
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn enabled_4(&mut self) -> ENABLED_4_W {
        ENABLED_4_W { w: self }
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn enabled_5(&mut self) -> ENABLED_5_W {
        ENABLED_5_W { w: self }
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn enabled_6(&mut self) -> ENABLED_6_W {
        ENABLED_6_W { w: self }
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn enabled_7(&mut self) -> ENABLED_7_W {
        ENABLED_7_W { w: self }
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn enabled_8(&mut self) -> ENABLED_8_W {
        ENABLED_8_W { w: self }
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn enabled_9(&mut self) -> ENABLED_9_W {
        ENABLED_9_W { w: self }
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn enabled_10(&mut self) -> ENABLED_10_W {
        ENABLED_10_W { w: self }
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn enabled_11(&mut self) -> ENABLED_11_W {
        ENABLED_11_W { w: self }
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn enabled_12(&mut self) -> ENABLED_12_W {
        ENABLED_12_W { w: self }
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn enabled_13(&mut self) -> ENABLED_13_W {
        ENABLED_13_W { w: self }
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn enabled_14(&mut self) -> ENABLED_14_W {
        ENABLED_14_W { w: self }
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn enabled_15(&mut self) -> ENABLED_15_W {
        ENABLED_15_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
    #[inline(always)]
    pub fn disabled_0(&mut self) -> DISABLED_0_W {
        DISABLED_0_W { w: self }
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn disabled_1(&mut self) -> DISABLED_1_W {
        DISABLED_1_W { w: self }
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn disabled_2(&mut self) -> DISABLED_2_W {
        DISABLED_2_W { w: self }
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn disabled_3(&mut self) -> DISABLED_3_W {
        DISABLED_3_W { w: self }
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn disabled_4(&mut self) -> DISABLED_4_W {
        DISABLED_4_W { w: self }
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn disabled_5(&mut self) -> DISABLED_5_W {
        DISABLED_5_W { w: self }
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    pub fn disabled_6(&mut self) -> DISABLED_6_W {
        DISABLED_6_W { w: self }
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn disabled_7(&mut self) -> DISABLED_7_W {
        DISABLED_7_W { w: self }
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn disabled_8(&mut self) -> DISABLED_8_W {
        DISABLED_8_W { w: self }
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn disabled_9(&mut self) -> DISABLED_9_W {
        DISABLED_9_W { w: self }
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn disabled_10(&mut self) -> DISABLED_10_W {
        DISABLED_10_W { w: self }
    }
    #[doc = "Bit 27 - N/A"]
    #[inline(always)]
    pub fn disabled_11(&mut self) -> DISABLED_11_W {
        DISABLED_11_W { w: self }
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    pub fn disabled_12(&mut self) -> DISABLED_12_W {
        DISABLED_12_W { w: self }
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    pub fn disabled_13(&mut self) -> DISABLED_13_W {
        DISABLED_13_W { w: self }
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn disabled_14(&mut self) -> DISABLED_14_W {
        DISABLED_14_W { w: self }
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn disabled_15(&mut self) -> DISABLED_15_W {
        DISABLED_15_W { w: self }
    }
}
