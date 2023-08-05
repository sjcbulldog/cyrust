#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Programmable IO port registers"]
use core::prelude::v1::derive;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prt {
    ptr: *mut u8,
}
unsafe impl Send for Prt {}
unsafe impl Sync for Prt {}
impl Prt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Synchronization control register"]
    #[inline(always)]
    pub const fn sync_ctl(self) -> crate::common::Reg<SyncCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "LUT component input selection"]
    #[inline(always)]
    pub const fn lut_sel(self, n: usize) -> crate::common::Reg<LutSel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize + n * 4usize) as _) }
    }
    #[doc = "LUT component control register"]
    #[inline(always)]
    pub const fn lut_ctl(self, n: usize) -> crate::common::Reg<LutCtl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize + n * 4usize) as _) }
    }
    #[doc = "Data unit component input selection"]
    #[inline(always)]
    pub const fn du_sel(self) -> crate::common::Reg<DuSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "Data unit component control register"]
    #[inline(always)]
    pub const fn du_ctl(self) -> crate::common::Reg<DuCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
}
#[doc = "Programmable IO configuration"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smartio {
    ptr: *mut u8,
}
unsafe impl Send for Smartio {}
unsafe impl Sync for Smartio {}
impl Smartio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Programmable IO port registers"]
    #[inline(always)]
    pub const fn prt(self, n: usize) -> Prt {
        assert!(n < 10usize);
        unsafe { Prt::from_ptr(self.ptr.add(0usize + n * 256usize) as _) }
    }
}
#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\] is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
    #[inline(always)]
    pub const fn bypass(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\] is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and SMARTIO fabric is always bypassed. '0': No bypass (programmable SMARTIO fabric is exposed). '1': Bypass (programmable SMARTIOIO fabric is hidden)."]
    #[inline(always)]
    pub fn set_bypass(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
    #[inline(always)]
    pub const fn clock_src(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_smartio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_smartio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_smartio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_smartio' (note that 'clk_smartio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_smartio_pos_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': Same as '17'. Note that the M0S8 SMARTIO version used the Hibernate reset for this value, but the MXS40 SMARTIO version does not support Hibernate functionality. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': asynchronous mode/'1'. Select this when clockless operation is configured. NOTE: Two positive edges of the selected clock are required for the block to be enabled (to deactivate reset). In asynchronous (clockless) mode clk_sys is used to enable the block, but is not available for clocking."]
    #[inline(always)]
    pub fn set_clock_src(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\] is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\] is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
    #[inline(always)]
    pub const fn hld_ovr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "IO cell hold override functionality. In DeepSleep power mode, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the SMARTIO is supposed to deliver DeepSleep output functionality on these IO pads. This field is used to control the hold override functionality from the SMARTIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The SMARTIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\] is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\] is '0'), the SMARTIO sets hold override to 'pwr_hld_ovr_hib' to enable SMARTIO functionality in DeepSleep power mode (but disables it in Hibernate or Stop power mode)."]
    #[inline(always)]
    pub fn set_hld_ovr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
    #[inline(always)]
    pub const fn pipeline_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
    #[inline(always)]
    pub fn set_pipeline_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "Data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "Data unit input data source."]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data unit input data source."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
#[doc = "Data unit component control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DuCtl(pub u32);
impl DuCtl {
    #[doc = "Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    pub const fn du_size(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    pub fn set_du_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    pub const fn du_opc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    pub fn set_du_opc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for DuCtl {
    #[inline(always)]
    fn default() -> DuCtl {
        DuCtl(0)
    }
}
#[doc = "Data unit component input selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DuSel(pub u32);
impl DuSel {
    #[doc = "Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
    #[inline(always)]
    pub const fn du_tr0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
    #[inline(always)]
    pub fn set_du_tr0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub const fn du_tr1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn set_du_tr1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub const fn du_tr2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn set_du_tr2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
    #[inline(always)]
    pub const fn du_data0_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
    #[inline(always)]
    pub fn set_du_data0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    pub const fn du_data1_sel(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    pub fn set_du_data1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for DuSel {
    #[inline(always)]
    fn default() -> DuSel {
        DuSel(0)
    }
}
#[doc = "LUT component control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LutCtl(pub u32);
impl LutCtl {
    #[doc = "LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    pub const fn lut(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    pub fn set_lut(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    pub const fn lut_opc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    pub fn set_lut_opc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for LutCtl {
    #[inline(always)]
    fn default() -> LutCtl {
        LutCtl(0)
    }
}
#[doc = "LUT component input selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LutSel(pub u32);
impl LutSel {
    #[doc = "LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\] (for LUTs 0, 1, 2, 3); chip_data\\[4\\] (for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\] (for LUTs 0, 1, 2, 3); chip_data\\[5\\] (for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\] (for LUTs 0, 1, 2, 3); chip_data\\[6\\] (for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\] (for LUTs 0, 1, 2, 3); chip_data\\[7\\] (for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\] (for LUTs 0, 1, 2, 3); io_data_in\\[4\\] (for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\] (for LUTs 0, 1, 2, 3); io_data_in\\[5\\] (for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\] (for LUTs 0, 1, 2, 3); io_data_in\\[6\\] (for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\] (for LUTs 0, 1, 2, 3); io_data_in\\[7\\] (for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub const fn lut_tr0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\] (for LUTs 0, 1, 2, 3); chip_data\\[4\\] (for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\] (for LUTs 0, 1, 2, 3); chip_data\\[5\\] (for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\] (for LUTs 0, 1, 2, 3); chip_data\\[6\\] (for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\] (for LUTs 0, 1, 2, 3); chip_data\\[7\\] (for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\] (for LUTs 0, 1, 2, 3); io_data_in\\[4\\] (for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\] (for LUTs 0, 1, 2, 3); io_data_in\\[5\\] (for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\] (for LUTs 0, 1, 2, 3); io_data_in\\[6\\] (for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\] (for LUTs 0, 1, 2, 3); io_data_in\\[7\\] (for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn set_lut_tr0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\] (for LUTs 0, 1, 2, 3); chip_data\\[4\\] (for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\] (for LUTs 0, 1, 2, 3); chip_data\\[5\\] (for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\] (for LUTs 0, 1, 2, 3); chip_data\\[6\\] (for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\] (for LUTs 0, 1, 2, 3); chip_data\\[7\\] (for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\] (for LUTs 0, 1, 2, 3); io_data_in\\[4\\] (for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\] (for LUTs 0, 1, 2, 3); io_data_in\\[5\\] (for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\] (for LUTs 0, 1, 2, 3); io_data_in\\[6\\] (for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\] (for LUTs 0, 1, 2, 3); io_data_in\\[7\\] (for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub const fn lut_tr1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\] (for LUTs 0, 1, 2, 3); chip_data\\[4\\] (for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\] (for LUTs 0, 1, 2, 3); chip_data\\[5\\] (for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\] (for LUTs 0, 1, 2, 3); chip_data\\[6\\] (for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\] (for LUTs 0, 1, 2, 3); chip_data\\[7\\] (for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\] (for LUTs 0, 1, 2, 3); io_data_in\\[4\\] (for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\] (for LUTs 0, 1, 2, 3); io_data_in\\[5\\] (for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\] (for LUTs 0, 1, 2, 3); io_data_in\\[6\\] (for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\] (for LUTs 0, 1, 2, 3); io_data_in\\[7\\] (for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn set_lut_tr1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
    #[inline(always)]
    pub const fn lut_tr2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
    #[inline(always)]
    pub fn set_lut_tr2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for LutSel {
    #[inline(always)]
    fn default() -> LutSel {
        LutSel(0)
    }
}
#[doc = "Synchronization control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncCtl(pub u32);
impl SyncCtl {
    #[doc = "Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\] is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub const fn io_sync_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\] is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn set_io_sync_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\] is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub const fn chip_sync_en(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\] is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn set_chip_sync_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for SyncCtl {
    #[inline(always)]
    fn default() -> SyncCtl {
        SyncCtl(0)
    }
}
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
