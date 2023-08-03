#[doc = "Reader of register BOOT_CTRL_R"]
pub type R = crate::R<u16, super::BOOT_CTRL_R>;
#[doc = "Writer for register BOOT_CTRL_R"]
pub type W = crate::W<u16, super::BOOT_CTRL_R>;
#[doc = "Register BOOT_CTRL_R `reset()`'s with value 0"]
impl crate::ResetValue for super::BOOT_CTRL_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `MAN_BOOT_EN`"]
pub type MAN_BOOT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAN_BOOT_EN`"]
pub struct MAN_BOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAN_BOOT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `VALIDATE_BOOT`"]
pub struct VALIDATE_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> VALIDATE_BOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `BOOT_ACK_ENABLE`"]
pub type BOOT_ACK_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_ACK_ENABLE`"]
pub struct BOOT_ACK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ACK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `BOOT_TOUT_CNT`"]
pub type BOOT_TOUT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BOOT_TOUT_CNT`"]
pub struct BOOT_TOUT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_TOUT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    pub fn man_boot_en(&self) -> MAN_BOOT_EN_R {
        MAN_BOOT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    pub fn boot_ack_enable(&self) -> BOOT_ACK_ENABLE_R {
        BOOT_ACK_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    pub fn boot_tout_cnt(&self) -> BOOT_TOUT_CNT_R {
        BOOT_TOUT_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    pub fn man_boot_en(&mut self) -> MAN_BOOT_EN_W {
        MAN_BOOT_EN_W { w: self }
    }
    #[doc = "Bit 7 - Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: - 0x1 (TRUE): Validate Mandatory boot enable bit - 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
    #[inline(always)]
    pub fn validate_boot(&mut self) -> VALIDATE_BOOT_W {
        VALIDATE_BOOT_W { w: self }
    }
    #[doc = "Bit 8 - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    pub fn boot_ack_enable(&mut self) -> BOOT_ACK_ENABLE_W {
        BOOT_ACK_ENABLE_W { w: self }
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    pub fn boot_tout_cnt(&mut self) -> BOOT_TOUT_CNT_W {
        BOOT_TOUT_CNT_W { w: self }
    }
}
