#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `FAULT`"]
pub type FAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT`"]
pub struct FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT_W<'a> {
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
    #[doc = "Bit 0 - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to this field to clear the interrupt cause to '0'. SW clear STATUS.VALID to '0' to enable capture of the next fault. Note that when there is an enabled pending fault source, the pending fault source is captured immediately and INTR.FAULT is immediately activated (set to '1')."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to this field to clear the interrupt cause to '0'. SW clear STATUS.VALID to '0' to enable capture of the next fault. Note that when there is an enabled pending fault source, the pending fault source is captured immediately and INTR.FAULT is immediately activated (set to '1')."]
    #[inline(always)]
    pub fn fault(&mut self) -> FAULT_W {
        FAULT_W { w: self }
    }
}
