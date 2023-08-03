#[doc = "Reader of register CM4_SYSTEM_INT_CTL[%s]"]
pub type R = crate::R<u32, super::CM4_SYSTEM_INT_CTL>;
#[doc = "Writer for register CM4_SYSTEM_INT_CTL[%s]"]
pub type W = crate::W<u32, super::CM4_SYSTEM_INT_CTL>;
#[doc = "Register CM4_SYSTEM_INT_CTL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CM4_SYSTEM_INT_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `CPU_INT_IDX`"]
pub type CPU_INT_IDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU_INT_IDX`"]
pub struct CPU_INT_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_INT_IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CPU_INT_VALID`"]
pub type CPU_INT_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_INT_VALID`"]
pub struct CPU_INT_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_INT_VALID_W<'a> {
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
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn cpu_int_idx(&self) -> CPU_INT_IDX_R {
        CPU_INT_IDX_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn cpu_int_valid(&self) -> CPU_INT_VALID_R {
        CPU_INT_VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn cpu_int_idx(&mut self) -> CPU_INT_IDX_W {
        CPU_INT_IDX_W { w: self }
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn cpu_int_valid(&mut self) -> CPU_INT_VALID_W {
        CPU_INT_VALID_W { w: self }
    }
}
