#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Fault structures"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fault {
    ptr: *mut u8,
}
unsafe impl Send for Fault {}
unsafe impl Sync for Fault {}
impl Fault {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Fault structure"]
    #[inline(always)]
    pub const fn struct_(self, n: usize) -> Struct {
        assert!(n < 2usize);
        unsafe { Struct::from_ptr(self.ptr.add(0usize + n * 256usize) as _) }
    }
}
#[doc = "Fault structure"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Struct {
    ptr: *mut u8,
}
unsafe impl Send for Struct {}
unsafe impl Sync for Struct {}
impl Struct {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Fault control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Fault status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Fault data"]
    #[inline(always)]
    pub const fn data(self, n: usize) -> crate::common::Reg<Data, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize + n * 4usize) as _) }
    }
    #[doc = "Fault pending 0"]
    #[inline(always)]
    pub const fn pending0(self) -> crate::common::Reg<Pending0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Fault pending 1"]
    #[inline(always)]
    pub const fn pending1(self) -> crate::common::Reg<Pending1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Fault pending 2"]
    #[inline(always)]
    pub const fn pending2(self) -> crate::common::Reg<Pending2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Fault mask 0"]
    #[inline(always)]
    pub const fn mask0(self) -> crate::common::Reg<Mask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Fault mask 1"]
    #[inline(always)]
    pub const fn mask1(self) -> crate::common::Reg<Mask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Fault mask 2"]
    #[inline(always)]
    pub const fn mask2(self) -> crate::common::Reg<Mask2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(200usize) as _) }
    }
    #[doc = "Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(204usize) as _) }
    }
}
#[doc = "Fault control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    pub const fn tr_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    pub fn set_tr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    pub const fn out_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    pub fn set_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    pub const fn reset_req_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    pub fn set_reset_req_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "Fault data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "Captured fault source data. Note: the fault source index STATUS.IDX specifies the format of the DATA registers."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Captured fault source data. Note: the fault source index STATUS.IDX specifies the format of the DATA registers."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
#[doc = "Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to these field to clear the interrupt cause to '0'."]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to these field to clear the interrupt cause to '0'."]
    #[inline(always)]
    pub fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Interrupt masked"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Interrupt set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub const fn fault(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn set_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Fault mask 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask0(pub u32);
impl Mask0 {
    #[doc = "Fault source enables: Bits 31-0: Fault sources 31 to 0."]
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Fault source enables: Bits 31-0: Fault sources 31 to 0."]
    #[inline(always)]
    pub fn set_source(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mask0 {
    #[inline(always)]
    fn default() -> Mask0 {
        Mask0(0)
    }
}
#[doc = "Fault mask 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask1(pub u32);
impl Mask1 {
    #[doc = "Fault source enables: Bits 31-0: Fault sources 63 to 32."]
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Fault source enables: Bits 31-0: Fault sources 63 to 32."]
    #[inline(always)]
    pub fn set_source(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mask1 {
    #[inline(always)]
    fn default() -> Mask1 {
        Mask1(0)
    }
}
#[doc = "Fault mask 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask2(pub u32);
impl Mask2 {
    #[doc = "Fault source enables: Bits 31-0: Fault sources 95 to 64."]
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Fault source enables: Bits 31-0: Fault sources 95 to 64."]
    #[inline(always)]
    pub fn set_source(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mask2 {
    #[inline(always)]
    fn default() -> Mask2 {
        Mask2(0)
    }
}
#[doc = "Fault pending 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pending0(pub u32);
impl Pending0 {
    #[doc = "This field specifies the following sources: Bit 0: CM0 MPU. Bit 1: CRYPTO MPU. Bit 2: DW 0 MPU. Bit 3: DW 1 MPU. ... Bit 14: CM4 code bus MPU. Bit 15: DAP MPU. Bit 16: CM4 s+G92ystem bus MPU. Bit 28: Peripheral master interface 0 PPU. Bit 29: Peripheral master interface 1 PPU. Bit 30: Peripheral master interface 2 PPU. Bit 31: Peripheral master interface 3 PPU."]
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field specifies the following sources: Bit 0: CM0 MPU. Bit 1: CRYPTO MPU. Bit 2: DW 0 MPU. Bit 3: DW 1 MPU. ... Bit 14: CM4 code bus MPU. Bit 15: DAP MPU. Bit 16: CM4 s+G92ystem bus MPU. Bit 28: Peripheral master interface 0 PPU. Bit 29: Peripheral master interface 1 PPU. Bit 30: Peripheral master interface 2 PPU. Bit 31: Peripheral master interface 3 PPU."]
    #[inline(always)]
    pub fn set_source(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pending0 {
    #[inline(always)]
    fn default() -> Pending0 {
        Pending0(0)
    }
}
#[doc = "Fault pending 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pending1(pub u32);
impl Pending1 {
    #[doc = "This field specifies the following sources: Bit 0: Peripheral group 0 PPU. Bit 1: Peripheral group 1 PPU. Bit 2: Peripheral group 2 PPU. Bit 3: Peripheral group 3 PPU. Bit 4: Peripheral group 4 PPU. Bit 5: Peripheral group 5 PPU. Bit 6: Peripheral group 6 PPU. Bit 7: Peripheral group 7 PPU. ... Bit 15: Peripheral group 15 PPU. Bit 18: Flash controller, main interface, bus error."]
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field specifies the following sources: Bit 0: Peripheral group 0 PPU. Bit 1: Peripheral group 1 PPU. Bit 2: Peripheral group 2 PPU. Bit 3: Peripheral group 3 PPU. Bit 4: Peripheral group 4 PPU. Bit 5: Peripheral group 5 PPU. Bit 6: Peripheral group 6 PPU. Bit 7: Peripheral group 7 PPU. ... Bit 15: Peripheral group 15 PPU. Bit 18: Flash controller, main interface, bus error."]
    #[inline(always)]
    pub fn set_source(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pending1 {
    #[inline(always)]
    fn default() -> Pending1 {
        Pending1(0)
    }
}
#[doc = "Fault pending 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pending2(pub u32);
impl Pending2 {
    #[doc = "This field specifies the following sources: Bit 0 - 31: TBD."]
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field specifies the following sources: Bit 0 - 31: TBD."]
    #[inline(always)]
    pub fn set_source(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pending2 {
    #[inline(always)]
    fn default() -> Pending2 {
        Pending2(0)
    }
}
#[doc = "Fault status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    pub const fn idx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    pub fn set_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Valid indication: '0': Invalid. '1': Valid. HW sets this field to '1' when new fault source data is captured. New fault source data is ONLY captured when VALID is '0'. SW can clear this field to '0' when the fault is handled (by SW)."]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Valid indication: '0': Invalid. '1': Valid. HW sets this field to '1' when new fault source data is captured. New fault source data is ONLY captured when VALID is '0'. SW can clear this field to '0' when the fault is handled (by SW)."]
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
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
