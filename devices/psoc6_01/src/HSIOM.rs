#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "High Speed IO Matrix (HSIOM)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsiom {
    ptr: *mut u8,
}
unsafe impl Send for Hsiom {}
unsafe impl Sync for Hsiom {}
impl Hsiom {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "HSIOM port registers"]
    #[inline(always)]
    pub const fn prt(self, n: usize) -> Prt {
        assert!(n < 15usize);
        unsafe { Prt::from_ptr(self.ptr.add(0usize + n * 16usize) as _) }
    }
    #[doc = "AMUX splitter cell control"]
    #[inline(always)]
    pub const fn amux_split_ctl(
        self,
        n: usize,
    ) -> crate::common::Reg<AmuxSplitCtl, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8192usize + n * 4usize) as _) }
    }
}
#[doc = "HSIOM port registers"]
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
    #[doc = "Port selection 0"]
    #[inline(always)]
    pub const fn port_sel0(self) -> crate::common::Reg<PortSel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Port selection 1"]
    #[inline(always)]
    pub const fn port_sel1(self) -> crate::common::Reg<PortSel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
}
#[doc = "AMUX splitter cell control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AmuxSplitCtl(pub u32);
impl AmuxSplitCtl {
    #[doc = "T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub const fn switch_aa_sl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn set_switch_aa_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub const fn switch_aa_sr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn set_switch_aa_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub const fn switch_aa_s0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn set_switch_aa_s0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub const fn switch_bb_sl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub fn set_switch_bb_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub const fn switch_bb_sr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub fn set_switch_bb_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub const fn switch_bb_s0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub fn set_switch_bb_s0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for AmuxSplitCtl {
    #[inline(always)]
    fn default() -> AmuxSplitCtl {
        AmuxSplitCtl(0)
    }
}
#[doc = "Port selection 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortSel0(pub u32);
impl PortSel0 {
    #[doc = "Selects connection for IO pin 0 route."]
    #[inline(always)]
    pub const fn io0_sel(&self) -> Io0Sel {
        let val = (self.0 >> 0usize) & 0x1f;
        Io0Sel::from_bits(val as u8)
    }
    #[doc = "Selects connection for IO pin 0 route."]
    #[inline(always)]
    pub fn set_io0_sel(&mut self, val: Io0Sel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Selects connection for IO pin 1 route."]
    #[inline(always)]
    pub const fn io1_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects connection for IO pin 1 route."]
    #[inline(always)]
    pub fn set_io1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Selects connection for IO pin 2 route."]
    #[inline(always)]
    pub const fn io2_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects connection for IO pin 2 route."]
    #[inline(always)]
    pub fn set_io2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Selects connection for IO pin 3 route."]
    #[inline(always)]
    pub const fn io3_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects connection for IO pin 3 route."]
    #[inline(always)]
    pub fn set_io3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for PortSel0 {
    #[inline(always)]
    fn default() -> PortSel0 {
        PortSel0(0)
    }
}
#[doc = "Port selection 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortSel1(pub u32);
impl PortSel1 {
    #[doc = "Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
    #[inline(always)]
    pub const fn io4_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
    #[inline(always)]
    pub fn set_io4_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Selects connection for IO pin 5 route."]
    #[inline(always)]
    pub const fn io5_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects connection for IO pin 5 route."]
    #[inline(always)]
    pub fn set_io5_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Selects connection for IO pin 6 route."]
    #[inline(always)]
    pub const fn io6_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects connection for IO pin 6 route."]
    #[inline(always)]
    pub fn set_io6_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Selects connection for IO pin 7 route."]
    #[inline(always)]
    pub const fn io7_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects connection for IO pin 7 route."]
    #[inline(always)]
    pub fn set_io7_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for PortSel1 {
    #[inline(always)]
    fn default() -> PortSel1 {
        PortSel1(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Io0Sel {
    #[doc = "GPIO controls 'out'"]
    GPIO = 0,
    #[doc = "GPIO controls 'out', DSI controls 'output enable'"]
    GPIO_DSI = 0x01,
    #[doc = "DSI controls 'out' and 'output enable'"]
    DSI_DSI = 0x02,
    #[doc = "DSI controls 'out', GPIO controls 'output enable'"]
    DSI_GPIO = 0x03,
    #[doc = "Analog mux bus A"]
    AMUXA = 0x04,
    #[doc = "Analog mux bus B"]
    AMUXB = 0x05,
    #[doc = "Analog mux bus A, DSI control"]
    AMUXA_DSI = 0x06,
    #[doc = "Analog mux bus B, DSI control"]
    AMUXB_DSI = 0x07,
    #[doc = "Active functionality 0"]
    ACT_0 = 0x08,
    #[doc = "Active functionality 1"]
    ACT_1 = 0x09,
    #[doc = "Active functionality 2"]
    ACT_2 = 0x0a,
    #[doc = "Active functionality 3"]
    ACT_3 = 0x0b,
    #[doc = "DeepSleep functionality 0"]
    DS_0 = 0x0c,
    #[doc = "DeepSleep functionality 1"]
    DS_1 = 0x0d,
    #[doc = "DeepSleep functionality 2"]
    DS_2 = 0x0e,
    #[doc = "DeepSleep functionality 3"]
    DS_3 = 0x0f,
    #[doc = "Active functionality 4"]
    ACT_4 = 0x10,
    #[doc = "Active functionality 5"]
    ACT_5 = 0x11,
    #[doc = "Active functionality 6"]
    ACT_6 = 0x12,
    #[doc = "Active functionality 7"]
    ACT_7 = 0x13,
    #[doc = "Active functionality 8"]
    ACT_8 = 0x14,
    #[doc = "Active functionality 9"]
    ACT_9 = 0x15,
    #[doc = "Active functionality 10"]
    ACT_10 = 0x16,
    #[doc = "Active functionality 11"]
    ACT_11 = 0x17,
    #[doc = "Active functionality 12"]
    ACT_12 = 0x18,
    #[doc = "Active functionality 13"]
    ACT_13 = 0x19,
    #[doc = "Active functionality 14"]
    ACT_14 = 0x1a,
    #[doc = "Active functionality 15"]
    ACT_15 = 0x1b,
    #[doc = "DeepSleep functionality 4"]
    DS_4 = 0x1c,
    #[doc = "DeepSleep functionality 5"]
    DS_5 = 0x1d,
    #[doc = "DeepSleep functionality 6"]
    DS_6 = 0x1e,
    #[doc = "DeepSleep functionality 7"]
    DS_7 = 0x1f,
}
impl Io0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Io0Sel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Io0Sel {
    #[inline(always)]
    fn from(val: u8) -> Io0Sel {
        Io0Sel::from_bits(val)
    }
}
impl From<Io0Sel> for u8 {
    #[inline(always)]
    fn from(val: Io0Sel) -> u8 {
        Io0Sel::to_bits(val)
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
