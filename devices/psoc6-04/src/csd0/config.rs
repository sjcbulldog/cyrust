#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0x0400_0000"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400_0000
    }
}
#[doc = "Select Iref supply.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREF_SEL_A {
    #[doc = "0: select SRSS Iref (default)"]
    IREF_SRSS = 0,
    #[doc = "1: select PASS.AREF Iref, only available if PASS IP is on the chip."]
    IREF_PASS = 1,
}
impl From<IREF_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: IREF_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IREF_SEL`"]
pub type IREF_SEL_R = crate::R<bool, IREF_SEL_A>;
impl IREF_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREF_SEL_A {
        match self.bits {
            false => IREF_SEL_A::IREF_SRSS,
            true => IREF_SEL_A::IREF_PASS,
        }
    }
    #[doc = "Checks if the value of the field is `IREF_SRSS`"]
    #[inline(always)]
    pub fn is_iref_srss(&self) -> bool {
        *self == IREF_SEL_A::IREF_SRSS
    }
    #[doc = "Checks if the value of the field is `IREF_PASS`"]
    #[inline(always)]
    pub fn is_iref_pass(&self) -> bool {
        *self == IREF_SEL_A::IREF_PASS
    }
}
#[doc = "Write proxy for field `IREF_SEL`"]
pub struct IREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IREF_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREF_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "select SRSS Iref (default)"]
    #[inline(always)]
    pub fn iref_srss(self) -> &'a mut W {
        self.variant(IREF_SEL_A::IREF_SRSS)
    }
    #[doc = "select PASS.AREF Iref, only available if PASS IP is on the chip."]
    #[inline(always)]
    pub fn iref_pass(self) -> &'a mut W {
        self.variant(IREF_SEL_A::IREF_PASS)
    }
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
#[doc = "Reader of field `FILTER_DELAY`"]
pub type FILTER_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTER_DELAY`"]
pub struct FILTER_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Selects the delay by which csd_shield is delayed relative to csd_sense.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHIELD_DELAY_A {
    #[doc = "0: Delay line is off, csd_shield=csd_sense"]
    OFF = 0,
    #[doc = "1: Introduces a 5ns delay (typ)"]
    D5NS = 1,
    #[doc = "2: Introduces a 10ns delay (typ)"]
    D10NS = 2,
    #[doc = "3: Introduces a 20ns delay (typ)"]
    D20NS = 3,
}
impl From<SHIELD_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: SHIELD_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SHIELD_DELAY`"]
pub type SHIELD_DELAY_R = crate::R<u8, SHIELD_DELAY_A>;
impl SHIELD_DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIELD_DELAY_A {
        match self.bits {
            0 => SHIELD_DELAY_A::OFF,
            1 => SHIELD_DELAY_A::D5NS,
            2 => SHIELD_DELAY_A::D10NS,
            3 => SHIELD_DELAY_A::D20NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SHIELD_DELAY_A::OFF
    }
    #[doc = "Checks if the value of the field is `D5NS`"]
    #[inline(always)]
    pub fn is_d5ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D5NS
    }
    #[doc = "Checks if the value of the field is `D10NS`"]
    #[inline(always)]
    pub fn is_d10ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D10NS
    }
    #[doc = "Checks if the value of the field is `D20NS`"]
    #[inline(always)]
    pub fn is_d20ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D20NS
    }
}
#[doc = "Write proxy for field `SHIELD_DELAY`"]
pub struct SHIELD_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIELD_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHIELD_DELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Delay line is off, csd_shield=csd_sense"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::OFF)
    }
    #[doc = "Introduces a 5ns delay (typ)"]
    #[inline(always)]
    pub fn d5ns(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::D5NS)
    }
    #[doc = "Introduces a 10ns delay (typ)"]
    #[inline(always)]
    pub fn d10ns(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::D10NS)
    }
    #[doc = "Introduces a 20ns delay (typ)"]
    #[inline(always)]
    pub fn d20ns(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::D20NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `SENSE_EN`"]
pub type SENSE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSE_EN`"]
pub struct SENSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE_EN_W<'a> {
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
#[doc = "Enables full wave cap sensing mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULL_WAVE_A {
    #[doc = "0: Half Wave mode (normal).\nIn this mode the comparator always trips in the same direction (positive or negative edge) and the same Vref, i.e. no polarity change."]
    HALFWAVE = 0,
    #[doc = "1: Full Wave mode.\nIn this mode the comparator trips in opposite direction and with different Vref in each phase, i.e. the polarity flips."]
    FULLWAVE = 1,
}
impl From<FULL_WAVE_A> for bool {
    #[inline(always)]
    fn from(variant: FULL_WAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FULL_WAVE`"]
pub type FULL_WAVE_R = crate::R<bool, FULL_WAVE_A>;
impl FULL_WAVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FULL_WAVE_A {
        match self.bits {
            false => FULL_WAVE_A::HALFWAVE,
            true => FULL_WAVE_A::FULLWAVE,
        }
    }
    #[doc = "Checks if the value of the field is `HALFWAVE`"]
    #[inline(always)]
    pub fn is_halfwave(&self) -> bool {
        *self == FULL_WAVE_A::HALFWAVE
    }
    #[doc = "Checks if the value of the field is `FULLWAVE`"]
    #[inline(always)]
    pub fn is_fullwave(&self) -> bool {
        *self == FULL_WAVE_A::FULLWAVE
    }
}
#[doc = "Write proxy for field `FULL_WAVE`"]
pub struct FULL_WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_WAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FULL_WAVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Half Wave mode (normal). In this mode the comparator always trips in the same direction (positive or negative edge) and the same Vref, i.e. no polarity change."]
    #[inline(always)]
    pub fn halfwave(self) -> &'a mut W {
        self.variant(FULL_WAVE_A::HALFWAVE)
    }
    #[doc = "Full Wave mode. In this mode the comparator trips in opposite direction and with different Vref in each phase, i.e. the polarity flips."]
    #[inline(always)]
    pub fn fullwave(self) -> &'a mut W {
        self.variant(FULL_WAVE_A::FULLWAVE)
    }
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
#[doc = "Enables mutual cap sensing mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTUAL_CAP_A {
    #[doc = "0: Self-cap mode (configure sense line as CSD_SENSE)"]
    SELFCAP = 0,
    #[doc = "1: Mutual-cap mode (configure Tx line as CSD_SENSE, inverted Tx line as CSD_SHIELD and Rx Line as AMUXA). In this mode the polarity bit of the IDAC is controlled by csd_sense."]
    MUTUALCAP = 1,
}
impl From<MUTUAL_CAP_A> for bool {
    #[inline(always)]
    fn from(variant: MUTUAL_CAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUTUAL_CAP`"]
pub type MUTUAL_CAP_R = crate::R<bool, MUTUAL_CAP_A>;
impl MUTUAL_CAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTUAL_CAP_A {
        match self.bits {
            false => MUTUAL_CAP_A::SELFCAP,
            true => MUTUAL_CAP_A::MUTUALCAP,
        }
    }
    #[doc = "Checks if the value of the field is `SELFCAP`"]
    #[inline(always)]
    pub fn is_selfcap(&self) -> bool {
        *self == MUTUAL_CAP_A::SELFCAP
    }
    #[doc = "Checks if the value of the field is `MUTUALCAP`"]
    #[inline(always)]
    pub fn is_mutualcap(&self) -> bool {
        *self == MUTUAL_CAP_A::MUTUALCAP
    }
}
#[doc = "Write proxy for field `MUTUAL_CAP`"]
pub struct MUTUAL_CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTUAL_CAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTUAL_CAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Self-cap mode (configure sense line as CSD_SENSE)"]
    #[inline(always)]
    pub fn selfcap(self) -> &'a mut W {
        self.variant(MUTUAL_CAP_A::SELFCAP)
    }
    #[doc = "Mutual-cap mode (configure Tx line as CSD_SENSE, inverted Tx line as CSD_SHIELD and Rx Line as AMUXA). In this mode the polarity bit of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub fn mutualcap(self) -> &'a mut W {
        self.variant(MUTUAL_CAP_A::MUTUALCAP)
    }
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
#[doc = "Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSX_DUAL_CNT_A {
    #[doc = "0: Use one counter for both phases (source and sink)."]
    ONE = 0,
    #[doc = "1: Use two counters, separate count for when csd_sense is high and when csd_sense is low."]
    TWO = 1,
}
impl From<CSX_DUAL_CNT_A> for bool {
    #[inline(always)]
    fn from(variant: CSX_DUAL_CNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSX_DUAL_CNT`"]
pub type CSX_DUAL_CNT_R = crate::R<bool, CSX_DUAL_CNT_A>;
impl CSX_DUAL_CNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSX_DUAL_CNT_A {
        match self.bits {
            false => CSX_DUAL_CNT_A::ONE,
            true => CSX_DUAL_CNT_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSX_DUAL_CNT_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSX_DUAL_CNT_A::TWO
    }
}
#[doc = "Write proxy for field `CSX_DUAL_CNT`"]
pub struct CSX_DUAL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSX_DUAL_CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSX_DUAL_CNT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use one counter for both phases (source and sink)."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CSX_DUAL_CNT_A::ONE)
    }
    #[doc = "Use two counters, separate count for when csd_sense is high and when csd_sense is low."]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CSX_DUAL_CNT_A::TWO)
    }
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
#[doc = "Select what to output on the dsi_count bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSI_COUNT_SEL_A {
    #[doc = "0: depending on the dsi_count_val_sel input either output  RESULT_VAL1.VALUE (0) or RESULT_VAL2.VALUE (1) on the dsi_count bus. Note that dsi_count_val_sel is not synchronized, i.e. it controls the mux combinatorially."]
    CSD_RESULT = 0,
    #[doc = "1: output ADC_RES.VIN_CNT on the dsi_count bus"]
    ADC_RESULT = 1,
}
impl From<DSI_COUNT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_COUNT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSI_COUNT_SEL`"]
pub type DSI_COUNT_SEL_R = crate::R<bool, DSI_COUNT_SEL_A>;
impl DSI_COUNT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_COUNT_SEL_A {
        match self.bits {
            false => DSI_COUNT_SEL_A::CSD_RESULT,
            true => DSI_COUNT_SEL_A::ADC_RESULT,
        }
    }
    #[doc = "Checks if the value of the field is `CSD_RESULT`"]
    #[inline(always)]
    pub fn is_csd_result(&self) -> bool {
        *self == DSI_COUNT_SEL_A::CSD_RESULT
    }
    #[doc = "Checks if the value of the field is `ADC_RESULT`"]
    #[inline(always)]
    pub fn is_adc_result(&self) -> bool {
        *self == DSI_COUNT_SEL_A::ADC_RESULT
    }
}
#[doc = "Write proxy for field `DSI_COUNT_SEL`"]
pub struct DSI_COUNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_COUNT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSI_COUNT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "depending on the dsi_count_val_sel input either output RESULT_VAL1.VALUE (0) or RESULT_VAL2.VALUE (1) on the dsi_count bus. Note that dsi_count_val_sel is not synchronized, i.e. it controls the mux combinatorially."]
    #[inline(always)]
    pub fn csd_result(self) -> &'a mut W {
        self.variant(DSI_COUNT_SEL_A::CSD_RESULT)
    }
    #[doc = "output ADC_RES.VIN_CNT on the dsi_count bus"]
    #[inline(always)]
    pub fn adc_result(self) -> &'a mut W {
        self.variant(DSI_COUNT_SEL_A::ADC_RESULT)
    }
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
#[doc = "Reader of field `DSI_SAMPLE_EN`"]
pub type DSI_SAMPLE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_SAMPLE_EN`"]
pub struct DSI_SAMPLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_SAMPLE_EN_W<'a> {
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
#[doc = "Reader of field `SAMPLE_SYNC`"]
pub type SAMPLE_SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAMPLE_SYNC`"]
pub struct SAMPLE_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_SYNC_W<'a> {
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
#[doc = "Reader of field `DSI_SENSE_EN`"]
pub type DSI_SENSE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_SENSE_EN`"]
pub struct DSI_SENSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_SENSE_EN_W<'a> {
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
#[doc = "Reader of field `LP_MODE`"]
pub type LP_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LP_MODE`"]
pub struct LP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_MODE_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bit 0 - Select Iref supply."]
    #[inline(always)]
    pub fn iref_sel(&self) -> IREF_SEL_R {
        IREF_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    pub fn filter_delay(&self) -> FILTER_DELAY_R {
        FILTER_DELAY_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    pub fn shield_delay(&self) -> SHIELD_DELAY_R {
        SHIELD_DELAY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    pub fn sense_en(&self) -> SENSE_EN_R {
        SENSE_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables full wave cap sensing mode"]
    #[inline(always)]
    pub fn full_wave(&self) -> FULL_WAVE_R {
        FULL_WAVE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables mutual cap sensing mode"]
    #[inline(always)]
    pub fn mutual_cap(&self) -> MUTUAL_CAP_R {
        MUTUAL_CAP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    pub fn csx_dual_cnt(&self) -> CSX_DUAL_CNT_R {
        CSX_DUAL_CNT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Select what to output on the dsi_count bus."]
    #[inline(always)]
    pub fn dsi_count_sel(&self) -> DSI_COUNT_SEL_R {
        DSI_COUNT_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    pub fn dsi_sample_en(&self) -> DSI_SAMPLE_EN_R {
        DSI_SAMPLE_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    pub fn sample_sync(&self) -> SAMPLE_SYNC_R {
        SAMPLE_SYNC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    pub fn dsi_sense_en(&self) -> DSI_SENSE_EN_R {
        DSI_SENSE_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Iref supply."]
    #[inline(always)]
    pub fn iref_sel(&mut self) -> IREF_SEL_W {
        IREF_SEL_W { w: self }
    }
    #[doc = "Bits 4:8 - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    pub fn filter_delay(&mut self) -> FILTER_DELAY_W {
        FILTER_DELAY_W { w: self }
    }
    #[doc = "Bits 10:11 - Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    pub fn shield_delay(&mut self) -> SHIELD_DELAY_W {
        SHIELD_DELAY_W { w: self }
    }
    #[doc = "Bit 12 - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    pub fn sense_en(&mut self) -> SENSE_EN_W {
        SENSE_EN_W { w: self }
    }
    #[doc = "Bit 17 - Enables full wave cap sensing mode"]
    #[inline(always)]
    pub fn full_wave(&mut self) -> FULL_WAVE_W {
        FULL_WAVE_W { w: self }
    }
    #[doc = "Bit 18 - Enables mutual cap sensing mode"]
    #[inline(always)]
    pub fn mutual_cap(&mut self) -> MUTUAL_CAP_W {
        MUTUAL_CAP_W { w: self }
    }
    #[doc = "Bit 19 - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    pub fn csx_dual_cnt(&mut self) -> CSX_DUAL_CNT_W {
        CSX_DUAL_CNT_W { w: self }
    }
    #[doc = "Bit 24 - Select what to output on the dsi_count bus."]
    #[inline(always)]
    pub fn dsi_count_sel(&mut self) -> DSI_COUNT_SEL_W {
        DSI_COUNT_SEL_W { w: self }
    }
    #[doc = "Bit 25 - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    pub fn dsi_sample_en(&mut self) -> DSI_SAMPLE_EN_W {
        DSI_SAMPLE_EN_W { w: self }
    }
    #[doc = "Bit 26 - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    pub fn sample_sync(&mut self) -> SAMPLE_SYNC_W {
        SAMPLE_SYNC_W { w: self }
    }
    #[doc = "Bit 27 - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    pub fn dsi_sense_en(&mut self) -> DSI_SENSE_EN_W {
        DSI_SENSE_EN_W { w: self }
    }
    #[doc = "Bit 30 - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LP_MODE_W {
        LP_MODE_W { w: self }
    }
    #[doc = "Bit 31 - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
