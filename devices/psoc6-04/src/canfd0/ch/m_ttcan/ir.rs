#[doc = "Reader of register IR"]
pub type R = crate::R<u32, super::IR>;
#[doc = "Writer for register IR"]
pub type W = crate::W<u32, super::IR>;
#[doc = "Register IR `reset()`'s with value 0"]
impl crate::ResetValue for super::IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RF0N`"]
pub type RF0N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF0N`"]
pub struct RF0N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0N_W<'a> {
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
#[doc = "Reader of field `RF0W`"]
pub type RF0W_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF0W`"]
pub struct RF0W_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0W_W<'a> {
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
#[doc = "Reader of field `RF0F`"]
pub type RF0F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF0F`"]
pub struct RF0F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0F_W<'a> {
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
#[doc = "Reader of field `RF0L_`"]
pub type RF0L__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF0L_`"]
pub struct RF0L__W<'a> {
    w: &'a mut W,
}
impl<'a> RF0L__W<'a> {
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
#[doc = "Reader of field `RF1N`"]
pub type RF1N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF1N`"]
pub struct RF1N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1N_W<'a> {
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
#[doc = "Reader of field `RF1W`"]
pub type RF1W_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF1W`"]
pub struct RF1W_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1W_W<'a> {
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
#[doc = "Reader of field `RF1F`"]
pub type RF1F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF1F`"]
pub struct RF1F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1F_W<'a> {
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
#[doc = "Reader of field `RF1L_`"]
pub type RF1L__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF1L_`"]
pub struct RF1L__W<'a> {
    w: &'a mut W,
}
impl<'a> RF1L__W<'a> {
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
#[doc = "Reader of field `HPM`"]
pub type HPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPM`"]
pub struct HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_W<'a> {
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
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC`"]
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
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
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCF`"]
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
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
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFE`"]
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
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
#[doc = "Reader of field `TEFN`"]
pub type TEFN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEFN`"]
pub struct TEFN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFN_W<'a> {
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
#[doc = "Reader of field `TEFW`"]
pub type TEFW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEFW`"]
pub struct TEFW_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFW_W<'a> {
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
#[doc = "Reader of field `TEFF`"]
pub type TEFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEFF`"]
pub struct TEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFF_W<'a> {
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
#[doc = "Reader of field `TEFL_`"]
pub type TEFL__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEFL_`"]
pub struct TEFL__W<'a> {
    w: &'a mut W,
}
impl<'a> TEFL__W<'a> {
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
#[doc = "Reader of field `TSW`"]
pub type TSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSW`"]
pub struct TSW_W<'a> {
    w: &'a mut W,
}
impl<'a> TSW_W<'a> {
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
#[doc = "Reader of field `MRAF`"]
pub type MRAF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MRAF`"]
pub struct MRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAF_W<'a> {
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
#[doc = "Reader of field `TOO`"]
pub type TOO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOO`"]
pub struct TOO_W<'a> {
    w: &'a mut W,
}
impl<'a> TOO_W<'a> {
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
#[doc = "Reader of field `DRX`"]
pub type DRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRX`"]
pub struct DRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DRX_W<'a> {
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
#[doc = "Reader of field `BEC`"]
pub type BEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEC`"]
pub struct BEC_W<'a> {
    w: &'a mut W,
}
impl<'a> BEC_W<'a> {
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
#[doc = "Reader of field `BEU`"]
pub type BEU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEU`"]
pub struct BEU_W<'a> {
    w: &'a mut W,
}
impl<'a> BEU_W<'a> {
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
#[doc = "Reader of field `ELO`"]
pub type ELO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELO`"]
pub struct ELO_W<'a> {
    w: &'a mut W,
}
impl<'a> ELO_W<'a> {
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
#[doc = "Reader of field `EP_`"]
pub type EP__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP_`"]
pub struct EP__W<'a> {
    w: &'a mut W,
}
impl<'a> EP__W<'a> {
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
#[doc = "Reader of field `EW_`"]
pub type EW__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EW_`"]
pub struct EW__W<'a> {
    w: &'a mut W,
}
impl<'a> EW__W<'a> {
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
#[doc = "Reader of field `BO_`"]
pub type BO__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BO_`"]
pub struct BO__W<'a> {
    w: &'a mut W,
}
impl<'a> BO__W<'a> {
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
#[doc = "Reader of field `WDI`"]
pub type WDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDI`"]
pub struct WDI_W<'a> {
    w: &'a mut W,
}
impl<'a> WDI_W<'a> {
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
#[doc = "Reader of field `PEA`"]
pub type PEA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEA`"]
pub struct PEA_W<'a> {
    w: &'a mut W,
}
impl<'a> PEA_W<'a> {
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
#[doc = "Reader of field `PED`"]
pub type PED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PED`"]
pub struct PED_W<'a> {
    w: &'a mut W,
}
impl<'a> PED_W<'a> {
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
#[doc = "Reader of field `ARA`"]
pub type ARA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARA`"]
pub struct ARA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark"]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn rf0l_(&self) -> RF0L__R {
        RF0L__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark"]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub fn rf1l_(&self) -> RF1L__R {
        RF1L__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High Priority Message 0= No high priority message received 1= High priority message received"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed 0= No transmission completed 1= Transmission completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark"]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
    #[inline(always)]
    pub fn tefl_(&self) -> TEFL__R {
        TEFL__R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure The flag is set, when the Rx Handler - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_TTCAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred"]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred 0= No timeout 1= Timeout reached"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into a Rx Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - M_TTCAN reports correctable ECC fault to the generic fault structure, this bit always reads as 0. Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_ttcan_aeim_berr\\[0\\]
generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)"]
    #[inline(always)]
    pub fn bec(&self) -> BEC_R {
        BEC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_ttcan_aeim_berr\\[1\\]
generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)"]
    #[inline(always)]
    pub fn beu(&self) -> BEU_R {
        BEU_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed"]
    #[inline(always)]
    pub fn ep_(&self) -> EP__R {
        EP__R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed"]
    #[inline(always)]
    pub fn ew_(&self) -> EW__R {
        EW__R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed"]
    #[inline(always)]
    pub fn bo_(&self) -> BO__R {
        BO__R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC != 0,7)"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC != 0,7)"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W {
        RF0N_W { w: self }
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark"]
    #[inline(always)]
    pub fn rf0w(&mut self) -> RF0W_W {
        RF0W_W { w: self }
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W {
        RF0F_W { w: self }
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn rf0l_(&mut self) -> RF0L__W {
        RF0L__W { w: self }
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W {
        RF1N_W { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark"]
    #[inline(always)]
    pub fn rf1w(&mut self) -> RF1W_W {
        RF1W_W { w: self }
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W {
        RF1F_W { w: self }
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub fn rf1l_(&mut self) -> RF1L__W {
        RF1L__W { w: self }
    }
    #[doc = "Bit 8 - High Priority Message 0= No high priority message received 1= High priority message received"]
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W {
        HPM_W { w: self }
    }
    #[doc = "Bit 9 - Transmission Completed 0= No transmission completed 1= Transmission completed"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    #[doc = "Bit 11 - Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element"]
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W {
        TEFN_W { w: self }
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark"]
    #[inline(always)]
    pub fn tefw(&mut self) -> TEFW_W {
        TEFW_W { w: self }
    }
    #[doc = "Bit 14 - Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W {
        TEFF_W { w: self }
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
    #[inline(always)]
    pub fn tefl_(&mut self) -> TEFL__W {
        TEFL__W { w: self }
    }
    #[doc = "Bit 16 - Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W {
        TSW_W { w: self }
    }
    #[doc = "Bit 17 - Message RAM Access Failure The flag is set, when the Rx Handler - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_TTCAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred"]
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W {
        MRAF_W { w: self }
    }
    #[doc = "Bit 18 - Timeout Occurred 0= No timeout 1= Timeout reached"]
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W {
        TOO_W { w: self }
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into a Rx Buffer"]
    #[inline(always)]
    pub fn drx(&mut self) -> DRX_W {
        DRX_W { w: self }
    }
    #[doc = "Bit 20 - M_TTCAN reports correctable ECC fault to the generic fault structure, this bit always reads as 0. Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_ttcan_aeim_berr\\[0\\]
generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)"]
    #[inline(always)]
    pub fn bec(&mut self) -> BEC_W {
        BEC_W { w: self }
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_ttcan_aeim_berr\\[1\\]
generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)"]
    #[inline(always)]
    pub fn beu(&mut self) -> BEU_W {
        BEU_W { w: self }
    }
    #[doc = "Bit 22 - Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred"]
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W {
        ELO_W { w: self }
    }
    #[doc = "Bit 23 - Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed"]
    #[inline(always)]
    pub fn ep_(&mut self) -> EP__W {
        EP__W { w: self }
    }
    #[doc = "Bit 24 - Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed"]
    #[inline(always)]
    pub fn ew_(&mut self) -> EW__W {
        EW__W { w: self }
    }
    #[doc = "Bit 25 - Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed"]
    #[inline(always)]
    pub fn bo_(&mut self) -> BO__W {
        BO__W { w: self }
    }
    #[doc = "Bit 26 - Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY"]
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W {
        WDI_W { w: self }
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC != 0,7)"]
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W {
        PEA_W { w: self }
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC != 0,7)"]
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W {
        PED_W { w: self }
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W {
        ARA_W { w: self }
    }
}
