#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Timer/Counter/PWM Counter Module"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt {
    ptr: *mut u8,
}
unsafe impl Send for Cnt {}
unsafe impl Sync for Cnt {}
impl Cnt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Counter control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<CntCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Counter status register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Counter count register"]
    #[inline(always)]
    pub const fn counter(self) -> crate::common::Reg<Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Counter compare/capture register"]
    #[inline(always)]
    pub const fn cc(self) -> crate::common::Reg<Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Counter buffered compare/capture register"]
    #[inline(always)]
    pub const fn cc_buff(self) -> crate::common::Reg<CcBuff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Counter period register"]
    #[inline(always)]
    pub const fn period(self) -> crate::common::Reg<Period, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Counter buffered period register"]
    #[inline(always)]
    pub const fn period_buff(self) -> crate::common::Reg<PeriodBuff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Counter trigger control register 0"]
    #[inline(always)]
    pub const fn tr_ctrl0(self) -> crate::common::Reg<TrCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Counter trigger control register 1"]
    #[inline(always)]
    pub const fn tr_ctrl1(self) -> crate::common::Reg<TrCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Counter trigger control register 2"]
    #[inline(always)]
    pub const fn tr_ctrl2(self) -> crate::common::Reg<TrCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Interrupt request register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
}
#[doc = "Timer/Counter/PWM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcpwm0 {
    ptr: *mut u8,
}
unsafe impl Send for Tcpwm0 {}
unsafe impl Sync for Tcpwm0 {}
impl Tcpwm0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TCPWM control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<Tcpwm0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "TCPWM control clear register"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<CtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "TCPWM control set register"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<CtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "TCPWM capture command register"]
    #[inline(always)]
    pub const fn cmd_capture(self) -> crate::common::Reg<CmdCapture, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "TCPWM reload command register"]
    #[inline(always)]
    pub const fn cmd_reload(self) -> crate::common::Reg<CmdReload, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "TCPWM stop command register"]
    #[inline(always)]
    pub const fn cmd_stop(self) -> crate::common::Reg<CmdStop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "TCPWM start command register"]
    #[inline(always)]
    pub const fn cmd_start(self) -> crate::common::Reg<CmdStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "TCPWM Counter interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(self) -> crate::common::Reg<IntrCause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Timer/Counter/PWM Counter Module"]
    #[inline(always)]
    pub const fn cnt(self, n: usize) -> Cnt {
        assert!(n < 8usize);
        unsafe { Cnt::from_ptr(self.ptr.add(256usize + n * 64usize) as _) }
    }
}
#[doc = "Counter compare/capture register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc = "In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
    #[inline(always)]
    pub const fn cc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
    #[inline(always)]
    pub fn set_cc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cc {
    #[inline(always)]
    fn default() -> Cc {
        Cc(0)
    }
}
#[doc = "Counter buffered compare/capture register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcBuff(pub u32);
impl CcBuff {
    #[doc = "Additional buffer for counter CC register."]
    #[inline(always)]
    pub const fn cc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Additional buffer for counter CC register."]
    #[inline(always)]
    pub fn set_cc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CcBuff {
    #[inline(always)]
    fn default() -> CcBuff {
        CcBuff(0)
    }
}
#[doc = "TCPWM capture command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdCapture(pub u32);
impl CmdCapture {
    #[doc = "Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub const fn counter_capture(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn set_counter_capture(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdCapture {
    #[inline(always)]
    fn default() -> CmdCapture {
        CmdCapture(0)
    }
}
#[doc = "TCPWM reload command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdReload(pub u32);
impl CmdReload {
    #[doc = "Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub const fn counter_reload(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn set_counter_reload(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdReload {
    #[inline(always)]
    fn default() -> CmdReload {
        CmdReload(0)
    }
}
#[doc = "TCPWM start command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdStart(pub u32);
impl CmdStart {
    #[doc = "Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub const fn counter_start(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn set_counter_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdStart {
    #[inline(always)]
    fn default() -> CmdStart {
        CmdStart(0)
    }
}
#[doc = "TCPWM stop command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdStop(pub u32);
impl CmdStop {
    #[doc = "Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub const fn counter_stop(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn set_counter_stop(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdStop {
    #[inline(always)]
    fn default() -> CmdStop {
        CmdStop(0)
    }
}
#[doc = "Counter control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CntCtrl(pub u32);
impl CntCtrl {
    #[doc = "Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub const fn auto_reload_cc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn set_auto_reload_cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    pub const fn auto_reload_period(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    pub fn set_auto_reload_period(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub const fn pwm_sync_kill(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub fn set_pwm_sync_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub const fn pwm_stop_on_kill(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn set_pwm_stop_on_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    pub const fn generic(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    pub fn set_generic(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Determines counter direction."]
    #[inline(always)]
    pub const fn up_down_mode(&self) -> UpDownMode {
        let val = (self.0 >> 16usize) & 0x03;
        UpDownMode::from_bits(val as u8)
    }
    #[doc = "Determines counter direction."]
    #[inline(always)]
    pub fn set_up_down_mode(&mut self, val: UpDownMode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub const fn one_shot(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub fn set_one_shot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\] and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    pub const fn quadrature_mode(&self) -> QuadratureMode {
        let val = (self.0 >> 20usize) & 0x03;
        QuadratureMode::from_bits(val as u8)
    }
    #[doc = "In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\] and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    pub fn set_quadrature_mode(&mut self, val: QuadratureMode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Counter mode."]
    #[inline(always)]
    pub const fn mode(&self) -> Mode {
        let val = (self.0 >> 24usize) & 0x07;
        Mode::from_bits(val as u8)
    }
    #[doc = "Counter mode."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: Mode) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for CntCtrl {
    #[inline(always)]
    fn default() -> CntCtrl {
        CntCtrl(0)
    }
}
#[doc = "Counter count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Counter(pub u32);
impl Counter {
    #[doc = "16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub const fn counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub fn set_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Counter {
    #[inline(always)]
    fn default() -> Counter {
        Counter(0)
    }
}
#[doc = "TCPWM control clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlClr(pub u32);
impl CtrlClr {
    #[doc = "Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub const fn counter_enabled(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn set_counter_enabled(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CtrlClr {
    #[inline(always)]
    fn default() -> CtrlClr {
        CtrlClr(0)
    }
}
#[doc = "TCPWM control set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlSet(pub u32);
impl CtrlSet {
    #[doc = "Alias of CTRL that only allows enabling of counters. A write access: '0': Does nothing. '1': Sets respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub const fn counter_enabled(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Alias of CTRL that only allows enabling of counters. A write access: '0': Does nothing. '1': Sets respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn set_counter_enabled(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CtrlSet {
    #[inline(always)]
    fn default() -> CtrlSet {
        CtrlSet(0)
    }
}
#[doc = "Interrupt request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn cc_match(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_cc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "TCPWM Counter interrupt cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause(pub u32);
impl IntrCause {
    #[doc = "Counters interrupt signal active. If the counter is disabled through CTRL.COUNTER_ENABLED, the associated interrupt field is immediately set to '0'."]
    #[inline(always)]
    pub const fn counter_int(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counters interrupt signal active. If the counter is disabled through CTRL.COUNTER_ENABLED, the associated interrupt field is immediately set to '0'."]
    #[inline(always)]
    pub fn set_counter_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IntrCause {
    #[inline(always)]
    fn default() -> IntrCause {
        IntrCause(0)
    }
}
#[doc = "Interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn cc_match(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_cc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn cc_match(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_cc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn cc_match(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_cc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Counter period register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Period(pub u32);
impl Period {
    #[doc = "Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    pub const fn period(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    pub fn set_period(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Period {
    #[inline(always)]
    fn default() -> Period {
        Period(0)
    }
}
#[doc = "Counter buffered period register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PeriodBuff(pub u32);
impl PeriodBuff {
    #[doc = "Additional buffer for counter PERIOD register."]
    #[inline(always)]
    pub const fn period(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Additional buffer for counter PERIOD register."]
    #[inline(always)]
    pub fn set_period(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PeriodBuff {
    #[inline(always)]
    fn default() -> PeriodBuff {
        PeriodBuff(0)
    }
}
#[doc = "Counter status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
    #[inline(always)]
    pub const fn down(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
    #[inline(always)]
    pub fn set_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
    #[inline(always)]
    pub const fn generic(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
    #[inline(always)]
    pub fn set_generic(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "When '0', the counter is NOT running. When '1', the counter is running."]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When '0', the counter is NOT running. When '1', the counter is running."]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "TCPWM control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcpwm0Ctrl(pub u32);
impl Tcpwm0Ctrl {
    #[doc = "Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
    #[inline(always)]
    pub const fn counter_enabled(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
    #[inline(always)]
    pub fn set_counter_enabled(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcpwm0Ctrl {
    #[inline(always)]
    fn default() -> Tcpwm0Ctrl {
        Tcpwm0Ctrl(0)
    }
}
#[doc = "Counter trigger control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCtrl0(pub u32);
impl TrCtrl0 {
    #[doc = "Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub const fn capture_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn set_capture_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub const fn count_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub fn set_count_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with 0x8000 (counter midpoint)."]
    #[inline(always)]
    pub const fn reload_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with 0x8000 (counter midpoint)."]
    #[inline(always)]
    pub fn set_reload_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub const fn stop_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn set_stop_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub const fn start_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn set_start_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TrCtrl0 {
    #[inline(always)]
    fn default() -> TrCtrl0 {
        TrCtrl0(0)
    }
}
#[doc = "Counter trigger control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCtrl1(pub u32);
impl TrCtrl1 {
    #[doc = "A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    pub const fn capture_edge(&self) -> CaptureEdge {
        let val = (self.0 >> 0usize) & 0x03;
        CaptureEdge::from_bits(val as u8)
    }
    #[doc = "A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    pub fn set_capture_edge(&mut self, val: CaptureEdge) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub const fn count_edge(&self) -> CountEdge {
        let val = (self.0 >> 2usize) & 0x03;
        CountEdge::from_bits(val as u8)
    }
    #[doc = "A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub fn set_count_edge(&mut self, val: CountEdge) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub const fn reload_edge(&self) -> ReloadEdge {
        let val = (self.0 >> 4usize) & 0x03;
        ReloadEdge::from_bits(val as u8)
    }
    #[doc = "A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub fn set_reload_edge(&mut self, val: ReloadEdge) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub const fn stop_edge(&self) -> StopEdge {
        let val = (self.0 >> 6usize) & 0x03;
        StopEdge::from_bits(val as u8)
    }
    #[doc = "A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub fn set_stop_edge(&mut self, val: StopEdge) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub const fn start_edge(&self) -> StartEdge {
        let val = (self.0 >> 8usize) & 0x03;
        StartEdge::from_bits(val as u8)
    }
    #[doc = "A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub fn set_start_edge(&mut self, val: StartEdge) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for TrCtrl1 {
    #[inline(always)]
    fn default() -> TrCtrl1 {
        TrCtrl1(0)
    }
}
#[doc = "Counter trigger control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCtrl2(pub u32);
impl TrCtrl2 {
    #[doc = "Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub const fn cc_match_mode(&self) -> CcMatchMode {
        let val = (self.0 >> 0usize) & 0x03;
        CcMatchMode::from_bits(val as u8)
    }
    #[doc = "Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub fn set_cc_match_mode(&mut self, val: CcMatchMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    pub const fn overflow_mode(&self) -> OverflowMode {
        let val = (self.0 >> 2usize) & 0x03;
        OverflowMode::from_bits(val as u8)
    }
    #[doc = "Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn set_overflow_mode(&mut self, val: OverflowMode) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    pub const fn underflow_mode(&self) -> UnderflowMode {
        let val = (self.0 >> 4usize) & 0x03;
        UnderflowMode::from_bits(val as u8)
    }
    #[doc = "Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    pub fn set_underflow_mode(&mut self, val: UnderflowMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for TrCtrl2 {
    #[inline(always)]
    fn default() -> TrCtrl2 {
        TrCtrl2(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CaptureEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl CaptureEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CaptureEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CaptureEdge {
    #[inline(always)]
    fn from(val: u8) -> CaptureEdge {
        CaptureEdge::from_bits(val)
    }
}
impl From<CaptureEdge> for u8 {
    #[inline(always)]
    fn from(val: CaptureEdge) -> u8 {
        CaptureEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CcMatchMode {
    #[doc = "Set to '1'"]
    SET = 0,
    #[doc = "Set to '0'"]
    CLEAR = 0x01,
    #[doc = "Invert"]
    INVERT = 0x02,
    #[doc = "No Change"]
    NO_CHANGE = 0x03,
}
impl CcMatchMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcMatchMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcMatchMode {
    #[inline(always)]
    fn from(val: u8) -> CcMatchMode {
        CcMatchMode::from_bits(val)
    }
}
impl From<CcMatchMode> for u8 {
    #[inline(always)]
    fn from(val: CcMatchMode) -> u8 {
        CcMatchMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CountEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl CountEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CountEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CountEdge {
    #[inline(always)]
    fn from(val: u8) -> CountEdge {
        CountEdge::from_bits(val)
    }
}
impl From<CountEdge> for u8 {
    #[inline(always)]
    fn from(val: CountEdge) -> u8 {
        CountEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Timer mode"]
    TIMER = 0,
    _RESERVED_1 = 0x01,
    #[doc = "Capture mode"]
    CAPTURE = 0x02,
    #[doc = "Quadrature encoding mode"]
    QUAD = 0x03,
    #[doc = "Pulse width modulation (PWM) mode"]
    PWM = 0x04,
    #[doc = "PWM with deadtime insertion mode"]
    PWM_DT = 0x05,
    #[doc = "Pseudo random pulse width modulation"]
    PWM_PR = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OverflowMode {
    #[doc = "Set to '1'"]
    SET = 0,
    #[doc = "Set to '0'"]
    CLEAR = 0x01,
    #[doc = "Invert"]
    INVERT = 0x02,
    #[doc = "No Change"]
    NO_CHANGE = 0x03,
}
impl OverflowMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OverflowMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OverflowMode {
    #[inline(always)]
    fn from(val: u8) -> OverflowMode {
        OverflowMode::from_bits(val)
    }
}
impl From<OverflowMode> for u8 {
    #[inline(always)]
    fn from(val: OverflowMode) -> u8 {
        OverflowMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum QuadratureMode {
    #[doc = "X1 encoding (QUAD mode)"]
    X1 = 0,
    #[doc = "X2 encoding (QUAD mode)"]
    X2 = 0x01,
    #[doc = "X4 encoding (QUAD mode)"]
    X4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl QuadratureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QuadratureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QuadratureMode {
    #[inline(always)]
    fn from(val: u8) -> QuadratureMode {
        QuadratureMode::from_bits(val)
    }
}
impl From<QuadratureMode> for u8 {
    #[inline(always)]
    fn from(val: QuadratureMode) -> u8 {
        QuadratureMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReloadEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl ReloadEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReloadEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReloadEdge {
    #[inline(always)]
    fn from(val: u8) -> ReloadEdge {
        ReloadEdge::from_bits(val)
    }
}
impl From<ReloadEdge> for u8 {
    #[inline(always)]
    fn from(val: ReloadEdge) -> u8 {
        ReloadEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StartEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl StartEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StartEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StartEdge {
    #[inline(always)]
    fn from(val: u8) -> StartEdge {
        StartEdge::from_bits(val)
    }
}
impl From<StartEdge> for u8 {
    #[inline(always)]
    fn from(val: StartEdge) -> u8 {
        StartEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StopEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl StopEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StopEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StopEdge {
    #[inline(always)]
    fn from(val: u8) -> StopEdge {
        StopEdge::from_bits(val)
    }
}
impl From<StopEdge> for u8 {
    #[inline(always)]
    fn from(val: StopEdge) -> u8 {
        StopEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UnderflowMode {
    #[doc = "Set to '1'"]
    SET = 0,
    #[doc = "Set to '0'"]
    CLEAR = 0x01,
    #[doc = "Invert"]
    INVERT = 0x02,
    #[doc = "No Change"]
    NO_CHANGE = 0x03,
}
impl UnderflowMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnderflowMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnderflowMode {
    #[inline(always)]
    fn from(val: u8) -> UnderflowMode {
        UnderflowMode::from_bits(val)
    }
}
impl From<UnderflowMode> for u8 {
    #[inline(always)]
    fn from(val: UnderflowMode) -> u8 {
        UnderflowMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UpDownMode {
    #[doc = "Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    COUNT_UP = 0,
    #[doc = "Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_DOWN = 0x01,
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_UPDN1 = 0x02,
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    COUNT_UPDN2 = 0x03,
}
impl UpDownMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UpDownMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UpDownMode {
    #[inline(always)]
    fn from(val: u8) -> UpDownMode {
        UpDownMode::from_bits(val)
    }
}
impl From<UpDownMode> for u8 {
    #[inline(always)]
    fn from(val: UpDownMode) -> u8 {
        UpDownMode::to_bits(val)
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
