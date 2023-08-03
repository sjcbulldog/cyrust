#[doc = "SDMA System Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmasa_r](sdmasa_r) module"]
pub type SDMASA_R = crate::Reg<u32, _SDMASA_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMASA_R;
#[doc = "`read()` method returns [sdmasa_r::R](sdmasa_r::R) reader structure"]
impl crate::Readable for SDMASA_R {}
#[doc = "`write(|w| ..)` method takes [sdmasa_r::W](sdmasa_r::W) writer structure"]
impl crate::Writable for SDMASA_R {}
#[doc = "SDMA System Address register"]
pub mod sdmasa_r;
#[doc = "Block Size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blocksize_r](blocksize_r) module"]
pub type BLOCKSIZE_R = crate::Reg<u16, _BLOCKSIZE_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLOCKSIZE_R;
#[doc = "`read()` method returns [blocksize_r::R](blocksize_r::R) reader structure"]
impl crate::Readable for BLOCKSIZE_R {}
#[doc = "`write(|w| ..)` method takes [blocksize_r::W](blocksize_r::W) writer structure"]
impl crate::Writable for BLOCKSIZE_R {}
#[doc = "Block Size register"]
pub mod blocksize_r;
#[doc = "16-bit Block Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blockcount_r](blockcount_r) module"]
pub type BLOCKCOUNT_R = crate::Reg<u16, _BLOCKCOUNT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLOCKCOUNT_R;
#[doc = "`read()` method returns [blockcount_r::R](blockcount_r::R) reader structure"]
impl crate::Readable for BLOCKCOUNT_R {}
#[doc = "`write(|w| ..)` method takes [blockcount_r::W](blockcount_r::W) writer structure"]
impl crate::Writable for BLOCKCOUNT_R {}
#[doc = "16-bit Block Count register"]
pub mod blockcount_r;
#[doc = "Argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argument_r](argument_r) module"]
pub type ARGUMENT_R = crate::Reg<u32, _ARGUMENT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARGUMENT_R;
#[doc = "`read()` method returns [argument_r::R](argument_r::R) reader structure"]
impl crate::Readable for ARGUMENT_R {}
#[doc = "`write(|w| ..)` method takes [argument_r::W](argument_r::W) writer structure"]
impl crate::Writable for ARGUMENT_R {}
#[doc = "Argument register"]
pub mod argument_r;
#[doc = "Transfer Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xfer_mode_r](xfer_mode_r) module"]
pub type XFER_MODE_R = crate::Reg<u16, _XFER_MODE_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XFER_MODE_R;
#[doc = "`read()` method returns [xfer_mode_r::R](xfer_mode_r::R) reader structure"]
impl crate::Readable for XFER_MODE_R {}
#[doc = "`write(|w| ..)` method takes [xfer_mode_r::W](xfer_mode_r::W) writer structure"]
impl crate::Writable for XFER_MODE_R {}
#[doc = "Transfer Mode register"]
pub mod xfer_mode_r;
#[doc = "Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_r](cmd_r) module"]
pub type CMD_R = crate::Reg<u16, _CMD_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_R;
#[doc = "`read()` method returns [cmd_r::R](cmd_r::R) reader structure"]
impl crate::Readable for CMD_R {}
#[doc = "`write(|w| ..)` method takes [cmd_r::W](cmd_r::W) writer structure"]
impl crate::Writable for CMD_R {}
#[doc = "Command register"]
pub mod cmd_r;
#[doc = "Response Register 0/1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp01_r](resp01_r) module"]
pub type RESP01_R = crate::Reg<u32, _RESP01_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP01_R;
#[doc = "`read()` method returns [resp01_r::R](resp01_r::R) reader structure"]
impl crate::Readable for RESP01_R {}
#[doc = "Response Register 0/1"]
pub mod resp01_r;
#[doc = "Response Register 2/3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp23_r](resp23_r) module"]
pub type RESP23_R = crate::Reg<u32, _RESP23_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP23_R;
#[doc = "`read()` method returns [resp23_r::R](resp23_r::R) reader structure"]
impl crate::Readable for RESP23_R {}
#[doc = "Response Register 2/3"]
pub mod resp23_r;
#[doc = "Response Register 4/5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp45_r](resp45_r) module"]
pub type RESP45_R = crate::Reg<u32, _RESP45_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP45_R;
#[doc = "`read()` method returns [resp45_r::R](resp45_r::R) reader structure"]
impl crate::Readable for RESP45_R {}
#[doc = "Response Register 4/5"]
pub mod resp45_r;
#[doc = "Response Register 6/7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp67_r](resp67_r) module"]
pub type RESP67_R = crate::Reg<u32, _RESP67_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP67_R;
#[doc = "`read()` method returns [resp67_r::R](resp67_r::R) reader structure"]
impl crate::Readable for RESP67_R {}
#[doc = "Response Register 6/7"]
pub mod resp67_r;
#[doc = "Buffer Data Port Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_data_r](buf_data_r) module"]
pub type BUF_DATA_R = crate::Reg<u32, _BUF_DATA_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF_DATA_R;
#[doc = "`read()` method returns [buf_data_r::R](buf_data_r::R) reader structure"]
impl crate::Readable for BUF_DATA_R {}
#[doc = "`write(|w| ..)` method takes [buf_data_r::W](buf_data_r::W) writer structure"]
impl crate::Writable for BUF_DATA_R {}
#[doc = "Buffer Data Port Register"]
pub mod buf_data_r;
#[doc = "Present State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstate_reg](pstate_reg) module"]
pub type PSTATE_REG = crate::Reg<u32, _PSTATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSTATE_REG;
#[doc = "`read()` method returns [pstate_reg::R](pstate_reg::R) reader structure"]
impl crate::Readable for PSTATE_REG {}
#[doc = "Present State Register"]
pub mod pstate_reg;
#[doc = "Host Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctrl1_r](host_ctrl1_r) module"]
pub type HOST_CTRL1_R = crate::Reg<u8, _HOST_CTRL1_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CTRL1_R;
#[doc = "`read()` method returns [host_ctrl1_r::R](host_ctrl1_r::R) reader structure"]
impl crate::Readable for HOST_CTRL1_R {}
#[doc = "`write(|w| ..)` method takes [host_ctrl1_r::W](host_ctrl1_r::W) writer structure"]
impl crate::Writable for HOST_CTRL1_R {}
#[doc = "Host Control 1 Register"]
pub mod host_ctrl1_r;
#[doc = "Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl_r](pwr_ctrl_r) module"]
pub type PWR_CTRL_R = crate::Reg<u8, _PWR_CTRL_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_CTRL_R;
#[doc = "`read()` method returns [pwr_ctrl_r::R](pwr_ctrl_r::R) reader structure"]
impl crate::Readable for PWR_CTRL_R {}
#[doc = "`write(|w| ..)` method takes [pwr_ctrl_r::W](pwr_ctrl_r::W) writer structure"]
impl crate::Writable for PWR_CTRL_R {}
#[doc = "Power Control Register"]
pub mod pwr_ctrl_r;
#[doc = "Block Gap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgap_ctrl_r](bgap_ctrl_r) module"]
pub type BGAP_CTRL_R = crate::Reg<u8, _BGAP_CTRL_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGAP_CTRL_R;
#[doc = "`read()` method returns [bgap_ctrl_r::R](bgap_ctrl_r::R) reader structure"]
impl crate::Readable for BGAP_CTRL_R {}
#[doc = "`write(|w| ..)` method takes [bgap_ctrl_r::W](bgap_ctrl_r::W) writer structure"]
impl crate::Writable for BGAP_CTRL_R {}
#[doc = "Block Gap Control Register"]
pub mod bgap_ctrl_r;
#[doc = "Wakeup Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wup_ctrl_r](wup_ctrl_r) module"]
pub type WUP_CTRL_R = crate::Reg<u8, _WUP_CTRL_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUP_CTRL_R;
#[doc = "`read()` method returns [wup_ctrl_r::R](wup_ctrl_r::R) reader structure"]
impl crate::Readable for WUP_CTRL_R {}
#[doc = "`write(|w| ..)` method takes [wup_ctrl_r::W](wup_ctrl_r::W) writer structure"]
impl crate::Writable for WUP_CTRL_R {}
#[doc = "Wakeup Control Register"]
pub mod wup_ctrl_r;
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl_r](clk_ctrl_r) module"]
pub type CLK_CTRL_R = crate::Reg<u16, _CLK_CTRL_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CTRL_R;
#[doc = "`read()` method returns [clk_ctrl_r::R](clk_ctrl_r::R) reader structure"]
impl crate::Readable for CLK_CTRL_R {}
#[doc = "`write(|w| ..)` method takes [clk_ctrl_r::W](clk_ctrl_r::W) writer structure"]
impl crate::Writable for CLK_CTRL_R {}
#[doc = "Clock Control Register"]
pub mod clk_ctrl_r;
#[doc = "Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tout_ctrl_r](tout_ctrl_r) module"]
pub type TOUT_CTRL_R = crate::Reg<u8, _TOUT_CTRL_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOUT_CTRL_R;
#[doc = "`read()` method returns [tout_ctrl_r::R](tout_ctrl_r::R) reader structure"]
impl crate::Readable for TOUT_CTRL_R {}
#[doc = "`write(|w| ..)` method takes [tout_ctrl_r::W](tout_ctrl_r::W) writer structure"]
impl crate::Writable for TOUT_CTRL_R {}
#[doc = "Timeout Control Register"]
pub mod tout_ctrl_r;
#[doc = "Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_rst_r](sw_rst_r) module"]
pub type SW_RST_R = crate::Reg<u8, _SW_RST_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_RST_R;
#[doc = "`read()` method returns [sw_rst_r::R](sw_rst_r::R) reader structure"]
impl crate::Readable for SW_RST_R {}
#[doc = "`write(|w| ..)` method takes [sw_rst_r::W](sw_rst_r::W) writer structure"]
impl crate::Writable for SW_RST_R {}
#[doc = "Software Reset Register"]
pub mod sw_rst_r;
#[doc = "Normal Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [normal_int_stat_r](normal_int_stat_r) module"]
pub type NORMAL_INT_STAT_R = crate::Reg<u16, _NORMAL_INT_STAT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NORMAL_INT_STAT_R;
#[doc = "`read()` method returns [normal_int_stat_r::R](normal_int_stat_r::R) reader structure"]
impl crate::Readable for NORMAL_INT_STAT_R {}
#[doc = "`write(|w| ..)` method takes [normal_int_stat_r::W](normal_int_stat_r::W) writer structure"]
impl crate::Writable for NORMAL_INT_STAT_R {}
#[doc = "Normal Interrupt Status Register"]
pub mod normal_int_stat_r;
#[doc = "Error Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_int_stat_r](error_int_stat_r) module"]
pub type ERROR_INT_STAT_R = crate::Reg<u16, _ERROR_INT_STAT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERROR_INT_STAT_R;
#[doc = "`read()` method returns [error_int_stat_r::R](error_int_stat_r::R) reader structure"]
impl crate::Readable for ERROR_INT_STAT_R {}
#[doc = "`write(|w| ..)` method takes [error_int_stat_r::W](error_int_stat_r::W) writer structure"]
impl crate::Writable for ERROR_INT_STAT_R {}
#[doc = "Error Interrupt Status Register"]
pub mod error_int_stat_r;
#[doc = "Normal Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [normal_int_stat_en_r](normal_int_stat_en_r) module"]
pub type NORMAL_INT_STAT_EN_R = crate::Reg<u16, _NORMAL_INT_STAT_EN_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NORMAL_INT_STAT_EN_R;
#[doc = "`read()` method returns [normal_int_stat_en_r::R](normal_int_stat_en_r::R) reader structure"]
impl crate::Readable for NORMAL_INT_STAT_EN_R {}
#[doc = "`write(|w| ..)` method takes [normal_int_stat_en_r::W](normal_int_stat_en_r::W) writer structure"]
impl crate::Writable for NORMAL_INT_STAT_EN_R {}
#[doc = "Normal Interrupt Status Enable Register"]
pub mod normal_int_stat_en_r;
#[doc = "Error Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_int_stat_en_r](error_int_stat_en_r) module"]
pub type ERROR_INT_STAT_EN_R = crate::Reg<u16, _ERROR_INT_STAT_EN_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERROR_INT_STAT_EN_R;
#[doc = "`read()` method returns [error_int_stat_en_r::R](error_int_stat_en_r::R) reader structure"]
impl crate::Readable for ERROR_INT_STAT_EN_R {}
#[doc = "`write(|w| ..)` method takes [error_int_stat_en_r::W](error_int_stat_en_r::W) writer structure"]
impl crate::Writable for ERROR_INT_STAT_EN_R {}
#[doc = "Error Interrupt Status Enable Register"]
pub mod error_int_stat_en_r;
#[doc = "Normal Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [normal_int_signal_en_r](normal_int_signal_en_r) module"]
pub type NORMAL_INT_SIGNAL_EN_R = crate::Reg<u16, _NORMAL_INT_SIGNAL_EN_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NORMAL_INT_SIGNAL_EN_R;
#[doc = "`read()` method returns [normal_int_signal_en_r::R](normal_int_signal_en_r::R) reader structure"]
impl crate::Readable for NORMAL_INT_SIGNAL_EN_R {}
#[doc = "`write(|w| ..)` method takes [normal_int_signal_en_r::W](normal_int_signal_en_r::W) writer structure"]
impl crate::Writable for NORMAL_INT_SIGNAL_EN_R {}
#[doc = "Normal Interrupt Signal Enable Register"]
pub mod normal_int_signal_en_r;
#[doc = "Error Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_int_signal_en_r](error_int_signal_en_r) module"]
pub type ERROR_INT_SIGNAL_EN_R = crate::Reg<u16, _ERROR_INT_SIGNAL_EN_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERROR_INT_SIGNAL_EN_R;
#[doc = "`read()` method returns [error_int_signal_en_r::R](error_int_signal_en_r::R) reader structure"]
impl crate::Readable for ERROR_INT_SIGNAL_EN_R {}
#[doc = "`write(|w| ..)` method takes [error_int_signal_en_r::W](error_int_signal_en_r::W) writer structure"]
impl crate::Writable for ERROR_INT_SIGNAL_EN_R {}
#[doc = "Error Interrupt Signal Enable Register"]
pub mod error_int_signal_en_r;
#[doc = "Auto CMD Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auto_cmd_stat_r](auto_cmd_stat_r) module"]
pub type AUTO_CMD_STAT_R = crate::Reg<u16, _AUTO_CMD_STAT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTO_CMD_STAT_R;
#[doc = "`read()` method returns [auto_cmd_stat_r::R](auto_cmd_stat_r::R) reader structure"]
impl crate::Readable for AUTO_CMD_STAT_R {}
#[doc = "Auto CMD Status Register"]
pub mod auto_cmd_stat_r;
#[doc = "Host Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctrl2_r](host_ctrl2_r) module"]
pub type HOST_CTRL2_R = crate::Reg<u16, _HOST_CTRL2_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CTRL2_R;
#[doc = "`read()` method returns [host_ctrl2_r::R](host_ctrl2_r::R) reader structure"]
impl crate::Readable for HOST_CTRL2_R {}
#[doc = "`write(|w| ..)` method takes [host_ctrl2_r::W](host_ctrl2_r::W) writer structure"]
impl crate::Writable for HOST_CTRL2_R {}
#[doc = "Host Control 2 Register"]
pub mod host_ctrl2_r;
#[doc = "Capabilities 1 Register - 0 to 31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capabilities1_r](capabilities1_r) module"]
pub type CAPABILITIES1_R = crate::Reg<u32, _CAPABILITIES1_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPABILITIES1_R;
#[doc = "`read()` method returns [capabilities1_r::R](capabilities1_r::R) reader structure"]
impl crate::Readable for CAPABILITIES1_R {}
#[doc = "Capabilities 1 Register - 0 to 31"]
pub mod capabilities1_r;
#[doc = "Capabilities Register - 32 to 63\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capabilities2_r](capabilities2_r) module"]
pub type CAPABILITIES2_R = crate::Reg<u32, _CAPABILITIES2_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPABILITIES2_R;
#[doc = "`read()` method returns [capabilities2_r::R](capabilities2_r::R) reader structure"]
impl crate::Readable for CAPABILITIES2_R {}
#[doc = "Capabilities Register - 32 to 63"]
pub mod capabilities2_r;
#[doc = "Current Capabilities Register - 0 to 31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curr_capabilities1_r](curr_capabilities1_r) module"]
pub type CURR_CAPABILITIES1_R = crate::Reg<u32, _CURR_CAPABILITIES1_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURR_CAPABILITIES1_R;
#[doc = "`read()` method returns [curr_capabilities1_r::R](curr_capabilities1_r::R) reader structure"]
impl crate::Readable for CURR_CAPABILITIES1_R {}
#[doc = "Current Capabilities Register - 0 to 31"]
pub mod curr_capabilities1_r;
#[doc = "Maximum Current Capabilities Register - 32 to 63\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curr_capabilities2_r](curr_capabilities2_r) module"]
pub type CURR_CAPABILITIES2_R = crate::Reg<u32, _CURR_CAPABILITIES2_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURR_CAPABILITIES2_R;
#[doc = "`read()` method returns [curr_capabilities2_r::R](curr_capabilities2_r::R) reader structure"]
impl crate::Readable for CURR_CAPABILITIES2_R {}
#[doc = "Maximum Current Capabilities Register - 32 to 63"]
pub mod curr_capabilities2_r;
#[doc = "Force Event Register for Auto CMD Error Status register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_auto_cmd_stat_r](force_auto_cmd_stat_r) module"]
pub type FORCE_AUTO_CMD_STAT_R = crate::Reg<u16, _FORCE_AUTO_CMD_STAT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORCE_AUTO_CMD_STAT_R;
#[doc = "`write(|w| ..)` method takes [force_auto_cmd_stat_r::W](force_auto_cmd_stat_r::W) writer structure"]
impl crate::Writable for FORCE_AUTO_CMD_STAT_R {}
#[doc = "Force Event Register for Auto CMD Error Status register"]
pub mod force_auto_cmd_stat_r;
#[doc = "Force Event Register for Error Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_error_int_stat_r](force_error_int_stat_r) module"]
pub type FORCE_ERROR_INT_STAT_R = crate::Reg<u16, _FORCE_ERROR_INT_STAT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORCE_ERROR_INT_STAT_R;
#[doc = "`read()` method returns [force_error_int_stat_r::R](force_error_int_stat_r::R) reader structure"]
impl crate::Readable for FORCE_ERROR_INT_STAT_R {}
#[doc = "`write(|w| ..)` method takes [force_error_int_stat_r::W](force_error_int_stat_r::W) writer structure"]
impl crate::Writable for FORCE_ERROR_INT_STAT_R {}
#[doc = "Force Event Register for Error Interrupt Status"]
pub mod force_error_int_stat_r;
#[doc = "ADMA Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_err_stat_r](adma_err_stat_r) module"]
pub type ADMA_ERR_STAT_R = crate::Reg<u8, _ADMA_ERR_STAT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMA_ERR_STAT_R;
#[doc = "`read()` method returns [adma_err_stat_r::R](adma_err_stat_r::R) reader structure"]
impl crate::Readable for ADMA_ERR_STAT_R {}
#[doc = "ADMA Error Status Register"]
pub mod adma_err_stat_r;
#[doc = "ADMA System Address Register - Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_sa_low_r](adma_sa_low_r) module"]
pub type ADMA_SA_LOW_R = crate::Reg<u32, _ADMA_SA_LOW_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMA_SA_LOW_R;
#[doc = "`read()` method returns [adma_sa_low_r::R](adma_sa_low_r::R) reader structure"]
impl crate::Readable for ADMA_SA_LOW_R {}
#[doc = "`write(|w| ..)` method takes [adma_sa_low_r::W](adma_sa_low_r::W) writer structure"]
impl crate::Writable for ADMA_SA_LOW_R {}
#[doc = "ADMA System Address Register - Low"]
pub mod adma_sa_low_r;
#[doc = "ADMA3 Integrated Descriptor Address Register - Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_id_low_r](adma_id_low_r) module"]
pub type ADMA_ID_LOW_R = crate::Reg<u32, _ADMA_ID_LOW_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMA_ID_LOW_R;
#[doc = "`read()` method returns [adma_id_low_r::R](adma_id_low_r::R) reader structure"]
impl crate::Readable for ADMA_ID_LOW_R {}
#[doc = "`write(|w| ..)` method takes [adma_id_low_r::W](adma_id_low_r::W) writer structure"]
impl crate::Writable for ADMA_ID_LOW_R {}
#[doc = "ADMA3 Integrated Descriptor Address Register - Low"]
pub mod adma_id_low_r;
#[doc = "Host Controller Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_cntrl_vers_r](host_cntrl_vers_r) module"]
pub type HOST_CNTRL_VERS_R = crate::Reg<u16, _HOST_CNTRL_VERS_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CNTRL_VERS_R;
#[doc = "`read()` method returns [host_cntrl_vers_r::R](host_cntrl_vers_r::R) reader structure"]
impl crate::Readable for HOST_CNTRL_VERS_R {}
#[doc = "Host Controller Version"]
pub mod host_cntrl_vers_r;
#[doc = "Command Queuing Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqver](cqver) module"]
pub type CQVER = crate::Reg<u32, _CQVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQVER;
#[doc = "`read()` method returns [cqver::R](cqver::R) reader structure"]
impl crate::Readable for CQVER {}
#[doc = "Command Queuing Version register"]
pub mod cqver;
#[doc = "Command Queuing Capabilities register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcap](cqcap) module"]
pub type CQCAP = crate::Reg<u32, _CQCAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQCAP;
#[doc = "`read()` method returns [cqcap::R](cqcap::R) reader structure"]
impl crate::Readable for CQCAP {}
#[doc = "Command Queuing Capabilities register"]
pub mod cqcap;
#[doc = "Command Queuing Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcfg](cqcfg) module"]
pub type CQCFG = crate::Reg<u32, _CQCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQCFG;
#[doc = "`read()` method returns [cqcfg::R](cqcfg::R) reader structure"]
impl crate::Readable for CQCFG {}
#[doc = "`write(|w| ..)` method takes [cqcfg::W](cqcfg::W) writer structure"]
impl crate::Writable for CQCFG {}
#[doc = "Command Queuing Configuration register"]
pub mod cqcfg;
#[doc = "Command Queuing Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqctl](cqctl) module"]
pub type CQCTL = crate::Reg<u32, _CQCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQCTL;
#[doc = "`read()` method returns [cqctl::R](cqctl::R) reader structure"]
impl crate::Readable for CQCTL {}
#[doc = "`write(|w| ..)` method takes [cqctl::W](cqctl::W) writer structure"]
impl crate::Writable for CQCTL {}
#[doc = "Command Queuing Control register"]
pub mod cqctl;
#[doc = "Command Queuing Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqis](cqis) module"]
pub type CQIS = crate::Reg<u32, _CQIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQIS;
#[doc = "`read()` method returns [cqis::R](cqis::R) reader structure"]
impl crate::Readable for CQIS {}
#[doc = "`write(|w| ..)` method takes [cqis::W](cqis::W) writer structure"]
impl crate::Writable for CQIS {}
#[doc = "Command Queuing Interrupt Status register"]
pub mod cqis;
#[doc = "Command Queuing Interrupt Status Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqise](cqise) module"]
pub type CQISE = crate::Reg<u32, _CQISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQISE;
#[doc = "`read()` method returns [cqise::R](cqise::R) reader structure"]
impl crate::Readable for CQISE {}
#[doc = "`write(|w| ..)` method takes [cqise::W](cqise::W) writer structure"]
impl crate::Writable for CQISE {}
#[doc = "Command Queuing Interrupt Status Enable register"]
pub mod cqise;
#[doc = "Command Queuing Interrupt signal enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqisge](cqisge) module"]
pub type CQISGE = crate::Reg<u32, _CQISGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQISGE;
#[doc = "`read()` method returns [cqisge::R](cqisge::R) reader structure"]
impl crate::Readable for CQISGE {}
#[doc = "`write(|w| ..)` method takes [cqisge::W](cqisge::W) writer structure"]
impl crate::Writable for CQISGE {}
#[doc = "Command Queuing Interrupt signal enable register"]
pub mod cqisge;
#[doc = "Command Queuing Interrupt Coalescing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqic](cqic) module"]
pub type CQIC = crate::Reg<u32, _CQIC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQIC;
#[doc = "`read()` method returns [cqic::R](cqic::R) reader structure"]
impl crate::Readable for CQIC {}
#[doc = "`write(|w| ..)` method takes [cqic::W](cqic::W) writer structure"]
impl crate::Writable for CQIC {}
#[doc = "Command Queuing Interrupt Coalescing register"]
pub mod cqic;
#[doc = "Command Queuing Task Descriptor List Base Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqtdlba](cqtdlba) module"]
pub type CQTDLBA = crate::Reg<u32, _CQTDLBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQTDLBA;
#[doc = "`read()` method returns [cqtdlba::R](cqtdlba::R) reader structure"]
impl crate::Readable for CQTDLBA {}
#[doc = "`write(|w| ..)` method takes [cqtdlba::W](cqtdlba::W) writer structure"]
impl crate::Writable for CQTDLBA {}
#[doc = "Command Queuing Task Descriptor List Base Address register"]
pub mod cqtdlba;
#[doc = "Command Queuing DoorBell register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqtdbr](cqtdbr) module"]
pub type CQTDBR = crate::Reg<u32, _CQTDBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQTDBR;
#[doc = "`read()` method returns [cqtdbr::R](cqtdbr::R) reader structure"]
impl crate::Readable for CQTDBR {}
#[doc = "`write(|w| ..)` method takes [cqtdbr::W](cqtdbr::W) writer structure"]
impl crate::Writable for CQTDBR {}
#[doc = "Command Queuing DoorBell register"]
pub mod cqtdbr;
#[doc = "Command Queuing TaskClear Notification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqtcn](cqtcn) module"]
pub type CQTCN = crate::Reg<u32, _CQTCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQTCN;
#[doc = "`read()` method returns [cqtcn::R](cqtcn::R) reader structure"]
impl crate::Readable for CQTCN {}
#[doc = "`write(|w| ..)` method takes [cqtcn::W](cqtcn::W) writer structure"]
impl crate::Writable for CQTCN {}
#[doc = "Command Queuing TaskClear Notification register"]
pub mod cqtcn;
#[doc = "Device queue status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqdqs](cqdqs) module"]
pub type CQDQS = crate::Reg<u32, _CQDQS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQDQS;
#[doc = "`read()` method returns [cqdqs::R](cqdqs::R) reader structure"]
impl crate::Readable for CQDQS {}
#[doc = "Device queue status register"]
pub mod cqdqs;
#[doc = "Device pending tasks register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqdpt](cqdpt) module"]
pub type CQDPT = crate::Reg<u32, _CQDPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQDPT;
#[doc = "`read()` method returns [cqdpt::R](cqdpt::R) reader structure"]
impl crate::Readable for CQDPT {}
#[doc = "Device pending tasks register"]
pub mod cqdpt;
#[doc = "Command Queuing DoorBell register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqtclr](cqtclr) module"]
pub type CQTCLR = crate::Reg<u32, _CQTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQTCLR;
#[doc = "`read()` method returns [cqtclr::R](cqtclr::R) reader structure"]
impl crate::Readable for CQTCLR {}
#[doc = "`write(|w| ..)` method takes [cqtclr::W](cqtclr::W) writer structure"]
impl crate::Writable for CQTCLR {}
#[doc = "Command Queuing DoorBell register"]
pub mod cqtclr;
#[doc = "CQ Send Status Configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqssc1](cqssc1) module"]
pub type CQSSC1 = crate::Reg<u32, _CQSSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQSSC1;
#[doc = "`read()` method returns [cqssc1::R](cqssc1::R) reader structure"]
impl crate::Readable for CQSSC1 {}
#[doc = "`write(|w| ..)` method takes [cqssc1::W](cqssc1::W) writer structure"]
impl crate::Writable for CQSSC1 {}
#[doc = "CQ Send Status Configuration 1 register"]
pub mod cqssc1;
#[doc = "CQ Send Status Configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqssc2](cqssc2) module"]
pub type CQSSC2 = crate::Reg<u32, _CQSSC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQSSC2;
#[doc = "`read()` method returns [cqssc2::R](cqssc2::R) reader structure"]
impl crate::Readable for CQSSC2 {}
#[doc = "`write(|w| ..)` method takes [cqssc2::W](cqssc2::W) writer structure"]
impl crate::Writable for CQSSC2 {}
#[doc = "CQ Send Status Configuration 2 register"]
pub mod cqssc2;
#[doc = "Command response for direct command register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcrdct](cqcrdct) module"]
pub type CQCRDCT = crate::Reg<u32, _CQCRDCT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQCRDCT;
#[doc = "`read()` method returns [cqcrdct::R](cqcrdct::R) reader structure"]
impl crate::Readable for CQCRDCT {}
#[doc = "Command response for direct command register"]
pub mod cqcrdct;
#[doc = "Command response mode error mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqrmem](cqrmem) module"]
pub type CQRMEM = crate::Reg<u32, _CQRMEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQRMEM;
#[doc = "`read()` method returns [cqrmem::R](cqrmem::R) reader structure"]
impl crate::Readable for CQRMEM {}
#[doc = "`write(|w| ..)` method takes [cqrmem::W](cqrmem::W) writer structure"]
impl crate::Writable for CQRMEM {}
#[doc = "Command response mode error mask register"]
pub mod cqrmem;
#[doc = "CQ Task Error Information register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqterri](cqterri) module"]
pub type CQTERRI = crate::Reg<u32, _CQTERRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQTERRI;
#[doc = "`read()` method returns [cqterri::R](cqterri::R) reader structure"]
impl crate::Readable for CQTERRI {}
#[doc = "CQ Task Error Information register"]
pub mod cqterri;
#[doc = "CQ Command response index\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcri](cqcri) module"]
pub type CQCRI = crate::Reg<u32, _CQCRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQCRI;
#[doc = "`read()` method returns [cqcri::R](cqcri::R) reader structure"]
impl crate::Readable for CQCRI {}
#[doc = "CQ Command response index"]
pub mod cqcri;
#[doc = "CQ Command response argument register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcra](cqcra) module"]
pub type CQCRA = crate::Reg<u32, _CQCRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CQCRA;
#[doc = "`read()` method returns [cqcra::R](cqcra::R) reader structure"]
impl crate::Readable for CQCRA {}
#[doc = "CQ Command response argument register"]
pub mod cqcra;
#[doc = "MSHC version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mshc_ver_id_r](mshc_ver_id_r) module"]
pub type MSHC_VER_ID_R = crate::Reg<u32, _MSHC_VER_ID_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSHC_VER_ID_R;
#[doc = "`read()` method returns [mshc_ver_id_r::R](mshc_ver_id_r::R) reader structure"]
impl crate::Readable for MSHC_VER_ID_R {}
#[doc = "MSHC version"]
pub mod mshc_ver_id_r;
#[doc = "MSHC version type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mshc_ver_type_r](mshc_ver_type_r) module"]
pub type MSHC_VER_TYPE_R = crate::Reg<u32, _MSHC_VER_TYPE_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSHC_VER_TYPE_R;
#[doc = "`read()` method returns [mshc_ver_type_r::R](mshc_ver_type_r::R) reader structure"]
impl crate::Readable for MSHC_VER_TYPE_R {}
#[doc = "MSHC version type"]
pub mod mshc_ver_type_r;
#[doc = "MSHC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mshc_ctrl_r](mshc_ctrl_r) module"]
pub type MSHC_CTRL_R = crate::Reg<u8, _MSHC_CTRL_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSHC_CTRL_R;
#[doc = "`read()` method returns [mshc_ctrl_r::R](mshc_ctrl_r::R) reader structure"]
impl crate::Readable for MSHC_CTRL_R {}
#[doc = "`write(|w| ..)` method takes [mshc_ctrl_r::W](mshc_ctrl_r::W) writer structure"]
impl crate::Writable for MSHC_CTRL_R {}
#[doc = "MSHC Control register"]
pub mod mshc_ctrl_r;
#[doc = "MBIU Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbiu_ctrl_r](mbiu_ctrl_r) module"]
pub type MBIU_CTRL_R = crate::Reg<u8, _MBIU_CTRL_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBIU_CTRL_R;
#[doc = "`read()` method returns [mbiu_ctrl_r::R](mbiu_ctrl_r::R) reader structure"]
impl crate::Readable for MBIU_CTRL_R {}
#[doc = "`write(|w| ..)` method takes [mbiu_ctrl_r::W](mbiu_ctrl_r::W) writer structure"]
impl crate::Writable for MBIU_CTRL_R {}
#[doc = "MBIU Control register"]
pub mod mbiu_ctrl_r;
#[doc = "eMMC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_ctrl_r](emmc_ctrl_r) module"]
pub type EMMC_CTRL_R = crate::Reg<u16, _EMMC_CTRL_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMMC_CTRL_R;
#[doc = "`read()` method returns [emmc_ctrl_r::R](emmc_ctrl_r::R) reader structure"]
impl crate::Readable for EMMC_CTRL_R {}
#[doc = "`write(|w| ..)` method takes [emmc_ctrl_r::W](emmc_ctrl_r::W) writer structure"]
impl crate::Writable for EMMC_CTRL_R {}
#[doc = "eMMC Control register"]
pub mod emmc_ctrl_r;
#[doc = "eMMC Boot Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_ctrl_r](boot_ctrl_r) module"]
pub type BOOT_CTRL_R = crate::Reg<u16, _BOOT_CTRL_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOT_CTRL_R;
#[doc = "`read()` method returns [boot_ctrl_r::R](boot_ctrl_r::R) reader structure"]
impl crate::Readable for BOOT_CTRL_R {}
#[doc = "`write(|w| ..)` method takes [boot_ctrl_r::W](boot_ctrl_r::W) writer structure"]
impl crate::Writable for BOOT_CTRL_R {}
#[doc = "eMMC Boot Control register"]
pub mod boot_ctrl_r;
#[doc = "General Purpose Input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_in_r](gp_in_r) module"]
pub type GP_IN_R = crate::Reg<u32, _GP_IN_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP_IN_R;
#[doc = "`read()` method returns [gp_in_r::R](gp_in_r::R) reader structure"]
impl crate::Readable for GP_IN_R {}
#[doc = "General Purpose Input register"]
pub mod gp_in_r;
#[doc = "General Purpose Output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_out_r](gp_out_r) module"]
pub type GP_OUT_R = crate::Reg<u32, _GP_OUT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP_OUT_R;
#[doc = "`read()` method returns [gp_out_r::R](gp_out_r::R) reader structure"]
impl crate::Readable for GP_OUT_R {}
#[doc = "`write(|w| ..)` method takes [gp_out_r::W](gp_out_r::W) writer structure"]
impl crate::Writable for GP_OUT_R {}
#[doc = "General Purpose Output register"]
pub mod gp_out_r;
