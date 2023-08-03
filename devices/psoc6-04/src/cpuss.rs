#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Identity"]
    pub identity: IDENTITY,
    #[doc = "0x04 - CM4 status"]
    pub cm4_status: CM4_STATUS,
    #[doc = "0x08 - CM4 clock control"]
    pub cm4_clock_ctl: CM4_CLOCK_CTL,
    #[doc = "0x0c - CM4 control"]
    pub cm4_ctl: CM4_CTL,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - CM4 interrupt 0 status"]
    pub cm4_int0_status: CM4_INT0_STATUS,
    #[doc = "0x104 - CM4 interrupt 1 status"]
    pub cm4_int1_status: CM4_INT1_STATUS,
    #[doc = "0x108 - CM4 interrupt 2 status"]
    pub cm4_int2_status: CM4_INT2_STATUS,
    #[doc = "0x10c - CM4 interrupt 3 status"]
    pub cm4_int3_status: CM4_INT3_STATUS,
    #[doc = "0x110 - CM4 interrupt 4 status"]
    pub cm4_int4_status: CM4_INT4_STATUS,
    #[doc = "0x114 - CM4 interrupt 5 status"]
    pub cm4_int5_status: CM4_INT5_STATUS,
    #[doc = "0x118 - CM4 interrupt 6 status"]
    pub cm4_int6_status: CM4_INT6_STATUS,
    #[doc = "0x11c - CM4 interrupt 7 status"]
    pub cm4_int7_status: CM4_INT7_STATUS,
    _reserved12: [u8; 224usize],
    #[doc = "0x200 - CM4 vector table base"]
    pub cm4_vector_table_base: CM4_VECTOR_TABLE_BASE,
    _reserved13: [u8; 60usize],
    #[doc = "0x240 - CM4 NMI control"]
    pub cm4_nmi_ctl: [CM4_NMI_CTL; 4],
    _reserved14: [u8; 176usize],
    #[doc = "0x300 - UDB power control"]
    pub udb_pwr_ctl: UDB_PWR_CTL,
    #[doc = "0x304 - UDB power control"]
    pub udb_pwr_delay_ctl: UDB_PWR_DELAY_CTL,
    _reserved16: [u8; 3320usize],
    #[doc = "0x1000 - CM0+ control"]
    pub cm0_ctl: CM0_CTL,
    #[doc = "0x1004 - CM0+ status"]
    pub cm0_status: CM0_STATUS,
    #[doc = "0x1008 - CM0+ clock control"]
    pub cm0_clock_ctl: CM0_CLOCK_CTL,
    _reserved19: [u8; 244usize],
    #[doc = "0x1100 - CM0+ interrupt 0 status"]
    pub cm0_int0_status: CM0_INT0_STATUS,
    #[doc = "0x1104 - CM0+ interrupt 1 status"]
    pub cm0_int1_status: CM0_INT1_STATUS,
    #[doc = "0x1108 - CM0+ interrupt 2 status"]
    pub cm0_int2_status: CM0_INT2_STATUS,
    #[doc = "0x110c - CM0+ interrupt 3 status"]
    pub cm0_int3_status: CM0_INT3_STATUS,
    #[doc = "0x1110 - CM0+ interrupt 4 status"]
    pub cm0_int4_status: CM0_INT4_STATUS,
    #[doc = "0x1114 - CM0+ interrupt 5 status"]
    pub cm0_int5_status: CM0_INT5_STATUS,
    #[doc = "0x1118 - CM0+ interrupt 6 status"]
    pub cm0_int6_status: CM0_INT6_STATUS,
    #[doc = "0x111c - CM0+ interrupt 7 status"]
    pub cm0_int7_status: CM0_INT7_STATUS,
    #[doc = "0x1120 - CM0+ vector table base"]
    pub cm0_vector_table_base: CM0_VECTOR_TABLE_BASE,
    _reserved28: [u8; 28usize],
    #[doc = "0x1140 - CM0+ NMI control"]
    pub cm0_nmi_ctl: [CM0_NMI_CTL; 4],
    _reserved29: [u8; 176usize],
    #[doc = "0x1200 - CM4 power control"]
    pub cm4_pwr_ctl: CM4_PWR_CTL,
    #[doc = "0x1204 - CM4 power control"]
    pub cm4_pwr_delay_ctl: CM4_PWR_DELAY_CTL,
    _reserved31: [u8; 248usize],
    #[doc = "0x1300 - RAM 0 control"]
    pub ram0_ctl0: RAM0_CTL0,
    #[doc = "0x1304 - RAM 0 status"]
    pub ram0_status: RAM0_STATUS,
    _reserved33: [u8; 56usize],
    #[doc = "0x1340 - RAM 0 power control"]
    pub ram0_pwr_macro_ctl: [RAM0_PWR_MACRO_CTL; 16],
    #[doc = "0x1380 - RAM 1 control"]
    pub ram1_ctl0: RAM1_CTL0,
    #[doc = "0x1384 - RAM 1 status"]
    pub ram1_status: RAM1_STATUS,
    #[doc = "0x1388 - RAM 1 power control"]
    pub ram1_pwr_ctl: RAM1_PWR_CTL,
    _reserved37: [u8; 20usize],
    #[doc = "0x13a0 - RAM 2 control"]
    pub ram2_ctl0: RAM2_CTL0,
    #[doc = "0x13a4 - RAM 2 status"]
    pub ram2_status: RAM2_STATUS,
    #[doc = "0x13a8 - RAM 2 power control"]
    pub ram2_pwr_ctl: RAM2_PWR_CTL,
    _reserved40: [u8; 20usize],
    #[doc = "0x13c0 - Power up delay used for all SRAM power domains"]
    pub ram_pwr_delay_ctl: RAM_PWR_DELAY_CTL,
    #[doc = "0x13c4 - ROM control"]
    pub rom_ctl: ROM_CTL,
    #[doc = "0x13c8 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved43: [u8; 52usize],
    #[doc = "0x1400 - Product identifier and version (same as CoreSight RomTables)"]
    pub product_id: PRODUCT_ID,
    _reserved44: [u8; 12usize],
    #[doc = "0x1410 - Debug port status"]
    pub dp_status: DP_STATUS,
    #[doc = "0x1414 - Access port control"]
    pub ap_ctl: AP_CTL,
    _reserved46: [u8; 232usize],
    #[doc = "0x1500 - Buffer control"]
    pub buff_ctl: BUFF_CTL,
    _reserved47: [u8; 252usize],
    #[doc = "0x1600 - SysTick timer control"]
    pub systick_ctl: SYSTICK_CTL,
    _reserved48: [u8; 256usize],
    #[doc = "0x1704 - Memory BIST status"]
    pub mbist_stat: MBIST_STAT,
    _reserved49: [u8; 248usize],
    #[doc = "0x1800 - Calibration support set and read"]
    pub cal_sup_set: CAL_SUP_SET,
    #[doc = "0x1804 - Calibration support clear and reset"]
    pub cal_sup_clr: CAL_SUP_CLR,
    _reserved51: [u8; 2040usize],
    #[doc = "0x2000 - CM0+ protection context control"]
    pub cm0_pc_ctl: CM0_PC_CTL,
    _reserved52: [u8; 60usize],
    #[doc = "0x2040 - CM0+ protection context 0 handler"]
    pub cm0_pc0_handler: CM0_PC0_HANDLER,
    #[doc = "0x2044 - CM0+ protection context 1 handler"]
    pub cm0_pc1_handler: CM0_PC1_HANDLER,
    #[doc = "0x2048 - CM0+ protection context 2 handler"]
    pub cm0_pc2_handler: CM0_PC2_HANDLER,
    #[doc = "0x204c - CM0+ protection context 3 handler"]
    pub cm0_pc3_handler: CM0_PC3_HANDLER,
    _reserved56: [u8; 116usize],
    #[doc = "0x20c4 - Protection status"]
    pub protection: PROTECTION,
    _reserved57: [u8; 56usize],
    #[doc = "0x2100 - ROM trim control"]
    pub trim_rom_ctl: TRIM_ROM_CTL,
    #[doc = "0x2104 - RAM trim control"]
    pub trim_ram_ctl: TRIM_RAM_CTL,
    _reserved59: [u8; 24312usize],
    #[doc = "0x8000 - CM0+ system interrupt control"]
    pub cm0_system_int_ctl: [CM0_SYSTEM_INT_CTL; 1023],
    _reserved60: [u8; 4100usize],
    #[doc = "0xa000 - CM4 system interrupt control"]
    pub cm4_system_int_ctl: [CM4_SYSTEM_INT_CTL; 1023],
}
#[doc = "Identity\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [identity](identity) module"]
pub type IDENTITY = crate::Reg<u32, _IDENTITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDENTITY;
#[doc = "`read()` method returns [identity::R](identity::R) reader structure"]
impl crate::Readable for IDENTITY {}
#[doc = "Identity"]
pub mod identity;
#[doc = "CM4 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_status](cm4_status) module"]
pub type CM4_STATUS = crate::Reg<u32, _CM4_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_STATUS;
#[doc = "`read()` method returns [cm4_status::R](cm4_status::R) reader structure"]
impl crate::Readable for CM4_STATUS {}
#[doc = "CM4 status"]
pub mod cm4_status;
#[doc = "CM4 clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_clock_ctl](cm4_clock_ctl) module"]
pub type CM4_CLOCK_CTL = crate::Reg<u32, _CM4_CLOCK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CLOCK_CTL;
#[doc = "`read()` method returns [cm4_clock_ctl::R](cm4_clock_ctl::R) reader structure"]
impl crate::Readable for CM4_CLOCK_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_clock_ctl::W](cm4_clock_ctl::W) writer structure"]
impl crate::Writable for CM4_CLOCK_CTL {}
#[doc = "CM4 clock control"]
pub mod cm4_clock_ctl;
#[doc = "CM4 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ctl](cm4_ctl) module"]
pub type CM4_CTL = crate::Reg<u32, _CM4_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CTL;
#[doc = "`read()` method returns [cm4_ctl::R](cm4_ctl::R) reader structure"]
impl crate::Readable for CM4_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_ctl::W](cm4_ctl::W) writer structure"]
impl crate::Writable for CM4_CTL {}
#[doc = "CM4 control"]
pub mod cm4_ctl;
#[doc = "CM4 interrupt 0 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_int0_status](cm4_int0_status) module"]
pub type CM4_INT0_STATUS = crate::Reg<u32, _CM4_INT0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_INT0_STATUS;
#[doc = "`read()` method returns [cm4_int0_status::R](cm4_int0_status::R) reader structure"]
impl crate::Readable for CM4_INT0_STATUS {}
#[doc = "CM4 interrupt 0 status"]
pub mod cm4_int0_status;
#[doc = "CM4 interrupt 1 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_int1_status](cm4_int1_status) module"]
pub type CM4_INT1_STATUS = crate::Reg<u32, _CM4_INT1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_INT1_STATUS;
#[doc = "`read()` method returns [cm4_int1_status::R](cm4_int1_status::R) reader structure"]
impl crate::Readable for CM4_INT1_STATUS {}
#[doc = "CM4 interrupt 1 status"]
pub mod cm4_int1_status;
#[doc = "CM4 interrupt 2 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_int2_status](cm4_int2_status) module"]
pub type CM4_INT2_STATUS = crate::Reg<u32, _CM4_INT2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_INT2_STATUS;
#[doc = "`read()` method returns [cm4_int2_status::R](cm4_int2_status::R) reader structure"]
impl crate::Readable for CM4_INT2_STATUS {}
#[doc = "CM4 interrupt 2 status"]
pub mod cm4_int2_status;
#[doc = "CM4 interrupt 3 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_int3_status](cm4_int3_status) module"]
pub type CM4_INT3_STATUS = crate::Reg<u32, _CM4_INT3_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_INT3_STATUS;
#[doc = "`read()` method returns [cm4_int3_status::R](cm4_int3_status::R) reader structure"]
impl crate::Readable for CM4_INT3_STATUS {}
#[doc = "CM4 interrupt 3 status"]
pub mod cm4_int3_status;
#[doc = "CM4 interrupt 4 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_int4_status](cm4_int4_status) module"]
pub type CM4_INT4_STATUS = crate::Reg<u32, _CM4_INT4_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_INT4_STATUS;
#[doc = "`read()` method returns [cm4_int4_status::R](cm4_int4_status::R) reader structure"]
impl crate::Readable for CM4_INT4_STATUS {}
#[doc = "CM4 interrupt 4 status"]
pub mod cm4_int4_status;
#[doc = "CM4 interrupt 5 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_int5_status](cm4_int5_status) module"]
pub type CM4_INT5_STATUS = crate::Reg<u32, _CM4_INT5_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_INT5_STATUS;
#[doc = "`read()` method returns [cm4_int5_status::R](cm4_int5_status::R) reader structure"]
impl crate::Readable for CM4_INT5_STATUS {}
#[doc = "CM4 interrupt 5 status"]
pub mod cm4_int5_status;
#[doc = "CM4 interrupt 6 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_int6_status](cm4_int6_status) module"]
pub type CM4_INT6_STATUS = crate::Reg<u32, _CM4_INT6_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_INT6_STATUS;
#[doc = "`read()` method returns [cm4_int6_status::R](cm4_int6_status::R) reader structure"]
impl crate::Readable for CM4_INT6_STATUS {}
#[doc = "CM4 interrupt 6 status"]
pub mod cm4_int6_status;
#[doc = "CM4 interrupt 7 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_int7_status](cm4_int7_status) module"]
pub type CM4_INT7_STATUS = crate::Reg<u32, _CM4_INT7_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_INT7_STATUS;
#[doc = "`read()` method returns [cm4_int7_status::R](cm4_int7_status::R) reader structure"]
impl crate::Readable for CM4_INT7_STATUS {}
#[doc = "CM4 interrupt 7 status"]
pub mod cm4_int7_status;
#[doc = "CM4 vector table base\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_vector_table_base](cm4_vector_table_base) module"]
pub type CM4_VECTOR_TABLE_BASE = crate::Reg<u32, _CM4_VECTOR_TABLE_BASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_VECTOR_TABLE_BASE;
#[doc = "`read()` method returns [cm4_vector_table_base::R](cm4_vector_table_base::R) reader structure"]
impl crate::Readable for CM4_VECTOR_TABLE_BASE {}
#[doc = "`write(|w| ..)` method takes [cm4_vector_table_base::W](cm4_vector_table_base::W) writer structure"]
impl crate::Writable for CM4_VECTOR_TABLE_BASE {}
#[doc = "CM4 vector table base"]
pub mod cm4_vector_table_base;
#[doc = "CM4 NMI control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_nmi_ctl](cm4_nmi_ctl) module"]
pub type CM4_NMI_CTL = crate::Reg<u32, _CM4_NMI_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_NMI_CTL;
#[doc = "`read()` method returns [cm4_nmi_ctl::R](cm4_nmi_ctl::R) reader structure"]
impl crate::Readable for CM4_NMI_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_nmi_ctl::W](cm4_nmi_ctl::W) writer structure"]
impl crate::Writable for CM4_NMI_CTL {}
#[doc = "CM4 NMI control"]
pub mod cm4_nmi_ctl;
#[doc = "UDB power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udb_pwr_ctl](udb_pwr_ctl) module"]
pub type UDB_PWR_CTL = crate::Reg<u32, _UDB_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDB_PWR_CTL;
#[doc = "`read()` method returns [udb_pwr_ctl::R](udb_pwr_ctl::R) reader structure"]
impl crate::Readable for UDB_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [udb_pwr_ctl::W](udb_pwr_ctl::W) writer structure"]
impl crate::Writable for UDB_PWR_CTL {}
#[doc = "UDB power control"]
pub mod udb_pwr_ctl;
#[doc = "UDB power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udb_pwr_delay_ctl](udb_pwr_delay_ctl) module"]
pub type UDB_PWR_DELAY_CTL = crate::Reg<u32, _UDB_PWR_DELAY_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDB_PWR_DELAY_CTL;
#[doc = "`read()` method returns [udb_pwr_delay_ctl::R](udb_pwr_delay_ctl::R) reader structure"]
impl crate::Readable for UDB_PWR_DELAY_CTL {}
#[doc = "`write(|w| ..)` method takes [udb_pwr_delay_ctl::W](udb_pwr_delay_ctl::W) writer structure"]
impl crate::Writable for UDB_PWR_DELAY_CTL {}
#[doc = "UDB power control"]
pub mod udb_pwr_delay_ctl;
#[doc = "CM0+ control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ctl](cm0_ctl) module"]
pub type CM0_CTL = crate::Reg<u32, _CM0_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CTL;
#[doc = "`read()` method returns [cm0_ctl::R](cm0_ctl::R) reader structure"]
impl crate::Readable for CM0_CTL {}
#[doc = "`write(|w| ..)` method takes [cm0_ctl::W](cm0_ctl::W) writer structure"]
impl crate::Writable for CM0_CTL {}
#[doc = "CM0+ control"]
pub mod cm0_ctl;
#[doc = "CM0+ status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_status](cm0_status) module"]
pub type CM0_STATUS = crate::Reg<u32, _CM0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_STATUS;
#[doc = "`read()` method returns [cm0_status::R](cm0_status::R) reader structure"]
impl crate::Readable for CM0_STATUS {}
#[doc = "CM0+ status"]
pub mod cm0_status;
#[doc = "CM0+ clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_clock_ctl](cm0_clock_ctl) module"]
pub type CM0_CLOCK_CTL = crate::Reg<u32, _CM0_CLOCK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CLOCK_CTL;
#[doc = "`read()` method returns [cm0_clock_ctl::R](cm0_clock_ctl::R) reader structure"]
impl crate::Readable for CM0_CLOCK_CTL {}
#[doc = "`write(|w| ..)` method takes [cm0_clock_ctl::W](cm0_clock_ctl::W) writer structure"]
impl crate::Writable for CM0_CLOCK_CTL {}
#[doc = "CM0+ clock control"]
pub mod cm0_clock_ctl;
#[doc = "CM0+ interrupt 0 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int0_status](cm0_int0_status) module"]
pub type CM0_INT0_STATUS = crate::Reg<u32, _CM0_INT0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT0_STATUS;
#[doc = "`read()` method returns [cm0_int0_status::R](cm0_int0_status::R) reader structure"]
impl crate::Readable for CM0_INT0_STATUS {}
#[doc = "CM0+ interrupt 0 status"]
pub mod cm0_int0_status;
#[doc = "CM0+ interrupt 1 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int1_status](cm0_int1_status) module"]
pub type CM0_INT1_STATUS = crate::Reg<u32, _CM0_INT1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT1_STATUS;
#[doc = "`read()` method returns [cm0_int1_status::R](cm0_int1_status::R) reader structure"]
impl crate::Readable for CM0_INT1_STATUS {}
#[doc = "CM0+ interrupt 1 status"]
pub mod cm0_int1_status;
#[doc = "CM0+ interrupt 2 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int2_status](cm0_int2_status) module"]
pub type CM0_INT2_STATUS = crate::Reg<u32, _CM0_INT2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT2_STATUS;
#[doc = "`read()` method returns [cm0_int2_status::R](cm0_int2_status::R) reader structure"]
impl crate::Readable for CM0_INT2_STATUS {}
#[doc = "CM0+ interrupt 2 status"]
pub mod cm0_int2_status;
#[doc = "CM0+ interrupt 3 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int3_status](cm0_int3_status) module"]
pub type CM0_INT3_STATUS = crate::Reg<u32, _CM0_INT3_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT3_STATUS;
#[doc = "`read()` method returns [cm0_int3_status::R](cm0_int3_status::R) reader structure"]
impl crate::Readable for CM0_INT3_STATUS {}
#[doc = "CM0+ interrupt 3 status"]
pub mod cm0_int3_status;
#[doc = "CM0+ interrupt 4 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int4_status](cm0_int4_status) module"]
pub type CM0_INT4_STATUS = crate::Reg<u32, _CM0_INT4_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT4_STATUS;
#[doc = "`read()` method returns [cm0_int4_status::R](cm0_int4_status::R) reader structure"]
impl crate::Readable for CM0_INT4_STATUS {}
#[doc = "CM0+ interrupt 4 status"]
pub mod cm0_int4_status;
#[doc = "CM0+ interrupt 5 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int5_status](cm0_int5_status) module"]
pub type CM0_INT5_STATUS = crate::Reg<u32, _CM0_INT5_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT5_STATUS;
#[doc = "`read()` method returns [cm0_int5_status::R](cm0_int5_status::R) reader structure"]
impl crate::Readable for CM0_INT5_STATUS {}
#[doc = "CM0+ interrupt 5 status"]
pub mod cm0_int5_status;
#[doc = "CM0+ interrupt 6 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int6_status](cm0_int6_status) module"]
pub type CM0_INT6_STATUS = crate::Reg<u32, _CM0_INT6_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT6_STATUS;
#[doc = "`read()` method returns [cm0_int6_status::R](cm0_int6_status::R) reader structure"]
impl crate::Readable for CM0_INT6_STATUS {}
#[doc = "CM0+ interrupt 6 status"]
pub mod cm0_int6_status;
#[doc = "CM0+ interrupt 7 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int7_status](cm0_int7_status) module"]
pub type CM0_INT7_STATUS = crate::Reg<u32, _CM0_INT7_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT7_STATUS;
#[doc = "`read()` method returns [cm0_int7_status::R](cm0_int7_status::R) reader structure"]
impl crate::Readable for CM0_INT7_STATUS {}
#[doc = "CM0+ interrupt 7 status"]
pub mod cm0_int7_status;
#[doc = "CM0+ vector table base\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_vector_table_base](cm0_vector_table_base) module"]
pub type CM0_VECTOR_TABLE_BASE = crate::Reg<u32, _CM0_VECTOR_TABLE_BASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_VECTOR_TABLE_BASE;
#[doc = "`read()` method returns [cm0_vector_table_base::R](cm0_vector_table_base::R) reader structure"]
impl crate::Readable for CM0_VECTOR_TABLE_BASE {}
#[doc = "`write(|w| ..)` method takes [cm0_vector_table_base::W](cm0_vector_table_base::W) writer structure"]
impl crate::Writable for CM0_VECTOR_TABLE_BASE {}
#[doc = "CM0+ vector table base"]
pub mod cm0_vector_table_base;
#[doc = "CM0+ NMI control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_nmi_ctl](cm0_nmi_ctl) module"]
pub type CM0_NMI_CTL = crate::Reg<u32, _CM0_NMI_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_NMI_CTL;
#[doc = "`read()` method returns [cm0_nmi_ctl::R](cm0_nmi_ctl::R) reader structure"]
impl crate::Readable for CM0_NMI_CTL {}
#[doc = "`write(|w| ..)` method takes [cm0_nmi_ctl::W](cm0_nmi_ctl::W) writer structure"]
impl crate::Writable for CM0_NMI_CTL {}
#[doc = "CM0+ NMI control"]
pub mod cm0_nmi_ctl;
#[doc = "CM4 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_pwr_ctl](cm4_pwr_ctl) module"]
pub type CM4_PWR_CTL = crate::Reg<u32, _CM4_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_PWR_CTL;
#[doc = "`read()` method returns [cm4_pwr_ctl::R](cm4_pwr_ctl::R) reader structure"]
impl crate::Readable for CM4_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_pwr_ctl::W](cm4_pwr_ctl::W) writer structure"]
impl crate::Writable for CM4_PWR_CTL {}
#[doc = "CM4 power control"]
pub mod cm4_pwr_ctl;
#[doc = "CM4 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_pwr_delay_ctl](cm4_pwr_delay_ctl) module"]
pub type CM4_PWR_DELAY_CTL = crate::Reg<u32, _CM4_PWR_DELAY_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_PWR_DELAY_CTL;
#[doc = "`read()` method returns [cm4_pwr_delay_ctl::R](cm4_pwr_delay_ctl::R) reader structure"]
impl crate::Readable for CM4_PWR_DELAY_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_pwr_delay_ctl::W](cm4_pwr_delay_ctl::W) writer structure"]
impl crate::Writable for CM4_PWR_DELAY_CTL {}
#[doc = "CM4 power control"]
pub mod cm4_pwr_delay_ctl;
#[doc = "RAM 0 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram0_ctl0](ram0_ctl0) module"]
pub type RAM0_CTL0 = crate::Reg<u32, _RAM0_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM0_CTL0;
#[doc = "`read()` method returns [ram0_ctl0::R](ram0_ctl0::R) reader structure"]
impl crate::Readable for RAM0_CTL0 {}
#[doc = "`write(|w| ..)` method takes [ram0_ctl0::W](ram0_ctl0::W) writer structure"]
impl crate::Writable for RAM0_CTL0 {}
#[doc = "RAM 0 control"]
pub mod ram0_ctl0;
#[doc = "RAM 0 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram0_status](ram0_status) module"]
pub type RAM0_STATUS = crate::Reg<u32, _RAM0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM0_STATUS;
#[doc = "`read()` method returns [ram0_status::R](ram0_status::R) reader structure"]
impl crate::Readable for RAM0_STATUS {}
#[doc = "RAM 0 status"]
pub mod ram0_status;
#[doc = "RAM 0 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram0_pwr_macro_ctl](ram0_pwr_macro_ctl) module"]
pub type RAM0_PWR_MACRO_CTL = crate::Reg<u32, _RAM0_PWR_MACRO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM0_PWR_MACRO_CTL;
#[doc = "`read()` method returns [ram0_pwr_macro_ctl::R](ram0_pwr_macro_ctl::R) reader structure"]
impl crate::Readable for RAM0_PWR_MACRO_CTL {}
#[doc = "`write(|w| ..)` method takes [ram0_pwr_macro_ctl::W](ram0_pwr_macro_ctl::W) writer structure"]
impl crate::Writable for RAM0_PWR_MACRO_CTL {}
#[doc = "RAM 0 power control"]
pub mod ram0_pwr_macro_ctl;
#[doc = "RAM 1 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram1_ctl0](ram1_ctl0) module"]
pub type RAM1_CTL0 = crate::Reg<u32, _RAM1_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM1_CTL0;
#[doc = "`read()` method returns [ram1_ctl0::R](ram1_ctl0::R) reader structure"]
impl crate::Readable for RAM1_CTL0 {}
#[doc = "`write(|w| ..)` method takes [ram1_ctl0::W](ram1_ctl0::W) writer structure"]
impl crate::Writable for RAM1_CTL0 {}
#[doc = "RAM 1 control"]
pub mod ram1_ctl0;
#[doc = "RAM 1 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram1_status](ram1_status) module"]
pub type RAM1_STATUS = crate::Reg<u32, _RAM1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM1_STATUS;
#[doc = "`read()` method returns [ram1_status::R](ram1_status::R) reader structure"]
impl crate::Readable for RAM1_STATUS {}
#[doc = "RAM 1 status"]
pub mod ram1_status;
#[doc = "RAM 1 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram1_pwr_ctl](ram1_pwr_ctl) module"]
pub type RAM1_PWR_CTL = crate::Reg<u32, _RAM1_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM1_PWR_CTL;
#[doc = "`read()` method returns [ram1_pwr_ctl::R](ram1_pwr_ctl::R) reader structure"]
impl crate::Readable for RAM1_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [ram1_pwr_ctl::W](ram1_pwr_ctl::W) writer structure"]
impl crate::Writable for RAM1_PWR_CTL {}
#[doc = "RAM 1 power control"]
pub mod ram1_pwr_ctl;
#[doc = "RAM 2 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram2_ctl0](ram2_ctl0) module"]
pub type RAM2_CTL0 = crate::Reg<u32, _RAM2_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM2_CTL0;
#[doc = "`read()` method returns [ram2_ctl0::R](ram2_ctl0::R) reader structure"]
impl crate::Readable for RAM2_CTL0 {}
#[doc = "`write(|w| ..)` method takes [ram2_ctl0::W](ram2_ctl0::W) writer structure"]
impl crate::Writable for RAM2_CTL0 {}
#[doc = "RAM 2 control"]
pub mod ram2_ctl0;
#[doc = "RAM 2 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram2_status](ram2_status) module"]
pub type RAM2_STATUS = crate::Reg<u32, _RAM2_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM2_STATUS;
#[doc = "`read()` method returns [ram2_status::R](ram2_status::R) reader structure"]
impl crate::Readable for RAM2_STATUS {}
#[doc = "RAM 2 status"]
pub mod ram2_status;
#[doc = "RAM 2 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram2_pwr_ctl](ram2_pwr_ctl) module"]
pub type RAM2_PWR_CTL = crate::Reg<u32, _RAM2_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM2_PWR_CTL;
#[doc = "`read()` method returns [ram2_pwr_ctl::R](ram2_pwr_ctl::R) reader structure"]
impl crate::Readable for RAM2_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [ram2_pwr_ctl::W](ram2_pwr_ctl::W) writer structure"]
impl crate::Writable for RAM2_PWR_CTL {}
#[doc = "RAM 2 power control"]
pub mod ram2_pwr_ctl;
#[doc = "Power up delay used for all SRAM power domains\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_pwr_delay_ctl](ram_pwr_delay_ctl) module"]
pub type RAM_PWR_DELAY_CTL = crate::Reg<u32, _RAM_PWR_DELAY_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_PWR_DELAY_CTL;
#[doc = "`read()` method returns [ram_pwr_delay_ctl::R](ram_pwr_delay_ctl::R) reader structure"]
impl crate::Readable for RAM_PWR_DELAY_CTL {}
#[doc = "`write(|w| ..)` method takes [ram_pwr_delay_ctl::W](ram_pwr_delay_ctl::W) writer structure"]
impl crate::Writable for RAM_PWR_DELAY_CTL {}
#[doc = "Power up delay used for all SRAM power domains"]
pub mod ram_pwr_delay_ctl;
#[doc = "ROM control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_ctl](rom_ctl) module"]
pub type ROM_CTL = crate::Reg<u32, _ROM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_CTL;
#[doc = "`read()` method returns [rom_ctl::R](rom_ctl::R) reader structure"]
impl crate::Readable for ROM_CTL {}
#[doc = "`write(|w| ..)` method takes [rom_ctl::W](rom_ctl::W) writer structure"]
impl crate::Writable for ROM_CTL {}
#[doc = "ROM control"]
pub mod rom_ctl;
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
#[doc = "Product identifier and version (same as CoreSight RomTables)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [product_id](product_id) module"]
pub type PRODUCT_ID = crate::Reg<u32, _PRODUCT_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRODUCT_ID;
#[doc = "`read()` method returns [product_id::R](product_id::R) reader structure"]
impl crate::Readable for PRODUCT_ID {}
#[doc = "Product identifier and version (same as CoreSight RomTables)"]
pub mod product_id;
#[doc = "Debug port status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dp_status](dp_status) module"]
pub type DP_STATUS = crate::Reg<u32, _DP_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DP_STATUS;
#[doc = "`read()` method returns [dp_status::R](dp_status::R) reader structure"]
impl crate::Readable for DP_STATUS {}
#[doc = "Debug port status"]
pub mod dp_status;
#[doc = "Access port control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ap_ctl](ap_ctl) module"]
pub type AP_CTL = crate::Reg<u32, _AP_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AP_CTL;
#[doc = "`read()` method returns [ap_ctl::R](ap_ctl::R) reader structure"]
impl crate::Readable for AP_CTL {}
#[doc = "`write(|w| ..)` method takes [ap_ctl::W](ap_ctl::W) writer structure"]
impl crate::Writable for AP_CTL {}
#[doc = "Access port control"]
pub mod ap_ctl;
#[doc = "Buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buff_ctl](buff_ctl) module"]
pub type BUFF_CTL = crate::Reg<u32, _BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFF_CTL;
#[doc = "`read()` method returns [buff_ctl::R](buff_ctl::R) reader structure"]
impl crate::Readable for BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [buff_ctl::W](buff_ctl::W) writer structure"]
impl crate::Writable for BUFF_CTL {}
#[doc = "Buffer control"]
pub mod buff_ctl;
#[doc = "SysTick timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systick_ctl](systick_ctl) module"]
pub type SYSTICK_CTL = crate::Reg<u32, _SYSTICK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICK_CTL;
#[doc = "`read()` method returns [systick_ctl::R](systick_ctl::R) reader structure"]
impl crate::Readable for SYSTICK_CTL {}
#[doc = "`write(|w| ..)` method takes [systick_ctl::W](systick_ctl::W) writer structure"]
impl crate::Writable for SYSTICK_CTL {}
#[doc = "SysTick timer control"]
pub mod systick_ctl;
#[doc = "Memory BIST status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_stat](mbist_stat) module"]
pub type MBIST_STAT = crate::Reg<u32, _MBIST_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBIST_STAT;
#[doc = "`read()` method returns [mbist_stat::R](mbist_stat::R) reader structure"]
impl crate::Readable for MBIST_STAT {}
#[doc = "Memory BIST status"]
pub mod mbist_stat;
#[doc = "Calibration support set and read\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_sup_set](cal_sup_set) module"]
pub type CAL_SUP_SET = crate::Reg<u32, _CAL_SUP_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_SUP_SET;
#[doc = "`read()` method returns [cal_sup_set::R](cal_sup_set::R) reader structure"]
impl crate::Readable for CAL_SUP_SET {}
#[doc = "`write(|w| ..)` method takes [cal_sup_set::W](cal_sup_set::W) writer structure"]
impl crate::Writable for CAL_SUP_SET {}
#[doc = "Calibration support set and read"]
pub mod cal_sup_set;
#[doc = "Calibration support clear and reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_sup_clr](cal_sup_clr) module"]
pub type CAL_SUP_CLR = crate::Reg<u32, _CAL_SUP_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_SUP_CLR;
#[doc = "`read()` method returns [cal_sup_clr::R](cal_sup_clr::R) reader structure"]
impl crate::Readable for CAL_SUP_CLR {}
#[doc = "`write(|w| ..)` method takes [cal_sup_clr::W](cal_sup_clr::W) writer structure"]
impl crate::Writable for CAL_SUP_CLR {}
#[doc = "Calibration support clear and reset"]
pub mod cal_sup_clr;
#[doc = "CM0+ protection context control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_pc_ctl](cm0_pc_ctl) module"]
pub type CM0_PC_CTL = crate::Reg<u32, _CM0_PC_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_PC_CTL;
#[doc = "`read()` method returns [cm0_pc_ctl::R](cm0_pc_ctl::R) reader structure"]
impl crate::Readable for CM0_PC_CTL {}
#[doc = "`write(|w| ..)` method takes [cm0_pc_ctl::W](cm0_pc_ctl::W) writer structure"]
impl crate::Writable for CM0_PC_CTL {}
#[doc = "CM0+ protection context control"]
pub mod cm0_pc_ctl;
#[doc = "CM0+ protection context 0 handler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_pc0_handler](cm0_pc0_handler) module"]
pub type CM0_PC0_HANDLER = crate::Reg<u32, _CM0_PC0_HANDLER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_PC0_HANDLER;
#[doc = "`read()` method returns [cm0_pc0_handler::R](cm0_pc0_handler::R) reader structure"]
impl crate::Readable for CM0_PC0_HANDLER {}
#[doc = "`write(|w| ..)` method takes [cm0_pc0_handler::W](cm0_pc0_handler::W) writer structure"]
impl crate::Writable for CM0_PC0_HANDLER {}
#[doc = "CM0+ protection context 0 handler"]
pub mod cm0_pc0_handler;
#[doc = "CM0+ protection context 1 handler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_pc1_handler](cm0_pc1_handler) module"]
pub type CM0_PC1_HANDLER = crate::Reg<u32, _CM0_PC1_HANDLER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_PC1_HANDLER;
#[doc = "`read()` method returns [cm0_pc1_handler::R](cm0_pc1_handler::R) reader structure"]
impl crate::Readable for CM0_PC1_HANDLER {}
#[doc = "`write(|w| ..)` method takes [cm0_pc1_handler::W](cm0_pc1_handler::W) writer structure"]
impl crate::Writable for CM0_PC1_HANDLER {}
#[doc = "CM0+ protection context 1 handler"]
pub mod cm0_pc1_handler;
#[doc = "CM0+ protection context 2 handler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_pc2_handler](cm0_pc2_handler) module"]
pub type CM0_PC2_HANDLER = crate::Reg<u32, _CM0_PC2_HANDLER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_PC2_HANDLER;
#[doc = "`read()` method returns [cm0_pc2_handler::R](cm0_pc2_handler::R) reader structure"]
impl crate::Readable for CM0_PC2_HANDLER {}
#[doc = "`write(|w| ..)` method takes [cm0_pc2_handler::W](cm0_pc2_handler::W) writer structure"]
impl crate::Writable for CM0_PC2_HANDLER {}
#[doc = "CM0+ protection context 2 handler"]
pub mod cm0_pc2_handler;
#[doc = "CM0+ protection context 3 handler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_pc3_handler](cm0_pc3_handler) module"]
pub type CM0_PC3_HANDLER = crate::Reg<u32, _CM0_PC3_HANDLER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_PC3_HANDLER;
#[doc = "`read()` method returns [cm0_pc3_handler::R](cm0_pc3_handler::R) reader structure"]
impl crate::Readable for CM0_PC3_HANDLER {}
#[doc = "`write(|w| ..)` method takes [cm0_pc3_handler::W](cm0_pc3_handler::W) writer structure"]
impl crate::Writable for CM0_PC3_HANDLER {}
#[doc = "CM0+ protection context 3 handler"]
pub mod cm0_pc3_handler;
#[doc = "Protection status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [protection](protection) module"]
pub type PROTECTION = crate::Reg<u32, _PROTECTION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROTECTION;
#[doc = "`read()` method returns [protection::R](protection::R) reader structure"]
impl crate::Readable for PROTECTION {}
#[doc = "`write(|w| ..)` method takes [protection::W](protection::W) writer structure"]
impl crate::Writable for PROTECTION {}
#[doc = "Protection status"]
pub mod protection;
#[doc = "ROM trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_rom_ctl](trim_rom_ctl) module"]
pub type TRIM_ROM_CTL = crate::Reg<u32, _TRIM_ROM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_ROM_CTL;
#[doc = "`read()` method returns [trim_rom_ctl::R](trim_rom_ctl::R) reader structure"]
impl crate::Readable for TRIM_ROM_CTL {}
#[doc = "`write(|w| ..)` method takes [trim_rom_ctl::W](trim_rom_ctl::W) writer structure"]
impl crate::Writable for TRIM_ROM_CTL {}
#[doc = "ROM trim control"]
pub mod trim_rom_ctl;
#[doc = "RAM trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ram_ctl](trim_ram_ctl) module"]
pub type TRIM_RAM_CTL = crate::Reg<u32, _TRIM_RAM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_RAM_CTL;
#[doc = "`read()` method returns [trim_ram_ctl::R](trim_ram_ctl::R) reader structure"]
impl crate::Readable for TRIM_RAM_CTL {}
#[doc = "`write(|w| ..)` method takes [trim_ram_ctl::W](trim_ram_ctl::W) writer structure"]
impl crate::Writable for TRIM_RAM_CTL {}
#[doc = "RAM trim control"]
pub mod trim_ram_ctl;
#[doc = "CM0+ system interrupt control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_system_int_ctl](cm0_system_int_ctl) module"]
pub type CM0_SYSTEM_INT_CTL = crate::Reg<u32, _CM0_SYSTEM_INT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_SYSTEM_INT_CTL;
#[doc = "`read()` method returns [cm0_system_int_ctl::R](cm0_system_int_ctl::R) reader structure"]
impl crate::Readable for CM0_SYSTEM_INT_CTL {}
#[doc = "`write(|w| ..)` method takes [cm0_system_int_ctl::W](cm0_system_int_ctl::W) writer structure"]
impl crate::Writable for CM0_SYSTEM_INT_CTL {}
#[doc = "CM0+ system interrupt control"]
pub mod cm0_system_int_ctl;
#[doc = "CM4 system interrupt control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_system_int_ctl](cm4_system_int_ctl) module"]
pub type CM4_SYSTEM_INT_CTL = crate::Reg<u32, _CM4_SYSTEM_INT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_SYSTEM_INT_CTL;
#[doc = "`read()` method returns [cm4_system_int_ctl::R](cm4_system_int_ctl::R) reader structure"]
impl crate::Readable for CM4_SYSTEM_INT_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_system_int_ctl::W](cm4_system_int_ctl::W) writer structure"]
impl crate::Writable for CM4_SYSTEM_INT_CTL {}
#[doc = "CM4 system interrupt control"]
pub mod cm4_system_int_ctl;
