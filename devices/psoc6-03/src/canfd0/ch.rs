#[doc = "Receive FIFO Top control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop_ctl](rxftop_ctl) module"]
pub type RXFTOP_CTL = crate::Reg<u32, _RXFTOP_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFTOP_CTL;
#[doc = "`read()` method returns [rxftop_ctl::R](rxftop_ctl::R) reader structure"]
impl crate::Readable for RXFTOP_CTL {}
#[doc = "`write(|w| ..)` method takes [rxftop_ctl::W](rxftop_ctl::W) writer structure"]
impl crate::Writable for RXFTOP_CTL {}
#[doc = "Receive FIFO Top control"]
pub mod rxftop_ctl;
#[doc = "Receive FIFO 0 Top Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop0_stat](rxftop0_stat) module"]
pub type RXFTOP0_STAT = crate::Reg<u32, _RXFTOP0_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFTOP0_STAT;
#[doc = "`read()` method returns [rxftop0_stat::R](rxftop0_stat::R) reader structure"]
impl crate::Readable for RXFTOP0_STAT {}
#[doc = "Receive FIFO 0 Top Status"]
pub mod rxftop0_stat;
#[doc = "Receive FIFO 0 Top Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop0_data](rxftop0_data) module"]
pub type RXFTOP0_DATA = crate::Reg<u32, _RXFTOP0_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFTOP0_DATA;
#[doc = "`read()` method returns [rxftop0_data::R](rxftop0_data::R) reader structure"]
impl crate::Readable for RXFTOP0_DATA {}
#[doc = "Receive FIFO 0 Top Data"]
pub mod rxftop0_data;
#[doc = "Receive FIFO 1 Top Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop1_stat](rxftop1_stat) module"]
pub type RXFTOP1_STAT = crate::Reg<u32, _RXFTOP1_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFTOP1_STAT;
#[doc = "`read()` method returns [rxftop1_stat::R](rxftop1_stat::R) reader structure"]
impl crate::Readable for RXFTOP1_STAT {}
#[doc = "Receive FIFO 1 Top Status"]
pub mod rxftop1_stat;
#[doc = "Receive FIFO 1 Top Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop1_data](rxftop1_data) module"]
pub type RXFTOP1_DATA = crate::Reg<u32, _RXFTOP1_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFTOP1_DATA;
#[doc = "`read()` method returns [rxftop1_data::R](rxftop1_data::R) reader structure"]
impl crate::Readable for RXFTOP1_DATA {}
#[doc = "Receive FIFO 1 Top Data"]
pub mod rxftop1_data;
#[doc = r"Register block"]
#[repr(C)]
pub struct M_TTCAN {
    #[doc = "0x00 - Core Release Register"]
    pub crel: self::m_ttcan::CREL,
    #[doc = "0x04 - Endian Register"]
    pub endn: self::m_ttcan::ENDN,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Data Bit Timing & Prescaler Register"]
    pub dbtp: self::m_ttcan::DBTP,
    #[doc = "0x10 - Test Register"]
    pub test: self::m_ttcan::TEST,
    #[doc = "0x14 - RAM Watchdog"]
    pub rwd: self::m_ttcan::RWD,
    #[doc = "0x18 - CC Control Register"]
    pub cccr: self::m_ttcan::CCCR,
    #[doc = "0x1c - Nominal Bit Timing & Prescaler Register"]
    pub nbtp: self::m_ttcan::NBTP,
    #[doc = "0x20 - Timestamp Counter Configuration"]
    pub tscc: self::m_ttcan::TSCC,
    #[doc = "0x24 - Timestamp Counter Value"]
    pub tscv: self::m_ttcan::TSCV,
    #[doc = "0x28 - Timeout Counter Configuration"]
    pub tocc: self::m_ttcan::TOCC,
    #[doc = "0x2c - Timeout Counter Value"]
    pub tocv: self::m_ttcan::TOCV,
    _reserved11: [u8; 16usize],
    #[doc = "0x40 - Error Counter Register"]
    pub ecr: self::m_ttcan::ECR,
    #[doc = "0x44 - Protocol Status Register"]
    pub psr: self::m_ttcan::PSR,
    #[doc = "0x48 - Transmitter Delay Compensation Register"]
    pub tdcr: self::m_ttcan::TDCR,
    _reserved14: [u8; 4usize],
    #[doc = "0x50 - Interrupt Register"]
    pub ir: self::m_ttcan::IR,
    #[doc = "0x54 - Interrupt Enable"]
    pub ie: self::m_ttcan::IE,
    #[doc = "0x58 - Interrupt Line Select"]
    pub ils: self::m_ttcan::ILS,
    #[doc = "0x5c - Interrupt Line Enable"]
    pub ile: self::m_ttcan::ILE,
    _reserved18: [u8; 32usize],
    #[doc = "0x80 - Global Filter Configuration"]
    pub gfc: self::m_ttcan::GFC,
    #[doc = "0x84 - Standard ID Filter Configuration"]
    pub sidfc: self::m_ttcan::SIDFC,
    #[doc = "0x88 - Extended ID Filter Configuration"]
    pub xidfc: self::m_ttcan::XIDFC,
    _reserved21: [u8; 4usize],
    #[doc = "0x90 - Extended ID AND Mask"]
    pub xidam: self::m_ttcan::XIDAM,
    #[doc = "0x94 - High Priority Message Status"]
    pub hpms: self::m_ttcan::HPMS,
    #[doc = "0x98 - New Data 1"]
    pub ndat1: self::m_ttcan::NDAT1,
    #[doc = "0x9c - New Data 2"]
    pub ndat2: self::m_ttcan::NDAT2,
    #[doc = "0xa0 - Rx FIFO 0 Configuration"]
    pub rxf0c: self::m_ttcan::RXF0C,
    #[doc = "0xa4 - Rx FIFO 0 Status"]
    pub rxf0s: self::m_ttcan::RXF0S,
    #[doc = "0xa8 - Rx FIFO 0 Acknowledge"]
    pub rxf0a: self::m_ttcan::RXF0A,
    #[doc = "0xac - Rx Buffer Configuration"]
    pub rxbc: self::m_ttcan::RXBC,
    #[doc = "0xb0 - Rx FIFO 1 Configuration"]
    pub rxf1c: self::m_ttcan::RXF1C,
    #[doc = "0xb4 - Rx FIFO 1 Status"]
    pub rxf1s: self::m_ttcan::RXF1S,
    #[doc = "0xb8 - Rx FIFO 1 Acknowledge"]
    pub rxf1a: self::m_ttcan::RXF1A,
    #[doc = "0xbc - Rx Buffer / FIFO Element Size Configuration"]
    pub rxesc: self::m_ttcan::RXESC,
    #[doc = "0xc0 - Tx Buffer Configuration"]
    pub txbc: self::m_ttcan::TXBC,
    #[doc = "0xc4 - Tx FIFO/Queue Status"]
    pub txfqs: self::m_ttcan::TXFQS,
    #[doc = "0xc8 - Tx Buffer Element Size Configuration"]
    pub txesc: self::m_ttcan::TXESC,
    #[doc = "0xcc - Tx Buffer Request Pending"]
    pub txbrp: self::m_ttcan::TXBRP,
    #[doc = "0xd0 - Tx Buffer Add Request"]
    pub txbar: self::m_ttcan::TXBAR,
    #[doc = "0xd4 - Tx Buffer Cancellation Request"]
    pub txbcr: self::m_ttcan::TXBCR,
    #[doc = "0xd8 - Tx Buffer Transmission Occurred"]
    pub txbto: self::m_ttcan::TXBTO,
    #[doc = "0xdc - Tx Buffer Cancellation Finished"]
    pub txbcf: self::m_ttcan::TXBCF,
    #[doc = "0xe0 - Tx Buffer Transmission Interrupt Enable"]
    pub txbtie: self::m_ttcan::TXBTIE,
    #[doc = "0xe4 - Tx Buffer Cancellation Finished Interrupt Enable"]
    pub txbcie: self::m_ttcan::TXBCIE,
    _reserved43: [u8; 8usize],
    #[doc = "0xf0 - Tx Event FIFO Configuration"]
    pub txefc: self::m_ttcan::TXEFC,
    #[doc = "0xf4 - Tx Event FIFO Status"]
    pub txefs: self::m_ttcan::TXEFS,
    #[doc = "0xf8 - Tx Event FIFO Acknowledge"]
    pub txefa: self::m_ttcan::TXEFA,
    _reserved46: [u8; 4usize],
    #[doc = "0x100 - TT Trigger Memory Configuration"]
    pub tttmc: self::m_ttcan::TTTMC,
    #[doc = "0x104 - TT Reference Message Configuration"]
    pub ttrmc: self::m_ttcan::TTRMC,
    #[doc = "0x108 - TT Operation Configuration"]
    pub ttocf: self::m_ttcan::TTOCF,
    #[doc = "0x10c - TT Matrix Limits"]
    pub ttmlm: self::m_ttcan::TTMLM,
    #[doc = "0x110 - TUR Configuration"]
    pub turcf: self::m_ttcan::TURCF,
    #[doc = "0x114 - TT Operation Control"]
    pub ttocn: self::m_ttcan::TTOCN,
    #[doc = "0x118 - TT Global Time Preset"]
    pub ttgtp: self::m_ttcan::TTGTP,
    #[doc = "0x11c - TT Time Mark"]
    pub tttmk: self::m_ttcan::TTTMK,
    #[doc = "0x120 - TT Interrupt Register"]
    pub ttir: self::m_ttcan::TTIR,
    #[doc = "0x124 - TT Interrupt Enable"]
    pub ttie: self::m_ttcan::TTIE,
    #[doc = "0x128 - TT Interrupt Line Select"]
    pub ttils: self::m_ttcan::TTILS,
    #[doc = "0x12c - TT Operation Status"]
    pub ttost: self::m_ttcan::TTOST,
    #[doc = "0x130 - TUR Numerator Actual"]
    pub turna: self::m_ttcan::TURNA,
    #[doc = "0x134 - TT Local & Global Time"]
    pub ttlgt: self::m_ttcan::TTLGT,
    #[doc = "0x138 - TT Cycle Time & Count"]
    pub ttctc: self::m_ttcan::TTCTC,
    #[doc = "0x13c - TT Capture Time"]
    pub ttcpt: self::m_ttcan::TTCPT,
    #[doc = "0x140 - TT Cycle Sync Mark"]
    pub ttcsm: self::m_ttcan::TTCSM,
}
#[doc = r"Register block"]
#[doc = "TTCAN 3PIP, includes FD"]
pub mod m_ttcan;
