#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "CPU subsystem (CPUSS)"]
use core::prelude::v1::derive;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuss {
    ptr: *mut u8,
}
unsafe impl Send for Cpuss {}
unsafe impl Sync for Cpuss {}
impl Cpuss {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CM0+ control"]
    #[inline(always)]
    pub const fn cm0_ctl(self) -> crate::common::Reg<Cm0Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "CM0+ status"]
    #[inline(always)]
    pub const fn cm0_status(self) -> crate::common::Reg<Cm0Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "CM0+ clock control"]
    #[inline(always)]
    pub const fn cm0_clock_ctl(self) -> crate::common::Reg<Cm0ClockCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "CM0+ interrupt control 0"]
    #[inline(always)]
    pub const fn cm0_int_ctl0(self) -> crate::common::Reg<Cm0IntCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "CM0+ interrupt control 1"]
    #[inline(always)]
    pub const fn cm0_int_ctl1(self) -> crate::common::Reg<Cm0IntCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "CM0+ interrupt control 2"]
    #[inline(always)]
    pub const fn cm0_int_ctl2(self) -> crate::common::Reg<Cm0IntCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "CM0+ interrupt control 3"]
    #[inline(always)]
    pub const fn cm0_int_ctl3(self) -> crate::common::Reg<Cm0IntCtl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "CM0+ interrupt control 4"]
    #[inline(always)]
    pub const fn cm0_int_ctl4(self) -> crate::common::Reg<Cm0IntCtl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "CM0+ interrupt control 5"]
    #[inline(always)]
    pub const fn cm0_int_ctl5(self) -> crate::common::Reg<Cm0IntCtl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "CM0+ interrupt control 6"]
    #[inline(always)]
    pub const fn cm0_int_ctl6(self) -> crate::common::Reg<Cm0IntCtl6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "CM0+ interrupt control 7"]
    #[inline(always)]
    pub const fn cm0_int_ctl7(self) -> crate::common::Reg<Cm0IntCtl7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "CM4 power control"]
    #[inline(always)]
    pub const fn cm4_pwr_ctl(self) -> crate::common::Reg<Cm4PwrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "CM4 power control"]
    #[inline(always)]
    pub const fn cm4_pwr_delay_ctl(self) -> crate::common::Reg<Cm4PwrDelayCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "CM4 status"]
    #[inline(always)]
    pub const fn cm4_status(self) -> crate::common::Reg<Cm4Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "CM4 clock control"]
    #[inline(always)]
    pub const fn cm4_clock_ctl(self) -> crate::common::Reg<Cm4ClockCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "CM4 NMI control"]
    #[inline(always)]
    pub const fn cm4_nmi_ctl(self) -> crate::common::Reg<Cm4NmiCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "RAM 0 control 0"]
    #[inline(always)]
    pub const fn ram0_ctl0(self) -> crate::common::Reg<Ram0Ctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "RAM 0 power control"]
    #[inline(always)]
    pub const fn ram0_pwr_macro_ctl(
        self,
        n: usize,
    ) -> crate::common::Reg<Ram0PwrMacroCtl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(320usize + n * 4usize) as _) }
    }
    #[doc = "RAM 1 control 0"]
    #[inline(always)]
    pub const fn ram1_ctl0(self) -> crate::common::Reg<Ram1Ctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "RAM1 power control"]
    #[inline(always)]
    pub const fn ram1_pwr_ctl(self) -> crate::common::Reg<Ram1PwrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(400usize) as _) }
    }
    #[doc = "RAM 2 control 0"]
    #[inline(always)]
    pub const fn ram2_ctl0(self) -> crate::common::Reg<Ram2Ctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(416usize) as _) }
    }
    #[doc = "RAM2 power control"]
    #[inline(always)]
    pub const fn ram2_pwr_ctl(self) -> crate::common::Reg<Ram2PwrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(432usize) as _) }
    }
    #[doc = "Power up delay used for all SRAM power domains"]
    #[inline(always)]
    pub const fn ram_pwr_delay_ctl(self) -> crate::common::Reg<RamPwrDelayCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(448usize) as _) }
    }
    #[doc = "ROM control"]
    #[inline(always)]
    pub const fn rom_ctl(self) -> crate::common::Reg<RomCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(464usize) as _) }
    }
    #[doc = "UDB power control"]
    #[inline(always)]
    pub const fn udb_pwr_ctl(self) -> crate::common::Reg<UdbPwrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(496usize) as _) }
    }
    #[doc = "UDB power control"]
    #[inline(always)]
    pub const fn udb_pwr_delay_ctl(self) -> crate::common::Reg<UdbPwrDelayCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(500usize) as _) }
    }
    #[doc = "Debug port status"]
    #[inline(always)]
    pub const fn dp_status(self) -> crate::common::Reg<DpStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[doc = "Buffer control"]
    #[inline(always)]
    pub const fn buff_ctl(self) -> crate::common::Reg<BuffCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize) as _) }
    }
    #[doc = "DDFT control"]
    #[inline(always)]
    pub const fn ddft_ctl(self) -> crate::common::Reg<DdftCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(560usize) as _) }
    }
    #[doc = "SysTick timer control"]
    #[inline(always)]
    pub const fn systick_ctl(self) -> crate::common::Reg<SystickCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(576usize) as _) }
    }
    #[doc = "CM0+ vector table base"]
    #[inline(always)]
    pub const fn cm0_vector_table_base(
        self,
    ) -> crate::common::Reg<Cm0VectorTableBase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(688usize) as _) }
    }
    #[doc = "CM4 vector table base"]
    #[inline(always)]
    pub const fn cm4_vector_table_base(
        self,
    ) -> crate::common::Reg<Cm4VectorTableBase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(704usize) as _) }
    }
    #[doc = "CM0+ protection context 0 handler"]
    #[inline(always)]
    pub const fn cm0_pc0_handler(self) -> crate::common::Reg<Cm0Pc0Handler, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(800usize) as _) }
    }
    #[doc = "Identity"]
    #[inline(always)]
    pub const fn identity(self) -> crate::common::Reg<Identity, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "Protection status"]
    #[inline(always)]
    pub const fn protection(self) -> crate::common::Reg<Protection, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1280usize) as _) }
    }
    #[doc = "CM0+ NMI control"]
    #[inline(always)]
    pub const fn cm0_nmi_ctl(self) -> crate::common::Reg<Cm0NmiCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1312usize) as _) }
    }
    #[doc = "Access port control"]
    #[inline(always)]
    pub const fn ap_ctl(self) -> crate::common::Reg<ApCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1344usize) as _) }
    }
    #[doc = "Memory BIST status"]
    #[inline(always)]
    pub const fn mbist_stat(self) -> crate::common::Reg<MbistStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1440usize) as _) }
    }
    #[doc = "ROM trim control"]
    #[inline(always)]
    pub const fn trim_rom_ctl(self) -> crate::common::Reg<TrimRomCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(61440usize) as _) }
    }
    #[doc = "RAM trim control"]
    #[inline(always)]
    pub const fn trim_ram_ctl(self) -> crate::common::Reg<TrimRamCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(61444usize) as _) }
    }
}
#[doc = "Access port control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApCtl(pub u32);
impl ApCtl {
    #[doc = "Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn cm0_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_cm0_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn cm4_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_cm4_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn sys_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_sys_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub const fn cm0_disable(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub fn set_cm0_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub const fn cm4_disable(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub fn set_cm4_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub const fn sys_disable(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub fn set_sys_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for ApCtl {
    #[inline(always)]
    fn default() -> ApCtl {
        ApCtl(0)
    }
}
#[doc = "Buffer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BuffCtl(pub u32);
impl BuffCtl {
    #[doc = "Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    pub const fn write_buff(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    pub fn set_write_buff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for BuffCtl {
    #[inline(always)]
    fn default() -> BuffCtl {
        BuffCtl(0)
    }
}
#[doc = "CM0+ clock control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0ClockCtl(pub u32);
impl Cm0ClockCtl {
    #[doc = "Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn slow_int_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_slow_int_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi <= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
    #[inline(always)]
    pub const fn peri_int_div(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi <= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
    #[inline(always)]
    pub fn set_peri_int_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cm0ClockCtl {
    #[inline(always)]
    fn default() -> Cm0ClockCtl {
        Cm0ClockCtl(0)
    }
}
#[doc = "CM0+ control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0Ctl(pub u32);
impl Cm0Ctl {
    #[doc = "Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
    #[inline(always)]
    pub const fn slv_stall(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
    #[inline(always)]
    pub fn set_slv_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn set_vectkeystat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cm0Ctl {
    #[inline(always)]
    fn default() -> Cm0Ctl {
        Cm0Ctl(0)
    }
}
#[doc = "CM0+ interrupt control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0IntCtl0(pub u32);
impl Cm0IntCtl0 {
    #[doc = "System interrupt select for CPU interrupt source 0. If the field value is 240, no system interrupt is connected and the CPU interrupt source is always '0'/de-activated."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 0. If the field value is 240, no system interrupt is connected and the CPU interrupt source is always '0'/de-activated."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 1."]
    #[inline(always)]
    pub const fn mux1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 1."]
    #[inline(always)]
    pub fn set_mux1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 2."]
    #[inline(always)]
    pub const fn mux2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 2."]
    #[inline(always)]
    pub fn set_mux2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 3."]
    #[inline(always)]
    pub const fn mux3_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 3."]
    #[inline(always)]
    pub fn set_mux3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cm0IntCtl0 {
    #[inline(always)]
    fn default() -> Cm0IntCtl0 {
        Cm0IntCtl0(0)
    }
}
#[doc = "CM0+ interrupt control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0IntCtl1(pub u32);
impl Cm0IntCtl1 {
    #[doc = "System interrupt select for CPU interrupt source 4."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 4."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 5."]
    #[inline(always)]
    pub const fn mux1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 5."]
    #[inline(always)]
    pub fn set_mux1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 6."]
    #[inline(always)]
    pub const fn mux2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 6."]
    #[inline(always)]
    pub fn set_mux2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 7."]
    #[inline(always)]
    pub const fn mux3_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 7."]
    #[inline(always)]
    pub fn set_mux3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cm0IntCtl1 {
    #[inline(always)]
    fn default() -> Cm0IntCtl1 {
        Cm0IntCtl1(0)
    }
}
#[doc = "CM0+ interrupt control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0IntCtl2(pub u32);
impl Cm0IntCtl2 {
    #[doc = "System interrupt select for CPU interrupt source 8."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 8."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 9."]
    #[inline(always)]
    pub const fn mux1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 9."]
    #[inline(always)]
    pub fn set_mux1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 10."]
    #[inline(always)]
    pub const fn mux2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 10."]
    #[inline(always)]
    pub fn set_mux2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 11."]
    #[inline(always)]
    pub const fn mux3_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 11."]
    #[inline(always)]
    pub fn set_mux3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cm0IntCtl2 {
    #[inline(always)]
    fn default() -> Cm0IntCtl2 {
        Cm0IntCtl2(0)
    }
}
#[doc = "CM0+ interrupt control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0IntCtl3(pub u32);
impl Cm0IntCtl3 {
    #[doc = "System interrupt select for CPU interrupt source 12."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 12."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 13."]
    #[inline(always)]
    pub const fn mux1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 13."]
    #[inline(always)]
    pub fn set_mux1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 14."]
    #[inline(always)]
    pub const fn mux2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 14."]
    #[inline(always)]
    pub fn set_mux2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 15."]
    #[inline(always)]
    pub const fn mux3_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 15."]
    #[inline(always)]
    pub fn set_mux3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cm0IntCtl3 {
    #[inline(always)]
    fn default() -> Cm0IntCtl3 {
        Cm0IntCtl3(0)
    }
}
#[doc = "CM0+ interrupt control 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0IntCtl4(pub u32);
impl Cm0IntCtl4 {
    #[doc = "System interrupt select for CPU interrupt source 16."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 16."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 17."]
    #[inline(always)]
    pub const fn mux1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 17."]
    #[inline(always)]
    pub fn set_mux1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 18."]
    #[inline(always)]
    pub const fn mux2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 18."]
    #[inline(always)]
    pub fn set_mux2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 19."]
    #[inline(always)]
    pub const fn mux3_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 19."]
    #[inline(always)]
    pub fn set_mux3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cm0IntCtl4 {
    #[inline(always)]
    fn default() -> Cm0IntCtl4 {
        Cm0IntCtl4(0)
    }
}
#[doc = "CM0+ interrupt control 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0IntCtl5(pub u32);
impl Cm0IntCtl5 {
    #[doc = "System interrupt select for CPU interrupt source 20."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 20."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 21."]
    #[inline(always)]
    pub const fn mux1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 21."]
    #[inline(always)]
    pub fn set_mux1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 22."]
    #[inline(always)]
    pub const fn mux2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 22."]
    #[inline(always)]
    pub fn set_mux2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 23."]
    #[inline(always)]
    pub const fn mux3_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 23."]
    #[inline(always)]
    pub fn set_mux3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cm0IntCtl5 {
    #[inline(always)]
    fn default() -> Cm0IntCtl5 {
        Cm0IntCtl5(0)
    }
}
#[doc = "CM0+ interrupt control 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0IntCtl6(pub u32);
impl Cm0IntCtl6 {
    #[doc = "System interrupt select for CPU interrupt source 24."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 24."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 25."]
    #[inline(always)]
    pub const fn mux1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 25."]
    #[inline(always)]
    pub fn set_mux1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 26."]
    #[inline(always)]
    pub const fn mux2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 26."]
    #[inline(always)]
    pub fn set_mux2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 27."]
    #[inline(always)]
    pub const fn mux3_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 27."]
    #[inline(always)]
    pub fn set_mux3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cm0IntCtl6 {
    #[inline(always)]
    fn default() -> Cm0IntCtl6 {
        Cm0IntCtl6(0)
    }
}
#[doc = "CM0+ interrupt control 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0IntCtl7(pub u32);
impl Cm0IntCtl7 {
    #[doc = "System interrupt select for CPU interrupt source 28."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 28."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 29."]
    #[inline(always)]
    pub const fn mux1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 29."]
    #[inline(always)]
    pub fn set_mux1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 30."]
    #[inline(always)]
    pub const fn mux2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 30."]
    #[inline(always)]
    pub fn set_mux2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "System interrupt select for CPU interrupt source 31."]
    #[inline(always)]
    pub const fn mux3_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU interrupt source 31."]
    #[inline(always)]
    pub fn set_mux3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cm0IntCtl7 {
    #[inline(always)]
    fn default() -> Cm0IntCtl7 {
        Cm0IntCtl7(0)
    }
}
#[doc = "CM0+ NMI control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0NmiCtl(pub u32);
impl Cm0NmiCtl {
    #[doc = "System interrupt select for CPU NMI. The reset value ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU NMI. The reset value ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cm0NmiCtl {
    #[inline(always)]
    fn default() -> Cm0NmiCtl {
        Cm0NmiCtl(0)
    }
}
#[doc = "CM0+ protection context 0 handler"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0Pc0Handler(pub u32);
impl Cm0Pc0Handler {
    #[doc = "Address of the protection context 0 handler. This field is used to detect entry to Cypress 'trusted' code through an exception/interrupt."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of the protection context 0 handler. This field is used to detect entry to Cypress 'trusted' code through an exception/interrupt."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm0Pc0Handler {
    #[inline(always)]
    fn default() -> Cm0Pc0Handler {
        Cm0Pc0Handler(0)
    }
}
#[doc = "CM0+ status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0Status(pub u32);
impl Cm0Status {
    #[doc = "Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
    #[inline(always)]
    pub const fn sleeping(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
    #[inline(always)]
    pub fn set_sleeping(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
    #[inline(always)]
    pub const fn sleepdeep(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
    #[inline(always)]
    pub fn set_sleepdeep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Cm0Status {
    #[inline(always)]
    fn default() -> Cm0Status {
        Cm0Status(0)
    }
}
#[doc = "CM0+ vector table base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0VectorTableBase(pub u32);
impl Cm0VectorTableBase {
    #[doc = "Address of CM0+ vector table. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    pub const fn addr24(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Address of CM0+ vector table. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    pub fn set_addr24(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Cm0VectorTableBase {
    #[inline(always)]
    fn default() -> Cm0VectorTableBase {
        Cm0VectorTableBase(0)
    }
}
#[doc = "CM4 clock control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4ClockCtl(pub u32);
impl Cm4ClockCtl {
    #[doc = "Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub const fn fast_int_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn set_fast_int_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Cm4ClockCtl {
    #[inline(always)]
    fn default() -> Cm4ClockCtl {
        Cm4ClockCtl(0)
    }
}
#[doc = "CM4 NMI control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4NmiCtl(pub u32);
impl Cm4NmiCtl {
    #[doc = "System interrupt select for CPU NMI. The reset value ensure that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub const fn mux0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "System interrupt select for CPU NMI. The reset value ensure that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn set_mux0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cm4NmiCtl {
    #[inline(always)]
    fn default() -> Cm4NmiCtl {
        Cm4NmiCtl(0)
    }
}
#[doc = "CM4 power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4PwrCtl(pub u32);
impl Cm4PwrCtl {
    #[doc = "Set Power mode for CM4"]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> Cm4PwrCtlPwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        Cm4PwrCtlPwrMode::from_bits(val as u8)
    }
    #[doc = "Set Power mode for CM4"]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: Cm4PwrCtlPwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn set_vectkeystat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cm4PwrCtl {
    #[inline(always)]
    fn default() -> Cm4PwrCtl {
        Cm4PwrCtl(0)
    }
}
#[doc = "CM4 power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4PwrDelayCtl(pub u32);
impl Cm4PwrDelayCtl {
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub const fn up(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn set_up(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Cm4PwrDelayCtl {
    #[inline(always)]
    fn default() -> Cm4PwrDelayCtl {
        Cm4PwrDelayCtl(0)
    }
}
#[doc = "CM4 status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4Status(pub u32);
impl Cm4Status {
    #[doc = "Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
    #[inline(always)]
    pub const fn sleeping(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
    #[inline(always)]
    pub fn set_sleeping(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
    #[inline(always)]
    pub const fn sleepdeep(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
    #[inline(always)]
    pub fn set_sleepdeep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "After a PWR_MODE change this flag indicates if the new power mode has taken effect or not. Note: this flag can also change as a result of a change in debug power up req"]
    #[inline(always)]
    pub const fn pwr_done(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "After a PWR_MODE change this flag indicates if the new power mode has taken effect or not. Note: this flag can also change as a result of a change in debug power up req"]
    #[inline(always)]
    pub fn set_pwr_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Cm4Status {
    #[inline(always)]
    fn default() -> Cm4Status {
        Cm4Status(0)
    }
}
#[doc = "CM4 vector table base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm4VectorTableBase(pub u32);
impl Cm4VectorTableBase {
    #[doc = "Address of CM4 vector table. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
    #[inline(always)]
    pub const fn addr22(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Address of CM4 vector table. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
    #[inline(always)]
    pub fn set_addr22(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Cm4VectorTableBase {
    #[inline(always)]
    fn default() -> Cm4VectorTableBase {
        Cm4VectorTableBase(0)
    }
}
#[doc = "DDFT control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DdftCtl(pub u32);
impl DdftCtl {
    #[doc = "Select signal for CPUSS DDFT\\[0\\] 0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\] (Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\] (Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\] (Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub const fn ddft_out0_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Select signal for CPUSS DDFT\\[0\\] 0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\] (Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\] (Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\] (Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn set_ddft_out0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Select signal for CPUSS DDFT\\[0\\] 0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\] (Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\] (Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\] (Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub const fn ddft_out1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Select signal for CPUSS DDFT\\[0\\] 0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\] (Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\] (Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\] (Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn set_ddft_out1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for DdftCtl {
    #[inline(always)]
    fn default() -> DdftCtl {
        DdftCtl(0)
    }
}
#[doc = "Debug port status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpStatus(pub u32);
impl DpStatus {
    #[doc = "Specifies if the SWJ debug port is connected; i.e. debug host interface is active: '0': Not connected/not active. '1': Connected/active."]
    #[inline(always)]
    pub const fn swj_connected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the SWJ debug port is connected; i.e. debug host interface is active: '0': Not connected/not active. '1': Connected/active."]
    #[inline(always)]
    pub fn set_swj_connected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Specifies if SWJ debug is enabled, i.e. CDBGPWRUPACK is '1' and thus debug clocks are on: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn swj_debug_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if SWJ debug is enabled, i.e. CDBGPWRUPACK is '1' and thus debug clocks are on: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_swj_debug_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Specifies if the JTAG or SWD interface is selected. This signal is valid when DP_CTL.PTM_SEL is '0' (SWJ mode selected) and SWJ_CONNECTED is '1' (SWJ is connected). '0': SWD selected. '1': JTAG selected."]
    #[inline(always)]
    pub const fn swj_jtag_sel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the JTAG or SWD interface is selected. This signal is valid when DP_CTL.PTM_SEL is '0' (SWJ mode selected) and SWJ_CONNECTED is '1' (SWJ is connected). '0': SWD selected. '1': JTAG selected."]
    #[inline(always)]
    pub fn set_swj_jtag_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for DpStatus {
    #[inline(always)]
    fn default() -> DpStatus {
        DpStatus(0)
    }
}
#[doc = "Identity"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Identity(pub u32);
impl Identity {
    #[doc = "This field specifies the privileged setting ('0': user mode; '1': privileged mode) of the transfer that reads the register."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies the privileged setting ('0': user mode; '1': privileged mode) of the transfer that reads the register."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This field specifies the security setting ('0': secure mode; '1': non-secure mode) of the transfer that reads the register."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies the security setting ('0': secure mode; '1': non-secure mode) of the transfer that reads the register."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This field specifies the protection context of the transfer that reads the register."]
    #[inline(always)]
    pub const fn pc(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "This field specifies the protection context of the transfer that reads the register."]
    #[inline(always)]
    pub fn set_pc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "This field specifies the bus master identifier of the transfer that reads the register."]
    #[inline(always)]
    pub const fn ms(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "This field specifies the bus master identifier of the transfer that reads the register."]
    #[inline(always)]
    pub fn set_ms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Identity {
    #[inline(always)]
    fn default() -> Identity {
        Identity(0)
    }
}
#[doc = "Memory BIST status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistStat(pub u32);
impl MbistStat {
    #[doc = "Flag indicating the BIST run is done. Note that after starting a BIST run this flag must be set before a new run can be started. For the first BIST run this will be 0."]
    #[inline(always)]
    pub const fn sfp_ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicating the BIST run is done. Note that after starting a BIST run this flag must be set before a new run can be started. For the first BIST run this will be 0."]
    #[inline(always)]
    pub fn set_sfp_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Report status of the BIST run, only valid if SFP_READY=1"]
    #[inline(always)]
    pub const fn sfp_fail(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Report status of the BIST run, only valid if SFP_READY=1"]
    #[inline(always)]
    pub fn set_sfp_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for MbistStat {
    #[inline(always)]
    fn default() -> MbistStat {
        MbistStat(0)
    }
}
#[doc = "Protection status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Protection(pub u32);
impl Protection {
    #[doc = "Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    pub const fn state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    pub fn set_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Protection {
    #[inline(always)]
    fn default() -> Protection {
        Protection(0)
    }
}
#[doc = "RAM 0 control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram0Ctl0(pub u32);
impl Ram0Ctl0 {
    #[doc = "Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub const fn slow_ws(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn set_slow_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub const fn fast_ws(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn set_fast_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Ram0Ctl0 {
    #[inline(always)]
    fn default() -> Ram0Ctl0 {
        Ram0Ctl0(0)
    }
}
#[doc = "RAM 0 power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram0PwrMacroCtl(pub u32);
impl Ram0PwrMacroCtl {
    #[doc = "Set Power mode for 1 SRAM0 Macro"]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> Ram0PwrMacroCtlPwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        Ram0PwrMacroCtlPwrMode::from_bits(val as u8)
    }
    #[doc = "Set Power mode for 1 SRAM0 Macro"]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: Ram0PwrMacroCtlPwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn set_vectkeystat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ram0PwrMacroCtl {
    #[inline(always)]
    fn default() -> Ram0PwrMacroCtl {
        Ram0PwrMacroCtl(0)
    }
}
#[doc = "RAM 1 control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram1Ctl0(pub u32);
impl Ram1Ctl0 {
    #[doc = "Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub const fn slow_ws(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn set_slow_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub const fn fast_ws(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn set_fast_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Ram1Ctl0 {
    #[inline(always)]
    fn default() -> Ram1Ctl0 {
        Ram1Ctl0(0)
    }
}
#[doc = "RAM1 power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram1PwrCtl(pub u32);
impl Ram1PwrCtl {
    #[doc = "Set Power mode for SRAM1"]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> Ram1PwrCtlPwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        Ram1PwrCtlPwrMode::from_bits(val as u8)
    }
    #[doc = "Set Power mode for SRAM1"]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: Ram1PwrCtlPwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn set_vectkeystat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ram1PwrCtl {
    #[inline(always)]
    fn default() -> Ram1PwrCtl {
        Ram1PwrCtl(0)
    }
}
#[doc = "RAM 2 control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram2Ctl0(pub u32);
impl Ram2Ctl0 {
    #[doc = "Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub const fn slow_ws(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn set_slow_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub const fn fast_ws(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn set_fast_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Ram2Ctl0 {
    #[inline(always)]
    fn default() -> Ram2Ctl0 {
        Ram2Ctl0(0)
    }
}
#[doc = "RAM2 power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram2PwrCtl(pub u32);
impl Ram2PwrCtl {
    #[doc = "Set Power mode for SRAM2"]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> Ram2PwrCtlPwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        Ram2PwrCtlPwrMode::from_bits(val as u8)
    }
    #[doc = "Set Power mode for SRAM2"]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: Ram2PwrCtlPwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn set_vectkeystat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ram2PwrCtl {
    #[inline(always)]
    fn default() -> Ram2PwrCtl {
        Ram2PwrCtl(0)
    }
}
#[doc = "Power up delay used for all SRAM power domains"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamPwrDelayCtl(pub u32);
impl RamPwrDelayCtl {
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub const fn up(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn set_up(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for RamPwrDelayCtl {
    #[inline(always)]
    fn default() -> RamPwrDelayCtl {
        RamPwrDelayCtl(0)
    }
}
#[doc = "ROM control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RomCtl(pub u32);
impl RomCtl {
    #[doc = "Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
    #[inline(always)]
    pub const fn slow_ws(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
    #[inline(always)]
    pub fn set_slow_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub const fn fast_ws(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn set_fast_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for RomCtl {
    #[inline(always)]
    fn default() -> RomCtl {
        RomCtl(0)
    }
}
#[doc = "SysTick timer control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystickCtl(pub u32);
impl SystickCtl {
    #[doc = "Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
    #[inline(always)]
    pub fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
    #[inline(always)]
    pub const fn clock_source(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
    #[inline(always)]
    pub fn set_clock_source(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
    #[inline(always)]
    pub const fn skew(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
    #[inline(always)]
    pub fn set_skew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
    #[inline(always)]
    pub const fn noref(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
    #[inline(always)]
    pub fn set_noref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SystickCtl {
    #[inline(always)]
    fn default() -> SystickCtl {
        SystickCtl(0)
    }
}
#[doc = "RAM trim control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimRamCtl(pub u32);
impl TrimRamCtl {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external RM\\[3:0\\] Read-Write margin setting."]
    #[inline(always)]
    pub const fn rme(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external RM\\[3:0\\] Read-Write margin setting."]
    #[inline(always)]
    pub fn set_rme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write Assist Pulse to control pulse width of negative voltage on SRAM bitline."]
    #[inline(always)]
    pub const fn wpulse(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Write Assist Pulse to control pulse width of negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn set_wpulse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Read Assist control for WL under-drive."]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Read Assist control for WL under-drive."]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Write assist enable control (Active High). - WA\\[1:0\\] Write Assist pins to control negative voltage on SRAM bitline."]
    #[inline(always)]
    pub const fn wa(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Write assist enable control (Active High). - WA\\[1:0\\] Write Assist pins to control negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn set_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
}
impl Default for TrimRamCtl {
    #[inline(always)]
    fn default() -> TrimRamCtl {
        TrimRamCtl(0)
    }
}
#[doc = "ROM trim control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimRomCtl(pub u32);
impl TrimRomCtl {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external pin Read-Write margin setting."]
    #[inline(always)]
    pub const fn rme(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external pin Read-Write margin setting."]
    #[inline(always)]
    pub fn set_rme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for TrimRomCtl {
    #[inline(always)]
    fn default() -> TrimRomCtl {
        TrimRomCtl(0)
    }
}
#[doc = "UDB power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdbPwrCtl(pub u32);
impl UdbPwrCtl {
    #[doc = "Set Power mode for UDBs"]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> UdbPwrCtlPwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        UdbPwrCtlPwrMode::from_bits(val as u8)
    }
    #[doc = "Set Power mode for UDBs"]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: UdbPwrCtlPwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub const fn vectkeystat(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn set_vectkeystat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for UdbPwrCtl {
    #[inline(always)]
    fn default() -> UdbPwrCtl {
        UdbPwrCtl(0)
    }
}
#[doc = "UDB power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdbPwrDelayCtl(pub u32);
impl UdbPwrDelayCtl {
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub const fn up(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn set_up(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for UdbPwrDelayCtl {
    #[inline(always)]
    fn default() -> UdbPwrDelayCtl {
        UdbPwrDelayCtl(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cm4PwrCtlPwrMode {
    #[doc = "Switch CM4 off Power off, clock off, isolate, reset and no retain."]
    OFF = 0,
    #[doc = "Reset CM4 Clock off, no isolated, no retain and reset. Note: The CM4 CPU has a AIRCR.SYSRESETREQ register field that allows the CM4 to reset the complete device (RESET only resets the CM4), resulting in a warm boot."]
    RESET = 0x01,
    #[doc = "Put CM4 in Retained mode This can only become effective if CM4 is in SleepDeep mode. Check PWR_DONE flag to see if CM4 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    RETAINED = 0x02,
    #[doc = "Switch CM4 on. Power on, clock on, no isolate, no reset and no retain."]
    ENABLED = 0x03,
}
impl Cm4PwrCtlPwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm4PwrCtlPwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm4PwrCtlPwrMode {
    #[inline(always)]
    fn from(val: u8) -> Cm4PwrCtlPwrMode {
        Cm4PwrCtlPwrMode::from_bits(val)
    }
}
impl From<Cm4PwrCtlPwrMode> for u8 {
    #[inline(always)]
    fn from(val: Cm4PwrCtlPwrMode) -> u8 {
        Cm4PwrCtlPwrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ram0PwrMacroCtlPwrMode {
    #[doc = "See CM4_PWR_CTL"]
    OFF = 0,
    #[doc = "undefined"]
    RSVD = 0x01,
    #[doc = "See CM4_PWR_CTL"]
    RETAINED = 0x02,
    #[doc = "See CM4_PWR_CTL"]
    ENABLED = 0x03,
}
impl Ram0PwrMacroCtlPwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram0PwrMacroCtlPwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram0PwrMacroCtlPwrMode {
    #[inline(always)]
    fn from(val: u8) -> Ram0PwrMacroCtlPwrMode {
        Ram0PwrMacroCtlPwrMode::from_bits(val)
    }
}
impl From<Ram0PwrMacroCtlPwrMode> for u8 {
    #[inline(always)]
    fn from(val: Ram0PwrMacroCtlPwrMode) -> u8 {
        Ram0PwrMacroCtlPwrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ram1PwrCtlPwrMode {
    #[doc = "See CM4_PWR_CTL"]
    OFF = 0,
    #[doc = "undefined"]
    RSVD = 0x01,
    #[doc = "See CM4_PWR_CTL"]
    RETAINED = 0x02,
    #[doc = "See CM4_PWR_CTL"]
    ENABLED = 0x03,
}
impl Ram1PwrCtlPwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram1PwrCtlPwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram1PwrCtlPwrMode {
    #[inline(always)]
    fn from(val: u8) -> Ram1PwrCtlPwrMode {
        Ram1PwrCtlPwrMode::from_bits(val)
    }
}
impl From<Ram1PwrCtlPwrMode> for u8 {
    #[inline(always)]
    fn from(val: Ram1PwrCtlPwrMode) -> u8 {
        Ram1PwrCtlPwrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ram2PwrCtlPwrMode {
    #[doc = "See CM4_PWR_CTL"]
    OFF = 0,
    #[doc = "undefined"]
    RSVD = 0x01,
    #[doc = "See CM4_PWR_CTL"]
    RETAINED = 0x02,
    #[doc = "See CM4_PWR_CTL"]
    ENABLED = 0x03,
}
impl Ram2PwrCtlPwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram2PwrCtlPwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram2PwrCtlPwrMode {
    #[inline(always)]
    fn from(val: u8) -> Ram2PwrCtlPwrMode {
        Ram2PwrCtlPwrMode::from_bits(val)
    }
}
impl From<Ram2PwrCtlPwrMode> for u8 {
    #[inline(always)]
    fn from(val: Ram2PwrCtlPwrMode) -> u8 {
        Ram2PwrCtlPwrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UdbPwrCtlPwrMode {
    #[doc = "See CM4_PWR_CTL"]
    OFF = 0,
    #[doc = "See CM4_PWR_CTL"]
    RESET = 0x01,
    #[doc = "See CM4_PWR_CTL"]
    RETAINED = 0x02,
    #[doc = "See CM4_PWR_CTL"]
    ENABLED = 0x03,
}
impl UdbPwrCtlPwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UdbPwrCtlPwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UdbPwrCtlPwrMode {
    #[inline(always)]
    fn from(val: u8) -> UdbPwrCtlPwrMode {
        UdbPwrCtlPwrMode::from_bits(val)
    }
}
impl From<UdbPwrCtlPwrMode> for u8 {
    #[inline(always)]
    fn from(val: UdbPwrCtlPwrMode) -> u8 {
        UdbPwrCtlPwrMode::to_bits(val)
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
