#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDX`"]
pub type IDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX`"]
pub struct IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VALID`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
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
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    pub fn idx(&mut self) -> IDX_W {
        IDX_W { w: self }
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
}
