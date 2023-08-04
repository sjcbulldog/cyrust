#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "PDM registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdm0 {
    ptr: *mut u8,
}
unsafe impl Send for Pdm0 {}
unsafe impl Sync for Pdm0 {}
impl Pdm0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Clock control"]
    #[inline(always)]
    pub const fn clock_ctl(self) -> crate::common::Reg<ClockCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Mode control"]
    #[inline(always)]
    pub const fn mode_ctl(self) -> crate::common::Reg<ModeCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Data control"]
    #[inline(always)]
    pub const fn data_ctl(self) -> crate::common::Reg<DataCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Command"]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Trigger control"]
    #[inline(always)]
    pub const fn tr_ctl(self) -> crate::common::Reg<TrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "RX FIFO control"]
    #[inline(always)]
    pub const fn rx_fifo_ctl(self) -> crate::common::Reg<RxFifoCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize) as _) }
    }
    #[doc = "RX FIFO status"]
    #[inline(always)]
    pub const fn rx_fifo_status(self) -> crate::common::Reg<RxFifoStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(772usize) as _) }
    }
    #[doc = "RX FIFO read"]
    #[inline(always)]
    pub const fn rx_fifo_rd(self) -> crate::common::Reg<RxFifoRd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(776usize) as _) }
    }
    #[doc = "RX FIFO silent read"]
    #[inline(always)]
    pub const fn rx_fifo_rd_silent(self) -> crate::common::Reg<RxFifoRdSilent, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(780usize) as _) }
    }
    #[doc = "Interrupt register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3840usize) as _) }
    }
    #[doc = "Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3844usize) as _) }
    }
    #[doc = "Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3848usize) as _) }
    }
    #[doc = "Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3852usize) as _) }
    }
}
#[doc = "Clock control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClockCtl(pub u32);
impl ClockCtl {
    #[doc = "PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
    #[inline(always)]
    pub const fn clk_clock_div(&self) -> ClkClockDiv {
        let val = (self.0 >> 0usize) & 0x03;
        ClkClockDiv::from_bits(val as u8)
    }
    #[doc = "PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
    #[inline(always)]
    pub fn set_clk_clock_div(&mut self, val: ClkClockDiv) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    pub const fn mclkq_clock_div(&self) -> MclkqClockDiv {
        let val = (self.0 >> 4usize) & 0x03;
        MclkqClockDiv::from_bits(val as u8)
    }
    #[doc = "MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    pub fn set_mclkq_clock_div(&mut self, val: MclkqClockDiv) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
    #[inline(always)]
    pub const fn cko_clock_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
    #[inline(always)]
    pub fn set_cko_clock_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
    #[inline(always)]
    pub const fn sinc_rate(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
    #[inline(always)]
    pub fn set_sinc_rate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for ClockCtl {
    #[inline(always)]
    fn default() -> ClockCtl {
        ClockCtl(0)
    }
}
#[doc = "Command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
    #[inline(always)]
    pub const fn stream_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
    #[inline(always)]
    pub fn set_stream_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        Cmd(0)
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
    #[inline(always)]
    pub const fn pga_r(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
    #[inline(always)]
    pub fn set_pga_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
    #[inline(always)]
    pub const fn pga_l(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
    #[inline(always)]
    pub fn set_pga_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
    #[inline(always)]
    pub const fn soft_mute(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
    #[inline(always)]
    pub fn set_soft_mute(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    pub const fn step_sel(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    pub fn set_step_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the PDM component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the PDM component: '0': Disabled. '1': Enabled."]
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
#[doc = "Data control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DataCtl(pub u32);
impl DataCtl {
    #[doc = "PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
    #[inline(always)]
    pub const fn word_len(&self) -> WordLen {
        let val = (self.0 >> 0usize) & 0x03;
        WordLen::from_bits(val as u8)
    }
    #[doc = "PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
    #[inline(always)]
    pub fn set_word_len(&mut self, val: WordLen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub const fn bit_extension(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub fn set_bit_extension(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for DataCtl {
    #[inline(always)]
    fn default() -> DataCtl {
        DataCtl(0)
    }
}
#[doc = "Interrupt register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
    #[inline(always)]
    pub const fn rx_trigger(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
    #[inline(always)]
    pub fn set_rx_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RX FIFO is not empty."]
    #[inline(always)]
    pub const fn rx_not_empty(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO is not empty."]
    #[inline(always)]
    pub fn set_rx_not_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Attempt to write to a full RX FIFO"]
    #[inline(always)]
    pub const fn rx_overflow(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to write to a full RX FIFO"]
    #[inline(always)]
    pub fn set_rx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Attempt to read from an empty RX FIFO"]
    #[inline(always)]
    pub const fn rx_underflow(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to read from an empty RX FIFO"]
    #[inline(always)]
    pub fn set_rx_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_trigger(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_not_empty(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_not_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_overflow(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_underflow(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Interrupt masked register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn rx_trigger(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_rx_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn rx_not_empty(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_rx_not_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn rx_overflow(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_rx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn rx_underflow(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_rx_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_trigger(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_not_empty(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_not_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_overflow(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_underflow(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Mode control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModeCtl(pub u32);
impl ModeCtl {
    #[doc = "Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
    #[inline(always)]
    pub const fn pcm_ch_set(&self) -> PcmChSet {
        let val = (self.0 >> 0usize) & 0x03;
        PcmChSet::from_bits(val as u8)
    }
    #[doc = "Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
    #[inline(always)]
    pub fn set_pcm_ch_set(&mut self, val: PcmChSet) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
    #[inline(always)]
    pub const fn swap_lr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
    #[inline(always)]
    pub fn set_swap_lr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
    #[inline(always)]
    pub const fn s_cycles(&self) -> SCycles {
        let val = (self.0 >> 8usize) & 0x07;
        SCycles::from_bits(val as u8)
    }
    #[doc = "Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
    #[inline(always)]
    pub fn set_s_cycles(&mut self, val: SCycles) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
    #[inline(always)]
    pub const fn cko_delay(&self) -> CkoDelay {
        let val = (self.0 >> 16usize) & 0x07;
        CkoDelay::from_bits(val as u8)
    }
    #[doc = "Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
    #[inline(always)]
    pub fn set_cko_delay(&mut self, val: CkoDelay) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\] (Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
    #[inline(always)]
    pub const fn hpf_gain(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\] (Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
    #[inline(always)]
    pub fn set_hpf_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
    #[inline(always)]
    pub const fn hpf_en_n(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
    #[inline(always)]
    pub fn set_hpf_en_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for ModeCtl {
    #[inline(always)]
    fn default() -> ModeCtl {
        ModeCtl(0)
    }
}
#[doc = "RX FIFO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxFifoCtl(pub u32);
impl RxFifoCtl {
    #[doc = "Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
    #[inline(always)]
    pub const fn trigger_level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
    #[inline(always)]
    pub fn set_trigger_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
    #[inline(always)]
    pub const fn freeze(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn set_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for RxFifoCtl {
    #[inline(always)]
    fn default() -> RxFifoCtl {
        RxFifoCtl(0)
    }
}
#[doc = "RX FIFO read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxFifoRd(pub u32);
impl RxFifoRd {
    #[doc = "Data read from the RX FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. Note: Don't access to this bit while RX_FIFO_CTL.CLEAR is '1'."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data read from the RX FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. Note: Don't access to this bit while RX_FIFO_CTL.CLEAR is '1'."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RxFifoRd {
    #[inline(always)]
    fn default() -> RxFifoRd {
        RxFifoRd(0)
    }
}
#[doc = "RX FIFO silent read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxFifoRdSilent(pub u32);
impl RxFifoRdSilent {
    #[doc = "Data read from the RX FIFO. Reading a data frame will NOT remove the data frame from the RX FIFO; i.e. behavior is similar to that of a PEEK operation. This field is used only for debugging purposes. Note: Don't access to this bit while RX_FIFO_CTL.CLEAR is '1'."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data read from the RX FIFO. Reading a data frame will NOT remove the data frame from the RX FIFO; i.e. behavior is similar to that of a PEEK operation. This field is used only for debugging purposes. Note: Don't access to this bit while RX_FIFO_CTL.CLEAR is '1'."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RxFifoRdSilent {
    #[inline(always)]
    fn default() -> RxFifoRdSilent {
        RxFifoRdSilent(0)
    }
}
#[doc = "RX FIFO status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxFifoStatus(pub u32);
impl RxFifoStatus {
    #[doc = "Number of entries in the RX FIFO. The field value is in the range \\[0, 255\\]. When this is zero, the RX FIFO is empty."]
    #[inline(always)]
    pub const fn used(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of entries in the RX FIFO. The field value is in the range \\[0, 255\\]. When this is zero, the RX FIFO is empty."]
    #[inline(always)]
    pub fn set_used(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX FIFO read pointer: RX FIFO location from which a data frame is read by the host.This field is used only for debugging purposes."]
    #[inline(always)]
    pub const fn rd_ptr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "RX FIFO read pointer: RX FIFO location from which a data frame is read by the host.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn set_rd_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "RX FIFO write pointer: RX FIFO location at which a new data frame is written by the hardware.This field is used only for debugging purposes."]
    #[inline(always)]
    pub const fn wr_ptr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "RX FIFO write pointer: RX FIFO location at which a new data frame is written by the hardware.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn set_wr_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for RxFifoStatus {
    #[inline(always)]
    fn default() -> RxFifoStatus {
        RxFifoStatus(0)
    }
}
#[doc = "Trigger control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCtl(pub u32);
impl TrCtl {
    #[doc = "Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn rx_req_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_rx_req_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for TrCtl {
    #[inline(always)]
    fn default() -> TrCtl {
        TrCtl(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CkoDelay {
    #[doc = "CLK_IS is 3*PDM_CLK period early"]
    ADV3 = 0,
    #[doc = "CLK_IS is 2*PDM_CLK period early"]
    ADV2 = 0x01,
    #[doc = "CLK_IS is 1*PDM_CLK period early"]
    ADV1 = 0x02,
    #[doc = "CLK_IS is the same as PDM_CKO"]
    NO_DELAY = 0x03,
    #[doc = "CLK_IS is 1*PDM_CLK period late"]
    DLY1 = 0x04,
    #[doc = "CLK_IS is 2*PDM_CLK period late"]
    DLY2 = 0x05,
    #[doc = "CLK_IS is 3*PDM_CLK period late"]
    DLY3 = 0x06,
    #[doc = "CLK_IS is 4*PDM_CLK period late"]
    DLY4 = 0x07,
}
impl CkoDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CkoDelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CkoDelay {
    #[inline(always)]
    fn from(val: u8) -> CkoDelay {
        CkoDelay::from_bits(val)
    }
}
impl From<CkoDelay> for u8 {
    #[inline(always)]
    fn from(val: CkoDelay) -> u8 {
        CkoDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClkClockDiv {
    #[doc = "Divide by 1"]
    DIVBY1 = 0,
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    DIVBY2 = 0x01,
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    DIVBY3 = 0x02,
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    DIVBY4 = 0x03,
}
impl ClkClockDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkClockDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkClockDiv {
    #[inline(always)]
    fn from(val: u8) -> ClkClockDiv {
        ClkClockDiv::from_bits(val)
    }
}
impl From<ClkClockDiv> for u8 {
    #[inline(always)]
    fn from(val: ClkClockDiv) -> u8 {
        ClkClockDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MclkqClockDiv {
    #[doc = "Divide by 1"]
    DIVBY1 = 0,
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    DIVBY2 = 0x01,
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    DIVBY3 = 0x02,
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    DIVBY4 = 0x03,
}
impl MclkqClockDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MclkqClockDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MclkqClockDiv {
    #[inline(always)]
    fn from(val: u8) -> MclkqClockDiv {
        MclkqClockDiv::from_bits(val)
    }
}
impl From<MclkqClockDiv> for u8 {
    #[inline(always)]
    fn from(val: MclkqClockDiv) -> u8 {
        MclkqClockDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PcmChSet {
    #[doc = "Channel disabled"]
    DISABLED = 0,
    #[doc = "Mono left channel enable"]
    MONO_L = 0x01,
    #[doc = "Mono right channel enable"]
    MONO_R = 0x02,
    #[doc = "Stereo channel enable"]
    STEREO = 0x03,
}
impl PcmChSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcmChSet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcmChSet {
    #[inline(always)]
    fn from(val: u8) -> PcmChSet {
        PcmChSet::from_bits(val)
    }
}
impl From<PcmChSet> for u8 {
    #[inline(always)]
    fn from(val: PcmChSet) -> u8 {
        PcmChSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SCycles {
    #[doc = "64steps"]
    STEP_NUM64 = 0,
    #[doc = "96steps"]
    STEP_NUM96 = 0x01,
    #[doc = "128steps"]
    STEP_NUM128 = 0x02,
    #[doc = "160steps"]
    STEP_NUM160 = 0x03,
    #[doc = "192steps"]
    STEP_NUM192 = 0x04,
    #[doc = "256steps"]
    STEP_NUM256 = 0x05,
    #[doc = "384steps"]
    STEP_NUM384 = 0x06,
    #[doc = "512steps"]
    STEP_NUM512 = 0x07,
}
impl SCycles {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCycles {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCycles {
    #[inline(always)]
    fn from(val: u8) -> SCycles {
        SCycles::from_bits(val)
    }
}
impl From<SCycles> for u8 {
    #[inline(always)]
    fn from(val: SCycles) -> u8 {
        SCycles::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WordLen {
    #[doc = "16-bit"]
    BIT_LEN16 = 0,
    #[doc = "18-bit"]
    BIT_LEN18 = 0x01,
    #[doc = "20-bit"]
    BIT_LEN20 = 0x02,
    #[doc = "24-bit"]
    BIT_LEN24 = 0x03,
}
impl WordLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WordLen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WordLen {
    #[inline(always)]
    fn from(val: u8) -> WordLen {
        WordLen::from_bits(val)
    }
}
impl From<WordLen> for u8 {
    #[inline(always)]
    fn from(val: WordLen) -> u8 {
        WordLen::to_bits(val)
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
