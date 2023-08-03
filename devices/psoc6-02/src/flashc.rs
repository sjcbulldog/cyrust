#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub flash_ctl: FLASH_CTL,
    #[doc = "0x04 - Flash power control"]
    pub flash_pwr_ctl: FLASH_PWR_CTL,
    #[doc = "0x08 - Command"]
    pub flash_cmd: FLASH_CMD,
    _reserved3: [u8; 660usize],
    #[doc = "0x2a0 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved4: [u8; 12usize],
    #[doc = "0x2b0 - eCT Flash SRAM ECC control 0"]
    pub fm_sram_ecc_ctl0: FM_SRAM_ECC_CTL0,
    #[doc = "0x2b4 - eCT Flash SRAM ECC control 1"]
    pub fm_sram_ecc_ctl1: FM_SRAM_ECC_CTL1,
    #[doc = "0x2b8 - eCT Flash SRAM ECC control 2"]
    pub fm_sram_ecc_ctl2: FM_SRAM_ECC_CTL2,
    #[doc = "0x2bc - eCT Flash SRAM ECC control 3"]
    pub fm_sram_ecc_ctl3: FM_SRAM_ECC_CTL3,
    _reserved8: [u8; 320usize],
    #[doc = "0x400 - CM0+ cache control"]
    pub cm0_ca_ctl0: CM0_CA_CTL0,
    #[doc = "0x404 - CM0+ cache control"]
    pub cm0_ca_ctl1: CM0_CA_CTL1,
    #[doc = "0x408 - CM0+ cache control"]
    pub cm0_ca_ctl2: CM0_CA_CTL2,
    _reserved11: [u8; 52usize],
    #[doc = "0x440 - CM0+ cache status 0"]
    pub cm0_ca_status0: CM0_CA_STATUS0,
    #[doc = "0x444 - CM0+ cache status 1"]
    pub cm0_ca_status1: CM0_CA_STATUS1,
    #[doc = "0x448 - CM0+ cache status 2"]
    pub cm0_ca_status2: CM0_CA_STATUS2,
    _reserved14: [u8; 20usize],
    #[doc = "0x460 - CM0+ interface status"]
    pub cm0_status: CM0_STATUS,
    _reserved15: [u8; 28usize],
    #[doc = "0x480 - CM4 cache control"]
    pub cm4_ca_ctl0: CM4_CA_CTL0,
    #[doc = "0x484 - CM4 cache control"]
    pub cm4_ca_ctl1: CM4_CA_CTL1,
    #[doc = "0x488 - CM4 cache control"]
    pub cm4_ca_ctl2: CM4_CA_CTL2,
    _reserved18: [u8; 52usize],
    #[doc = "0x4c0 - CM4 cache status 0"]
    pub cm4_ca_status0: CM4_CA_STATUS0,
    #[doc = "0x4c4 - CM4 cache status 1"]
    pub cm4_ca_status1: CM4_CA_STATUS1,
    #[doc = "0x4c8 - CM4 cache status 2"]
    pub cm4_ca_status2: CM4_CA_STATUS2,
    _reserved21: [u8; 20usize],
    #[doc = "0x4e0 - CM4 interface status"]
    pub cm4_status: CM4_STATUS,
    _reserved22: [u8; 28usize],
    #[doc = "0x500 - Cryptography buffer control"]
    pub crypto_buff_ctl: CRYPTO_BUFF_CTL,
    _reserved23: [u8; 124usize],
    #[doc = "0x580 - Datawire 0 buffer control"]
    pub dw0_buff_ctl: DW0_BUFF_CTL,
    _reserved24: [u8; 124usize],
    #[doc = "0x600 - Datawire 1 buffer control"]
    pub dw1_buff_ctl: DW1_BUFF_CTL,
    _reserved25: [u8; 124usize],
    #[doc = "0x680 - DMA controller buffer control"]
    pub dmac_buff_ctl: DMAC_BUFF_CTL,
    _reserved26: [u8; 124usize],
    #[doc = "0x700 - External master 0 buffer control"]
    pub ext_ms0_buff_ctl: EXT_MS0_BUFF_CTL,
    _reserved27: [u8; 124usize],
    #[doc = "0x780 - External master 1 buffer control"]
    pub ext_ms1_buff_ctl: EXT_MS1_BUFF_CTL,
    _reserved28: [u8; 59516usize],
    #[doc = "0xf000 - Flash Macro Registers"]
    pub fm_ctl: FM_CTL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct FM_CTL {
    #[doc = "0x00 - Flash macro control"]
    pub fm_ctl: self::fm_ctl::FM_CTL,
    #[doc = "0x04 - Status"]
    pub status: self::fm_ctl::STATUS,
    #[doc = "0x08 - Flash macro address"]
    pub fm_addr: self::fm_ctl::FM_ADDR,
    #[doc = "0x0c - Bookmark register - keeps the current FW HV seq"]
    pub bookmark: self::fm_ctl::BOOKMARK,
    #[doc = "0x10 - Regular flash geometry"]
    pub geometry: self::fm_ctl::GEOMETRY,
    #[doc = "0x14 - Supervisory flash geometry"]
    pub geometry_supervisory: self::fm_ctl::GEOMETRY_SUPERVISORY,
    #[doc = "0x18 - Analog control 0"]
    pub ana_ctl0: self::fm_ctl::ANA_CTL0,
    #[doc = "0x1c - Analog control 1"]
    pub ana_ctl1: self::fm_ctl::ANA_CTL1,
    _reserved8: [u8; 8usize],
    #[doc = "0x28 - Wait State control"]
    pub wait_ctl: self::fm_ctl::WAIT_CTL,
    _reserved9: [u8; 8usize],
    #[doc = "0x34 - Timer prescaler (clk_t to timer clock frequency divider)"]
    pub timer_clk_ctl: self::fm_ctl::TIMER_CLK_CTL,
    #[doc = "0x38 - Timer control"]
    pub timer_ctl: self::fm_ctl::TIMER_CTL,
    #[doc = "0x3c - MPCON clock"]
    pub aclk_ctl: self::fm_ctl::ACLK_CTL,
    #[doc = "0x40 - Interrupt"]
    pub intr: self::fm_ctl::INTR,
    #[doc = "0x44 - Interrupt set"]
    pub intr_set: self::fm_ctl::INTR_SET,
    #[doc = "0x48 - Interrupt mask"]
    pub intr_mask: self::fm_ctl::INTR_MASK,
    #[doc = "0x4c - Interrupt masked"]
    pub intr_masked: self::fm_ctl::INTR_MASKED,
    #[doc = "0x50 - Cal control BG LO trim bits"]
    pub cal_ctl0: self::fm_ctl::CAL_CTL0,
    #[doc = "0x54 - Cal control BG HI trim bits"]
    pub cal_ctl1: self::fm_ctl::CAL_CTL1,
    #[doc = "0x58 - Cal control BG LO&HI trim bits"]
    pub cal_ctl2: self::fm_ctl::CAL_CTL2,
    #[doc = "0x5c - Cal control osc trim bits, idac, sdac, itim"]
    pub cal_ctl3: self::fm_ctl::CAL_CTL3,
    #[doc = "0x60 - Cal Control Vlim, SA, fdiv, reg_act"]
    pub cal_ctl4: self::fm_ctl::CAL_CTL4,
    #[doc = "0x64 - Cal control"]
    pub cal_ctl5: self::fm_ctl::CAL_CTL5,
    #[doc = "0x68 - SA trim LP/ULP"]
    pub cal_ctl6: self::fm_ctl::CAL_CTL6,
    #[doc = "0x6c - Cal control"]
    pub cal_ctl7: self::fm_ctl::CAL_CTL7,
    _reserved24: [u8; 16usize],
    #[doc = "0x80 - Redundancy Control normal sectors 0,1"]
    pub red_ctl01: self::fm_ctl::RED_CTL01,
    #[doc = "0x84 - Redundancy Control normal sectors 2,3"]
    pub red_ctl23: self::fm_ctl::RED_CTL23,
    #[doc = "0x88 - Redundancy Control normal sectors 4,5"]
    pub red_ctl45: self::fm_ctl::RED_CTL45,
    #[doc = "0x8c - Redundancy Control normal sectors 6,7"]
    pub red_ctl67: self::fm_ctl::RED_CTL67,
    #[doc = "0x90 - Redundancy Control special sectors 0,1"]
    pub red_ctl_sm01: self::fm_ctl::RED_CTL_SM01,
    _reserved29: [u8; 4usize],
    #[doc = "0x98 - R-grant delay for program"]
    pub rgrant_delay_prg: self::fm_ctl::RGRANT_DELAY_PRG,
    _reserved30: [u8; 4usize],
    #[doc = "0xa0 - HV Pulse Delay for seq 1&2 pre"]
    pub pw_seq12: self::fm_ctl::PW_SEQ12,
    #[doc = "0xa4 - HV Pulse Delay for seq2 post & seq3"]
    pub pw_seq23: self::fm_ctl::PW_SEQ23,
    #[doc = "0xa8 - R-grant delay scale for erase"]
    pub rgrant_scale_ers: self::fm_ctl::RGRANT_SCALE_ERS,
    #[doc = "0xac - R-grant delay for erase"]
    pub rgrant_delay_ers: self::fm_ctl::RGRANT_DELAY_ERS,
    _reserved34: [u8; 1868usize],
    #[doc = "0x7fc - Flash macro write page latches all"]
    pub fm_pl_wrdata_all: self::fm_ctl::FM_PL_WRDATA_ALL,
    #[doc = "0x800 - Flash macro Page Latches data"]
    pub fm_pl_data: [self::fm_ctl::FM_PL_DATA; 256],
    #[doc = "0xc00 - Flash macro memory sense amplifier and column decoder data"]
    pub fm_mem_data: [self::fm_ctl::FM_MEM_DATA; 256],
}
#[doc = r"Register block"]
#[doc = "Flash Macro Registers"]
pub mod fm_ctl;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ctl](flash_ctl) module"]
pub type FLASH_CTL = crate::Reg<u32, _FLASH_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_CTL;
#[doc = "`read()` method returns [flash_ctl::R](flash_ctl::R) reader structure"]
impl crate::Readable for FLASH_CTL {}
#[doc = "`write(|w| ..)` method takes [flash_ctl::W](flash_ctl::W) writer structure"]
impl crate::Writable for FLASH_CTL {}
#[doc = "Control"]
pub mod flash_ctl;
#[doc = "Flash power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_pwr_ctl](flash_pwr_ctl) module"]
pub type FLASH_PWR_CTL = crate::Reg<u32, _FLASH_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_PWR_CTL;
#[doc = "`read()` method returns [flash_pwr_ctl::R](flash_pwr_ctl::R) reader structure"]
impl crate::Readable for FLASH_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [flash_pwr_ctl::W](flash_pwr_ctl::W) writer structure"]
impl crate::Writable for FLASH_PWR_CTL {}
#[doc = "Flash power control"]
pub mod flash_pwr_ctl;
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_cmd](flash_cmd) module"]
pub type FLASH_CMD = crate::Reg<u32, _FLASH_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_CMD;
#[doc = "`read()` method returns [flash_cmd::R](flash_cmd::R) reader structure"]
impl crate::Readable for FLASH_CMD {}
#[doc = "`write(|w| ..)` method takes [flash_cmd::W](flash_cmd::W) writer structure"]
impl crate::Writable for FLASH_CMD {}
#[doc = "Command"]
pub mod flash_cmd;
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
#[doc = "eCT Flash SRAM ECC control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_sram_ecc_ctl0](fm_sram_ecc_ctl0) module"]
pub type FM_SRAM_ECC_CTL0 = crate::Reg<u32, _FM_SRAM_ECC_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_SRAM_ECC_CTL0;
#[doc = "`read()` method returns [fm_sram_ecc_ctl0::R](fm_sram_ecc_ctl0::R) reader structure"]
impl crate::Readable for FM_SRAM_ECC_CTL0 {}
#[doc = "`write(|w| ..)` method takes [fm_sram_ecc_ctl0::W](fm_sram_ecc_ctl0::W) writer structure"]
impl crate::Writable for FM_SRAM_ECC_CTL0 {}
#[doc = "eCT Flash SRAM ECC control 0"]
pub mod fm_sram_ecc_ctl0;
#[doc = "eCT Flash SRAM ECC control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_sram_ecc_ctl1](fm_sram_ecc_ctl1) module"]
pub type FM_SRAM_ECC_CTL1 = crate::Reg<u32, _FM_SRAM_ECC_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_SRAM_ECC_CTL1;
#[doc = "`read()` method returns [fm_sram_ecc_ctl1::R](fm_sram_ecc_ctl1::R) reader structure"]
impl crate::Readable for FM_SRAM_ECC_CTL1 {}
#[doc = "`write(|w| ..)` method takes [fm_sram_ecc_ctl1::W](fm_sram_ecc_ctl1::W) writer structure"]
impl crate::Writable for FM_SRAM_ECC_CTL1 {}
#[doc = "eCT Flash SRAM ECC control 1"]
pub mod fm_sram_ecc_ctl1;
#[doc = "eCT Flash SRAM ECC control 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_sram_ecc_ctl2](fm_sram_ecc_ctl2) module"]
pub type FM_SRAM_ECC_CTL2 = crate::Reg<u32, _FM_SRAM_ECC_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_SRAM_ECC_CTL2;
#[doc = "`read()` method returns [fm_sram_ecc_ctl2::R](fm_sram_ecc_ctl2::R) reader structure"]
impl crate::Readable for FM_SRAM_ECC_CTL2 {}
#[doc = "eCT Flash SRAM ECC control 2"]
pub mod fm_sram_ecc_ctl2;
#[doc = "eCT Flash SRAM ECC control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_sram_ecc_ctl3](fm_sram_ecc_ctl3) module"]
pub type FM_SRAM_ECC_CTL3 = crate::Reg<u32, _FM_SRAM_ECC_CTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_SRAM_ECC_CTL3;
#[doc = "`read()` method returns [fm_sram_ecc_ctl3::R](fm_sram_ecc_ctl3::R) reader structure"]
impl crate::Readable for FM_SRAM_ECC_CTL3 {}
#[doc = "`write(|w| ..)` method takes [fm_sram_ecc_ctl3::W](fm_sram_ecc_ctl3::W) writer structure"]
impl crate::Writable for FM_SRAM_ECC_CTL3 {}
#[doc = "eCT Flash SRAM ECC control 3"]
pub mod fm_sram_ecc_ctl3;
#[doc = "CM0+ cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_ctl0](cm0_ca_ctl0) module"]
pub type CM0_CA_CTL0 = crate::Reg<u32, _CM0_CA_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_CTL0;
#[doc = "`read()` method returns [cm0_ca_ctl0::R](cm0_ca_ctl0::R) reader structure"]
impl crate::Readable for CM0_CA_CTL0 {}
#[doc = "`write(|w| ..)` method takes [cm0_ca_ctl0::W](cm0_ca_ctl0::W) writer structure"]
impl crate::Writable for CM0_CA_CTL0 {}
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl0;
#[doc = "CM0+ cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_ctl1](cm0_ca_ctl1) module"]
pub type CM0_CA_CTL1 = crate::Reg<u32, _CM0_CA_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_CTL1;
#[doc = "`read()` method returns [cm0_ca_ctl1::R](cm0_ca_ctl1::R) reader structure"]
impl crate::Readable for CM0_CA_CTL1 {}
#[doc = "`write(|w| ..)` method takes [cm0_ca_ctl1::W](cm0_ca_ctl1::W) writer structure"]
impl crate::Writable for CM0_CA_CTL1 {}
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl1;
#[doc = "CM0+ cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_ctl2](cm0_ca_ctl2) module"]
pub type CM0_CA_CTL2 = crate::Reg<u32, _CM0_CA_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_CTL2;
#[doc = "`read()` method returns [cm0_ca_ctl2::R](cm0_ca_ctl2::R) reader structure"]
impl crate::Readable for CM0_CA_CTL2 {}
#[doc = "`write(|w| ..)` method takes [cm0_ca_ctl2::W](cm0_ca_ctl2::W) writer structure"]
impl crate::Writable for CM0_CA_CTL2 {}
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl2;
#[doc = "CM0+ cache status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_status0](cm0_ca_status0) module"]
pub type CM0_CA_STATUS0 = crate::Reg<u32, _CM0_CA_STATUS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_STATUS0;
#[doc = "`read()` method returns [cm0_ca_status0::R](cm0_ca_status0::R) reader structure"]
impl crate::Readable for CM0_CA_STATUS0 {}
#[doc = "CM0+ cache status 0"]
pub mod cm0_ca_status0;
#[doc = "CM0+ cache status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_status1](cm0_ca_status1) module"]
pub type CM0_CA_STATUS1 = crate::Reg<u32, _CM0_CA_STATUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_STATUS1;
#[doc = "`read()` method returns [cm0_ca_status1::R](cm0_ca_status1::R) reader structure"]
impl crate::Readable for CM0_CA_STATUS1 {}
#[doc = "CM0+ cache status 1"]
pub mod cm0_ca_status1;
#[doc = "CM0+ cache status 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_status2](cm0_ca_status2) module"]
pub type CM0_CA_STATUS2 = crate::Reg<u32, _CM0_CA_STATUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CA_STATUS2;
#[doc = "`read()` method returns [cm0_ca_status2::R](cm0_ca_status2::R) reader structure"]
impl crate::Readable for CM0_CA_STATUS2 {}
#[doc = "CM0+ cache status 2"]
pub mod cm0_ca_status2;
#[doc = "CM0+ interface status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_status](cm0_status) module"]
pub type CM0_STATUS = crate::Reg<u32, _CM0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_STATUS;
#[doc = "`read()` method returns [cm0_status::R](cm0_status::R) reader structure"]
impl crate::Readable for CM0_STATUS {}
#[doc = "`write(|w| ..)` method takes [cm0_status::W](cm0_status::W) writer structure"]
impl crate::Writable for CM0_STATUS {}
#[doc = "CM0+ interface status"]
pub mod cm0_status;
#[doc = "CM4 cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_ctl0](cm4_ca_ctl0) module"]
pub type CM4_CA_CTL0 = crate::Reg<u32, _CM4_CA_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_CTL0;
#[doc = "`read()` method returns [cm4_ca_ctl0::R](cm4_ca_ctl0::R) reader structure"]
impl crate::Readable for CM4_CA_CTL0 {}
#[doc = "`write(|w| ..)` method takes [cm4_ca_ctl0::W](cm4_ca_ctl0::W) writer structure"]
impl crate::Writable for CM4_CA_CTL0 {}
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl0;
#[doc = "CM4 cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_ctl1](cm4_ca_ctl1) module"]
pub type CM4_CA_CTL1 = crate::Reg<u32, _CM4_CA_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_CTL1;
#[doc = "`read()` method returns [cm4_ca_ctl1::R](cm4_ca_ctl1::R) reader structure"]
impl crate::Readable for CM4_CA_CTL1 {}
#[doc = "`write(|w| ..)` method takes [cm4_ca_ctl1::W](cm4_ca_ctl1::W) writer structure"]
impl crate::Writable for CM4_CA_CTL1 {}
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl1;
#[doc = "CM4 cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_ctl2](cm4_ca_ctl2) module"]
pub type CM4_CA_CTL2 = crate::Reg<u32, _CM4_CA_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_CTL2;
#[doc = "`read()` method returns [cm4_ca_ctl2::R](cm4_ca_ctl2::R) reader structure"]
impl crate::Readable for CM4_CA_CTL2 {}
#[doc = "`write(|w| ..)` method takes [cm4_ca_ctl2::W](cm4_ca_ctl2::W) writer structure"]
impl crate::Writable for CM4_CA_CTL2 {}
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl2;
#[doc = "CM4 cache status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status0](cm4_ca_status0) module"]
pub type CM4_CA_STATUS0 = crate::Reg<u32, _CM4_CA_STATUS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_STATUS0;
#[doc = "`read()` method returns [cm4_ca_status0::R](cm4_ca_status0::R) reader structure"]
impl crate::Readable for CM4_CA_STATUS0 {}
#[doc = "CM4 cache status 0"]
pub mod cm4_ca_status0;
#[doc = "CM4 cache status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status1](cm4_ca_status1) module"]
pub type CM4_CA_STATUS1 = crate::Reg<u32, _CM4_CA_STATUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_STATUS1;
#[doc = "`read()` method returns [cm4_ca_status1::R](cm4_ca_status1::R) reader structure"]
impl crate::Readable for CM4_CA_STATUS1 {}
#[doc = "CM4 cache status 1"]
pub mod cm4_ca_status1;
#[doc = "CM4 cache status 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status2](cm4_ca_status2) module"]
pub type CM4_CA_STATUS2 = crate::Reg<u32, _CM4_CA_STATUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CA_STATUS2;
#[doc = "`read()` method returns [cm4_ca_status2::R](cm4_ca_status2::R) reader structure"]
impl crate::Readable for CM4_CA_STATUS2 {}
#[doc = "CM4 cache status 2"]
pub mod cm4_ca_status2;
#[doc = "CM4 interface status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_status](cm4_status) module"]
pub type CM4_STATUS = crate::Reg<u32, _CM4_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_STATUS;
#[doc = "`read()` method returns [cm4_status::R](cm4_status::R) reader structure"]
impl crate::Readable for CM4_STATUS {}
#[doc = "`write(|w| ..)` method takes [cm4_status::W](cm4_status::W) writer structure"]
impl crate::Writable for CM4_STATUS {}
#[doc = "CM4 interface status"]
pub mod cm4_status;
#[doc = "Cryptography buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_buff_ctl](crypto_buff_ctl) module"]
pub type CRYPTO_BUFF_CTL = crate::Reg<u32, _CRYPTO_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_BUFF_CTL;
#[doc = "`read()` method returns [crypto_buff_ctl::R](crypto_buff_ctl::R) reader structure"]
impl crate::Readable for CRYPTO_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [crypto_buff_ctl::W](crypto_buff_ctl::W) writer structure"]
impl crate::Writable for CRYPTO_BUFF_CTL {}
#[doc = "Cryptography buffer control"]
pub mod crypto_buff_ctl;
#[doc = "Datawire 0 buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dw0_buff_ctl](dw0_buff_ctl) module"]
pub type DW0_BUFF_CTL = crate::Reg<u32, _DW0_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DW0_BUFF_CTL;
#[doc = "`read()` method returns [dw0_buff_ctl::R](dw0_buff_ctl::R) reader structure"]
impl crate::Readable for DW0_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [dw0_buff_ctl::W](dw0_buff_ctl::W) writer structure"]
impl crate::Writable for DW0_BUFF_CTL {}
#[doc = "Datawire 0 buffer control"]
pub mod dw0_buff_ctl;
#[doc = "Datawire 1 buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dw1_buff_ctl](dw1_buff_ctl) module"]
pub type DW1_BUFF_CTL = crate::Reg<u32, _DW1_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DW1_BUFF_CTL;
#[doc = "`read()` method returns [dw1_buff_ctl::R](dw1_buff_ctl::R) reader structure"]
impl crate::Readable for DW1_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [dw1_buff_ctl::W](dw1_buff_ctl::W) writer structure"]
impl crate::Writable for DW1_BUFF_CTL {}
#[doc = "Datawire 1 buffer control"]
pub mod dw1_buff_ctl;
#[doc = "DMA controller buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_buff_ctl](dmac_buff_ctl) module"]
pub type DMAC_BUFF_CTL = crate::Reg<u32, _DMAC_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAC_BUFF_CTL;
#[doc = "`read()` method returns [dmac_buff_ctl::R](dmac_buff_ctl::R) reader structure"]
impl crate::Readable for DMAC_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [dmac_buff_ctl::W](dmac_buff_ctl::W) writer structure"]
impl crate::Writable for DMAC_BUFF_CTL {}
#[doc = "DMA controller buffer control"]
pub mod dmac_buff_ctl;
#[doc = "External master 0 buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_ms0_buff_ctl](ext_ms0_buff_ctl) module"]
pub type EXT_MS0_BUFF_CTL = crate::Reg<u32, _EXT_MS0_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_MS0_BUFF_CTL;
#[doc = "`read()` method returns [ext_ms0_buff_ctl::R](ext_ms0_buff_ctl::R) reader structure"]
impl crate::Readable for EXT_MS0_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [ext_ms0_buff_ctl::W](ext_ms0_buff_ctl::W) writer structure"]
impl crate::Writable for EXT_MS0_BUFF_CTL {}
#[doc = "External master 0 buffer control"]
pub mod ext_ms0_buff_ctl;
#[doc = "External master 1 buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_ms1_buff_ctl](ext_ms1_buff_ctl) module"]
pub type EXT_MS1_BUFF_CTL = crate::Reg<u32, _EXT_MS1_BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_MS1_BUFF_CTL;
#[doc = "`read()` method returns [ext_ms1_buff_ctl::R](ext_ms1_buff_ctl::R) reader structure"]
impl crate::Readable for EXT_MS1_BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [ext_ms1_buff_ctl::W](ext_ms1_buff_ctl::W) writer structure"]
impl crate::Writable for EXT_MS1_BUFF_CTL {}
#[doc = "External master 1 buffer control"]
pub mod ext_ms1_buff_ctl;
