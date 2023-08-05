#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "IPC interrupt structure"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrStruct {
    ptr: *mut u8,
}
unsafe impl Send for IntrStruct {}
unsafe impl Sync for IntrStruct {}
impl IntrStruct {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
}
#[doc = "IPC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc {
    ptr: *mut u8,
}
unsafe impl Send for Ipc {}
unsafe impl Sync for Ipc {}
impl Ipc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IPC structure"]
    #[inline(always)]
    pub const fn struct_(self, n: usize) -> Struct {
        assert!(n < 16usize);
        unsafe { Struct::from_ptr(self.ptr.add(0usize + n * 32usize) as _) }
    }
    #[doc = "IPC interrupt structure"]
    #[inline(always)]
    pub const fn intr_struct(self, n: usize) -> IntrStruct {
        assert!(n < 16usize);
        unsafe { IntrStruct::from_ptr(self.ptr.add(4096usize + n * 32usize) as _) }
    }
}
#[doc = "IPC structure"]
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
    #[doc = "IPC acquire"]
    #[inline(always)]
    pub const fn acquire(self) -> crate::common::Reg<Acquire, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "IPC release"]
    #[inline(always)]
    pub const fn release(self) -> crate::common::Reg<Release, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "IPC notification"]
    #[inline(always)]
    pub const fn notify(self) -> crate::common::Reg<Notify, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "IPC data 0"]
    #[inline(always)]
    pub const fn data0(self) -> crate::common::Reg<Data0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "IPC data 1"]
    #[inline(always)]
    pub const fn data1(self) -> crate::common::Reg<Data1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "IPC lock status"]
    #[inline(always)]
    pub const fn lock_status(self) -> crate::common::Reg<LockStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
}
#[doc = "IPC acquire"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acquire(pub u32);
impl Acquire {
    #[doc = "User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the access that successfully acquired the lock."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the access that successfully acquired the lock."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure/non-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the access that successfully acquired the lock."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure/non-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the access that successfully acquired the lock."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This field specifies the protection context that successfully acquired the lock."]
    #[inline(always)]
    pub const fn pc(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "This field specifies the protection context that successfully acquired the lock."]
    #[inline(always)]
    pub fn set_pc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "This field specifies the bus master identifier that successfully acquired the lock."]
    #[inline(always)]
    pub const fn ms(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "This field specifies the bus master identifier that successfully acquired the lock."]
    #[inline(always)]
    pub fn set_ms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Specifies if the lock is successfully acquired or not (reading the ACQUIRE register can have affect on SUCCESS and LOCK_STATUS.ACQUIRED): '0': Not successfully acquired; i.e. the lock was already acquired by another read transaction and not released. The P, NS, PC and MS fields reflect the access attributes of the transaction that previously successfully acuired the lock; the fields are NOT affected by the current access. '1': Successfully acquired. The P, NS, PC and MS fields reflect the access attributes of the current access. Note that this field is NOT SW writable. A lock is released by writing to the associated RELEASE register (irrespective of the write value)."]
    #[inline(always)]
    pub const fn success(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the lock is successfully acquired or not (reading the ACQUIRE register can have affect on SUCCESS and LOCK_STATUS.ACQUIRED): '0': Not successfully acquired; i.e. the lock was already acquired by another read transaction and not released. The P, NS, PC and MS fields reflect the access attributes of the transaction that previously successfully acuired the lock; the fields are NOT affected by the current access. '1': Successfully acquired. The P, NS, PC and MS fields reflect the access attributes of the current access. Note that this field is NOT SW writable. A lock is released by writing to the associated RELEASE register (irrespective of the write value)."]
    #[inline(always)]
    pub fn set_success(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Acquire {
    #[inline(always)]
    fn default() -> Acquire {
        Acquire(0)
    }
}
#[doc = "IPC data 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data0(pub u32);
impl Data0 {
    #[doc = "This field holds a 32-bit data element that is associated with the IPC structure."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field holds a 32-bit data element that is associated with the IPC structure."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Data0 {
    #[inline(always)]
    fn default() -> Data0 {
        Data0(0)
    }
}
#[doc = "IPC data 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data1(pub u32);
impl Data1 {
    #[doc = "This field holds a 32-bit data element that is associated with the IPC structure."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field holds a 32-bit data element that is associated with the IPC structure."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Data1 {
    #[inline(always)]
    fn default() -> Data1 {
        Data1(0)
    }
}
#[doc = "Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub const fn release(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub fn set_release(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub const fn notify(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub fn set_notify(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
    pub const fn release(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn set_release(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub const fn notify(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn set_notify(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn release(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_release(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub const fn notify(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn set_notify(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
    pub const fn release(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn set_release(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub const fn notify(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn set_notify(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "IPC lock status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LockStatus(pub u32);
impl LockStatus {
    #[doc = "This field specifies the user/privileged access control: '0': user mode. '1': privileged mode."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies the user/privileged access control: '0': user mode. '1': privileged mode."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This field specifies the secure/non-secure access control: '0': secure. '1': non-secure."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies the secure/non-secure access control: '0': secure. '1': non-secure."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This field specifies the protection context that successfully acquired the lock."]
    #[inline(always)]
    pub const fn pc(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "This field specifies the protection context that successfully acquired the lock."]
    #[inline(always)]
    pub fn set_pc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "This field specifies the bus master identifier that successfully acquired the lock."]
    #[inline(always)]
    pub const fn ms(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "This field specifies the bus master identifier that successfully acquired the lock."]
    #[inline(always)]
    pub fn set_ms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Specifies if the lock is acquired. This field is set to '1', if a ACQUIRE read transfer successfully acquires the lock (the ACQUIRE read transfer returns ACQUIRE.SUCCESS as '1'). If zero, P, NS, PC, and MS are not valid."]
    #[inline(always)]
    pub const fn acquired(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the lock is acquired. This field is set to '1', if a ACQUIRE read transfer successfully acquires the lock (the ACQUIRE read transfer returns ACQUIRE.SUCCESS as '1'). If zero, P, NS, PC, and MS are not valid."]
    #[inline(always)]
    pub fn set_acquired(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for LockStatus {
    #[inline(always)]
    fn default() -> LockStatus {
        LockStatus(0)
    }
}
#[doc = "IPC notification"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Notify(pub u32);
impl Notify {
    #[doc = "This field allows for the generation of notification events to the IPC interrupt structures. The IPC notification cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_NOTIFY\\[\\] is set to '1'. SW writes a '1' to the bit fields to generate a notify event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    pub const fn intr_notify(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field allows for the generation of notification events to the IPC interrupt structures. The IPC notification cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_NOTIFY\\[\\] is set to '1'. SW writes a '1' to the bit fields to generate a notify event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    pub fn set_intr_notify(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Notify {
    #[inline(always)]
    fn default() -> Notify {
        Notify(0)
    }
}
#[doc = "IPC release"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Release(pub u32);
impl Release {
    #[doc = "Writing this field releases a lock and allows for the generation of release events to the IPC interrupt structures, but only when the lock is acquired (LOCK_STATUS.ACQUIRED is '1'). The IPC release cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_RELEASE\\[\\] is set to '1'. SW writes a '1' to the bit fields to generate a release event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    pub const fn intr_release(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Writing this field releases a lock and allows for the generation of release events to the IPC interrupt structures, but only when the lock is acquired (LOCK_STATUS.ACQUIRED is '1'). The IPC release cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_RELEASE\\[\\] is set to '1'. SW writes a '1' to the bit fields to generate a release event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    pub fn set_intr_release(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Release {
    #[inline(always)]
    fn default() -> Release {
        Release(0)
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
