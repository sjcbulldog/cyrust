#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "SAR ADC with Sequencer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sar {
    ptr: *mut u8,
}
unsafe impl Send for Sar {}
unsafe impl Sync for Sar {}
impl Sar {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Analog control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Sample control register."]
    #[inline(always)]
    pub const fn sample_ctrl(self) -> crate::common::Reg<SampleCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Sample time specification ST0 and ST1"]
    #[inline(always)]
    pub const fn sample_time01(self) -> crate::common::Reg<SampleTime01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Sample time specification ST2 and ST3"]
    #[inline(always)]
    pub const fn sample_time23(self) -> crate::common::Reg<SampleTime23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Global range detect threshold register."]
    #[inline(always)]
    pub const fn range_thres(self) -> crate::common::Reg<RangeThres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Global range detect mode register."]
    #[inline(always)]
    pub const fn range_cond_reg(self) -> crate::common::Reg<RangeCondReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Enable bits for the channels"]
    #[inline(always)]
    pub const fn chan_en(self) -> crate::common::Reg<ChanEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Start control register (firmware trigger)."]
    #[inline(always)]
    pub const fn start_ctrl(self) -> crate::common::Reg<StartCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Channel configuration register."]
    #[inline(always)]
    pub const fn chan_config(self, n: usize) -> crate::common::Reg<ChanConfig, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize + n * 4usize) as _) }
    }
    #[doc = "Channel working data register"]
    #[inline(always)]
    pub const fn chan_work(self, n: usize) -> crate::common::Reg<ChanWork, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
    #[doc = "Channel result data register"]
    #[inline(always)]
    pub const fn chan_result(self, n: usize) -> crate::common::Reg<ChanResult, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize + n * 4usize) as _) }
    }
    #[doc = "Channel working data register 'updated' bits"]
    #[inline(always)]
    pub const fn chan_work_updated(self) -> crate::common::Reg<ChanWorkUpdated, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Channel result data register 'updated' bits"]
    #[inline(always)]
    pub const fn chan_result_updated(
        self,
    ) -> crate::common::Reg<ChanResultUpdated, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "Channel working data register 'new value' bits"]
    #[inline(always)]
    pub const fn chan_work_newvalue(
        self,
    ) -> crate::common::Reg<ChanWorkNewvalue, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[doc = "Channel result data register 'new value' bits"]
    #[inline(always)]
    pub const fn chan_result_newvalue(
        self,
    ) -> crate::common::Reg<ChanResultNewvalue, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize) as _) }
    }
    #[doc = "Interrupt request register."]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(528usize) as _) }
    }
    #[doc = "Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(532usize) as _) }
    }
    #[doc = "Interrupt mask register."]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(536usize) as _) }
    }
    #[doc = "Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(540usize) as _) }
    }
    #[doc = "Saturate interrupt request register."]
    #[inline(always)]
    pub const fn saturate_intr(self) -> crate::common::Reg<SaturateIntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize) as _) }
    }
    #[doc = "Saturate interrupt set request register"]
    #[inline(always)]
    pub const fn saturate_intr_set(self) -> crate::common::Reg<SaturateIntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(548usize) as _) }
    }
    #[doc = "Saturate interrupt mask register."]
    #[inline(always)]
    pub const fn saturate_intr_mask(
        self,
    ) -> crate::common::Reg<SaturateIntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(552usize) as _) }
    }
    #[doc = "Saturate interrupt masked request register"]
    #[inline(always)]
    pub const fn saturate_intr_masked(
        self,
    ) -> crate::common::Reg<SaturateIntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(556usize) as _) }
    }
    #[doc = "Range detect interrupt request register."]
    #[inline(always)]
    pub const fn range_intr(self) -> crate::common::Reg<RangeIntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(560usize) as _) }
    }
    #[doc = "Range detect interrupt set request register"]
    #[inline(always)]
    pub const fn range_intr_set(self) -> crate::common::Reg<RangeIntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(564usize) as _) }
    }
    #[doc = "Range detect interrupt mask register."]
    #[inline(always)]
    pub const fn range_intr_mask(self) -> crate::common::Reg<RangeIntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(568usize) as _) }
    }
    #[doc = "Range interrupt masked request register"]
    #[inline(always)]
    pub const fn range_intr_masked(self) -> crate::common::Reg<RangeIntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(572usize) as _) }
    }
    #[doc = "Interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(self) -> crate::common::Reg<IntrCause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(576usize) as _) }
    }
    #[doc = "Injection channel configuration register."]
    #[inline(always)]
    pub const fn inj_chan_config(self) -> crate::common::Reg<InjChanConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(640usize) as _) }
    }
    #[doc = "Injection channel result register"]
    #[inline(always)]
    pub const fn inj_result(self) -> crate::common::Reg<InjResult, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(656usize) as _) }
    }
    #[doc = "Current status of internal SAR registers (mostly for debug)"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(672usize) as _) }
    }
    #[doc = "Current averaging status (for debug)"]
    #[inline(always)]
    pub const fn avg_stat(self) -> crate::common::Reg<AvgStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(676usize) as _) }
    }
    #[doc = "SARMUX Firmware switch controls"]
    #[inline(always)]
    pub const fn mux_switch0(self) -> crate::common::Reg<MuxSwitch0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize) as _) }
    }
    #[doc = "SARMUX Firmware switch control clear"]
    #[inline(always)]
    pub const fn mux_switch_clear0(self) -> crate::common::Reg<MuxSwitchClear0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(772usize) as _) }
    }
    #[doc = "SARMUX switch DSI control"]
    #[inline(always)]
    pub const fn mux_switch_ds_ctrl(
        self,
    ) -> crate::common::Reg<MuxSwitchDsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(832usize) as _) }
    }
    #[doc = "SARMUX switch Sar Sequencer control"]
    #[inline(always)]
    pub const fn mux_switch_sq_ctrl(
        self,
    ) -> crate::common::Reg<MuxSwitchSqCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(836usize) as _) }
    }
    #[doc = "SARMUX switch status"]
    #[inline(always)]
    pub const fn mux_switch_status(self) -> crate::common::Reg<MuxSwitchStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(840usize) as _) }
    }
    #[doc = "Analog trim register."]
    #[inline(always)]
    pub const fn ana_trim0(self) -> crate::common::Reg<AnaTrim0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3840usize) as _) }
    }
    #[doc = "Analog trim register."]
    #[inline(always)]
    pub const fn ana_trim1(self) -> crate::common::Reg<AnaTrim1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3844usize) as _) }
    }
}
#[doc = "Analog trim register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnaTrim0(pub u32);
impl AnaTrim0 {
    #[doc = "Attenuation cap trimming"]
    #[inline(always)]
    pub const fn cap_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Attenuation cap trimming"]
    #[inline(always)]
    pub fn set_cap_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Attenuation cap trimming"]
    #[inline(always)]
    pub const fn trimunit(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Attenuation cap trimming"]
    #[inline(always)]
    pub fn set_trimunit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for AnaTrim0 {
    #[inline(always)]
    fn default() -> AnaTrim0 {
        AnaTrim0(0)
    }
}
#[doc = "Analog trim register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnaTrim1(pub u32);
impl AnaTrim1 {
    #[doc = "SAR Reference buffer trim"]
    #[inline(always)]
    pub const fn sar_ref_buf_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "SAR Reference buffer trim"]
    #[inline(always)]
    pub fn set_sar_ref_buf_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for AnaTrim1 {
    #[inline(always)]
    fn default() -> AnaTrim1 {
        AnaTrim1(0)
    }
}
#[doc = "Current averaging status (for debug)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AvgStat(pub u32);
impl AvgStat {
    #[doc = "the current value of the averaging accumulator"]
    #[inline(always)]
    pub const fn cur_avg_accu(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "the current value of the averaging accumulator"]
    #[inline(always)]
    pub fn set_cur_avg_accu(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "If high then the SAR is in the middle of Interleaved averaging spanning several scans. While this bit is high the Firmware should not make any changes to the configuration registers otherwise some results may be incorrect. Note that the CUR_AVG_CNT status register below gives an indication how many more scans need to be done to complete the Interleaved averaging. This bit can be cleared by changing the averaging mode to ACCUNDUMP or by disabling the SAR."]
    #[inline(always)]
    pub const fn intrlv_busy(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "If high then the SAR is in the middle of Interleaved averaging spanning several scans. While this bit is high the Firmware should not make any changes to the configuration registers otherwise some results may be incorrect. Note that the CUR_AVG_CNT status register below gives an indication how many more scans need to be done to complete the Interleaved averaging. This bit can be cleared by changing the averaging mode to ACCUNDUMP or by disabling the SAR."]
    #[inline(always)]
    pub fn set_intrlv_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "the current value of the averaging counter. Note that the value shown is updated after the sampling time and therefore runs ahead of the accumulator update."]
    #[inline(always)]
    pub const fn cur_avg_cnt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "the current value of the averaging counter. Note that the value shown is updated after the sampling time and therefore runs ahead of the accumulator update."]
    #[inline(always)]
    pub fn set_cur_avg_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for AvgStat {
    #[inline(always)]
    fn default() -> AvgStat {
        AvgStat(0)
    }
}
#[doc = "Channel configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanConfig(pub u32);
impl ChanConfig {
    #[doc = "Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub const fn pos_pin_addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn set_pos_pin_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub const fn pos_port_addr(&self) -> PosPortAddr {
        let val = (self.0 >> 4usize) & 0x07;
        PosPortAddr::from_bits(val as u8)
    }
    #[doc = "Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn set_pos_port_addr(&mut self, val: PosPortAddr) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub const fn differential_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub fn set_differential_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub const fn avg_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn set_avg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub const fn sample_time_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn set_sample_time_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    pub const fn neg_pin_addr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    pub fn set_neg_pin_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub const fn neg_port_addr(&self) -> NegPortAddr {
        let val = (self.0 >> 20usize) & 0x07;
        NegPortAddr::from_bits(val as u8)
    }
    #[doc = "Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn set_neg_port_addr(&mut self, val: NegPortAddr) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    pub const fn neg_addr_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    pub fn set_neg_addr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub const fn dsi_out_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn set_dsi_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChanConfig {
    #[inline(always)]
    fn default() -> ChanConfig {
        ChanConfig(0)
    }
}
#[doc = "Enable bits for the channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanEn(pub u32);
impl ChanEn {
    #[doc = "Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub const fn chan_en(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub fn set_chan_en(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChanEn {
    #[inline(always)]
    fn default() -> ChanEn {
        ChanEn(0)
    }
}
#[doc = "Channel result data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanResult(pub u32);
impl ChanResult {
    #[doc = "SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub const fn result(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub fn set_result(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_RESULT_NEWVALUE register"]
    #[inline(always)]
    pub const fn chan_result_newvalue_mir(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_RESULT_NEWVALUE register"]
    #[inline(always)]
    pub fn set_chan_result_newvalue_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub const fn saturate_intr_mir(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub fn set_saturate_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_RANGE_INTR register"]
    #[inline(always)]
    pub const fn range_intr_mir(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_RANGE_INTR register"]
    #[inline(always)]
    pub fn set_range_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_RESULT_UPDATED register"]
    #[inline(always)]
    pub const fn chan_result_updated_mir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_RESULT_UPDATED register"]
    #[inline(always)]
    pub fn set_chan_result_updated_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChanResult {
    #[inline(always)]
    fn default() -> ChanResult {
        ChanResult(0)
    }
}
#[doc = "Channel result data register 'new value' bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanResultNewvalue(pub u32);
impl ChanResultNewvalue {
    #[doc = "If set the corresponding RESULT data received a new value, i.e. was sampled during the last scan and data was valid. In case of a UAB this New Value bit reflects the value of UAB.valid output, for anything else the data is always valid. In case of averaging this New Value bit is an OR of all the valid bits received by each conversion."]
    #[inline(always)]
    pub const fn chan_result_newvalue(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If set the corresponding RESULT data received a new value, i.e. was sampled during the last scan and data was valid. In case of a UAB this New Value bit reflects the value of UAB.valid output, for anything else the data is always valid. In case of averaging this New Value bit is an OR of all the valid bits received by each conversion."]
    #[inline(always)]
    pub fn set_chan_result_newvalue(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChanResultNewvalue {
    #[inline(always)]
    fn default() -> ChanResultNewvalue {
        ChanResultNewvalue(0)
    }
}
#[doc = "Channel result data register 'updated' bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanResultUpdated(pub u32);
impl ChanResultUpdated {
    #[doc = "If set the corresponding RESULT register was updated, i.e. was sampled during the previous scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub const fn chan_result_updated(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If set the corresponding RESULT register was updated, i.e. was sampled during the previous scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub fn set_chan_result_updated(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChanResultUpdated {
    #[inline(always)]
    fn default() -> ChanResultUpdated {
        ChanResultUpdated(0)
    }
}
#[doc = "Channel working data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanWork(pub u32);
impl ChanWork {
    #[doc = "SAR conversion working data of the channel. The data is written here right after sampling this channel."]
    #[inline(always)]
    pub const fn work(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SAR conversion working data of the channel. The data is written here right after sampling this channel."]
    #[inline(always)]
    pub fn set_work(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_WORK_NEWVALUE register"]
    #[inline(always)]
    pub const fn chan_work_newvalue_mir(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_WORK_NEWVALUE register"]
    #[inline(always)]
    pub fn set_chan_work_newvalue_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_WORK_UPDATED register"]
    #[inline(always)]
    pub const fn chan_work_updated_mir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_WORK_UPDATED register"]
    #[inline(always)]
    pub fn set_chan_work_updated_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChanWork {
    #[inline(always)]
    fn default() -> ChanWork {
        ChanWork(0)
    }
}
#[doc = "Channel working data register 'new value' bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanWorkNewvalue(pub u32);
impl ChanWorkNewvalue {
    #[doc = "If set the corresponding WORK data received a new value, i.e. was already sampled during the current scan and data was valid. In case of a UAB this New Value bit reflects the value of UAB.valid output, for anything else the data is always valid. In case of averaging this New Value bit is an OR of all the valid bits received by each conversion."]
    #[inline(always)]
    pub const fn chan_work_newvalue(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If set the corresponding WORK data received a new value, i.e. was already sampled during the current scan and data was valid. In case of a UAB this New Value bit reflects the value of UAB.valid output, for anything else the data is always valid. In case of averaging this New Value bit is an OR of all the valid bits received by each conversion."]
    #[inline(always)]
    pub fn set_chan_work_newvalue(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChanWorkNewvalue {
    #[inline(always)]
    fn default() -> ChanWorkNewvalue {
        ChanWorkNewvalue(0)
    }
}
#[doc = "Channel working data register 'updated' bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanWorkUpdated(pub u32);
impl ChanWorkUpdated {
    #[doc = "If set the corresponding WORK register was updated, i.e. was already sampled during the current scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub const fn chan_work_updated(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If set the corresponding WORK register was updated, i.e. was already sampled during the current scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub fn set_chan_work_updated(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChanWorkUpdated {
    #[inline(always)]
    fn default() -> ChanWorkUpdated {
        ChanWorkUpdated(0)
    }
}
#[doc = "Analog control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "VREF buffer low power mode."]
    #[inline(always)]
    pub const fn pwr_ctrl_vref(&self) -> PwrCtrlVref {
        let val = (self.0 >> 0usize) & 0x07;
        PwrCtrlVref::from_bits(val as u8)
    }
    #[doc = "VREF buffer low power mode."]
    #[inline(always)]
    pub fn set_pwr_ctrl_vref(&mut self, val: PwrCtrlVref) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "SARADC internal VREF selection."]
    #[inline(always)]
    pub const fn vref_sel(&self) -> VrefSel {
        let val = (self.0 >> 4usize) & 0x07;
        VrefSel::from_bits(val as u8)
    }
    #[doc = "SARADC internal VREF selection."]
    #[inline(always)]
    pub fn set_vref_sel(&mut self, val: VrefSel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    pub const fn vref_byp_cap_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    pub fn set_vref_byp_cap_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    pub const fn neg_sel(&self) -> NegSel {
        let val = (self.0 >> 9usize) & 0x07;
        NegSel::from_bits(val as u8)
    }
    #[doc = "SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    pub fn set_neg_sel(&mut self, val: NegSel) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    pub const fn sar_hw_ctrl_negvref(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    pub fn set_sar_hw_ctrl_negvref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Set the comparator latch delay in accordance with SAR conversion rate"]
    #[inline(always)]
    pub const fn comp_dly(&self) -> CompDly {
        let val = (self.0 >> 14usize) & 0x03;
        CompDly::from_bits(val as u8)
    }
    #[doc = "Set the comparator latch delay in accordance with SAR conversion rate"]
    #[inline(always)]
    pub fn set_comp_dly(&mut self, val: CompDly) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    pub const fn spare(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    pub fn set_spare(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "deprecated"]
    #[inline(always)]
    pub const fn boostpump_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "deprecated"]
    #[inline(always)]
    pub fn set_boostpump_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
    #[inline(always)]
    pub const fn refbuf_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
    #[inline(always)]
    pub fn set_refbuf_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Comparator power mode."]
    #[inline(always)]
    pub const fn comp_pwr(&self) -> CompPwr {
        let val = (self.0 >> 24usize) & 0x07;
        CompPwr::from_bits(val as u8)
    }
    #[doc = "Comparator power mode."]
    #[inline(always)]
    pub fn set_comp_pwr(&mut self, val: CompPwr) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "- 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub const fn deepsleep_on(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn set_deepsleep_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "- 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    pub const fn dsi_sync_config(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    pub fn set_dsi_sync_config(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    pub const fn dsi_mode(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    pub fn set_dsi_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    pub const fn switch_disable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    pub fn set_switch_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "- 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Injection channel configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InjChanConfig(pub u32);
impl InjChanConfig {
    #[doc = "Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub const fn inj_pin_addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub fn set_inj_pin_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub const fn inj_port_addr(&self) -> InjPortAddr {
        let val = (self.0 >> 4usize) & 0x07;
        InjPortAddr::from_bits(val as u8)
    }
    #[doc = "Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn set_inj_port_addr(&mut self, val: InjPortAddr) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub const fn inj_differential_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub fn set_inj_differential_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub const fn inj_avg_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn set_inj_avg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub const fn inj_sample_time_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn set_inj_sample_time_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub const fn inj_tailgating(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub fn set_inj_tailgating(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub const fn inj_start_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub fn set_inj_start_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for InjChanConfig {
    #[inline(always)]
    fn default() -> InjChanConfig {
        InjChanConfig(0)
    }
}
#[doc = "Injection channel result register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InjResult(pub u32);
impl InjResult {
    #[doc = "SAR conversion result of the channel."]
    #[inline(always)]
    pub const fn inj_result(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SAR conversion result of the channel."]
    #[inline(always)]
    pub fn set_inj_result(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The data in this register received a new value (only relevant for UAB, this bit shows the value of the UAB valid bit)"]
    #[inline(always)]
    pub const fn inj_newvalue(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "The data in this register received a new value (only relevant for UAB, this bit shows the value of the UAB valid bit)"]
    #[inline(always)]
    pub fn set_inj_newvalue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub const fn inj_collision_intr_mir(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn set_inj_collision_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub const fn inj_saturate_intr_mir(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn set_inj_saturate_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub const fn inj_range_intr_mir(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn set_inj_range_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub const fn inj_eoc_intr_mir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn set_inj_eoc_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for InjResult {
    #[inline(always)]
    fn default() -> InjResult {
        InjResult(0)
    }
}
#[doc = "Interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn eos_intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_eos_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn overflow_intr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_overflow_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn fw_collision_intr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_fw_collision_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn dsi_collision_intr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_dsi_collision_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn inj_eoc_intr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_inj_eoc_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn inj_saturate_intr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_inj_saturate_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn inj_range_intr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_inj_range_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 && INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn inj_collision_intr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 && INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_inj_collision_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause(pub u32);
impl IntrCause {
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn eos_masked_mir(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_eos_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn overflow_masked_mir(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_overflow_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn fw_collision_masked_mir(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_fw_collision_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn dsi_collision_masked_mir(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_dsi_collision_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn inj_eoc_masked_mir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_inj_eoc_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn inj_saturate_masked_mir(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_inj_saturate_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn inj_range_masked_mir(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_inj_range_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn inj_collision_masked_mir(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_inj_collision_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
    #[inline(always)]
    pub const fn saturate_masked_red(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
    #[inline(always)]
    pub fn set_saturate_masked_red(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
    #[inline(always)]
    pub const fn range_masked_red(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
    #[inline(always)]
    pub fn set_range_masked_red(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IntrCause {
    #[inline(always)]
    fn default() -> IntrCause {
        IntrCause(0)
    }
}
#[doc = "Interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn eos_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_eos_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn overflow_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_overflow_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn fw_collision_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_fw_collision_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn dsi_collision_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_dsi_collision_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_eoc_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_eoc_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_saturate_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_saturate_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_range_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_range_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_collision_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_collision_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn eos_masked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_eos_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn overflow_masked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_overflow_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn fw_collision_masked(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_fw_collision_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn dsi_collision_masked(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_dsi_collision_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn inj_eoc_masked(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_inj_eoc_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn inj_saturate_masked(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_inj_saturate_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn inj_range_masked(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_inj_range_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn inj_collision_masked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_inj_collision_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn eos_set(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_eos_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn overflow_set(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_overflow_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn fw_collision_set(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_fw_collision_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn dsi_collision_set(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_dsi_collision_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_eoc_set(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_eoc_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_saturate_set(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_saturate_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_range_set(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_range_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_collision_set(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_collision_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "SARMUX Firmware switch controls"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitch0(pub u32);
impl MuxSwitch0 {
    #[doc = "Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p0_vplus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p1_vplus(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p2_vplus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p2_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p3_vplus(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p3_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p4_vplus(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p4_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p5_vplus(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p5_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p6_vplus(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p6_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p7_vplus(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p7_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p0_vminus(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p1_vminus(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p2_vminus(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p2_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p3_vminus(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p3_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p4_vminus(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p4_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p5_vminus(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p5_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p6_vminus(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p6_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p7_vminus(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p7_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_vssa_vminus(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_vssa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_temp_vplus(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_temp_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vplus(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vplus(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vminus(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vminus(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vplus(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vplus(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vminus(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vminus(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p4_coreio0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p4_coreio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p5_coreio1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p5_coreio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p6_coreio2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p6_coreio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p7_coreio3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p7_coreio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for MuxSwitch0 {
    #[inline(always)]
    fn default() -> MuxSwitch0 {
        MuxSwitch0(0)
    }
}
#[doc = "SARMUX Firmware switch control clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitchClear0(pub u32);
impl MuxSwitchClear0 {
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p0_vplus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p1_vplus(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p2_vplus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p2_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p3_vplus(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p3_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_vplus(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_vplus(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_vplus(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_vplus(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p0_vminus(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p1_vminus(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p2_vminus(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p2_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p3_vminus(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p3_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_vminus(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_vminus(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_vminus(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_vminus(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_vssa_vminus(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_vssa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_temp_vplus(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_temp_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vplus(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vplus(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vminus(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vminus(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vplus(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vplus(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vminus(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vminus(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_coreio0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_coreio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_coreio1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_coreio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_coreio2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_coreio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_coreio3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_coreio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for MuxSwitchClear0 {
    #[inline(always)]
    fn default() -> MuxSwitchClear0 {
        MuxSwitchClear0(0)
    }
}
#[doc = "SARMUX switch DSI control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitchDsCtrl(pub u32);
impl MuxSwitchDsCtrl {
    #[doc = "for P0 switches"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_p0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "for P0 switches"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "for P1 switches"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_p1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "for P1 switches"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "for P2 switches"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_p2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "for P2 switches"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_p2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for P3 switches"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_p3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for P3 switches"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_p3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "for P4 switches"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_p4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "for P4 switches"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_p4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "for P5 switches"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_p5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "for P5 switches"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "for P6 switches"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_p6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "for P6 switches"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_p6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "for P7 switches"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_p7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "for P7 switches"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_p7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "for vssa switch"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_vssa(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "for vssa switch"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_vssa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "for temp switch"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_temp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "for temp switch"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "for amuxbusa switch"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_amuxbusa(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "for amuxbusa switch"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_amuxbusa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "for amuxbusb switches"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_amuxbusb(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "for amuxbusb switches"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_amuxbusb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "for sarbus0 switch"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_sarbus0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "for sarbus0 switch"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_sarbus0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "for sarbus1 switch"]
    #[inline(always)]
    pub const fn mux_ds_ctrl_sarbus1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "for sarbus1 switch"]
    #[inline(always)]
    pub fn set_mux_ds_ctrl_sarbus1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for MuxSwitchDsCtrl {
    #[inline(always)]
    fn default() -> MuxSwitchDsCtrl {
        MuxSwitchDsCtrl(0)
    }
}
#[doc = "SARMUX switch Sar Sequencer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitchSqCtrl(pub u32);
impl MuxSwitchSqCtrl {
    #[doc = "for P0 switches"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_p0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "for P0 switches"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "for P1 switches"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_p1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "for P1 switches"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "for P2 switches"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_p2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "for P2 switches"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_p2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for P3 switches"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_p3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for P3 switches"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_p3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "for P4 switches"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_p4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "for P4 switches"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_p4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "for P5 switches"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_p5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "for P5 switches"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "for P6 switches"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_p6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "for P6 switches"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_p6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "for P7 switches"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_p7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "for P7 switches"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_p7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "for vssa switch"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_vssa(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "for vssa switch"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_vssa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "for temp switch"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_temp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "for temp switch"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "for amuxbusa switch"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_amuxbusa(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "for amuxbusa switch"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_amuxbusa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "for amuxbusb switches"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_amuxbusb(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "for amuxbusb switches"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_amuxbusb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "for sarbus0 switch"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_sarbus0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "for sarbus0 switch"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_sarbus0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "for sarbus1 switch"]
    #[inline(always)]
    pub const fn mux_sq_ctrl_sarbus1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "for sarbus1 switch"]
    #[inline(always)]
    pub fn set_mux_sq_ctrl_sarbus1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for MuxSwitchSqCtrl {
    #[inline(always)]
    fn default() -> MuxSwitchSqCtrl {
        MuxSwitchSqCtrl(0)
    }
}
#[doc = "SARMUX switch status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitchStatus(pub u32);
impl MuxSwitchStatus {
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p0_vplus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p1_vplus(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p2_vplus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p2_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p3_vplus(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p3_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_vplus(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_vplus(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_vplus(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_vplus(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p0_vminus(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p1_vminus(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p2_vminus(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p2_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p3_vminus(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p3_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_vminus(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_vminus(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_vminus(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_vminus(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_vssa_vminus(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_vssa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_temp_vplus(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_temp_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vplus(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vplus(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vminus(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vminus(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vplus(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vplus(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vminus(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vminus(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for MuxSwitchStatus {
    #[inline(always)]
    fn default() -> MuxSwitchStatus {
        MuxSwitchStatus(0)
    }
}
#[doc = "Global range detect mode register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeCondReg(pub u32);
impl RangeCondReg {
    #[doc = "Range condition select."]
    #[inline(always)]
    pub const fn range_cond(&self) -> RangeCond {
        let val = (self.0 >> 30usize) & 0x03;
        RangeCond::from_bits(val as u8)
    }
    #[doc = "Range condition select."]
    #[inline(always)]
    pub fn set_range_cond(&mut self, val: RangeCond) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for RangeCondReg {
    #[inline(always)]
    fn default() -> RangeCondReg {
        RangeCondReg(0)
    }
}
#[doc = "Range detect interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeIntr(pub u32);
impl RangeIntr {
    #[doc = "Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn range_intr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_range_intr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RangeIntr {
    #[inline(always)]
    fn default() -> RangeIntr {
        RangeIntr(0)
    }
}
#[doc = "Range detect interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeIntrMask(pub u32);
impl RangeIntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn range_mask(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_range_mask(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RangeIntrMask {
    #[inline(always)]
    fn default() -> RangeIntrMask {
        RangeIntrMask(0)
    }
}
#[doc = "Range interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeIntrMasked(pub u32);
impl RangeIntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn range_masked(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_range_masked(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RangeIntrMasked {
    #[inline(always)]
    fn default() -> RangeIntrMasked {
        RangeIntrMasked(0)
    }
}
#[doc = "Range detect interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeIntrSet(pub u32);
impl RangeIntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn range_set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_range_set(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RangeIntrSet {
    #[inline(always)]
    fn default() -> RangeIntrSet {
        RangeIntrSet(0)
    }
}
#[doc = "Global range detect threshold register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeThres(pub u32);
impl RangeThres {
    #[doc = "Low threshold for range detect."]
    #[inline(always)]
    pub const fn range_low(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Low threshold for range detect."]
    #[inline(always)]
    pub fn set_range_low(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "High threshold for range detect."]
    #[inline(always)]
    pub const fn range_high(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "High threshold for range detect."]
    #[inline(always)]
    pub fn set_range_high(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for RangeThres {
    #[inline(always)]
    fn default() -> RangeThres {
        RangeThres(0)
    }
}
#[doc = "Sample control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SampleCtrl(pub u32);
impl SampleCtrl {
    #[doc = "Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    pub const fn left_align(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    pub fn set_left_align(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
    #[inline(always)]
    pub const fn single_ended_signed(&self) -> SingleEndedSigned {
        let val = (self.0 >> 2usize) & 0x01;
        SingleEndedSigned::from_bits(val as u8)
    }
    #[doc = "Output data from a single ended conversion as a signed value If AVG_MODE = 1 (Interleaved averaging), then SINGLE_ENDED_SIGNED must be configured identically to DIFFERENTIAL_SIGNED."]
    #[inline(always)]
    pub fn set_single_ended_signed(&mut self, val: SingleEndedSigned) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
    #[inline(always)]
    pub const fn differential_signed(&self) -> DifferentialSigned {
        let val = (self.0 >> 3usize) & 0x01;
        DifferentialSigned::from_bits(val as u8)
    }
    #[doc = "Output data from a differential conversion as a signed value when DIFFERENTIAL_EN or NEG_ADDR_EN is set to 1 If AVG_MODE = 1 (Interleaved averaging), then DIFFERENTIAL_SIGNED must be configured identically to SINGLE_ENDED_SIGNED."]
    #[inline(always)]
    pub fn set_differential_signed(&mut self, val: DifferentialSigned) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1<<(AVG_CNT+1)) = \\[2..256\\] times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=<3)."]
    #[inline(always)]
    pub const fn avg_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Averaging Count for channels that have averaging enabled (AVG_EN). A channel will be sampled (1<<(AVG_CNT+1)) = \\[2..256\\] times. - In ACCUNDUMP mode (1st order accumulate and dump filter) a channel will be sampled back to back, the average result is calculated and stored and then the next enabled channel is sampled. If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3). - In INTERLEAVED mode one sample is taken per triggered scan, only in the scan where the final averaging count is reached a valid average is calculated and stored in the RESULT register (by definition the same scan for all the channels that have averaging enabled). In all other scans the RESULT register for averaged channels will have an invalid result and the intermediate accumulated value is stored in the 16-bit WORK register. In this mode make sure that the averaging count is low enough to ensure that the intermediate value does not exceed 16-bits otherwise the MSBs will be lost. So for a 12-bit resolution the averaging count should be set to 16 or less (AVG_CNT=<3)."]
    #[inline(always)]
    pub fn set_avg_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
    #[inline(always)]
    pub const fn avg_shift(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Averaging shifting: after averaging the result is shifted right to fit in 12 bits."]
    #[inline(always)]
    pub fn set_avg_shift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
    #[inline(always)]
    pub const fn avg_mode(&self) -> AvgMode {
        let val = (self.0 >> 8usize) & 0x01;
        AvgMode::from_bits(val as u8)
    }
    #[doc = "Averaging mode, in DSI mode this bit is ignored and only AccuNDump mode is available."]
    #[inline(always)]
    pub fn set_avg_mode(&mut self, val: AvgMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "- 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    pub const fn continuous(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: Wait for next FW_TRIGGER (one shot) or hardware trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    pub fn set_continuous(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "- 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    pub const fn dsi_trigger_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: firmware trigger only: disable hardware trigger tr_sar_in. - 1: enable hardware trigger tr_sar_in (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    pub fn set_dsi_trigger_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "- 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    pub const fn dsi_trigger_level(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: trigger signal is a pulse input, a positive edge detected on the trigger signal triggers a new scan. - 1: trigger signal is a level input, as long as the trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    pub fn set_dsi_trigger_level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "- 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub const fn dsi_sync_trigger(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: bypass clock domain synchronization of the trigger signal. - 1: synchronize the trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub fn set_dsi_sync_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
    #[inline(always)]
    pub const fn uab_scan_mode(&self) -> UabScanMode {
        let val = (self.0 >> 22usize) & 0x01;
        UabScanMode::from_bits(val as u8)
    }
    #[doc = "Select whether UABs are scheduled or unscheduled. When no UAB is scanned this selection is ignored."]
    #[inline(always)]
    pub fn set_uab_scan_mode(&mut self, val: UabScanMode) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
    #[inline(always)]
    pub const fn repeat_invalid(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "For unscheduled UAB_SCAN_MODE only, do the following if an invalid sample is received: - 0: use the last known valid sample for that channel and clear the NEWVALUE flag - 1: repeat the conversions until a valid sample is received (caveat: could be never if the UAB valid window is incorrectly schedule w.r.t. SAR sampling)"]
    #[inline(always)]
    pub fn set_repeat_invalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
    #[inline(always)]
    pub const fn valid_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Static UAB Valid select 0=UAB0 half 0 Valid output 1=UAB0 half 1 Valid output 2=UAB1 half 0 Valid output 3=UAB1 half 1 Valid output 4=UAB2 half 0 Valid output 5=UAB2 half 1 Valid output 6=UAB3 half 0 Valid output 7=UAB3 half 1 Valid output"]
    #[inline(always)]
    pub fn set_valid_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Enable static UAB Valid selection (override Hardware)"]
    #[inline(always)]
    pub const fn valid_sel_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable static UAB Valid selection (override Hardware)"]
    #[inline(always)]
    pub fn set_valid_sel_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
    #[inline(always)]
    pub const fn valid_ignore(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore UAB valid signal, including the dynamic/Hardware from AROUTE and the static Valid selection from the VALID_SEL fields above"]
    #[inline(always)]
    pub fn set_valid_ignore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
    #[inline(always)]
    pub const fn trigger_out_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SAR output trigger enable (used for UAB synchronization). To ensure multiple UABs starting at the same trigger it is recommended to use this bit to temporarily disable the trigger output until all those UABs are set to run (UAB.SRAM_CTRL.RUN=1)."]
    #[inline(always)]
    pub fn set_trigger_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
    #[inline(always)]
    pub const fn eos_dsi_out_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a trigger pulse is send on the tr_sar_out signal."]
    #[inline(always)]
    pub fn set_eos_dsi_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SampleCtrl {
    #[inline(always)]
    fn default() -> SampleCtrl {
        SampleCtrl(0)
    }
}
#[doc = "Sample time specification ST0 and ST1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SampleTime01(pub u32);
impl SampleTime01 {
    #[doc = "Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub const fn sample_time0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub fn set_sample_time0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Sample time1"]
    #[inline(always)]
    pub const fn sample_time1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sample time1"]
    #[inline(always)]
    pub fn set_sample_time1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for SampleTime01 {
    #[inline(always)]
    fn default() -> SampleTime01 {
        SampleTime01(0)
    }
}
#[doc = "Sample time specification ST2 and ST3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SampleTime23(pub u32);
impl SampleTime23 {
    #[doc = "Sample time2"]
    #[inline(always)]
    pub const fn sample_time2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sample time2"]
    #[inline(always)]
    pub fn set_sample_time2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Sample time3"]
    #[inline(always)]
    pub const fn sample_time3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sample time3"]
    #[inline(always)]
    pub fn set_sample_time3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for SampleTime23 {
    #[inline(always)]
    fn default() -> SampleTime23 {
        SampleTime23(0)
    }
}
#[doc = "Saturate interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaturateIntr(pub u32);
impl SaturateIntr {
    #[doc = "Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn saturate_intr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_saturate_intr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SaturateIntr {
    #[inline(always)]
    fn default() -> SaturateIntr {
        SaturateIntr(0)
    }
}
#[doc = "Saturate interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaturateIntrMask(pub u32);
impl SaturateIntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn saturate_mask(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_saturate_mask(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SaturateIntrMask {
    #[inline(always)]
    fn default() -> SaturateIntrMask {
        SaturateIntrMask(0)
    }
}
#[doc = "Saturate interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaturateIntrMasked(pub u32);
impl SaturateIntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn saturate_masked(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_saturate_masked(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SaturateIntrMasked {
    #[inline(always)]
    fn default() -> SaturateIntrMasked {
        SaturateIntrMasked(0)
    }
}
#[doc = "Saturate interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaturateIntrSet(pub u32);
impl SaturateIntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn saturate_set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_saturate_set(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SaturateIntrSet {
    #[inline(always)]
    fn default() -> SaturateIntrSet {
        SaturateIntrSet(0)
    }
}
#[doc = "Start control register (firmware trigger)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StartCtrl(pub u32);
impl StartCtrl {
    #[doc = "When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub const fn fw_trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub fn set_fw_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for StartCtrl {
    #[inline(always)]
    fn default() -> StartCtrl {
        StartCtrl(0)
    }
}
#[doc = "Current status of internal SAR registers (mostly for debug)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "current channel being sampled (channel 16 indicates the injection channel), only valid if BUSY."]
    #[inline(always)]
    pub const fn cur_chan(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "current channel being sampled (channel 16 indicates the injection channel), only valid if BUSY."]
    #[inline(always)]
    pub fn set_cur_chan(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "the current switch status, including DSI and sequencer controls, of the switch in the SARADC that shorts NEG with VREF input (see NEG_SEL)."]
    #[inline(always)]
    pub const fn sw_vref_neg(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "the current switch status, including DSI and sequencer controls, of the switch in the SARADC that shorts NEG with VREF input (see NEG_SEL)."]
    #[inline(always)]
    pub fn set_sw_vref_neg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "If high then the SAR is busy with a conversion. This bit is always high when CONTINUOUS is set. Firmware should wait for this bit to be low before putting the SAR in power down."]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If high then the SAR is busy with a conversion. This bit is always high when CONTINUOUS is set. Firmware should wait for this bit to be low before putting the SAR in power down."]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AvgMode {
    #[doc = "Accumulate and Dump (1st order accumulate and dump filter): a channel will be sampled back to back and averaged"]
    ACCUNDUMP = 0,
    #[doc = "Interleaved: Each scan (trigger) one sample is taken per channel and averaged over several scans."]
    INTERLEAVED = 0x01,
}
impl AvgMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AvgMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AvgMode {
    #[inline(always)]
    fn from(val: u8) -> AvgMode {
        AvgMode::from_bits(val)
    }
}
impl From<AvgMode> for u8 {
    #[inline(always)]
    fn from(val: AvgMode) -> u8 {
        AvgMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CompDly {
    #[doc = "2.5ns delay, use this for 2.5Msps"]
    D2P5 = 0,
    #[doc = "4.0ns delay, use this for 2.0Msps"]
    D4 = 0x01,
    #[doc = "10ns delay, use this for 1.5Msps"]
    D10 = 0x02,
    #[doc = "12ns delay, use this for 1.0Msps or less"]
    D12 = 0x03,
}
impl CompDly {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CompDly {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CompDly {
    #[inline(always)]
    fn from(val: u8) -> CompDly {
        CompDly::from_bits(val)
    }
}
impl From<CompDly> for u8 {
    #[inline(always)]
    fn from(val: CompDly) -> u8 {
        CompDly::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CompPwr {
    #[doc = "Power = 100 percent, Use this for SAR Clock Frequency greater than 18MHz"]
    P100 = 0,
    #[doc = "N/A"]
    P80 = 0x01,
    #[doc = "Power = 60 percent, Use this for SAR Clock Frequency greater than 1.8MHz up to 18MHz."]
    P60 = 0x02,
    #[doc = "N/A"]
    P50 = 0x03,
    #[doc = "N/A"]
    P40 = 0x04,
    #[doc = "N/A"]
    P30 = 0x05,
    #[doc = "Power = 20 percent, Use this for SAR Clock Frequency less than or equal to 1.8MHz"]
    P20 = 0x06,
    #[doc = "N/A"]
    P10 = 0x07,
}
impl CompPwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CompPwr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CompPwr {
    #[inline(always)]
    fn from(val: u8) -> CompPwr {
        CompPwr::from_bits(val)
    }
}
impl From<CompPwr> for u8 {
    #[inline(always)]
    fn from(val: CompPwr) -> u8 {
        CompPwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DifferentialSigned {
    #[doc = "result data is unsigned (zero extended if needed)"]
    UNSIGNED = 0,
    #[doc = "Default: result data is signed (sign extended if needed)"]
    SIGNED = 0x01,
}
impl DifferentialSigned {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DifferentialSigned {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DifferentialSigned {
    #[inline(always)]
    fn from(val: u8) -> DifferentialSigned {
        DifferentialSigned::from_bits(val)
    }
}
impl From<DifferentialSigned> for u8 {
    #[inline(always)]
    fn from(val: DifferentialSigned) -> u8 {
        DifferentialSigned::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InjPortAddr {
    #[doc = "SARMUX pins."]
    SARMUX = 0,
    #[doc = "CTB0"]
    CTB0 = 0x01,
    #[doc = "CTB1"]
    CTB1 = 0x02,
    #[doc = "CTB2"]
    CTB2 = 0x03,
    #[doc = "CTB3"]
    CTB3 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "AROUTE virtual port"]
    AROUTE_VIRT = 0x06,
    #[doc = "SARMUX virtual port"]
    SARMUX_VIRT = 0x07,
}
impl InjPortAddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InjPortAddr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InjPortAddr {
    #[inline(always)]
    fn from(val: u8) -> InjPortAddr {
        InjPortAddr::from_bits(val)
    }
}
impl From<InjPortAddr> for u8 {
    #[inline(always)]
    fn from(val: InjPortAddr) -> u8 {
        InjPortAddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum NegPortAddr {
    #[doc = "SARMUX pins."]
    SARMUX = 0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2 = 0x05,
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1 = 0x06,
    #[doc = "SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT = 0x07,
}
impl NegPortAddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NegPortAddr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NegPortAddr {
    #[inline(always)]
    fn from(val: u8) -> NegPortAddr {
        NegPortAddr::from_bits(val)
    }
}
impl From<NegPortAddr> for u8 {
    #[inline(always)]
    fn from(val: NegPortAddr) -> u8 {
        NegPortAddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum NegSel {
    #[doc = "NEG input of SARADC is connected to 'vssa_kelvin', gives more precision around zero. Note this opens both SARADC internal switches, therefore use this value to insert a break-before-make cycle on those switches when SWITCH_DISABLE is high."]
    VSSA_KELVIN = 0,
    #[doc = "NEG input of SARADC is connected to VSSA in AROUTE close to the SARADC"]
    ART_VSSA = 0x01,
    #[doc = "NEG input of SARADC is connected to P1 pin of SARMUX"]
    P1 = 0x02,
    #[doc = "NEG input of SARADC is connected to P3 pin of SARMUX"]
    P3 = 0x03,
    #[doc = "NEG input of SARADC is connected to P5 pin of SARMUX"]
    P5 = 0x04,
    #[doc = "NEG input of SARADC is connected to P7 pin of SARMUX"]
    P7 = 0x05,
    #[doc = "NEG input of SARADC is connected to an ACORE in AROUTE"]
    ACORE = 0x06,
    #[doc = "NEG input of SARADC is shorted with VREF input of SARADC."]
    VREF = 0x07,
}
impl NegSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NegSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NegSel {
    #[inline(always)]
    fn from(val: u8) -> NegSel {
        NegSel::from_bits(val)
    }
}
impl From<NegSel> for u8 {
    #[inline(always)]
    fn from(val: NegSel) -> u8 {
        NegSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PosPortAddr {
    #[doc = "SARMUX pins."]
    SARMUX = 0,
    #[doc = "CTB0"]
    CTB0 = 0x01,
    #[doc = "CTB1"]
    CTB1 = 0x02,
    #[doc = "CTB2"]
    CTB2 = 0x03,
    #[doc = "CTB3"]
    CTB3 = 0x04,
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2 = 0x05,
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1 = 0x06,
    #[doc = "SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT = 0x07,
}
impl PosPortAddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PosPortAddr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PosPortAddr {
    #[inline(always)]
    fn from(val: u8) -> PosPortAddr {
        PosPortAddr::from_bits(val)
    }
}
impl From<PosPortAddr> for u8 {
    #[inline(always)]
    fn from(val: PosPortAddr) -> u8 {
        PosPortAddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PwrCtrlVref {
    #[doc = "full power (100 percent) (default), bypass cap, max clk_sar is 18MHz."]
    PWR_100 = 0,
    #[doc = "80 percent power"]
    PWR_80 = 0x01,
    #[doc = "60 percent power"]
    PWR_60 = 0x02,
    #[doc = "50 percent power"]
    PWR_50 = 0x03,
    #[doc = "40 percent power"]
    PWR_40 = 0x04,
    #[doc = "30 percent power"]
    PWR_30 = 0x05,
    #[doc = "20 percent power"]
    PWR_20 = 0x06,
    #[doc = "10 percent power"]
    PWR_10 = 0x07,
}
impl PwrCtrlVref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrCtrlVref {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrCtrlVref {
    #[inline(always)]
    fn from(val: u8) -> PwrCtrlVref {
        PwrCtrlVref::from_bits(val)
    }
}
impl From<PwrCtrlVref> for u8 {
    #[inline(always)]
    fn from(val: PwrCtrlVref) -> u8 {
        PwrCtrlVref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RangeCond {
    #[doc = "result < RANGE_LOW"]
    BELOW = 0,
    #[doc = "RANGE_LOW <= result < RANGE_HIGH"]
    INSIDE = 0x01,
    #[doc = "RANGE_HIGH <= result"]
    ABOVE = 0x02,
    #[doc = "result < RANGE_LOW || RANGE_HIGH <= result"]
    OUTSIDE = 0x03,
}
impl RangeCond {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RangeCond {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RangeCond {
    #[inline(always)]
    fn from(val: u8) -> RangeCond {
        RangeCond::from_bits(val)
    }
}
impl From<RangeCond> for u8 {
    #[inline(always)]
    fn from(val: RangeCond) -> u8 {
        RangeCond::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SingleEndedSigned {
    #[doc = "Default: result data is unsigned (zero extended if needed)"]
    UNSIGNED = 0,
    #[doc = "result data is signed (sign extended if needed)"]
    SIGNED = 0x01,
}
impl SingleEndedSigned {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleEndedSigned {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleEndedSigned {
    #[inline(always)]
    fn from(val: u8) -> SingleEndedSigned {
        SingleEndedSigned::from_bits(val)
    }
}
impl From<SingleEndedSigned> for u8 {
    #[inline(always)]
    fn from(val: SingleEndedSigned) -> u8 {
        SingleEndedSigned::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UabScanMode {
    #[doc = "Unscheduled UABs: one or more of the UABs scanned by the SAR is not scheduled, for each channel that scans a UAB the SAR will wait for a positive edge on the trigger output of that UAB. Caveat: in this mode the length of SAR scan can be variable."]
    UNSCHEDULED = 0,
    #[doc = "Scheduled UABs: All UABs scanned by the SAR are assumed to be properly scheduled, i.e. their output is assumed to be valid when sampled by the SAR and the SAR does not wait. In this mode the length of the SAR scan is constant. This mode requires that the SAR scans strictly periodically, i.e. the SAR has to either run continuously or has to be triggered by a periodic hardware trigger (TCPWM or UDB timer). It also requires that the end of the UAB valid phase is precisely aligned with the end of the SAR sample period (using UAB.STARTUP_DELAY). Normally this scheduling is done by Creator."]
    SCHEDULED = 0x01,
}
impl UabScanMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UabScanMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UabScanMode {
    #[inline(always)]
    fn from(val: u8) -> UabScanMode {
        UabScanMode::from_bits(val)
    }
}
impl From<UabScanMode> for u8 {
    #[inline(always)]
    fn from(val: UabScanMode) -> u8 {
        UabScanMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum VrefSel {
    #[doc = "VREF0 from PRB (VREF buffer on)"]
    VREF0 = 0,
    #[doc = "VREF1 from PRB (VREF buffer on)"]
    VREF1 = 0x01,
    #[doc = "VREF2 from PRB (VREF buffer on)"]
    VREF2 = 0x02,
    #[doc = "VREF from AROUTE (VREF buffer on)"]
    VREF_AROUTE = 0x03,
    #[doc = "1.024V from BandGap (VREF buffer on)"]
    VBGR = 0x04,
    #[doc = "External precision Vref direct from a pin (low impedance path)."]
    VREF_EXT = 0x05,
    #[doc = "Vdda/2 (VREF buffer on)"]
    VDDA_DIV_2 = 0x06,
    #[doc = "Vdda."]
    VDDA = 0x07,
}
impl VrefSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VrefSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VrefSel {
    #[inline(always)]
    fn from(val: u8) -> VrefSel {
        VrefSel::from_bits(val)
    }
}
impl From<VrefSel> for u8 {
    #[inline(always)]
    fn from(val: VrefSel) -> u8 {
        VrefSel::to_bits(val)
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
