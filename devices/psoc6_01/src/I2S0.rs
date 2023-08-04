#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "I2S registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2s0 {
    ptr: *mut u8,
}
unsafe impl Send for I2s0 {}
unsafe impl Sync for I2s0 {}
impl I2s0 {
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
    #[doc = "Transmitter control"]
    #[inline(always)]
    pub const fn tx_ctl(self) -> crate::common::Reg<TxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Transmitter watchdog"]
    #[inline(always)]
    pub const fn tx_watchdog(self) -> crate::common::Reg<TxWatchdog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Receiver control"]
    #[inline(always)]
    pub const fn rx_ctl(self) -> crate::common::Reg<RxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "Receiver watchdog"]
    #[inline(always)]
    pub const fn rx_watchdog(self) -> crate::common::Reg<RxWatchdog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "TX FIFO control"]
    #[inline(always)]
    pub const fn tx_fifo_ctl(self) -> crate::common::Reg<TxFifoCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "TX FIFO status"]
    #[inline(always)]
    pub const fn tx_fifo_status(self) -> crate::common::Reg<TxFifoStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "TX FIFO write"]
    #[inline(always)]
    pub const fn tx_fifo_wr(self) -> crate::common::Reg<TxFifoWr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
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
    #[doc = "Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    pub const fn clock_div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    pub fn set_clock_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    pub const fn clock_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    pub fn set_clock_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
    #[doc = "Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn tx_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_tx_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    pub const fn tx_pause(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    pub fn set_tx_pause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn rx_start(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_rx_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
    #[doc = "Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn tx_enabled(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_tx_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn rx_enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_rx_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "Interrupt register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
    #[inline(always)]
    pub const fn tx_trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
    #[inline(always)]
    pub fn set_tx_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX FIFO is not full."]
    #[inline(always)]
    pub const fn tx_not_full(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO is not full."]
    #[inline(always)]
    pub fn set_tx_not_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TX FIFO is empty; i.e. it has 0 entries."]
    #[inline(always)]
    pub const fn tx_empty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO is empty; i.e. it has 0 entries."]
    #[inline(always)]
    pub fn set_tx_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Attempt to write to a full TX FIFO."]
    #[inline(always)]
    pub const fn tx_overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to write to a full TX FIFO."]
    #[inline(always)]
    pub fn set_tx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
    #[inline(always)]
    pub const fn tx_underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
    #[inline(always)]
    pub fn set_tx_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Triggers (sets to '1') when the Tx watchdog event occurs."]
    #[inline(always)]
    pub const fn tx_wd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Triggers (sets to '1') when the Tx watchdog event occurs."]
    #[inline(always)]
    pub fn set_tx_wd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
    #[inline(always)]
    pub const fn rx_trigger(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
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
    #[doc = "RX FIFO is full."]
    #[inline(always)]
    pub const fn rx_full(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO is full."]
    #[inline(always)]
    pub fn set_rx_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Attempt to write to a full RX FIFO."]
    #[inline(always)]
    pub const fn rx_overflow(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to write to a full RX FIFO."]
    #[inline(always)]
    pub fn set_rx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Attempt to read from an empty RX FIFO."]
    #[inline(always)]
    pub const fn rx_underflow(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to read from an empty RX FIFO."]
    #[inline(always)]
    pub fn set_rx_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Triggers (sets to '1') when the Rx watchdog event occurs."]
    #[inline(always)]
    pub const fn rx_wd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Triggers (sets to '1') when the Rx watchdog event occurs."]
    #[inline(always)]
    pub fn set_rx_wd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
    pub const fn tx_trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_not_full(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_not_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_empty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_wd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_wd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
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
    pub const fn rx_full(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_wd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_wd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
    pub const fn tx_trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tx_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn tx_not_full(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tx_not_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn tx_empty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tx_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn tx_overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn tx_underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tx_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn tx_wd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tx_wd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
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
    pub const fn rx_full(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_rx_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn rx_wd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_rx_wd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
    pub const fn tx_trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_not_full(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_not_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_empty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_wd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_wd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
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
    pub const fn rx_full(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_wd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_wd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Receiver control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxCtl(pub u32);
impl RxCtl {
    #[doc = "Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)"]
    #[inline(always)]
    pub const fn b_clock_inv(&self) -> RxCtlBClockInv {
        let val = (self.0 >> 3usize) & 0x01;
        RxCtlBClockInv::from_bits(val as u8)
    }
    #[doc = "Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)"]
    #[inline(always)]
    pub fn set_b_clock_inv(&mut self, val: RxCtlBClockInv) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)"]
    #[inline(always)]
    pub const fn ch_nr(&self) -> RxCtlChNr {
        let val = (self.0 >> 4usize) & 0x07;
        RxCtlChNr::from_bits(val as u8)
    }
    #[doc = "Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)"]
    #[inline(always)]
    pub fn set_ch_nr(&mut self, val: RxCtlChNr) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)"]
    #[inline(always)]
    pub const fn ms(&self) -> RxCtlMs {
        let val = (self.0 >> 7usize) & 0x01;
        RxCtlMs::from_bits(val as u8)
    }
    #[doc = "Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)"]
    #[inline(always)]
    pub fn set_ms(&mut self, val: RxCtlMs) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)"]
    #[inline(always)]
    pub const fn i2s_mode(&self) -> RxCtlI2sMode {
        let val = (self.0 >> 8usize) & 0x03;
        RxCtlI2sMode::from_bits(val as u8)
    }
    #[doc = "Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)"]
    #[inline(always)]
    pub fn set_i2s_mode(&mut self, val: RxCtlI2sMode) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'."]
    #[inline(always)]
    pub const fn ws_pulse(&self) -> RxCtlWsPulse {
        let val = (self.0 >> 10usize) & 0x01;
        RxCtlWsPulse::from_bits(val as u8)
    }
    #[doc = "Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'."]
    #[inline(always)]
    pub fn set_ws_pulse(&mut self, val: RxCtlWsPulse) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Set watchdog for 'rx_ws_in' '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn wd_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Set watchdog for 'rx_ws_in' '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_wd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)"]
    #[inline(always)]
    pub const fn ch_len(&self) -> RxCtlChLen {
        let val = (self.0 >> 16usize) & 0x07;
        RxCtlChLen::from_bits(val as u8)
    }
    #[doc = "Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)"]
    #[inline(always)]
    pub fn set_ch_len(&mut self, val: RxCtlChLen) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)"]
    #[inline(always)]
    pub const fn word_len(&self) -> RxCtlWordLen {
        let val = (self.0 >> 20usize) & 0x07;
        RxCtlWordLen::from_bits(val as u8)
    }
    #[doc = "Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)"]
    #[inline(always)]
    pub fn set_word_len(&mut self, val: RxCtlWordLen) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub const fn bit_extension(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub fn set_bit_extension(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "RX master bit clock polarity. When this bit is 1, the outgoing rx_sck signal is inverted after it has been transmitted from the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting.See RX_CTL.B_CLOCK_INV for more details."]
    #[inline(always)]
    pub const fn scko_pol(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "RX master bit clock polarity. When this bit is 1, the outgoing rx_sck signal is inverted after it has been transmitted from the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting.See RX_CTL.B_CLOCK_INV for more details."]
    #[inline(always)]
    pub fn set_scko_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "RX slave bit clock polarity. When this bit is 1, the incoming rx_sck signal is inverted before it is received by the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting. '0': When receiver is in slave mode, serial data is sampled on the rising bit clock edge '1': When receiver is in slave mode, serial data is sampled on the falling bit clock edge"]
    #[inline(always)]
    pub const fn scki_pol(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "RX slave bit clock polarity. When this bit is 1, the incoming rx_sck signal is inverted before it is received by the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting. '0': When receiver is in slave mode, serial data is sampled on the rising bit clock edge '1': When receiver is in slave mode, serial data is sampled on the falling bit clock edge"]
    #[inline(always)]
    pub fn set_scki_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for RxCtl {
    #[inline(always)]
    fn default() -> RxCtl {
        RxCtl(0)
    }
}
#[doc = "RX FIFO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxFifoCtl(pub u32);
impl RxFifoCtl {
    #[doc = "Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 253 in I2S mode or Left Justified (RX_CTL.I2S_MODE = '0' or '1'). In TDM mode (RX_CTL.I2S_MODE = '2' or '3'), it can configure up to \\[256 - (RX_CTL.CH_NR+2)\\]."]
    #[inline(always)]
    pub const fn trigger_level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 253 in I2S mode or Left Justified (RX_CTL.I2S_MODE = '0' or '1'). In TDM mode (RX_CTL.I2S_MODE = '2' or '3'), it can configure up to \\[256 - (RX_CTL.CH_NR+2)\\]."]
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
    #[doc = "When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer. This field is used only for debugging purposee."]
    #[inline(always)]
    pub const fn freeze(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer. This field is used only for debugging purposee."]
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
    #[doc = "Data read from the RX FIFO. Reading a data frame will remove the data frame from the RX FIFO; i.e. behavior is similar to that of a POP operation. Notes: - Don't access to this register while RX_FIFO_CTL.CLEAR is '1'. - Two stored data may be not valid after CMD.RX_START is set '1'. Therefore we recommend software discard those data."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data read from the RX FIFO. Reading a data frame will remove the data frame from the RX FIFO; i.e. behavior is similar to that of a POP operation. Notes: - Don't access to this register while RX_FIFO_CTL.CLEAR is '1'. - Two stored data may be not valid after CMD.RX_START is set '1'. Therefore we recommend software discard those data."]
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
    #[doc = "Data read from the RX FIFO. Reading a data frame will NOT remove the data frame from the RX FIFO; i.e. behavior is similar to that of a PEEK operation. This field is used only for debugging purposes. Notes: - Don't access to this register while RX_FIFO_CTL.CLEAR is '1'. - Two stored data may be not valid after CMD.RX_START is set '1'. Therefore we recommend software discard those data."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data read from the RX FIFO. Reading a data frame will NOT remove the data frame from the RX FIFO; i.e. behavior is similar to that of a PEEK operation. This field is used only for debugging purposes. Notes: - Don't access to this register while RX_FIFO_CTL.CLEAR is '1'. - Two stored data may be not valid after CMD.RX_START is set '1'. Therefore we recommend software discard those data."]
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
    #[doc = "Number of entries in the RX FIFO. The field value is in the range \\[0, 256\\]."]
    #[inline(always)]
    pub const fn used(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Number of entries in the RX FIFO. The field value is in the range \\[0, 256\\]."]
    #[inline(always)]
    pub fn set_used(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "RX FIFO read pointer: FIFO location from which a data frame is read by the host. This field is used only for debugging purposes."]
    #[inline(always)]
    pub const fn rd_ptr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "RX FIFO read pointer: FIFO location from which a data frame is read by the host. This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn set_rd_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "RX FIFO write pointer: FIFO location at which a new data frame is written by the hardware. This field is used only for debugging purposes."]
    #[inline(always)]
    pub const fn wr_ptr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "RX FIFO write pointer: FIFO location at which a new data frame is written by the hardware. This field is used only for debugging purposes."]
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
#[doc = "Receiver watchdog"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxWatchdog(pub u32);
impl RxWatchdog {
    #[doc = "Start value of the RX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub const fn wd_counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Start value of the RX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn set_wd_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RxWatchdog {
    #[inline(always)]
    fn default() -> RxWatchdog {
        RxWatchdog(0)
    }
}
#[doc = "Trigger control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCtl(pub u32);
impl TrCtl {
    #[doc = "Trigger output ('tr_i2s_tx_req') enable for requests of DMA transfer in transmission '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn tx_req_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger output ('tr_i2s_tx_req') enable for requests of DMA transfer in transmission '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_tx_req_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger output ('tr_i2s_rx_req') enable for requests of DMA transfer in reception '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn rx_req_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger output ('tr_i2s_rx_req') enable for requests of DMA transfer in reception '0': Disabled. '1': Enabled."]
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
#[doc = "Transmitter control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCtl(pub u32);
impl TxCtl {
    #[doc = "Serial data transmission is advanced by 0.5 SCK cycles. This bit is valid only in TX slave mode. When set to '1', the serial data will be transmitted 0.5 SCK cycles earlier than when set to '0'. 1) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK falling edge 2) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK rising edge that is 0.5 SCK cycles before the SCK falling edge in 1) 3) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK rising edge 4) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK falling edge that is 0.5 SCK cycles before the SCK rising edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual timing is generated by an internal clock that runs 8x the SCK frequency). The word sync (TX_WS) signal is not affected by this bit setting. Note: When Master mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.TX_BCLKINV)"]
    #[inline(always)]
    pub const fn b_clock_inv(&self) -> TxCtlBClockInv {
        let val = (self.0 >> 3usize) & 0x01;
        TxCtlBClockInv::from_bits(val as u8)
    }
    #[doc = "Serial data transmission is advanced by 0.5 SCK cycles. This bit is valid only in TX slave mode. When set to '1', the serial data will be transmitted 0.5 SCK cycles earlier than when set to '0'. 1) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK falling edge 2) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK rising edge that is 0.5 SCK cycles before the SCK falling edge in 1) 3) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK rising edge 4) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK falling edge that is 0.5 SCK cycles before the SCK rising edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual timing is generated by an internal clock that runs 8x the SCK frequency). The word sync (TX_WS) signal is not affected by this bit setting. Note: When Master mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.TX_BCLKINV)"]
    #[inline(always)]
    pub fn set_b_clock_inv(&mut self, val: TxCtlBClockInv) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHSET)"]
    #[inline(always)]
    pub const fn ch_nr(&self) -> TxCtlChNr {
        let val = (self.0 >> 4usize) & 0x07;
        TxCtlChNr::from_bits(val as u8)
    }
    #[doc = "Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHSET)"]
    #[inline(always)]
    pub fn set_ch_nr(&mut self, val: TxCtlChNr) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_MS)"]
    #[inline(always)]
    pub const fn ms(&self) -> TxCtlMs {
        let val = (self.0 >> 7usize) & 0x01;
        TxCtlMs::from_bits(val as u8)
    }
    #[doc = "Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_MS)"]
    #[inline(always)]
    pub fn set_ms(&mut self, val: TxCtlMs) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.TX_CFG.TX_I2S_MODE)"]
    #[inline(always)]
    pub const fn i2s_mode(&self) -> TxCtlI2sMode {
        let val = (self.0 >> 8usize) & 0x03;
        TxCtlI2sMode::from_bits(val as u8)
    }
    #[doc = "Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.TX_CFG.TX_I2S_MODE)"]
    #[inline(always)]
    pub fn set_i2s_mode(&mut self, val: TxCtlI2sMode) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_WS_PULSE) Note: When not TDM mode, must be '1'."]
    #[inline(always)]
    pub const fn ws_pulse(&self) -> TxCtlWsPulse {
        let val = (self.0 >> 10usize) & 0x01;
        TxCtlWsPulse::from_bits(val as u8)
    }
    #[doc = "Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_WS_PULSE) Note: When not TDM mode, must be '1'."]
    #[inline(always)]
    pub fn set_ws_pulse(&mut self, val: TxCtlWsPulse) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Set overhead value: '0': Set to '0' '1': Set to '1' (Note: This bit is connected to AR38U12.TX_CFG.TX_OVHDATA)"]
    #[inline(always)]
    pub const fn ovhdata(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set overhead value: '0': Set to '0' '1': Set to '1' (Note: This bit is connected to AR38U12.TX_CFG.TX_OVHDATA)"]
    #[inline(always)]
    pub fn set_ovhdata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set watchdog for 'tx_ws_in': '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn wd_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Set watchdog for 'tx_ws_in': '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_wd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHLEN)"]
    #[inline(always)]
    pub const fn ch_len(&self) -> TxCtlChLen {
        let val = (self.0 >> 16usize) & 0x07;
        TxCtlChLen::from_bits(val as u8)
    }
    #[doc = "Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHLEN)"]
    #[inline(always)]
    pub fn set_ch_len(&mut self, val: TxCtlChLen) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.TX_CFG.TX_IWL)"]
    #[inline(always)]
    pub const fn word_len(&self) -> TxCtlWordLen {
        let val = (self.0 >> 20usize) & 0x07;
        TxCtlWordLen::from_bits(val as u8)
    }
    #[doc = "Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.TX_CFG.TX_IWL)"]
    #[inline(always)]
    pub fn set_word_len(&mut self, val: TxCtlWordLen) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "TX master bit clock polarity. When this bit is 1, the outgoing tx_sck signal is inverted after it has been transmitted from the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. '0': When transmitter is in master mode, serial data is transmitted from the falling bit clock edge '1': When transmitter is in master mode, serial data is transmitted from the rising bit clock edge"]
    #[inline(always)]
    pub const fn scko_pol(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TX master bit clock polarity. When this bit is 1, the outgoing tx_sck signal is inverted after it has been transmitted from the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. '0': When transmitter is in master mode, serial data is transmitted from the falling bit clock edge '1': When transmitter is in master mode, serial data is transmitted from the rising bit clock edge"]
    #[inline(always)]
    pub fn set_scko_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TX slave bit clock polarity. When this bit is 1, the incoming tx_sck signal is inverted before it is received by the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. See TX_CTL.B_CLOCK_INV for more details."]
    #[inline(always)]
    pub const fn scki_pol(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TX slave bit clock polarity. When this bit is 1, the incoming tx_sck signal is inverted before it is received by the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. See TX_CTL.B_CLOCK_INV for more details."]
    #[inline(always)]
    pub fn set_scki_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for TxCtl {
    #[inline(always)]
    fn default() -> TxCtl {
        TxCtl(0)
    }
}
#[doc = "TX FIFO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxFifoCtl(pub u32);
impl TxFifoCtl {
    #[doc = "Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
    #[inline(always)]
    pub const fn trigger_level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
    #[inline(always)]
    pub fn set_trigger_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "When '1', the TX FIFO and TX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', the TX FIFO and TX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When '1', hardware reads from the TX FIFO do not remove FIFO entries. Freeze will not advance the TX FIFO read pointer. This field is used only for debugging purposes."]
    #[inline(always)]
    pub const fn freeze(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', hardware reads from the TX FIFO do not remove FIFO entries. Freeze will not advance the TX FIFO read pointer. This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn set_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for TxFifoCtl {
    #[inline(always)]
    fn default() -> TxFifoCtl {
        TxFifoCtl(0)
    }
}
#[doc = "TX FIFO status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxFifoStatus(pub u32);
impl TxFifoStatus {
    #[doc = "Number of entries in the TX FIFO. The field value is in the range \\[0, 256\\]."]
    #[inline(always)]
    pub const fn used(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Number of entries in the TX FIFO. The field value is in the range \\[0, 256\\]."]
    #[inline(always)]
    pub fn set_used(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "TX FIFO read pointer: FIFO location from which a data frame is read by the hardware.This field is used only for debugging purposes."]
    #[inline(always)]
    pub const fn rd_ptr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX FIFO read pointer: FIFO location from which a data frame is read by the hardware.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn set_rd_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "TX FIFO write pointer: FIFO location at which a new data frame is written by the host. This field is used only for debugging purposes."]
    #[inline(always)]
    pub const fn wr_ptr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "TX FIFO write pointer: FIFO location at which a new data frame is written by the host. This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn set_wr_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxFifoStatus {
    #[inline(always)]
    fn default() -> TxFifoStatus {
        TxFifoStatus(0)
    }
}
#[doc = "TX FIFO write"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxFifoWr(pub u32);
impl TxFifoWr {
    #[doc = "Data written into the TX FIFO. Behavior is similar to that of a PUSH operation. Note: Don't access to this register while TX_FIFO_CTL.CLEAR is '1'."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data written into the TX FIFO. Behavior is similar to that of a PUSH operation. Note: Don't access to this register while TX_FIFO_CTL.CLEAR is '1'."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TxFifoWr {
    #[inline(always)]
    fn default() -> TxFifoWr {
        TxFifoWr(0)
    }
}
#[doc = "Transmitter watchdog"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxWatchdog(pub u32);
impl TxWatchdog {
    #[doc = "Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub const fn wd_counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn set_wd_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TxWatchdog {
    #[inline(always)]
    fn default() -> TxWatchdog {
        TxWatchdog(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxCtlBClockInv {
    #[doc = "SDI received at SCK rising edge when RX_CTL.SCKO_POL=0"]
    RISING_EDGE_RX = 0,
    #[doc = "SDI received at SCK falling edge when RX_CTL.SCKO_POL=0"]
    FALLING_EDGE_RX = 0x01,
}
impl RxCtlBClockInv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxCtlBClockInv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxCtlBClockInv {
    #[inline(always)]
    fn from(val: u8) -> RxCtlBClockInv {
        RxCtlBClockInv::from_bits(val)
    }
}
impl From<RxCtlBClockInv> for u8 {
    #[inline(always)]
    fn from(val: RxCtlBClockInv) -> u8 {
        RxCtlBClockInv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxCtlChLen {
    #[doc = "8-bit"]
    BIT_LEN8 = 0,
    #[doc = "16-bit"]
    BIT_LEN16 = 0x01,
    #[doc = "18-bit"]
    BIT_LEN18 = 0x02,
    #[doc = "20-bit"]
    BIT_LEN20 = 0x03,
    #[doc = "24-bit"]
    BIT_LEN24 = 0x04,
    #[doc = "32-bit"]
    BIT_LEN32 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxCtlChLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxCtlChLen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxCtlChLen {
    #[inline(always)]
    fn from(val: u8) -> RxCtlChLen {
        RxCtlChLen::from_bits(val)
    }
}
impl From<RxCtlChLen> for u8 {
    #[inline(always)]
    fn from(val: RxCtlChLen) -> u8 {
        RxCtlChLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxCtlChNr {
    #[doc = "1 channel"]
    CH_NUM1 = 0,
    #[doc = "2 channels"]
    CH_NUM2 = 0x01,
    #[doc = "3 channels"]
    CH_NUM3 = 0x02,
    #[doc = "4 channels"]
    CH_NUM4 = 0x03,
    #[doc = "5 channels"]
    CH_NUM5 = 0x04,
    #[doc = "6 channels"]
    CH_NUM6 = 0x05,
    #[doc = "7 channels"]
    CH_NUM7 = 0x06,
    #[doc = "8 channels"]
    CH_NUM8 = 0x07,
}
impl RxCtlChNr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxCtlChNr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxCtlChNr {
    #[inline(always)]
    fn from(val: u8) -> RxCtlChNr {
        RxCtlChNr::from_bits(val)
    }
}
impl From<RxCtlChNr> for u8 {
    #[inline(always)]
    fn from(val: RxCtlChNr) -> u8 {
        RxCtlChNr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxCtlI2sMode {
    #[doc = "Left Justified"]
    LEFT_JUSTIFIED = 0,
    #[doc = "I2S mode"]
    I2S = 0x01,
    #[doc = "TDM mode A, the 1st Channel align to WSO Rising Edge"]
    TDM_A = 0x02,
    #[doc = "TDM mode B, the 1st Channel align to WSO Rising edge with1 SCK Delay"]
    TDM_B = 0x03,
}
impl RxCtlI2sMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxCtlI2sMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxCtlI2sMode {
    #[inline(always)]
    fn from(val: u8) -> RxCtlI2sMode {
        RxCtlI2sMode::from_bits(val)
    }
}
impl From<RxCtlI2sMode> for u8 {
    #[inline(always)]
    fn from(val: RxCtlI2sMode) -> u8 {
        RxCtlI2sMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxCtlMs {
    #[doc = "Slave"]
    SLAVE = 0,
    #[doc = "Master"]
    MASTER = 0x01,
}
impl RxCtlMs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxCtlMs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxCtlMs {
    #[inline(always)]
    fn from(val: u8) -> RxCtlMs {
        RxCtlMs::from_bits(val)
    }
}
impl From<RxCtlMs> for u8 {
    #[inline(always)]
    fn from(val: RxCtlMs) -> u8 {
        RxCtlMs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxCtlWordLen {
    #[doc = "8-bit"]
    BIT_LEN8 = 0,
    #[doc = "16-bit"]
    BIT_LEN16 = 0x01,
    #[doc = "18-bit"]
    BIT_LEN18 = 0x02,
    #[doc = "20-bit"]
    BIT_LEN20 = 0x03,
    #[doc = "24-bit"]
    BIT_LEN24 = 0x04,
    #[doc = "32-bit"]
    BIT_LEN32 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxCtlWordLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxCtlWordLen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxCtlWordLen {
    #[inline(always)]
    fn from(val: u8) -> RxCtlWordLen {
        RxCtlWordLen::from_bits(val)
    }
}
impl From<RxCtlWordLen> for u8 {
    #[inline(always)]
    fn from(val: RxCtlWordLen) -> u8 {
        RxCtlWordLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RxCtlWsPulse {
    #[doc = "Pulse width is 1 SCK period"]
    SCK_PERIOD = 0,
    #[doc = "Pulse width is 1 channel length"]
    CH_LENGTH = 0x01,
}
impl RxCtlWsPulse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxCtlWsPulse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxCtlWsPulse {
    #[inline(always)]
    fn from(val: u8) -> RxCtlWsPulse {
        RxCtlWsPulse::from_bits(val)
    }
}
impl From<RxCtlWsPulse> for u8 {
    #[inline(always)]
    fn from(val: RxCtlWsPulse) -> u8 {
        RxCtlWsPulse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxCtlBClockInv {
    #[doc = "SDO transmitted at SCK falling edge when TX_CTL.SCKI_POL=0"]
    FALLING_EDGE_TX = 0,
    #[doc = "SDO transmitted at SCK rising edge when TX_CTL.SCKI_POL=0"]
    RISING_EDGE_TX = 0x01,
}
impl TxCtlBClockInv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCtlBClockInv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCtlBClockInv {
    #[inline(always)]
    fn from(val: u8) -> TxCtlBClockInv {
        TxCtlBClockInv::from_bits(val)
    }
}
impl From<TxCtlBClockInv> for u8 {
    #[inline(always)]
    fn from(val: TxCtlBClockInv) -> u8 {
        TxCtlBClockInv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxCtlChLen {
    #[doc = "8-bit"]
    BIT_LEN8 = 0,
    #[doc = "16-bit"]
    BIT_LEN16 = 0x01,
    #[doc = "18-bit"]
    BIT_LEN18 = 0x02,
    #[doc = "20-bit"]
    BIT_LEN20 = 0x03,
    #[doc = "24-bit"]
    BIT_LEN24 = 0x04,
    #[doc = "32-bit"]
    BIT_LEN32 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TxCtlChLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCtlChLen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCtlChLen {
    #[inline(always)]
    fn from(val: u8) -> TxCtlChLen {
        TxCtlChLen::from_bits(val)
    }
}
impl From<TxCtlChLen> for u8 {
    #[inline(always)]
    fn from(val: TxCtlChLen) -> u8 {
        TxCtlChLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxCtlChNr {
    #[doc = "1 channel"]
    CH_NUM1 = 0,
    #[doc = "2 channels"]
    CH_NUM2 = 0x01,
    #[doc = "3 channels"]
    CH_NUM3 = 0x02,
    #[doc = "4 channels"]
    CH_NUM4 = 0x03,
    #[doc = "5 channels"]
    CH_NUM5 = 0x04,
    #[doc = "6 channels"]
    CH_NUM6 = 0x05,
    #[doc = "7 channels"]
    CH_NUM7 = 0x06,
    #[doc = "8 channels"]
    CH_NUM8 = 0x07,
}
impl TxCtlChNr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCtlChNr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCtlChNr {
    #[inline(always)]
    fn from(val: u8) -> TxCtlChNr {
        TxCtlChNr::from_bits(val)
    }
}
impl From<TxCtlChNr> for u8 {
    #[inline(always)]
    fn from(val: TxCtlChNr) -> u8 {
        TxCtlChNr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxCtlI2sMode {
    #[doc = "Left Justified"]
    LEFT_JUSTIFIED = 0,
    #[doc = "I2S mode"]
    I2S = 0x01,
    #[doc = "TDM mode A, the 1st Channel align to WSO Rising Edge"]
    TDM_A = 0x02,
    #[doc = "TDM mode B, the 1st Channel align to WSO Rising edge with1 SCK Delay"]
    TDM_B = 0x03,
}
impl TxCtlI2sMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCtlI2sMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCtlI2sMode {
    #[inline(always)]
    fn from(val: u8) -> TxCtlI2sMode {
        TxCtlI2sMode::from_bits(val)
    }
}
impl From<TxCtlI2sMode> for u8 {
    #[inline(always)]
    fn from(val: TxCtlI2sMode) -> u8 {
        TxCtlI2sMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxCtlMs {
    #[doc = "Slave"]
    SLAVE = 0,
    #[doc = "Master"]
    MASTER = 0x01,
}
impl TxCtlMs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCtlMs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCtlMs {
    #[inline(always)]
    fn from(val: u8) -> TxCtlMs {
        TxCtlMs::from_bits(val)
    }
}
impl From<TxCtlMs> for u8 {
    #[inline(always)]
    fn from(val: TxCtlMs) -> u8 {
        TxCtlMs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxCtlWordLen {
    #[doc = "8-bit"]
    BIT_LEN8 = 0,
    #[doc = "16-bit"]
    BIT_LEN16 = 0x01,
    #[doc = "18-bit"]
    BIT_LEN18 = 0x02,
    #[doc = "20-bit"]
    BIT_LEN20 = 0x03,
    #[doc = "24-bit"]
    BIT_LEN24 = 0x04,
    #[doc = "32-bit"]
    BIT_LEN32 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TxCtlWordLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCtlWordLen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCtlWordLen {
    #[inline(always)]
    fn from(val: u8) -> TxCtlWordLen {
        TxCtlWordLen::from_bits(val)
    }
}
impl From<TxCtlWordLen> for u8 {
    #[inline(always)]
    fn from(val: TxCtlWordLen) -> u8 {
        TxCtlWordLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TxCtlWsPulse {
    #[doc = "Pulse width is 1 SCK period"]
    SCK_PERIOD = 0,
    #[doc = "Pulse width is 1 channel length"]
    CH_LENGTH = 0x01,
}
impl TxCtlWsPulse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCtlWsPulse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCtlWsPulse {
    #[inline(always)]
    fn from(val: u8) -> TxCtlWsPulse {
        TxCtlWsPulse::from_bits(val)
    }
}
impl From<TxCtlWsPulse> for u8 {
    #[inline(always)]
    fn from(val: TxCtlWsPulse) -> u8 {
        TxCtlWsPulse::to_bits(val)
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
