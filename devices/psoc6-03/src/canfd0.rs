use ch::M_TTCAN ; 

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FIFO wrapper around M_TTCAN 3PIP, to enable DMA"]
    pub ch: CH,
    _reserved1: [u8; 3652usize],
    #[doc = "0x1000 - Global CAN control register"]
    pub ctl: CTL,
    #[doc = "0x1004 - Global CAN status register"]
    pub status: STATUS,
    _reserved3: [u8; 8usize],
    #[doc = "0x1010 - Consolidated interrupt0 cause register"]
    pub intr0_cause: INTR0_CAUSE,
    #[doc = "0x1014 - Consolidated interrupt1 cause register"]
    pub intr1_cause: INTR1_CAUSE,
    _reserved5: [u8; 8usize],
    #[doc = "0x1020 - Time Stamp control register"]
    pub ts_ctl: TS_CTL,
    #[doc = "0x1024 - Time Stamp counter value"]
    pub ts_cnt: TS_CNT,
    _reserved7: [u8; 88usize],
    #[doc = "0x1080 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    #[doc = "0x1084 - ECC error injection"]
    pub ecc_err_inj: ECC_ERR_INJ,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - TTCAN 3PIP, includes FD"]
    pub m_ttcan: M_TTCAN,
    _reserved1: [u8; 60usize],
    #[doc = "0x180 - Receive FIFO Top control"]
    pub rxftop_ctl: self::ch::RXFTOP_CTL,
    _reserved2: [u8; 28usize],
    #[doc = "0x1a0 - Receive FIFO 0 Top Status"]
    pub rxftop0_stat: self::ch::RXFTOP0_STAT,
    _reserved3: [u8; 4usize],
    #[doc = "0x1a8 - Receive FIFO 0 Top Data"]
    pub rxftop0_data: self::ch::RXFTOP0_DATA,
    _reserved4: [u8; 4usize],
    #[doc = "0x1b0 - Receive FIFO 1 Top Status"]
    pub rxftop1_stat: self::ch::RXFTOP1_STAT,
    _reserved5: [u8; 4usize],
    #[doc = "0x1b8 - Receive FIFO 1 Top Data"]
    pub rxftop1_data: self::ch::RXFTOP1_DATA,
}
#[doc = r"Register block"]
#[doc = "FIFO wrapper around M_TTCAN 3PIP, to enable DMA"]
pub mod ch;
#[doc = "Global CAN control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Global CAN control register"]
pub mod ctl;
#[doc = "Global CAN status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Global CAN status register"]
pub mod status;
#[doc = "Consolidated interrupt0 cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr0_cause](intr0_cause) module"]
pub type INTR0_CAUSE = crate::Reg<u32, _INTR0_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR0_CAUSE;
#[doc = "`read()` method returns [intr0_cause::R](intr0_cause::R) reader structure"]
impl crate::Readable for INTR0_CAUSE {}
#[doc = "Consolidated interrupt0 cause register"]
pub mod intr0_cause;
#[doc = "Consolidated interrupt1 cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr1_cause](intr1_cause) module"]
pub type INTR1_CAUSE = crate::Reg<u32, _INTR1_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR1_CAUSE;
#[doc = "`read()` method returns [intr1_cause::R](intr1_cause::R) reader structure"]
impl crate::Readable for INTR1_CAUSE {}
#[doc = "Consolidated interrupt1 cause register"]
pub mod intr1_cause;
#[doc = "Time Stamp control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts_ctl](ts_ctl) module"]
pub type TS_CTL = crate::Reg<u32, _TS_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TS_CTL;
#[doc = "`read()` method returns [ts_ctl::R](ts_ctl::R) reader structure"]
impl crate::Readable for TS_CTL {}
#[doc = "`write(|w| ..)` method takes [ts_ctl::W](ts_ctl::W) writer structure"]
impl crate::Writable for TS_CTL {}
#[doc = "Time Stamp control register"]
pub mod ts_ctl;
#[doc = "Time Stamp counter value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts_cnt](ts_cnt) module"]
pub type TS_CNT = crate::Reg<u32, _TS_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TS_CNT;
#[doc = "`read()` method returns [ts_cnt::R](ts_cnt::R) reader structure"]
impl crate::Readable for TS_CNT {}
#[doc = "`write(|w| ..)` method takes [ts_cnt::W](ts_cnt::W) writer structure"]
impl crate::Writable for TS_CNT {}
#[doc = "Time Stamp counter value"]
pub mod ts_cnt;
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
#[doc = "ECC error injection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_err_inj](ecc_err_inj) module"]
pub type ECC_ERR_INJ = crate::Reg<u32, _ECC_ERR_INJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECC_ERR_INJ;
#[doc = "`read()` method returns [ecc_err_inj::R](ecc_err_inj::R) reader structure"]
impl crate::Readable for ECC_ERR_INJ {}
#[doc = "`write(|w| ..)` method takes [ecc_err_inj::W](ecc_err_inj::W) writer structure"]
impl crate::Writable for ECC_ERR_INJ {}
#[doc = "ECC error injection"]
pub mod ecc_err_inj;
