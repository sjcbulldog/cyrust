#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 512usize],
    #[doc = "0x200 - Timeout control"]
    pub timeout_ctl: TIMEOUT_CTL,
    _reserved1: [u8; 28usize],
    #[doc = "0x220 - Trigger command"]
    pub tr_cmd: TR_CMD,
    _reserved2: [u8; 476usize],
    #[doc = "0x400 - Divider command"]
    pub div_cmd: DIV_CMD,
    _reserved3: [u8; 2044usize],
    #[doc = "0xc00 - Clock control"]
    pub clock_ctl: [CLOCK_CTL; 256],
    #[doc = "0x1000 - Divider control (for 8.0 divider)"]
    pub div_8_ctl: [DIV_8_CTL; 256],
    #[doc = "0x1400 - Divider control (for 16.0 divider)"]
    pub div_16_ctl: [DIV_16_CTL; 256],
    #[doc = "0x1800 - Divider control (for 16.5 divider)"]
    pub div_16_5_ctl: [DIV_16_5_CTL; 256],
    #[doc = "0x1c00 - Divider control (for 24.5 divider)"]
    pub div_24_5_ctl: [DIV_24_5_CTL; 255],
    _reserved8: [u8; 4usize],
    #[doc = "0x2000 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved9: [u8; 8188usize],
    #[doc = "0x4000 - Peripheral group structure"]
    pub gr0: GR,
    _reserved10: [u8; 12usize],
    #[doc = "0x4020 - Peripheral group structure"]
    pub gr1: GR,
    _reserved11: [u8; 12usize],
    #[doc = "0x4040 - Peripheral group structure"]
    pub gr2: GR,
    _reserved12: [u8; 12usize],
    #[doc = "0x4060 - Peripheral group structure"]
    pub gr3: GR,
    _reserved13: [u8; 12usize],
    #[doc = "0x4080 - Peripheral group structure"]
    pub gr4: GR,
    _reserved14: [u8; 12usize],
    #[doc = "0x40a0 - Peripheral group structure"]
    pub gr5: GR,
    _reserved15: [u8; 12usize],
    #[doc = "0x40c0 - Peripheral group structure"]
    pub gr6: GR,
    _reserved16: [u8; 12usize],
    #[doc = "0x40e0 - Peripheral group structure"]
    pub gr7: GR,
    _reserved17: [u8; 12usize],
    #[doc = "0x4100 - Peripheral group structure"]
    pub gr8: GR,
    _reserved18: [u8; 12usize],
    #[doc = "0x4120 - Peripheral group structure"]
    pub gr9: GR,
    _reserved19: [u8; 16076usize],
    #[doc = "0x8000 - Trigger group"]
    pub tr_gr: [TR_GR; 12],
    _reserved20: [u8; 4096usize],
    #[doc = "0xc000 - Trigger 1-to-1 group"]
    pub tr_1to1_gr: [TR_1TO1_GR; 9],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GR {
    #[doc = "0x00 - Clock control"]
    pub clock_ctl: self::gr::CLOCK_CTL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Slave control"]
    pub sl_ctl: self::gr::SL_CTL,
}
#[doc = r"Register block"]
#[doc = "Peripheral group structure"]
pub mod gr;
#[doc = r"Register block"]
#[repr(C)]
pub struct TR_GR {
    #[doc = "0x00 - Trigger control register"]
    pub tr_ctl: [self::tr_gr::TR_CTL; 256],
}
#[doc = r"Register block"]
#[doc = "Trigger group"]
pub mod tr_gr;
#[doc = r"Register block"]
#[repr(C)]
pub struct TR_1TO1_GR {
    #[doc = "0x00 - Trigger control register"]
    pub tr_ctl: [self::tr_1to1_gr::TR_CTL; 256],
}
#[doc = r"Register block"]
#[doc = "Trigger 1-to-1 group"]
pub mod tr_1to1_gr;
#[doc = "Timeout control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout_ctl](timeout_ctl) module"]
pub type TIMEOUT_CTL = crate::Reg<u32, _TIMEOUT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUT_CTL;
#[doc = "`read()` method returns [timeout_ctl::R](timeout_ctl::R) reader structure"]
impl crate::Readable for TIMEOUT_CTL {}
#[doc = "`write(|w| ..)` method takes [timeout_ctl::W](timeout_ctl::W) writer structure"]
impl crate::Writable for TIMEOUT_CTL {}
#[doc = "Timeout control"]
pub mod timeout_ctl;
#[doc = "Trigger command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_cmd](tr_cmd) module"]
pub type TR_CMD = crate::Reg<u32, _TR_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_CMD;
#[doc = "`read()` method returns [tr_cmd::R](tr_cmd::R) reader structure"]
impl crate::Readable for TR_CMD {}
#[doc = "`write(|w| ..)` method takes [tr_cmd::W](tr_cmd::W) writer structure"]
impl crate::Writable for TR_CMD {}
#[doc = "Trigger command"]
pub mod tr_cmd;
#[doc = "Divider command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_cmd](div_cmd) module"]
pub type DIV_CMD = crate::Reg<u32, _DIV_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_CMD;
#[doc = "`read()` method returns [div_cmd::R](div_cmd::R) reader structure"]
impl crate::Readable for DIV_CMD {}
#[doc = "`write(|w| ..)` method takes [div_cmd::W](div_cmd::W) writer structure"]
impl crate::Writable for DIV_CMD {}
#[doc = "Divider command"]
pub mod div_cmd;
#[doc = "Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctl](clock_ctl) module"]
pub type CLOCK_CTL = crate::Reg<u32, _CLOCK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_CTL;
#[doc = "`read()` method returns [clock_ctl::R](clock_ctl::R) reader structure"]
impl crate::Readable for CLOCK_CTL {}
#[doc = "`write(|w| ..)` method takes [clock_ctl::W](clock_ctl::W) writer structure"]
impl crate::Writable for CLOCK_CTL {}
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "Divider control (for 8.0 divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_8_ctl](div_8_ctl) module"]
pub type DIV_8_CTL = crate::Reg<u32, _DIV_8_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_8_CTL;
#[doc = "`read()` method returns [div_8_ctl::R](div_8_ctl::R) reader structure"]
impl crate::Readable for DIV_8_CTL {}
#[doc = "`write(|w| ..)` method takes [div_8_ctl::W](div_8_ctl::W) writer structure"]
impl crate::Writable for DIV_8_CTL {}
#[doc = "Divider control (for 8.0 divider)"]
pub mod div_8_ctl;
#[doc = "Divider control (for 16.0 divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_16_ctl](div_16_ctl) module"]
pub type DIV_16_CTL = crate::Reg<u32, _DIV_16_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_16_CTL;
#[doc = "`read()` method returns [div_16_ctl::R](div_16_ctl::R) reader structure"]
impl crate::Readable for DIV_16_CTL {}
#[doc = "`write(|w| ..)` method takes [div_16_ctl::W](div_16_ctl::W) writer structure"]
impl crate::Writable for DIV_16_CTL {}
#[doc = "Divider control (for 16.0 divider)"]
pub mod div_16_ctl;
#[doc = "Divider control (for 16.5 divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_16_5_ctl](div_16_5_ctl) module"]
pub type DIV_16_5_CTL = crate::Reg<u32, _DIV_16_5_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_16_5_CTL;
#[doc = "`read()` method returns [div_16_5_ctl::R](div_16_5_ctl::R) reader structure"]
impl crate::Readable for DIV_16_5_CTL {}
#[doc = "`write(|w| ..)` method takes [div_16_5_ctl::W](div_16_5_ctl::W) writer structure"]
impl crate::Writable for DIV_16_5_CTL {}
#[doc = "Divider control (for 16.5 divider)"]
pub mod div_16_5_ctl;
#[doc = "Divider control (for 24.5 divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_24_5_ctl](div_24_5_ctl) module"]
pub type DIV_24_5_CTL = crate::Reg<u32, _DIV_24_5_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_24_5_CTL;
#[doc = "`read()` method returns [div_24_5_ctl::R](div_24_5_ctl::R) reader structure"]
impl crate::Readable for DIV_24_5_CTL {}
#[doc = "`write(|w| ..)` method takes [div_24_5_ctl::W](div_24_5_ctl::W) writer structure"]
impl crate::Writable for DIV_24_5_CTL {}
#[doc = "Divider control (for 24.5 divider)"]
pub mod div_24_5_ctl;
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
