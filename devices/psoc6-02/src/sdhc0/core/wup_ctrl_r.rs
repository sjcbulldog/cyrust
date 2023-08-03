#[doc = "Reader of register WUP_CTRL_R"]
pub type R = crate::R<u8, super::WUP_CTRL_R>;
#[doc = "Writer for register WUP_CTRL_R"]
pub type W = crate::W<u8, super::WUP_CTRL_R>;
#[doc = "Register WUP_CTRL_R `reset()`'s with value 0"]
impl crate::ResetValue for super::WUP_CTRL_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `WUP_CARD_INT`"]
pub type WUP_CARD_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUP_CARD_INT`"]
pub struct WUP_CARD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP_CARD_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `WUP_CARD_INSERT`"]
pub type WUP_CARD_INSERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUP_CARD_INSERT`"]
pub struct WUP_CARD_INSERT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP_CARD_INSERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `WUP_CARD_REMOVAL`"]
pub type WUP_CARD_REMOVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUP_CARD_REMOVAL`"]
pub struct WUP_CARD_REMOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP_CARD_REMOVAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_int(&self) -> WUP_CARD_INT_R {
        WUP_CARD_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_insert(&self) -> WUP_CARD_INSERT_R {
        WUP_CARD_INSERT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_removal(&self) -> WUP_CARD_REMOVAL_R {
        WUP_CARD_REMOVAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_int(&mut self) -> WUP_CARD_INT_W {
        WUP_CARD_INT_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_insert(&mut self) -> WUP_CARD_INSERT_W {
        WUP_CARD_INSERT_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_removal(&mut self) -> WUP_CARD_REMOVAL_W {
        WUP_CARD_REMOVAL_W { w: self }
    }
}
