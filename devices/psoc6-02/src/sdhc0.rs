#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MMIO at SDHC wrapper level"]
    pub wrap: WRAP,
    _reserved1: [u8; 4092usize],
    #[doc = "0x1000 - MMIO for Synopsys Mobile Storage Host Controller IP"]
    pub core: CORE,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct WRAP {
    #[doc = "0x00 - Top level wrapper control"]
    pub ctl: self::wrap::CTL,
}
#[doc = r"Register block"]
#[doc = "MMIO at SDHC wrapper level"]
pub mod wrap;
#[doc = r"Register block"]
#[repr(C)]
pub struct CORE {
    #[doc = "0x00 - SDMA System Address register"]
    pub sdmasa_r: self::core::SDMASA_R,
    #[doc = "0x04 - Block Size register"]
    pub blocksize_r: self::core::BLOCKSIZE_R,
    #[doc = "0x06 - 16-bit Block Count register"]
    pub blockcount_r: self::core::BLOCKCOUNT_R,
    #[doc = "0x08 - Argument register"]
    pub argument_r: self::core::ARGUMENT_R,
    #[doc = "0x0c - Transfer Mode register"]
    pub xfer_mode_r: self::core::XFER_MODE_R,
    #[doc = "0x0e - Command register"]
    pub cmd_r: self::core::CMD_R,
    #[doc = "0x10 - Response Register 0/1"]
    pub resp01_r: self::core::RESP01_R,
    #[doc = "0x14 - Response Register 2/3"]
    pub resp23_r: self::core::RESP23_R,
    #[doc = "0x18 - Response Register 4/5"]
    pub resp45_r: self::core::RESP45_R,
    #[doc = "0x1c - Response Register 6/7"]
    pub resp67_r: self::core::RESP67_R,
    #[doc = "0x20 - Buffer Data Port Register"]
    pub buf_data_r: self::core::BUF_DATA_R,
    #[doc = "0x24 - Present State Register"]
    pub pstate_reg: self::core::PSTATE_REG,
    #[doc = "0x28 - Host Control 1 Register"]
    pub host_ctrl1_r: self::core::HOST_CTRL1_R,
    #[doc = "0x29 - Power Control Register"]
    pub pwr_ctrl_r: self::core::PWR_CTRL_R,
    #[doc = "0x2a - Block Gap Control Register"]
    pub bgap_ctrl_r: self::core::BGAP_CTRL_R,
    #[doc = "0x2b - Wakeup Control Register"]
    pub wup_ctrl_r: self::core::WUP_CTRL_R,
    #[doc = "0x2c - Clock Control Register"]
    pub clk_ctrl_r: self::core::CLK_CTRL_R,
    #[doc = "0x2e - Timeout Control Register"]
    pub tout_ctrl_r: self::core::TOUT_CTRL_R,
    #[doc = "0x2f - Software Reset Register"]
    pub sw_rst_r: self::core::SW_RST_R,
    #[doc = "0x30 - Normal Interrupt Status Register"]
    pub normal_int_stat_r: self::core::NORMAL_INT_STAT_R,
    #[doc = "0x32 - Error Interrupt Status Register"]
    pub error_int_stat_r: self::core::ERROR_INT_STAT_R,
    #[doc = "0x34 - Normal Interrupt Status Enable Register"]
    pub normal_int_stat_en_r: self::core::NORMAL_INT_STAT_EN_R,
    #[doc = "0x36 - Error Interrupt Status Enable Register"]
    pub error_int_stat_en_r: self::core::ERROR_INT_STAT_EN_R,
    #[doc = "0x38 - Normal Interrupt Signal Enable Register"]
    pub normal_int_signal_en_r: self::core::NORMAL_INT_SIGNAL_EN_R,
    #[doc = "0x3a - Error Interrupt Signal Enable Register"]
    pub error_int_signal_en_r: self::core::ERROR_INT_SIGNAL_EN_R,
    #[doc = "0x3c - Auto CMD Status Register"]
    pub auto_cmd_stat_r: self::core::AUTO_CMD_STAT_R,
    #[doc = "0x3e - Host Control 2 Register"]
    pub host_ctrl2_r: self::core::HOST_CTRL2_R,
    #[doc = "0x40 - Capabilities 1 Register - 0 to 31"]
    pub capabilities1_r: self::core::CAPABILITIES1_R,
    #[doc = "0x44 - Capabilities Register - 32 to 63"]
    pub capabilities2_r: self::core::CAPABILITIES2_R,
    #[doc = "0x48 - Current Capabilities Register - 0 to 31"]
    pub curr_capabilities1_r: self::core::CURR_CAPABILITIES1_R,
    #[doc = "0x4c - Maximum Current Capabilities Register - 32 to 63"]
    pub curr_capabilities2_r: self::core::CURR_CAPABILITIES2_R,
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status register"]
    pub force_auto_cmd_stat_r: self::core::FORCE_AUTO_CMD_STAT_R,
    #[doc = "0x52 - Force Event Register for Error Interrupt Status"]
    pub force_error_int_stat_r: self::core::FORCE_ERROR_INT_STAT_R,
    #[doc = "0x54 - ADMA Error Status Register"]
    pub adma_err_stat_r: self::core::ADMA_ERR_STAT_R,
    _reserved34: [u8; 3usize],
    #[doc = "0x58 - ADMA System Address Register - Low"]
    pub adma_sa_low_r: self::core::ADMA_SA_LOW_R,
    _reserved35: [u8; 28usize],
    #[doc = "0x78 - ADMA3 Integrated Descriptor Address Register - Low"]
    pub adma_id_low_r: self::core::ADMA_ID_LOW_R,
    _reserved36: [u8; 130usize],
    #[doc = "0xfe - Host Controller Version"]
    pub host_cntrl_vers_r: self::core::HOST_CNTRL_VERS_R,
    _reserved37: [u8; 128usize],
    #[doc = "0x180 - Command Queuing Version register"]
    pub cqver: self::core::CQVER,
    #[doc = "0x184 - Command Queuing Capabilities register"]
    pub cqcap: self::core::CQCAP,
    #[doc = "0x188 - Command Queuing Configuration register"]
    pub cqcfg: self::core::CQCFG,
    #[doc = "0x18c - Command Queuing Control register"]
    pub cqctl: self::core::CQCTL,
    #[doc = "0x190 - Command Queuing Interrupt Status register"]
    pub cqis: self::core::CQIS,
    #[doc = "0x194 - Command Queuing Interrupt Status Enable register"]
    pub cqise: self::core::CQISE,
    #[doc = "0x198 - Command Queuing Interrupt signal enable register"]
    pub cqisge: self::core::CQISGE,
    #[doc = "0x19c - Command Queuing Interrupt Coalescing register"]
    pub cqic: self::core::CQIC,
    #[doc = "0x1a0 - Command Queuing Task Descriptor List Base Address register"]
    pub cqtdlba: self::core::CQTDLBA,
    _reserved46: [u8; 4usize],
    #[doc = "0x1a8 - Command Queuing DoorBell register"]
    pub cqtdbr: self::core::CQTDBR,
    #[doc = "0x1ac - Command Queuing TaskClear Notification register"]
    pub cqtcn: self::core::CQTCN,
    #[doc = "0x1b0 - Device queue status register"]
    pub cqdqs: self::core::CQDQS,
    #[doc = "0x1b4 - Device pending tasks register"]
    pub cqdpt: self::core::CQDPT,
    #[doc = "0x1b8 - Command Queuing DoorBell register"]
    pub cqtclr: self::core::CQTCLR,
    _reserved51: [u8; 4usize],
    #[doc = "0x1c0 - CQ Send Status Configuration 1 register"]
    pub cqssc1: self::core::CQSSC1,
    #[doc = "0x1c4 - CQ Send Status Configuration 2 register"]
    pub cqssc2: self::core::CQSSC2,
    #[doc = "0x1c8 - Command response for direct command register"]
    pub cqcrdct: self::core::CQCRDCT,
    _reserved54: [u8; 4usize],
    #[doc = "0x1d0 - Command response mode error mask register"]
    pub cqrmem: self::core::CQRMEM,
    #[doc = "0x1d4 - CQ Task Error Information register"]
    pub cqterri: self::core::CQTERRI,
    #[doc = "0x1d8 - CQ Command response index"]
    pub cqcri: self::core::CQCRI,
    #[doc = "0x1dc - CQ Command response argument register"]
    pub cqcra: self::core::CQCRA,
    _reserved58: [u8; 800usize],
    #[doc = "0x500 - MSHC version"]
    pub mshc_ver_id_r: self::core::MSHC_VER_ID_R,
    #[doc = "0x504 - MSHC version type"]
    pub mshc_ver_type_r: self::core::MSHC_VER_TYPE_R,
    #[doc = "0x508 - MSHC Control register"]
    pub mshc_ctrl_r: self::core::MSHC_CTRL_R,
    _reserved61: [u8; 7usize],
    #[doc = "0x510 - MBIU Control register"]
    pub mbiu_ctrl_r: self::core::MBIU_CTRL_R,
    _reserved62: [u8; 27usize],
    #[doc = "0x52c - eMMC Control register"]
    pub emmc_ctrl_r: self::core::EMMC_CTRL_R,
    #[doc = "0x52e - eMMC Boot Control register"]
    pub boot_ctrl_r: self::core::BOOT_CTRL_R,
    #[doc = "0x530 - General Purpose Input register"]
    pub gp_in_r: self::core::GP_IN_R,
    #[doc = "0x534 - General Purpose Output register"]
    pub gp_out_r: self::core::GP_OUT_R,
}
#[doc = r"Register block"]
#[doc = "MMIO for Synopsys Mobile Storage Host Controller IP"]
pub mod core;
