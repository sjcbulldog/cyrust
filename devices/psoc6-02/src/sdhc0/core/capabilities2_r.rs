#[doc = "Reader of register CAPABILITIES2_R"]
pub type R = crate::R<u32, super::CAPABILITIES2_R>;
#[doc = "Reader of field `SDR50_SUPPORT`"]
pub type SDR50_SUPPORT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDR104_SUPPORT`"]
pub type SDR104_SUPPORT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DDR50_SUPPORT`"]
pub type DDR50_SUPPORT_R = crate::R<bool, bool>;
#[doc = "Reader of field `UHS2_SUPPORT`"]
pub type UHS2_SUPPORT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRV_TYPEA`"]
pub type DRV_TYPEA_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRV_TYPEC`"]
pub type DRV_TYPEC_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRV_TYPED`"]
pub type DRV_TYPED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RETUNE_CNT`"]
pub type RETUNE_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `USE_TUNING_SDR50`"]
pub type USE_TUNING_SDR50_R = crate::R<bool, bool>;
#[doc = "Reader of field `RE_TUNING_MODES`"]
pub type RE_TUNING_MODES_R = crate::R<u8, u8>;
#[doc = "Reader of field `CLK_MUL`"]
pub type CLK_MUL_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADMA3_SUPPORT`"]
pub type ADMA3_SUPPORT_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDD2_18V_SUPPORT`"]
pub type VDD2_18V_SUPPORT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SDR50 Support (UHS-I only) Thsi bit indicates that SDR50 is supported. The bit 13 (USE_TUNING_SDR50) indicates whether SDR50 requires tuning or not. Values: - 0x0 (FALSE): SDR50 is not supported - 0x1 (TRUE): SDR50 is supported"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> SDR50_SUPPORT_R {
        SDR50_SUPPORT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support (UHS-I only) This bit mentions that SDR104 requires tuning. Values: - 0x0 (FALSE): SDR104 is not supported - 0x1 (TRUE): SDR104 is supported (NOT ACTUALLY SUPPORTED)"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> SDR104_SUPPORT_R {
        SDR104_SUPPORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support (UHS-I only) Values: - 0x0 (FALSE): DDR50 is not supported - 0x1 (TRUE): DDR50 is supported"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> DDR50_SUPPORT_R {
        DDR50_SUPPORT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UHS-II Support (UHS-II only) This bit indicates whether Host Controller supports UHS-II. Values: - 0x0 (FALSE): UHS-II is not supported - 0x1 (TRUE): UHS-II is supported"]
    #[inline(always)]
    pub fn uhs2_support(&self) -> UHS2_SUPPORT_R {
        UHS2_SUPPORT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support (UHS-I only) This bit indicates support of Driver Type A for 1.8 Signaling. Values: - 0x0 (FALSE): Driver Type A is not supported - 0x1 (TRUE): Driver Type A is supported"]
    #[inline(always)]
    pub fn drv_typea(&self) -> DRV_TYPEA_R {
        DRV_TYPEA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support (UHS-I only) This bit indicates support of Driver Type C for 1.8 Signaling. Values: - 0x0 (FALSE): Driver Type C is not supported - 0x1 (TRUE): Driver Type C is supported"]
    #[inline(always)]
    pub fn drv_typec(&self) -> DRV_TYPEC_R {
        DRV_TYPEC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support (UHS-I only) This bit indicates support of Driver Type D for 1.8 Signaling. Values: - 0x0 (FALSE): Driver Type D is not supported - 0x1 (TRUE): Driver Type D is supported"]
    #[inline(always)]
    pub fn drv_typed(&self) -> DRV_TYPED_R {
        DRV_TYPED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - N/A"]
    #[inline(always)]
    pub fn retune_cnt(&self) -> RETUNE_CNT_R {
        RETUNE_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50 (UHS-I only) Values: - 0x0 (ZERO): SDR50 does not require tuning - 0x1 (ONE): SDR50 requires tuning"]
    #[inline(always)]
    pub fn use_tuning_sdr50(&self) -> USE_TUNING_SDR50_R {
        USE_TUNING_SDR50_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - N/A"]
    #[inline(always)]
    pub fn re_tuning_modes(&self) -> RE_TUNING_MODES_R {
        RE_TUNING_MODES_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier These bits indicate the clock multiplier of the programmable clock generator. Setting these bits to 0 means that the Host Controller does not support a programmable clock generator. - 0x0: Clock Multiplier is not Supported - 0x1: Clock Multiplier M = 2 - 0x2: Clock Multiplier M = 3 - ......... - 0xFF: Clock Multiplier M = 256"]
    #[inline(always)]
    pub fn clk_mul(&self) -> CLK_MUL_R {
        CLK_MUL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 27 - ADMA3 Support This bit indicates whether the Host Controller is capable of using ADMA3. Values: - 0x0 (FALSE): ADMA3 not Supported - 0x1 (TRUE): ADMA3 Supported"]
    #[inline(always)]
    pub fn adma3_support(&self) -> ADMA3_SUPPORT_R {
        ADMA3_SUPPORT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 1.8V VDD2 Support This bit indicates support of VDD2 for the Host System. Values: - 0x0 (FALSE): 1.8V VDD2 is not Supported - 0x1 (TRUE): 1.8V VDD2 is Supported"]
    #[inline(always)]
    pub fn vdd2_18v_support(&self) -> VDD2_18V_SUPPORT_R {
        VDD2_18V_SUPPORT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
