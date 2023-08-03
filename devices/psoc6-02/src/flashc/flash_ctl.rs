#[doc = "Reader of register FLASH_CTL"]
pub type R = crate::R<u32, super::FLASH_CTL>;
#[doc = "Writer for register FLASH_CTL"]
pub type W = crate::W<u32, super::FLASH_CTL>;
#[doc = "Register FLASH_CTL `reset()`'s with value 0x0011_0000"]
impl crate::ResetValue for super::FLASH_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_0000
    }
}
#[doc = "Reader of field `MAIN_WS`"]
pub type MAIN_WS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAIN_WS`"]
pub struct MAIN_WS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_WS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `MAIN_MAP`"]
pub type MAIN_MAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAIN_MAP`"]
pub struct MAIN_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_MAP_W<'a> {
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
#[doc = "Reader of field `WORK_MAP`"]
pub type WORK_MAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WORK_MAP`"]
pub struct WORK_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> WORK_MAP_W<'a> {
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
#[doc = "Reader of field `MAIN_BANK_MODE`"]
pub type MAIN_BANK_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAIN_BANK_MODE`"]
pub struct MAIN_BANK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_BANK_MODE_W<'a> {
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
#[doc = "Reader of field `WORK_BANK_MODE`"]
pub type WORK_BANK_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WORK_BANK_MODE`"]
pub struct WORK_BANK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WORK_BANK_MODE_W<'a> {
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
#[doc = "Reader of field `MAIN_ECC_EN`"]
pub type MAIN_ECC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAIN_ECC_EN`"]
pub struct MAIN_ECC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_ECC_EN_W<'a> {
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
#[doc = "Reader of field `MAIN_ECC_INJ_EN`"]
pub type MAIN_ECC_INJ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAIN_ECC_INJ_EN`"]
pub struct MAIN_ECC_INJ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_ECC_INJ_EN_W<'a> {
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
#[doc = "Reader of field `MAIN_ERR_SILENT`"]
pub type MAIN_ERR_SILENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAIN_ERR_SILENT`"]
pub struct MAIN_ERR_SILENT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_ERR_SILENT_W<'a> {
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
#[doc = "Reader of field `WORK_ECC_EN`"]
pub type WORK_ECC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WORK_ECC_EN`"]
pub struct WORK_ECC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WORK_ECC_EN_W<'a> {
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
#[doc = "Reader of field `WORK_ECC_INJ_EN`"]
pub type WORK_ECC_INJ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WORK_ECC_INJ_EN`"]
pub struct WORK_ECC_INJ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WORK_ECC_INJ_EN_W<'a> {
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
#[doc = "Reader of field `WORK_ERR_SILENT`"]
pub type WORK_ERR_SILENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WORK_ERR_SILENT`"]
pub struct WORK_ERR_SILENT_W<'a> {
    w: &'a mut W,
}
impl<'a> WORK_ERR_SILENT_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    pub fn main_ws(&self) -> MAIN_WS_R {
        MAIN_WS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn main_map(&self) -> MAIN_MAP_R {
        MAIN_MAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn work_map(&self) -> WORK_MAP_R {
        WORK_MAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn main_bank_mode(&self) -> MAIN_BANK_MODE_R {
        MAIN_BANK_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn work_bank_mode(&self) -> WORK_BANK_MODE_R {
        WORK_BANK_MODE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn main_ecc_en(&self) -> MAIN_ECC_EN_R {
        MAIN_ECC_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn main_ecc_inj_en(&self) -> MAIN_ECC_INJ_EN_R {
        MAIN_ECC_INJ_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
    #[inline(always)]
    pub fn main_err_silent(&self) -> MAIN_ERR_SILENT_R {
        MAIN_ERR_SILENT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn work_ecc_en(&self) -> WORK_ECC_EN_R {
        WORK_ECC_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn work_ecc_inj_en(&self) -> WORK_ECC_INJ_EN_R {
        WORK_ECC_INJ_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
    #[inline(always)]
    pub fn work_err_silent(&self) -> WORK_ERR_SILENT_R {
        WORK_ERR_SILENT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    pub fn main_ws(&mut self) -> MAIN_WS_W {
        MAIN_WS_W { w: self }
    }
    #[doc = "Bit 8 - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn main_map(&mut self) -> MAIN_MAP_W {
        MAIN_MAP_W { w: self }
    }
    #[doc = "Bit 9 - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn work_map(&mut self) -> WORK_MAP_W {
        WORK_MAP_W { w: self }
    }
    #[doc = "Bit 12 - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn main_bank_mode(&mut self) -> MAIN_BANK_MODE_W {
        MAIN_BANK_MODE_W { w: self }
    }
    #[doc = "Bit 13 - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn work_bank_mode(&mut self) -> WORK_BANK_MODE_W {
        WORK_BANK_MODE_W { w: self }
    }
    #[doc = "Bit 16 - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn main_ecc_en(&mut self) -> MAIN_ECC_EN_W {
        MAIN_ECC_EN_W { w: self }
    }
    #[doc = "Bit 17 - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn main_ecc_inj_en(&mut self) -> MAIN_ECC_INJ_EN_W {
        MAIN_ECC_INJ_EN_W { w: self }
    }
    #[doc = "Bit 18 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
    #[inline(always)]
    pub fn main_err_silent(&mut self) -> MAIN_ERR_SILENT_W {
        MAIN_ERR_SILENT_W { w: self }
    }
    #[doc = "Bit 20 - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn work_ecc_en(&mut self) -> WORK_ECC_EN_W {
        WORK_ECC_EN_W { w: self }
    }
    #[doc = "Bit 21 - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn work_ecc_inj_en(&mut self) -> WORK_ECC_INJ_EN_W {
        WORK_ECC_INJ_EN_W { w: self }
    }
    #[doc = "Bit 22 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
    #[inline(always)]
    pub fn work_err_silent(&mut self) -> WORK_ERR_SILENT_W {
        WORK_ERR_SILENT_W { w: self }
    }
}
