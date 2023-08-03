#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    _reserved2: [u8; 24usize],
    #[doc = "0x20 - Active descriptor control"]
    pub act_descr_ctl: ACT_DESCR_CTL,
    #[doc = "0x24 - Active descriptor source"]
    pub act_descr_src: ACT_DESCR_SRC,
    #[doc = "0x28 - Active descriptor destination"]
    pub act_descr_dst: ACT_DESCR_DST,
    _reserved5: [u8; 4usize],
    #[doc = "0x30 - Active descriptor X loop control"]
    pub act_descr_x_ctl: ACT_DESCR_X_CTL,
    #[doc = "0x34 - Active descriptor Y loop control"]
    pub act_descr_y_ctl: ACT_DESCR_Y_CTL,
    #[doc = "0x38 - Active descriptor next pointer"]
    pub act_descr_next_ptr: ACT_DESCR_NEXT_PTR,
    _reserved8: [u8; 4usize],
    #[doc = "0x40 - Active source"]
    pub act_src: ACT_SRC,
    #[doc = "0x44 - Active destination"]
    pub act_dst: ACT_DST,
    _reserved10: [u8; 56usize],
    #[doc = "0x80 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved11: [u8; 124usize],
    #[doc = "0x100 - CRC control"]
    pub crc_ctl: CRC_CTL,
    _reserved12: [u8; 12usize],
    #[doc = "0x110 - CRC data control"]
    pub crc_data_ctl: CRC_DATA_CTL,
    _reserved13: [u8; 12usize],
    #[doc = "0x120 - CRC polynomial control"]
    pub crc_pol_ctl: CRC_POL_CTL,
    _reserved14: [u8; 12usize],
    #[doc = "0x130 - CRC LFSR control"]
    pub crc_lfsr_ctl: CRC_LFSR_CTL,
    _reserved15: [u8; 12usize],
    #[doc = "0x140 - CRC remainder control"]
    pub crc_rem_ctl: CRC_REM_CTL,
    _reserved16: [u8; 4usize],
    #[doc = "0x148 - CRC remainder result"]
    pub crc_rem_result: CRC_REM_RESULT,
    _reserved17: [u8; 32436usize],
    #[doc = "0x8000 - DW channel structure"]
    pub ch_struct0: CH_STRUCT,
    _reserved18: [u8; 20usize],
    #[doc = "0x8040 - DW channel structure"]
    pub ch_struct1: CH_STRUCT,
    _reserved19: [u8; 20usize],
    #[doc = "0x8080 - DW channel structure"]
    pub ch_struct2: CH_STRUCT,
    _reserved20: [u8; 20usize],
    #[doc = "0x80c0 - DW channel structure"]
    pub ch_struct3: CH_STRUCT,
    _reserved21: [u8; 20usize],
    #[doc = "0x8100 - DW channel structure"]
    pub ch_struct4: CH_STRUCT,
    _reserved22: [u8; 20usize],
    #[doc = "0x8140 - DW channel structure"]
    pub ch_struct5: CH_STRUCT,
    _reserved23: [u8; 20usize],
    #[doc = "0x8180 - DW channel structure"]
    pub ch_struct6: CH_STRUCT,
    _reserved24: [u8; 20usize],
    #[doc = "0x81c0 - DW channel structure"]
    pub ch_struct7: CH_STRUCT,
    _reserved25: [u8; 20usize],
    #[doc = "0x8200 - DW channel structure"]
    pub ch_struct8: CH_STRUCT,
    _reserved26: [u8; 20usize],
    #[doc = "0x8240 - DW channel structure"]
    pub ch_struct9: CH_STRUCT,
    _reserved27: [u8; 20usize],
    #[doc = "0x8280 - DW channel structure"]
    pub ch_struct10: CH_STRUCT,
    _reserved28: [u8; 20usize],
    #[doc = "0x82c0 - DW channel structure"]
    pub ch_struct11: CH_STRUCT,
    _reserved29: [u8; 20usize],
    #[doc = "0x8300 - DW channel structure"]
    pub ch_struct12: CH_STRUCT,
    _reserved30: [u8; 20usize],
    #[doc = "0x8340 - DW channel structure"]
    pub ch_struct13: CH_STRUCT,
    _reserved31: [u8; 20usize],
    #[doc = "0x8380 - DW channel structure"]
    pub ch_struct14: CH_STRUCT,
    _reserved32: [u8; 20usize],
    #[doc = "0x83c0 - DW channel structure"]
    pub ch_struct15: CH_STRUCT,
    _reserved33: [u8; 20usize],
    #[doc = "0x8400 - DW channel structure"]
    pub ch_struct16: CH_STRUCT,
    _reserved34: [u8; 20usize],
    #[doc = "0x8440 - DW channel structure"]
    pub ch_struct17: CH_STRUCT,
    _reserved35: [u8; 20usize],
    #[doc = "0x8480 - DW channel structure"]
    pub ch_struct18: CH_STRUCT,
    _reserved36: [u8; 20usize],
    #[doc = "0x84c0 - DW channel structure"]
    pub ch_struct19: CH_STRUCT,
    _reserved37: [u8; 20usize],
    #[doc = "0x8500 - DW channel structure"]
    pub ch_struct20: CH_STRUCT,
    _reserved38: [u8; 20usize],
    #[doc = "0x8540 - DW channel structure"]
    pub ch_struct21: CH_STRUCT,
    _reserved39: [u8; 20usize],
    #[doc = "0x8580 - DW channel structure"]
    pub ch_struct22: CH_STRUCT,
    _reserved40: [u8; 20usize],
    #[doc = "0x85c0 - DW channel structure"]
    pub ch_struct23: CH_STRUCT,
    _reserved41: [u8; 20usize],
    #[doc = "0x8600 - DW channel structure"]
    pub ch_struct24: CH_STRUCT,
    _reserved42: [u8; 20usize],
    #[doc = "0x8640 - DW channel structure"]
    pub ch_struct25: CH_STRUCT,
    _reserved43: [u8; 20usize],
    #[doc = "0x8680 - DW channel structure"]
    pub ch_struct26: CH_STRUCT,
    _reserved44: [u8; 20usize],
    #[doc = "0x86c0 - DW channel structure"]
    pub ch_struct27: CH_STRUCT,
    _reserved45: [u8; 20usize],
    #[doc = "0x8700 - DW channel structure"]
    pub ch_struct28: CH_STRUCT,
    _reserved46: [u8; 20usize],
    #[doc = "0x8740 - DW channel structure"]
    pub ch_struct29: CH_STRUCT,
    _reserved47: [u8; 20usize],
    #[doc = "0x8780 - DW channel structure"]
    pub ch_struct30: CH_STRUCT,
    _reserved48: [u8; 20usize],
    #[doc = "0x87c0 - DW channel structure"]
    pub ch_struct31: CH_STRUCT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH_STRUCT {
    #[doc = "0x00 - Channel control"]
    pub ch_ctl: self::ch_struct::CH_CTL,
    #[doc = "0x04 - Channel status"]
    pub ch_status: self::ch_struct::CH_STATUS,
    #[doc = "0x08 - Channel current indices"]
    pub ch_idx: self::ch_struct::CH_IDX,
    #[doc = "0x0c - Channel current descriptor pointer"]
    pub ch_curr_ptr: self::ch_struct::CH_CURR_PTR,
    #[doc = "0x10 - Interrupt"]
    pub intr: self::ch_struct::INTR,
    #[doc = "0x14 - Interrupt set"]
    pub intr_set: self::ch_struct::INTR_SET,
    #[doc = "0x18 - Interrupt mask"]
    pub intr_mask: self::ch_struct::INTR_MASK,
    #[doc = "0x1c - Interrupt masked"]
    pub intr_masked: self::ch_struct::INTR_MASKED,
    #[doc = "0x20 - SRAM data 0"]
    pub sram_data0: self::ch_struct::SRAM_DATA0,
    #[doc = "0x24 - SRAM data 1"]
    pub sram_data1: self::ch_struct::SRAM_DATA1,
    #[doc = "0x28 - Channel software trigger"]
    pub tr_cmd: self::ch_struct::TR_CMD,
}
#[doc = r"Register block"]
#[doc = "DW channel structure"]
pub mod ch_struct;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control"]
pub mod ctl;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Active descriptor control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_ctl](act_descr_ctl) module"]
pub type ACT_DESCR_CTL = crate::Reg<u32, _ACT_DESCR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_CTL;
#[doc = "`read()` method returns [act_descr_ctl::R](act_descr_ctl::R) reader structure"]
impl crate::Readable for ACT_DESCR_CTL {}
#[doc = "Active descriptor control"]
pub mod act_descr_ctl;
#[doc = "Active descriptor source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_src](act_descr_src) module"]
pub type ACT_DESCR_SRC = crate::Reg<u32, _ACT_DESCR_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_SRC;
#[doc = "`read()` method returns [act_descr_src::R](act_descr_src::R) reader structure"]
impl crate::Readable for ACT_DESCR_SRC {}
#[doc = "Active descriptor source"]
pub mod act_descr_src;
#[doc = "Active descriptor destination\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_dst](act_descr_dst) module"]
pub type ACT_DESCR_DST = crate::Reg<u32, _ACT_DESCR_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_DST;
#[doc = "`read()` method returns [act_descr_dst::R](act_descr_dst::R) reader structure"]
impl crate::Readable for ACT_DESCR_DST {}
#[doc = "Active descriptor destination"]
pub mod act_descr_dst;
#[doc = "Active descriptor X loop control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_x_ctl](act_descr_x_ctl) module"]
pub type ACT_DESCR_X_CTL = crate::Reg<u32, _ACT_DESCR_X_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_X_CTL;
#[doc = "`read()` method returns [act_descr_x_ctl::R](act_descr_x_ctl::R) reader structure"]
impl crate::Readable for ACT_DESCR_X_CTL {}
#[doc = "Active descriptor X loop control"]
pub mod act_descr_x_ctl;
#[doc = "Active descriptor Y loop control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_y_ctl](act_descr_y_ctl) module"]
pub type ACT_DESCR_Y_CTL = crate::Reg<u32, _ACT_DESCR_Y_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_Y_CTL;
#[doc = "`read()` method returns [act_descr_y_ctl::R](act_descr_y_ctl::R) reader structure"]
impl crate::Readable for ACT_DESCR_Y_CTL {}
#[doc = "Active descriptor Y loop control"]
pub mod act_descr_y_ctl;
#[doc = "Active descriptor next pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_next_ptr](act_descr_next_ptr) module"]
pub type ACT_DESCR_NEXT_PTR = crate::Reg<u32, _ACT_DESCR_NEXT_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DESCR_NEXT_PTR;
#[doc = "`read()` method returns [act_descr_next_ptr::R](act_descr_next_ptr::R) reader structure"]
impl crate::Readable for ACT_DESCR_NEXT_PTR {}
#[doc = "Active descriptor next pointer"]
pub mod act_descr_next_ptr;
#[doc = "Active source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_src](act_src) module"]
pub type ACT_SRC = crate::Reg<u32, _ACT_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_SRC;
#[doc = "`read()` method returns [act_src::R](act_src::R) reader structure"]
impl crate::Readable for ACT_SRC {}
#[doc = "Active source"]
pub mod act_src;
#[doc = "Active destination\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_dst](act_dst) module"]
pub type ACT_DST = crate::Reg<u32, _ACT_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACT_DST;
#[doc = "`read()` method returns [act_dst::R](act_dst::R) reader structure"]
impl crate::Readable for ACT_DST {}
#[doc = "Active destination"]
pub mod act_dst;
#[doc = "ECC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_ctl](ecc_ctl) module"]
pub type ECC_CTL = crate::Reg<u32, _ECC_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_CTL;
#[doc = "`read()` method returns [ecc_ctl::R](ecc_ctl::R) reader structure"]
impl crate::Readable for ECC_CTL {}
#[doc = "`write(|w| ..)` method takes [ecc_ctl::W](ecc_ctl::W) writer structure"]
impl crate::Writable for ECC_CTL {}
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "CRC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_ctl](crc_ctl) module"]
pub type CRC_CTL = crate::Reg<u32, _CRC_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_CTL;
#[doc = "`read()` method returns [crc_ctl::R](crc_ctl::R) reader structure"]
impl crate::Readable for CRC_CTL {}
#[doc = "`write(|w| ..)` method takes [crc_ctl::W](crc_ctl::W) writer structure"]
impl crate::Writable for CRC_CTL {}
#[doc = "CRC control"]
pub mod crc_ctl;
#[doc = "CRC data control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_data_ctl](crc_data_ctl) module"]
pub type CRC_DATA_CTL = crate::Reg<u32, _CRC_DATA_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_DATA_CTL;
#[doc = "`read()` method returns [crc_data_ctl::R](crc_data_ctl::R) reader structure"]
impl crate::Readable for CRC_DATA_CTL {}
#[doc = "`write(|w| ..)` method takes [crc_data_ctl::W](crc_data_ctl::W) writer structure"]
impl crate::Writable for CRC_DATA_CTL {}
#[doc = "CRC data control"]
pub mod crc_data_ctl;
#[doc = "CRC polynomial control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_pol_ctl](crc_pol_ctl) module"]
pub type CRC_POL_CTL = crate::Reg<u32, _CRC_POL_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_POL_CTL;
#[doc = "`read()` method returns [crc_pol_ctl::R](crc_pol_ctl::R) reader structure"]
impl crate::Readable for CRC_POL_CTL {}
#[doc = "`write(|w| ..)` method takes [crc_pol_ctl::W](crc_pol_ctl::W) writer structure"]
impl crate::Writable for CRC_POL_CTL {}
#[doc = "CRC polynomial control"]
pub mod crc_pol_ctl;
#[doc = "CRC LFSR control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_lfsr_ctl](crc_lfsr_ctl) module"]
pub type CRC_LFSR_CTL = crate::Reg<u32, _CRC_LFSR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_LFSR_CTL;
#[doc = "`read()` method returns [crc_lfsr_ctl::R](crc_lfsr_ctl::R) reader structure"]
impl crate::Readable for CRC_LFSR_CTL {}
#[doc = "`write(|w| ..)` method takes [crc_lfsr_ctl::W](crc_lfsr_ctl::W) writer structure"]
impl crate::Writable for CRC_LFSR_CTL {}
#[doc = "CRC LFSR control"]
pub mod crc_lfsr_ctl;
#[doc = "CRC remainder control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_rem_ctl](crc_rem_ctl) module"]
pub type CRC_REM_CTL = crate::Reg<u32, _CRC_REM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_REM_CTL;
#[doc = "`read()` method returns [crc_rem_ctl::R](crc_rem_ctl::R) reader structure"]
impl crate::Readable for CRC_REM_CTL {}
#[doc = "`write(|w| ..)` method takes [crc_rem_ctl::W](crc_rem_ctl::W) writer structure"]
impl crate::Writable for CRC_REM_CTL {}
#[doc = "CRC remainder control"]
pub mod crc_rem_ctl;
#[doc = "CRC remainder result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_rem_result](crc_rem_result) module"]
pub type CRC_REM_RESULT = crate::Reg<u32, _CRC_REM_RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_REM_RESULT;
#[doc = "`read()` method returns [crc_rem_result::R](crc_rem_result::R) reader structure"]
impl crate::Readable for CRC_REM_RESULT {}
#[doc = "CRC remainder result"]
pub mod crc_rem_result;
