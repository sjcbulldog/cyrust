#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt cause register"]
    pub intr_cause: INTR_CAUSE,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Deepsleep clock select"]
    pub dpslp_clock_sel: DPSLP_CLOCK_SEL,
    #[doc = "0x14 - Analog power configuration"]
    pub ana_pwr_cfg: ANA_PWR_CFG,
    _reserved3: [u8; 8usize],
    #[doc = "0x20 - Clock select for CTBm"]
    pub ctbm_clock_sel: CTBM_CLOCK_SEL,
    _reserved4: [u8; 12usize],
    #[doc = "0x30 - Deepsleep control for SARv3"]
    pub sar_dpslp_ctrl: [SAR_DPSLP_CTRL; 2],
    _reserved5: [u8; 8usize],
    #[doc = "0x40 - Clock select for SARv3"]
    pub sar_clock_sel: [SAR_CLOCK_SEL; 2],
    _reserved6: [u8; 8usize],
    #[doc = "0x50 - SAR trigger scan control status"]
    pub sar_tr_scan_cnt_status: [SAR_TR_SCAN_CNT_STATUS; 2],
    _reserved7: [u8; 8usize],
    #[doc = "0x60 - SAR trigger scan control"]
    pub sar_tr_scan_cnt: SAR_TR_SCAN_CNT,
    #[doc = "0x64 - SAR HW trigger override"]
    pub sar_ovr_ctrl: SAR_OVR_CTRL,
    #[doc = "0x68 - SAR simultaneous trigger control"]
    pub sar_simult_ctrl: SAR_SIMULT_CTRL,
    #[doc = "0x6c - SAR simultaneous start control"]
    pub sar_simult_fw_start_ctrl: SAR_SIMULT_FW_START_CTRL,
    #[doc = "0x70 - SAR trigger out control"]
    pub sar_tr_out_ctrl: SAR_TR_OUT_CTRL,
    _reserved12: [u8; 140usize],
    #[doc = "0x100 - Programmable Analog Subsystem"]
    pub timer: TIMER,
    _reserved13: [u8; 244usize],
    #[doc = "0x200 - LPOSC configuration"]
    pub lposc: LPOSC,
    _reserved14: [u8; 244usize],
    #[doc = "0x300 - FIFO configuration"]
    pub fifo0: FIFO,
    _reserved15: [u8; 208usize],
    #[doc = "0x400 - FIFO configuration"]
    pub fifo1: FIFO,
    _reserved16: [u8; 2512usize],
    #[doc = "0xe00 - AREF configuration"]
    pub arefv2: AREFV2,
    _reserved17: [u8; 252usize],
    #[doc = "0xf00 - VREF Trim bits"]
    pub vref_trim0: VREF_TRIM0,
    #[doc = "0xf04 - VREF Trim bits"]
    pub vref_trim1: VREF_TRIM1,
    #[doc = "0xf08 - VREF Trim bits"]
    pub vref_trim2: VREF_TRIM2,
    #[doc = "0xf0c - VREF Trim bits"]
    pub vref_trim3: VREF_TRIM3,
    #[doc = "0xf10 - VREF Trim bits"]
    pub iztat_trim0: IZTAT_TRIM0,
    #[doc = "0xf14 - IZTAT Trim bits"]
    pub iztat_trim1: IZTAT_TRIM1,
    #[doc = "0xf18 - IPTAT Trim bits"]
    pub iptat_trim0: IPTAT_TRIM0,
    #[doc = "0xf1c - ICTAT Trim bits"]
    pub ictat_trim0: ICTAT_TRIM0,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TIMER {
    #[doc = "0x00 - Timer trigger control register"]
    pub ctrl: self::timer::CTRL,
    #[doc = "0x04 - Timer trigger configuration register"]
    pub config: self::timer::CONFIG,
    #[doc = "0x08 - Timer trigger period register"]
    pub period: self::timer::PERIOD,
}
#[doc = r"Register block"]
#[doc = "Programmable Analog Subsystem"]
pub mod timer;
#[doc = r"Register block"]
#[repr(C)]
pub struct LPOSC {
    #[doc = "0x00 - Low Power Oscillator control"]
    pub ctrl: self::lposc::CTRL,
    #[doc = "0x04 - Low Power Oscillator configuration register"]
    pub config: self::lposc::CONFIG,
    #[doc = "0x08 - Retention, Hidden"]
    pub adft: self::lposc::ADFT,
}
#[doc = r"Register block"]
#[doc = "LPOSC configuration"]
pub mod lposc;
#[doc = r"Register block"]
#[repr(C)]
pub struct FIFO {
    #[doc = "0x00 - FIFO control register"]
    pub ctrl: self::fifo::CTRL,
    #[doc = "0x04 - FIFO configuration register"]
    pub config: self::fifo::CONFIG,
    #[doc = "0x08 - FIFO clear register"]
    pub clear: self::fifo::CLEAR,
    #[doc = "0x0c - FIFO level register"]
    pub level: self::fifo::LEVEL,
    #[doc = "0x10 - FIFO used register"]
    pub used: self::fifo::USED,
    #[doc = "0x14 - FIFO status register"]
    pub status: self::fifo::STATUS,
    #[doc = "0x18 - FIFO read data register"]
    pub rd_data: self::fifo::RD_DATA,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Interrupt register"]
    pub intr: self::fifo::INTR,
    #[doc = "0x24 - Interrupt set register"]
    pub intr_set: self::fifo::INTR_SET,
    #[doc = "0x28 - Interrupt mask register"]
    pub intr_mask: self::fifo::INTR_MASK,
    #[doc = "0x2c - Interrupt masked register"]
    pub intr_masked: self::fifo::INTR_MASKED,
}
#[doc = r"Register block"]
#[doc = "FIFO configuration"]
pub mod fifo;
#[doc = r"Register block"]
#[repr(C)]
pub struct AREFV2 {
    #[doc = "0x00 - global AREF control"]
    pub aref_ctrl: self::arefv2::AREF_CTRL,
}
#[doc = r"Register block"]
#[doc = "AREF configuration"]
pub mod arefv2;
#[doc = "Interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](intr_cause) module"]
pub type INTR_CAUSE = crate::Reg<u32, _INTR_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE;
#[doc = "`read()` method returns [intr_cause::R](intr_cause::R) reader structure"]
impl crate::Readable for INTR_CAUSE {}
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "Deepsleep clock select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpslp_clock_sel](dpslp_clock_sel) module"]
pub type DPSLP_CLOCK_SEL = crate::Reg<u32, _DPSLP_CLOCK_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPSLP_CLOCK_SEL;
#[doc = "`read()` method returns [dpslp_clock_sel::R](dpslp_clock_sel::R) reader structure"]
impl crate::Readable for DPSLP_CLOCK_SEL {}
#[doc = "`write(|w| ..)` method takes [dpslp_clock_sel::W](dpslp_clock_sel::W) writer structure"]
impl crate::Writable for DPSLP_CLOCK_SEL {}
#[doc = "Deepsleep clock select"]
pub mod dpslp_clock_sel;
#[doc = "Analog power configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_pwr_cfg](ana_pwr_cfg) module"]
pub type ANA_PWR_CFG = crate::Reg<u32, _ANA_PWR_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_PWR_CFG;
#[doc = "`read()` method returns [ana_pwr_cfg::R](ana_pwr_cfg::R) reader structure"]
impl crate::Readable for ANA_PWR_CFG {}
#[doc = "`write(|w| ..)` method takes [ana_pwr_cfg::W](ana_pwr_cfg::W) writer structure"]
impl crate::Writable for ANA_PWR_CFG {}
#[doc = "Analog power configuration"]
pub mod ana_pwr_cfg;
#[doc = "Clock select for CTBm\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctbm_clock_sel](ctbm_clock_sel) module"]
pub type CTBM_CLOCK_SEL = crate::Reg<u32, _CTBM_CLOCK_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTBM_CLOCK_SEL;
#[doc = "`read()` method returns [ctbm_clock_sel::R](ctbm_clock_sel::R) reader structure"]
impl crate::Readable for CTBM_CLOCK_SEL {}
#[doc = "`write(|w| ..)` method takes [ctbm_clock_sel::W](ctbm_clock_sel::W) writer structure"]
impl crate::Writable for CTBM_CLOCK_SEL {}
#[doc = "Clock select for CTBm"]
pub mod ctbm_clock_sel;
#[doc = "Deepsleep control for SARv3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_dpslp_ctrl](sar_dpslp_ctrl) module"]
pub type SAR_DPSLP_CTRL = crate::Reg<u32, _SAR_DPSLP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_DPSLP_CTRL;
#[doc = "`read()` method returns [sar_dpslp_ctrl::R](sar_dpslp_ctrl::R) reader structure"]
impl crate::Readable for SAR_DPSLP_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_dpslp_ctrl::W](sar_dpslp_ctrl::W) writer structure"]
impl crate::Writable for SAR_DPSLP_CTRL {}
#[doc = "Deepsleep control for SARv3"]
pub mod sar_dpslp_ctrl;
#[doc = "Clock select for SARv3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_clock_sel](sar_clock_sel) module"]
pub type SAR_CLOCK_SEL = crate::Reg<u32, _SAR_CLOCK_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_CLOCK_SEL;
#[doc = "`read()` method returns [sar_clock_sel::R](sar_clock_sel::R) reader structure"]
impl crate::Readable for SAR_CLOCK_SEL {}
#[doc = "`write(|w| ..)` method takes [sar_clock_sel::W](sar_clock_sel::W) writer structure"]
impl crate::Writable for SAR_CLOCK_SEL {}
#[doc = "Clock select for SARv3"]
pub mod sar_clock_sel;
#[doc = "SAR trigger scan control status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tr_scan_cnt_status](sar_tr_scan_cnt_status) module"]
pub type SAR_TR_SCAN_CNT_STATUS = crate::Reg<u32, _SAR_TR_SCAN_CNT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TR_SCAN_CNT_STATUS;
#[doc = "`read()` method returns [sar_tr_scan_cnt_status::R](sar_tr_scan_cnt_status::R) reader structure"]
impl crate::Readable for SAR_TR_SCAN_CNT_STATUS {}
#[doc = "SAR trigger scan control status"]
pub mod sar_tr_scan_cnt_status;
#[doc = "SAR trigger scan control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tr_scan_cnt](sar_tr_scan_cnt) module"]
pub type SAR_TR_SCAN_CNT = crate::Reg<u32, _SAR_TR_SCAN_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TR_SCAN_CNT;
#[doc = "`read()` method returns [sar_tr_scan_cnt::R](sar_tr_scan_cnt::R) reader structure"]
impl crate::Readable for SAR_TR_SCAN_CNT {}
#[doc = "`write(|w| ..)` method takes [sar_tr_scan_cnt::W](sar_tr_scan_cnt::W) writer structure"]
impl crate::Writable for SAR_TR_SCAN_CNT {}
#[doc = "SAR trigger scan control"]
pub mod sar_tr_scan_cnt;
#[doc = "SAR HW trigger override\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_ovr_ctrl](sar_ovr_ctrl) module"]
pub type SAR_OVR_CTRL = crate::Reg<u32, _SAR_OVR_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_OVR_CTRL;
#[doc = "`read()` method returns [sar_ovr_ctrl::R](sar_ovr_ctrl::R) reader structure"]
impl crate::Readable for SAR_OVR_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_ovr_ctrl::W](sar_ovr_ctrl::W) writer structure"]
impl crate::Writable for SAR_OVR_CTRL {}
#[doc = "SAR HW trigger override"]
pub mod sar_ovr_ctrl;
#[doc = "SAR simultaneous trigger control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_simult_ctrl](sar_simult_ctrl) module"]
pub type SAR_SIMULT_CTRL = crate::Reg<u32, _SAR_SIMULT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_SIMULT_CTRL;
#[doc = "`read()` method returns [sar_simult_ctrl::R](sar_simult_ctrl::R) reader structure"]
impl crate::Readable for SAR_SIMULT_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_simult_ctrl::W](sar_simult_ctrl::W) writer structure"]
impl crate::Writable for SAR_SIMULT_CTRL {}
#[doc = "SAR simultaneous trigger control"]
pub mod sar_simult_ctrl;
#[doc = "SAR simultaneous start control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_simult_fw_start_ctrl](sar_simult_fw_start_ctrl) module"]
pub type SAR_SIMULT_FW_START_CTRL = crate::Reg<u32, _SAR_SIMULT_FW_START_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_SIMULT_FW_START_CTRL;
#[doc = "`read()` method returns [sar_simult_fw_start_ctrl::R](sar_simult_fw_start_ctrl::R) reader structure"]
impl crate::Readable for SAR_SIMULT_FW_START_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_simult_fw_start_ctrl::W](sar_simult_fw_start_ctrl::W) writer structure"]
impl crate::Writable for SAR_SIMULT_FW_START_CTRL {}
#[doc = "SAR simultaneous start control"]
pub mod sar_simult_fw_start_ctrl;
#[doc = "SAR trigger out control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tr_out_ctrl](sar_tr_out_ctrl) module"]
pub type SAR_TR_OUT_CTRL = crate::Reg<u32, _SAR_TR_OUT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR_TR_OUT_CTRL;
#[doc = "`read()` method returns [sar_tr_out_ctrl::R](sar_tr_out_ctrl::R) reader structure"]
impl crate::Readable for SAR_TR_OUT_CTRL {}
#[doc = "`write(|w| ..)` method takes [sar_tr_out_ctrl::W](sar_tr_out_ctrl::W) writer structure"]
impl crate::Writable for SAR_TR_OUT_CTRL {}
#[doc = "SAR trigger out control"]
pub mod sar_tr_out_ctrl;
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim0](vref_trim0) module"]
pub type VREF_TRIM0 = crate::Reg<u32, _VREF_TRIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF_TRIM0;
#[doc = "`read()` method returns [vref_trim0::R](vref_trim0::R) reader structure"]
impl crate::Readable for VREF_TRIM0 {}
#[doc = "`write(|w| ..)` method takes [vref_trim0::W](vref_trim0::W) writer structure"]
impl crate::Writable for VREF_TRIM0 {}
#[doc = "VREF Trim bits"]
pub mod vref_trim0;
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim1](vref_trim1) module"]
pub type VREF_TRIM1 = crate::Reg<u32, _VREF_TRIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF_TRIM1;
#[doc = "`read()` method returns [vref_trim1::R](vref_trim1::R) reader structure"]
impl crate::Readable for VREF_TRIM1 {}
#[doc = "`write(|w| ..)` method takes [vref_trim1::W](vref_trim1::W) writer structure"]
impl crate::Writable for VREF_TRIM1 {}
#[doc = "VREF Trim bits"]
pub mod vref_trim1;
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim2](vref_trim2) module"]
pub type VREF_TRIM2 = crate::Reg<u32, _VREF_TRIM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF_TRIM2;
#[doc = "`read()` method returns [vref_trim2::R](vref_trim2::R) reader structure"]
impl crate::Readable for VREF_TRIM2 {}
#[doc = "`write(|w| ..)` method takes [vref_trim2::W](vref_trim2::W) writer structure"]
impl crate::Writable for VREF_TRIM2 {}
#[doc = "VREF Trim bits"]
pub mod vref_trim2;
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim3](vref_trim3) module"]
pub type VREF_TRIM3 = crate::Reg<u32, _VREF_TRIM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF_TRIM3;
#[doc = "`read()` method returns [vref_trim3::R](vref_trim3::R) reader structure"]
impl crate::Readable for VREF_TRIM3 {}
#[doc = "`write(|w| ..)` method takes [vref_trim3::W](vref_trim3::W) writer structure"]
impl crate::Writable for VREF_TRIM3 {}
#[doc = "VREF Trim bits"]
pub mod vref_trim3;
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iztat_trim0](iztat_trim0) module"]
pub type IZTAT_TRIM0 = crate::Reg<u32, _IZTAT_TRIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IZTAT_TRIM0;
#[doc = "`read()` method returns [iztat_trim0::R](iztat_trim0::R) reader structure"]
impl crate::Readable for IZTAT_TRIM0 {}
#[doc = "`write(|w| ..)` method takes [iztat_trim0::W](iztat_trim0::W) writer structure"]
impl crate::Writable for IZTAT_TRIM0 {}
#[doc = "VREF Trim bits"]
pub mod iztat_trim0;
#[doc = "IZTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iztat_trim1](iztat_trim1) module"]
pub type IZTAT_TRIM1 = crate::Reg<u32, _IZTAT_TRIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IZTAT_TRIM1;
#[doc = "`read()` method returns [iztat_trim1::R](iztat_trim1::R) reader structure"]
impl crate::Readable for IZTAT_TRIM1 {}
#[doc = "`write(|w| ..)` method takes [iztat_trim1::W](iztat_trim1::W) writer structure"]
impl crate::Writable for IZTAT_TRIM1 {}
#[doc = "IZTAT Trim bits"]
pub mod iztat_trim1;
#[doc = "IPTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptat_trim0](iptat_trim0) module"]
pub type IPTAT_TRIM0 = crate::Reg<u32, _IPTAT_TRIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPTAT_TRIM0;
#[doc = "`read()` method returns [iptat_trim0::R](iptat_trim0::R) reader structure"]
impl crate::Readable for IPTAT_TRIM0 {}
#[doc = "`write(|w| ..)` method takes [iptat_trim0::W](iptat_trim0::W) writer structure"]
impl crate::Writable for IPTAT_TRIM0 {}
#[doc = "IPTAT Trim bits"]
pub mod iptat_trim0;
#[doc = "ICTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ictat_trim0](ictat_trim0) module"]
pub type ICTAT_TRIM0 = crate::Reg<u32, _ICTAT_TRIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICTAT_TRIM0;
#[doc = "`read()` method returns [ictat_trim0::R](ictat_trim0::R) reader structure"]
impl crate::Readable for ICTAT_TRIM0 {}
#[doc = "`write(|w| ..)` method takes [ictat_trim0::W](ictat_trim0::W) writer structure"]
impl crate::Writable for ICTAT_TRIM0 {}
#[doc = "ICTAT Trim bits"]
pub mod ictat_trim0;
