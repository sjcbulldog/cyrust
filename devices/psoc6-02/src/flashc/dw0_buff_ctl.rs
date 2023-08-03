#[doc = "Reader of register DW0_BUFF_CTL"]
pub type R = crate::R<u32, super::DW0_BUFF_CTL>;
#[doc = "Writer for register DW0_BUFF_CTL"]
pub type W = crate::W<u32, super::DW0_BUFF_CTL>;
#[doc = "Register DW0_BUFF_CTL `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::DW0_BUFF_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
#[doc = "Reader of field `PREF_EN`"]
pub type PREF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREF_EN`"]
pub struct PREF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREF_EN_W<'a> {
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
impl R {
    #[doc = "Bit 30 - See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W {
        PREF_EN_W { w: self }
    }
}
