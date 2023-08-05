#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Peripheral interconnect, master interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PeriMs {
    ptr: *mut u8,
}
unsafe impl Send for PeriMs {}
unsafe impl Sync for PeriMs {}
impl PeriMs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Programmable protection structure pair"]
    #[inline(always)]
    pub const fn ppu_pr(self, n: usize) -> PpuPr {
        assert!(n < 8usize);
        unsafe { PpuPr::from_ptr(self.ptr.add(0usize + n * 64usize) as _) }
    }
    #[doc = "Fixed protection structure pair"]
    #[inline(always)]
    pub const fn ppu_fx(self, n: usize) -> PpuFx {
        assert!(n < 229usize);
        unsafe { PpuFx::from_ptr(self.ptr.add(2048usize + n * 64usize) as _) }
    }
}
#[doc = "Fixed protection structure pair"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFx {
    ptr: *mut u8,
}
unsafe impl Send for PpuFx {}
unsafe impl Sync for PpuFx {}
impl PpuFx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Slave region, base address"]
    #[inline(always)]
    pub const fn sl_addr(self) -> crate::common::Reg<PpuFxSlAddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Slave region, size"]
    #[inline(always)]
    pub const fn sl_size(self) -> crate::common::Reg<PpuFxSlSize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Slave attributes 0"]
    #[inline(always)]
    pub const fn sl_att0(self) -> crate::common::Reg<PpuFxSlAtt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Slave attributes 1"]
    #[inline(always)]
    pub const fn sl_att1(self) -> crate::common::Reg<PpuFxSlAtt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Slave attributes 2"]
    #[inline(always)]
    pub const fn sl_att2(self) -> crate::common::Reg<PpuFxSlAtt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Slave attributes 3"]
    #[inline(always)]
    pub const fn sl_att3(self) -> crate::common::Reg<PpuFxSlAtt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Master region, base address"]
    #[inline(always)]
    pub const fn ms_addr(self) -> crate::common::Reg<PpuFxMsAddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Master region, size"]
    #[inline(always)]
    pub const fn ms_size(self) -> crate::common::Reg<PpuFxMsSize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Master attributes 0"]
    #[inline(always)]
    pub const fn ms_att0(self) -> crate::common::Reg<PpuFxMsAtt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Master attributes 1"]
    #[inline(always)]
    pub const fn ms_att1(self) -> crate::common::Reg<PpuFxMsAtt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Master attributes 2"]
    #[inline(always)]
    pub const fn ms_att2(self) -> crate::common::Reg<PpuFxMsAtt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Master attributes 3"]
    #[inline(always)]
    pub const fn ms_att3(self) -> crate::common::Reg<PpuFxMsAtt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
}
#[doc = "Programmable protection structure pair"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPr {
    ptr: *mut u8,
}
unsafe impl Send for PpuPr {}
unsafe impl Sync for PpuPr {}
impl PpuPr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Slave region, base address"]
    #[inline(always)]
    pub const fn sl_addr(self) -> crate::common::Reg<PpuPrSlAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Slave region, size"]
    #[inline(always)]
    pub const fn sl_size(self) -> crate::common::Reg<PpuPrSlSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Slave attributes 0"]
    #[inline(always)]
    pub const fn sl_att0(self) -> crate::common::Reg<PpuPrSlAtt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Slave attributes 1"]
    #[inline(always)]
    pub const fn sl_att1(self) -> crate::common::Reg<PpuPrSlAtt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Slave attributes 2"]
    #[inline(always)]
    pub const fn sl_att2(self) -> crate::common::Reg<PpuPrSlAtt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Slave attributes 3"]
    #[inline(always)]
    pub const fn sl_att3(self) -> crate::common::Reg<PpuPrSlAtt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Master region, base address"]
    #[inline(always)]
    pub const fn ms_addr(self) -> crate::common::Reg<PpuPrMsAddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Master region, size"]
    #[inline(always)]
    pub const fn ms_size(self) -> crate::common::Reg<PpuPrMsSize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Master attributes 0"]
    #[inline(always)]
    pub const fn ms_att0(self) -> crate::common::Reg<PpuPrMsAtt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Master attributes 1"]
    #[inline(always)]
    pub const fn ms_att1(self) -> crate::common::Reg<PpuPrMsAtt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Master attributes 2"]
    #[inline(always)]
    pub const fn ms_att2(self) -> crate::common::Reg<PpuPrMsAtt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Master attributes 3"]
    #[inline(always)]
    pub const fn ms_att3(self) -> crate::common::Reg<PpuPrMsAtt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
}
#[doc = "Master region, base address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxMsAddr(pub u32);
impl PpuFxMsAddr {
    #[doc = "This field specifies the base address of the master region. The base address of the region is the address of the SL_ADDR register."]
    #[inline(always)]
    pub const fn addr26(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "This field specifies the base address of the master region. The base address of the region is the address of the SL_ADDR register."]
    #[inline(always)]
    pub fn set_addr26(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for PpuFxMsAddr {
    #[inline(always)]
    fn default() -> PpuFxMsAddr {
        PpuFxMsAddr(0)
    }
}
#[doc = "Master attributes 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxMsAtt0(pub u32);
impl PpuFxMsAtt0 {
    #[doc = "Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub const fn pc0_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn set_pc0_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 1, user read enable."]
    #[inline(always)]
    pub const fn pc1_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, user read enable."]
    #[inline(always)]
    pub fn set_pc1_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 1, user write enable."]
    #[inline(always)]
    pub const fn pc1_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, user write enable."]
    #[inline(always)]
    pub fn set_pc1_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 1, privileged read enable."]
    #[inline(always)]
    pub const fn pc1_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, privileged read enable."]
    #[inline(always)]
    pub fn set_pc1_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 1, privileged write enable."]
    #[inline(always)]
    pub const fn pc1_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, privileged write enable."]
    #[inline(always)]
    pub fn set_pc1_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 1, non-secure."]
    #[inline(always)]
    pub const fn pc1_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, non-secure."]
    #[inline(always)]
    pub fn set_pc1_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 2, user read enable."]
    #[inline(always)]
    pub const fn pc2_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, user read enable."]
    #[inline(always)]
    pub fn set_pc2_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 2, user write enable."]
    #[inline(always)]
    pub const fn pc2_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, user write enable."]
    #[inline(always)]
    pub fn set_pc2_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 2, privileged read enable."]
    #[inline(always)]
    pub const fn pc2_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, privileged read enable."]
    #[inline(always)]
    pub fn set_pc2_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 2, privileged write enable."]
    #[inline(always)]
    pub const fn pc2_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, privileged write enable."]
    #[inline(always)]
    pub fn set_pc2_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 2, non-secure."]
    #[inline(always)]
    pub const fn pc2_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, non-secure."]
    #[inline(always)]
    pub fn set_pc2_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 3, user read enable."]
    #[inline(always)]
    pub const fn pc3_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, user read enable."]
    #[inline(always)]
    pub fn set_pc3_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 3, user write enable."]
    #[inline(always)]
    pub const fn pc3_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, user write enable."]
    #[inline(always)]
    pub fn set_pc3_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 3, privileged read enable."]
    #[inline(always)]
    pub const fn pc3_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, privileged read enable."]
    #[inline(always)]
    pub fn set_pc3_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 3, privileged write enable."]
    #[inline(always)]
    pub const fn pc3_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, privileged write enable."]
    #[inline(always)]
    pub fn set_pc3_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 3, non-secure."]
    #[inline(always)]
    pub const fn pc3_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, non-secure."]
    #[inline(always)]
    pub fn set_pc3_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuFxMsAtt0 {
    #[inline(always)]
    fn default() -> PpuFxMsAtt0 {
        PpuFxMsAtt0(0)
    }
}
#[doc = "Master attributes 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxMsAtt1(pub u32);
impl PpuFxMsAtt1 {
    #[doc = "Protection context 4, user read enable."]
    #[inline(always)]
    pub const fn pc4_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, user read enable."]
    #[inline(always)]
    pub fn set_pc4_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 4, user write enable."]
    #[inline(always)]
    pub const fn pc4_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, user write enable."]
    #[inline(always)]
    pub fn set_pc4_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 4, privileged read enable."]
    #[inline(always)]
    pub const fn pc4_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, privileged read enable."]
    #[inline(always)]
    pub fn set_pc4_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 4, privileged write enable."]
    #[inline(always)]
    pub const fn pc4_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, privileged write enable."]
    #[inline(always)]
    pub fn set_pc4_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 4, non-secure."]
    #[inline(always)]
    pub const fn pc4_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, non-secure."]
    #[inline(always)]
    pub fn set_pc4_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 5, user read enable."]
    #[inline(always)]
    pub const fn pc5_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, user read enable."]
    #[inline(always)]
    pub fn set_pc5_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 5, user write enable."]
    #[inline(always)]
    pub const fn pc5_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, user write enable."]
    #[inline(always)]
    pub fn set_pc5_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 5, privileged read enable."]
    #[inline(always)]
    pub const fn pc5_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, privileged read enable."]
    #[inline(always)]
    pub fn set_pc5_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 5, privileged write enable."]
    #[inline(always)]
    pub const fn pc5_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, privileged write enable."]
    #[inline(always)]
    pub fn set_pc5_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 5, non-secure."]
    #[inline(always)]
    pub const fn pc5_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, non-secure."]
    #[inline(always)]
    pub fn set_pc5_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 6, user read enable."]
    #[inline(always)]
    pub const fn pc6_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, user read enable."]
    #[inline(always)]
    pub fn set_pc6_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 6, user write enable."]
    #[inline(always)]
    pub const fn pc6_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, user write enable."]
    #[inline(always)]
    pub fn set_pc6_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 6, privileged read enable."]
    #[inline(always)]
    pub const fn pc6_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, privileged read enable."]
    #[inline(always)]
    pub fn set_pc6_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 6, privileged write enable."]
    #[inline(always)]
    pub const fn pc6_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, privileged write enable."]
    #[inline(always)]
    pub fn set_pc6_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 6, non-secure."]
    #[inline(always)]
    pub const fn pc6_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, non-secure."]
    #[inline(always)]
    pub fn set_pc6_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 7, user read enable."]
    #[inline(always)]
    pub const fn pc7_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, user read enable."]
    #[inline(always)]
    pub fn set_pc7_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 7, user write enable."]
    #[inline(always)]
    pub const fn pc7_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, user write enable."]
    #[inline(always)]
    pub fn set_pc7_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 7, privileged read enable."]
    #[inline(always)]
    pub const fn pc7_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, privileged read enable."]
    #[inline(always)]
    pub fn set_pc7_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 7, privileged write enable."]
    #[inline(always)]
    pub const fn pc7_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, privileged write enable."]
    #[inline(always)]
    pub fn set_pc7_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 7, non-secure."]
    #[inline(always)]
    pub const fn pc7_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, non-secure."]
    #[inline(always)]
    pub fn set_pc7_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuFxMsAtt1 {
    #[inline(always)]
    fn default() -> PpuFxMsAtt1 {
        PpuFxMsAtt1(0)
    }
}
#[doc = "Master attributes 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxMsAtt2(pub u32);
impl PpuFxMsAtt2 {
    #[doc = "Protection context 8, user read enable."]
    #[inline(always)]
    pub const fn pc8_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, user read enable."]
    #[inline(always)]
    pub fn set_pc8_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 8, user write enable."]
    #[inline(always)]
    pub const fn pc8_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, user write enable."]
    #[inline(always)]
    pub fn set_pc8_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 8, privileged read enable."]
    #[inline(always)]
    pub const fn pc8_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, privileged read enable."]
    #[inline(always)]
    pub fn set_pc8_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 8, privileged write enable."]
    #[inline(always)]
    pub const fn pc8_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, privileged write enable."]
    #[inline(always)]
    pub fn set_pc8_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 8, non-secure."]
    #[inline(always)]
    pub const fn pc8_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, non-secure."]
    #[inline(always)]
    pub fn set_pc8_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 9, user read enable."]
    #[inline(always)]
    pub const fn pc9_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, user read enable."]
    #[inline(always)]
    pub fn set_pc9_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 9, user write enable."]
    #[inline(always)]
    pub const fn pc9_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, user write enable."]
    #[inline(always)]
    pub fn set_pc9_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 9, privileged read enable."]
    #[inline(always)]
    pub const fn pc9_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, privileged read enable."]
    #[inline(always)]
    pub fn set_pc9_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 9, privileged write enable."]
    #[inline(always)]
    pub const fn pc9_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, privileged write enable."]
    #[inline(always)]
    pub fn set_pc9_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 9, non-secure."]
    #[inline(always)]
    pub const fn pc9_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, non-secure."]
    #[inline(always)]
    pub fn set_pc9_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 10, user read enable."]
    #[inline(always)]
    pub const fn pc10_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, user read enable."]
    #[inline(always)]
    pub fn set_pc10_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 10, user write enable."]
    #[inline(always)]
    pub const fn pc10_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, user write enable."]
    #[inline(always)]
    pub fn set_pc10_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 10, privileged read enable."]
    #[inline(always)]
    pub const fn pc10_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, privileged read enable."]
    #[inline(always)]
    pub fn set_pc10_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 10, privileged write enable."]
    #[inline(always)]
    pub const fn pc10_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, privileged write enable."]
    #[inline(always)]
    pub fn set_pc10_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 10, non-secure."]
    #[inline(always)]
    pub const fn pc10_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, non-secure."]
    #[inline(always)]
    pub fn set_pc10_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 11, user read enable."]
    #[inline(always)]
    pub const fn pc11_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, user read enable."]
    #[inline(always)]
    pub fn set_pc11_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 11, user write enable."]
    #[inline(always)]
    pub const fn pc11_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, user write enable."]
    #[inline(always)]
    pub fn set_pc11_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 11, privileged read enable."]
    #[inline(always)]
    pub const fn pc11_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, privileged read enable."]
    #[inline(always)]
    pub fn set_pc11_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 11, privileged write enable."]
    #[inline(always)]
    pub const fn pc11_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, privileged write enable."]
    #[inline(always)]
    pub fn set_pc11_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 11, non-secure."]
    #[inline(always)]
    pub const fn pc11_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, non-secure."]
    #[inline(always)]
    pub fn set_pc11_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuFxMsAtt2 {
    #[inline(always)]
    fn default() -> PpuFxMsAtt2 {
        PpuFxMsAtt2(0)
    }
}
#[doc = "Master attributes 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxMsAtt3(pub u32);
impl PpuFxMsAtt3 {
    #[doc = "Protection context 12, user read enable."]
    #[inline(always)]
    pub const fn pc12_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, user read enable."]
    #[inline(always)]
    pub fn set_pc12_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 12, user write enable."]
    #[inline(always)]
    pub const fn pc12_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, user write enable."]
    #[inline(always)]
    pub fn set_pc12_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 12, privileged read enable."]
    #[inline(always)]
    pub const fn pc12_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, privileged read enable."]
    #[inline(always)]
    pub fn set_pc12_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 12, privileged write enable."]
    #[inline(always)]
    pub const fn pc12_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, privileged write enable."]
    #[inline(always)]
    pub fn set_pc12_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 12, non-secure."]
    #[inline(always)]
    pub const fn pc12_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, non-secure."]
    #[inline(always)]
    pub fn set_pc12_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 13, user read enable."]
    #[inline(always)]
    pub const fn pc13_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, user read enable."]
    #[inline(always)]
    pub fn set_pc13_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 13, user write enable."]
    #[inline(always)]
    pub const fn pc13_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, user write enable."]
    #[inline(always)]
    pub fn set_pc13_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 13, privileged read enable."]
    #[inline(always)]
    pub const fn pc13_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, privileged read enable."]
    #[inline(always)]
    pub fn set_pc13_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 13, privileged write enable."]
    #[inline(always)]
    pub const fn pc13_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, privileged write enable."]
    #[inline(always)]
    pub fn set_pc13_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 13, non-secure."]
    #[inline(always)]
    pub const fn pc13_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, non-secure."]
    #[inline(always)]
    pub fn set_pc13_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 14, user read enable."]
    #[inline(always)]
    pub const fn pc14_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, user read enable."]
    #[inline(always)]
    pub fn set_pc14_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 14, user write enable."]
    #[inline(always)]
    pub const fn pc14_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, user write enable."]
    #[inline(always)]
    pub fn set_pc14_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 14, privileged read enable."]
    #[inline(always)]
    pub const fn pc14_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, privileged read enable."]
    #[inline(always)]
    pub fn set_pc14_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 14, privileged write enable."]
    #[inline(always)]
    pub const fn pc14_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, privileged write enable."]
    #[inline(always)]
    pub fn set_pc14_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 14, non-secure."]
    #[inline(always)]
    pub const fn pc14_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, non-secure."]
    #[inline(always)]
    pub fn set_pc14_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 15, user read enable."]
    #[inline(always)]
    pub const fn pc15_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, user read enable."]
    #[inline(always)]
    pub fn set_pc15_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 15, user write enable."]
    #[inline(always)]
    pub const fn pc15_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, user write enable."]
    #[inline(always)]
    pub fn set_pc15_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 15, privileged read enable."]
    #[inline(always)]
    pub const fn pc15_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, privileged read enable."]
    #[inline(always)]
    pub fn set_pc15_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 15, privileged write enable."]
    #[inline(always)]
    pub const fn pc15_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, privileged write enable."]
    #[inline(always)]
    pub fn set_pc15_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 15, non-secure."]
    #[inline(always)]
    pub const fn pc15_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, non-secure."]
    #[inline(always)]
    pub fn set_pc15_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuFxMsAtt3 {
    #[inline(always)]
    fn default() -> PpuFxMsAtt3 {
        PpuFxMsAtt3(0)
    }
}
#[doc = "Master region, size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxMsSize(pub u32);
impl PpuFxMsSize {
    #[doc = "This field specifies the size of the master region: '5': 64 B region The master region includes the SL_ADDR, SL_SIZE, SL_ATT0, ..., SL_ATT3, MS_ADDR, MS_SIZE, MS_ATT0, ..., MS_ATT3 registers. Therefore, the access privileges for all these registers is determined by MS_ATT0, ..., MS_ATT3."]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "This field specifies the size of the master region: '5': 64 B region The master region includes the SL_ADDR, SL_SIZE, SL_ATT0, ..., SL_ATT3, MS_ADDR, MS_SIZE, MS_ATT0, ..., MS_ATT3 registers. Therefore, the access privileges for all these registers is determined by MS_ATT0, ..., MS_ATT3."]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Master region enable: '1': Enabled."]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master region enable: '1': Enabled."]
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PpuFxMsSize {
    #[inline(always)]
    fn default() -> PpuFxMsSize {
        PpuFxMsSize(0)
    }
}
#[doc = "Slave region, base address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxSlAddr(pub u32);
impl PpuFxSlAddr {
    #[doc = "This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\] must be '0's."]
    #[inline(always)]
    pub const fn addr30(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\] must be '0's."]
    #[inline(always)]
    pub fn set_addr30(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PpuFxSlAddr {
    #[inline(always)]
    fn default() -> PpuFxSlAddr {
        PpuFxSlAddr(0)
    }
}
#[doc = "Slave attributes 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxSlAtt0(pub u32);
impl PpuFxSlAtt0 {
    #[doc = "Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub const fn pc0_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn set_pc0_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 1, user read enable."]
    #[inline(always)]
    pub const fn pc1_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, user read enable."]
    #[inline(always)]
    pub fn set_pc1_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 1, user write enable."]
    #[inline(always)]
    pub const fn pc1_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, user write enable."]
    #[inline(always)]
    pub fn set_pc1_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 1, privileged read enable."]
    #[inline(always)]
    pub const fn pc1_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, privileged read enable."]
    #[inline(always)]
    pub fn set_pc1_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 1, privileged write enable."]
    #[inline(always)]
    pub const fn pc1_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, privileged write enable."]
    #[inline(always)]
    pub fn set_pc1_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 1, non-secure."]
    #[inline(always)]
    pub const fn pc1_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, non-secure."]
    #[inline(always)]
    pub fn set_pc1_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 2, user read enable."]
    #[inline(always)]
    pub const fn pc2_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, user read enable."]
    #[inline(always)]
    pub fn set_pc2_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 2, user write enable."]
    #[inline(always)]
    pub const fn pc2_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, user write enable."]
    #[inline(always)]
    pub fn set_pc2_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 2, privileged read enable."]
    #[inline(always)]
    pub const fn pc2_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, privileged read enable."]
    #[inline(always)]
    pub fn set_pc2_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 2, privileged write enable."]
    #[inline(always)]
    pub const fn pc2_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, privileged write enable."]
    #[inline(always)]
    pub fn set_pc2_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 2, non-secure."]
    #[inline(always)]
    pub const fn pc2_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, non-secure."]
    #[inline(always)]
    pub fn set_pc2_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 3, user read enable."]
    #[inline(always)]
    pub const fn pc3_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, user read enable."]
    #[inline(always)]
    pub fn set_pc3_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 3, user write enable."]
    #[inline(always)]
    pub const fn pc3_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, user write enable."]
    #[inline(always)]
    pub fn set_pc3_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 3, privileged read enable."]
    #[inline(always)]
    pub const fn pc3_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, privileged read enable."]
    #[inline(always)]
    pub fn set_pc3_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 3, privileged write enable."]
    #[inline(always)]
    pub const fn pc3_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, privileged write enable."]
    #[inline(always)]
    pub fn set_pc3_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 3, non-secure."]
    #[inline(always)]
    pub const fn pc3_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, non-secure."]
    #[inline(always)]
    pub fn set_pc3_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuFxSlAtt0 {
    #[inline(always)]
    fn default() -> PpuFxSlAtt0 {
        PpuFxSlAtt0(0)
    }
}
#[doc = "Slave attributes 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxSlAtt1(pub u32);
impl PpuFxSlAtt1 {
    #[doc = "Protection context 4, user read enable."]
    #[inline(always)]
    pub const fn pc4_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, user read enable."]
    #[inline(always)]
    pub fn set_pc4_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 4, user write enable."]
    #[inline(always)]
    pub const fn pc4_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, user write enable."]
    #[inline(always)]
    pub fn set_pc4_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 4, privileged read enable."]
    #[inline(always)]
    pub const fn pc4_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, privileged read enable."]
    #[inline(always)]
    pub fn set_pc4_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 4, privileged write enable."]
    #[inline(always)]
    pub const fn pc4_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, privileged write enable."]
    #[inline(always)]
    pub fn set_pc4_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 4, non-secure."]
    #[inline(always)]
    pub const fn pc4_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, non-secure."]
    #[inline(always)]
    pub fn set_pc4_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 5, user read enable."]
    #[inline(always)]
    pub const fn pc5_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, user read enable."]
    #[inline(always)]
    pub fn set_pc5_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 5, user write enable."]
    #[inline(always)]
    pub const fn pc5_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, user write enable."]
    #[inline(always)]
    pub fn set_pc5_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 5, privileged read enable."]
    #[inline(always)]
    pub const fn pc5_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, privileged read enable."]
    #[inline(always)]
    pub fn set_pc5_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 5, privileged write enable."]
    #[inline(always)]
    pub const fn pc5_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, privileged write enable."]
    #[inline(always)]
    pub fn set_pc5_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 5, non-secure."]
    #[inline(always)]
    pub const fn pc5_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, non-secure."]
    #[inline(always)]
    pub fn set_pc5_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 6, user read enable."]
    #[inline(always)]
    pub const fn pc6_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, user read enable."]
    #[inline(always)]
    pub fn set_pc6_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 6, user write enable."]
    #[inline(always)]
    pub const fn pc6_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, user write enable."]
    #[inline(always)]
    pub fn set_pc6_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 6, privileged read enable."]
    #[inline(always)]
    pub const fn pc6_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, privileged read enable."]
    #[inline(always)]
    pub fn set_pc6_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 6, privileged write enable."]
    #[inline(always)]
    pub const fn pc6_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, privileged write enable."]
    #[inline(always)]
    pub fn set_pc6_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 6, non-secure."]
    #[inline(always)]
    pub const fn pc6_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, non-secure."]
    #[inline(always)]
    pub fn set_pc6_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 7, user read enable."]
    #[inline(always)]
    pub const fn pc7_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, user read enable."]
    #[inline(always)]
    pub fn set_pc7_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 7, user write enable."]
    #[inline(always)]
    pub const fn pc7_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, user write enable."]
    #[inline(always)]
    pub fn set_pc7_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 7, privileged read enable."]
    #[inline(always)]
    pub const fn pc7_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, privileged read enable."]
    #[inline(always)]
    pub fn set_pc7_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 7, privileged write enable."]
    #[inline(always)]
    pub const fn pc7_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, privileged write enable."]
    #[inline(always)]
    pub fn set_pc7_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 7, non-secure."]
    #[inline(always)]
    pub const fn pc7_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, non-secure."]
    #[inline(always)]
    pub fn set_pc7_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuFxSlAtt1 {
    #[inline(always)]
    fn default() -> PpuFxSlAtt1 {
        PpuFxSlAtt1(0)
    }
}
#[doc = "Slave attributes 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxSlAtt2(pub u32);
impl PpuFxSlAtt2 {
    #[doc = "Protection context 8, user read enable."]
    #[inline(always)]
    pub const fn pc8_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, user read enable."]
    #[inline(always)]
    pub fn set_pc8_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 8, user write enable."]
    #[inline(always)]
    pub const fn pc8_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, user write enable."]
    #[inline(always)]
    pub fn set_pc8_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 8, privileged read enable."]
    #[inline(always)]
    pub const fn pc8_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, privileged read enable."]
    #[inline(always)]
    pub fn set_pc8_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 8, privileged write enable."]
    #[inline(always)]
    pub const fn pc8_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, privileged write enable."]
    #[inline(always)]
    pub fn set_pc8_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 8, non-secure."]
    #[inline(always)]
    pub const fn pc8_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, non-secure."]
    #[inline(always)]
    pub fn set_pc8_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 9, user read enable."]
    #[inline(always)]
    pub const fn pc9_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, user read enable."]
    #[inline(always)]
    pub fn set_pc9_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 9, user write enable."]
    #[inline(always)]
    pub const fn pc9_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, user write enable."]
    #[inline(always)]
    pub fn set_pc9_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 9, privileged read enable."]
    #[inline(always)]
    pub const fn pc9_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, privileged read enable."]
    #[inline(always)]
    pub fn set_pc9_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 9, privileged write enable."]
    #[inline(always)]
    pub const fn pc9_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, privileged write enable."]
    #[inline(always)]
    pub fn set_pc9_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 9, non-secure."]
    #[inline(always)]
    pub const fn pc9_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, non-secure."]
    #[inline(always)]
    pub fn set_pc9_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 10, user read enable."]
    #[inline(always)]
    pub const fn pc10_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, user read enable."]
    #[inline(always)]
    pub fn set_pc10_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 10, user write enable."]
    #[inline(always)]
    pub const fn pc10_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, user write enable."]
    #[inline(always)]
    pub fn set_pc10_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 10, privileged read enable."]
    #[inline(always)]
    pub const fn pc10_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, privileged read enable."]
    #[inline(always)]
    pub fn set_pc10_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 10, privileged write enable."]
    #[inline(always)]
    pub const fn pc10_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, privileged write enable."]
    #[inline(always)]
    pub fn set_pc10_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 10, non-secure."]
    #[inline(always)]
    pub const fn pc10_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, non-secure."]
    #[inline(always)]
    pub fn set_pc10_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 11, user read enable."]
    #[inline(always)]
    pub const fn pc11_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, user read enable."]
    #[inline(always)]
    pub fn set_pc11_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 11, user write enable."]
    #[inline(always)]
    pub const fn pc11_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, user write enable."]
    #[inline(always)]
    pub fn set_pc11_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 11, privileged read enable."]
    #[inline(always)]
    pub const fn pc11_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, privileged read enable."]
    #[inline(always)]
    pub fn set_pc11_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 11, privileged write enable."]
    #[inline(always)]
    pub const fn pc11_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, privileged write enable."]
    #[inline(always)]
    pub fn set_pc11_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 11, non-secure."]
    #[inline(always)]
    pub const fn pc11_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, non-secure."]
    #[inline(always)]
    pub fn set_pc11_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuFxSlAtt2 {
    #[inline(always)]
    fn default() -> PpuFxSlAtt2 {
        PpuFxSlAtt2(0)
    }
}
#[doc = "Slave attributes 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxSlAtt3(pub u32);
impl PpuFxSlAtt3 {
    #[doc = "Protection context 12, user read enable."]
    #[inline(always)]
    pub const fn pc12_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, user read enable."]
    #[inline(always)]
    pub fn set_pc12_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 12, user write enable."]
    #[inline(always)]
    pub const fn pc12_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, user write enable."]
    #[inline(always)]
    pub fn set_pc12_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 12, privileged read enable."]
    #[inline(always)]
    pub const fn pc12_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, privileged read enable."]
    #[inline(always)]
    pub fn set_pc12_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 12, privileged write enable."]
    #[inline(always)]
    pub const fn pc12_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, privileged write enable."]
    #[inline(always)]
    pub fn set_pc12_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 12, non-secure."]
    #[inline(always)]
    pub const fn pc12_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, non-secure."]
    #[inline(always)]
    pub fn set_pc12_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 13, user read enable."]
    #[inline(always)]
    pub const fn pc13_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, user read enable."]
    #[inline(always)]
    pub fn set_pc13_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 13, user write enable."]
    #[inline(always)]
    pub const fn pc13_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, user write enable."]
    #[inline(always)]
    pub fn set_pc13_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 13, privileged read enable."]
    #[inline(always)]
    pub const fn pc13_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, privileged read enable."]
    #[inline(always)]
    pub fn set_pc13_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 13, privileged write enable."]
    #[inline(always)]
    pub const fn pc13_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, privileged write enable."]
    #[inline(always)]
    pub fn set_pc13_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 13, non-secure."]
    #[inline(always)]
    pub const fn pc13_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, non-secure."]
    #[inline(always)]
    pub fn set_pc13_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 14, user read enable."]
    #[inline(always)]
    pub const fn pc14_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, user read enable."]
    #[inline(always)]
    pub fn set_pc14_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 14, user write enable."]
    #[inline(always)]
    pub const fn pc14_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, user write enable."]
    #[inline(always)]
    pub fn set_pc14_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 14, privileged read enable."]
    #[inline(always)]
    pub const fn pc14_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, privileged read enable."]
    #[inline(always)]
    pub fn set_pc14_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 14, privileged write enable."]
    #[inline(always)]
    pub const fn pc14_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, privileged write enable."]
    #[inline(always)]
    pub fn set_pc14_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 14, non-secure."]
    #[inline(always)]
    pub const fn pc14_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, non-secure."]
    #[inline(always)]
    pub fn set_pc14_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 15, user read enable."]
    #[inline(always)]
    pub const fn pc15_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, user read enable."]
    #[inline(always)]
    pub fn set_pc15_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 15, user write enable."]
    #[inline(always)]
    pub const fn pc15_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, user write enable."]
    #[inline(always)]
    pub fn set_pc15_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 15, privileged read enable."]
    #[inline(always)]
    pub const fn pc15_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, privileged read enable."]
    #[inline(always)]
    pub fn set_pc15_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 15, privileged write enable."]
    #[inline(always)]
    pub const fn pc15_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, privileged write enable."]
    #[inline(always)]
    pub fn set_pc15_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 15, non-secure."]
    #[inline(always)]
    pub const fn pc15_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, non-secure."]
    #[inline(always)]
    pub fn set_pc15_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuFxSlAtt3 {
    #[inline(always)]
    fn default() -> PpuFxSlAtt3 {
        PpuFxSlAtt3(0)
    }
}
#[doc = "Slave region, size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuFxSlSize(pub u32);
impl PpuFxSlSize {
    #[doc = "This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PpuFxSlSize {
    #[inline(always)]
    fn default() -> PpuFxSlSize {
        PpuFxSlSize(0)
    }
}
#[doc = "Master region, base address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrMsAddr(pub u32);
impl PpuPrMsAddr {
    #[doc = "This field specifies the base address of the master region. The base address of the region is the address of the SL_ADDR register."]
    #[inline(always)]
    pub const fn addr26(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "This field specifies the base address of the master region. The base address of the region is the address of the SL_ADDR register."]
    #[inline(always)]
    pub fn set_addr26(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for PpuPrMsAddr {
    #[inline(always)]
    fn default() -> PpuPrMsAddr {
        PpuPrMsAddr(0)
    }
}
#[doc = "Master attributes 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrMsAtt0(pub u32);
impl PpuPrMsAtt0 {
    #[doc = "Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub const fn pc0_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn set_pc0_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 1, user read enable."]
    #[inline(always)]
    pub const fn pc1_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, user read enable."]
    #[inline(always)]
    pub fn set_pc1_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 1, user write enable."]
    #[inline(always)]
    pub const fn pc1_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, user write enable."]
    #[inline(always)]
    pub fn set_pc1_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 1, privileged read enable."]
    #[inline(always)]
    pub const fn pc1_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, privileged read enable."]
    #[inline(always)]
    pub fn set_pc1_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 1, privileged write enable."]
    #[inline(always)]
    pub const fn pc1_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, privileged write enable."]
    #[inline(always)]
    pub fn set_pc1_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 1, non-secure."]
    #[inline(always)]
    pub const fn pc1_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, non-secure."]
    #[inline(always)]
    pub fn set_pc1_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 2, user read enable."]
    #[inline(always)]
    pub const fn pc2_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, user read enable."]
    #[inline(always)]
    pub fn set_pc2_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 2, user write enable."]
    #[inline(always)]
    pub const fn pc2_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, user write enable."]
    #[inline(always)]
    pub fn set_pc2_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 2, privileged read enable."]
    #[inline(always)]
    pub const fn pc2_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, privileged read enable."]
    #[inline(always)]
    pub fn set_pc2_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 2, privileged write enable."]
    #[inline(always)]
    pub const fn pc2_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, privileged write enable."]
    #[inline(always)]
    pub fn set_pc2_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 2, non-secure."]
    #[inline(always)]
    pub const fn pc2_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, non-secure."]
    #[inline(always)]
    pub fn set_pc2_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 3, user read enable."]
    #[inline(always)]
    pub const fn pc3_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, user read enable."]
    #[inline(always)]
    pub fn set_pc3_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 3, user write enable."]
    #[inline(always)]
    pub const fn pc3_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, user write enable."]
    #[inline(always)]
    pub fn set_pc3_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 3, privileged read enable."]
    #[inline(always)]
    pub const fn pc3_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, privileged read enable."]
    #[inline(always)]
    pub fn set_pc3_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 3, privileged write enable."]
    #[inline(always)]
    pub const fn pc3_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, privileged write enable."]
    #[inline(always)]
    pub fn set_pc3_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 3, non-secure."]
    #[inline(always)]
    pub const fn pc3_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, non-secure."]
    #[inline(always)]
    pub fn set_pc3_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuPrMsAtt0 {
    #[inline(always)]
    fn default() -> PpuPrMsAtt0 {
        PpuPrMsAtt0(0)
    }
}
#[doc = "Master attributes 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrMsAtt1(pub u32);
impl PpuPrMsAtt1 {
    #[doc = "Protection context 4, user read enable."]
    #[inline(always)]
    pub const fn pc4_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, user read enable."]
    #[inline(always)]
    pub fn set_pc4_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 4, user write enable."]
    #[inline(always)]
    pub const fn pc4_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, user write enable."]
    #[inline(always)]
    pub fn set_pc4_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 4, privileged read enable."]
    #[inline(always)]
    pub const fn pc4_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, privileged read enable."]
    #[inline(always)]
    pub fn set_pc4_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 4, privileged write enable."]
    #[inline(always)]
    pub const fn pc4_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, privileged write enable."]
    #[inline(always)]
    pub fn set_pc4_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 4, non-secure."]
    #[inline(always)]
    pub const fn pc4_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, non-secure."]
    #[inline(always)]
    pub fn set_pc4_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 5, user read enable."]
    #[inline(always)]
    pub const fn pc5_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, user read enable."]
    #[inline(always)]
    pub fn set_pc5_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 5, user write enable."]
    #[inline(always)]
    pub const fn pc5_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, user write enable."]
    #[inline(always)]
    pub fn set_pc5_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 5, privileged read enable."]
    #[inline(always)]
    pub const fn pc5_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, privileged read enable."]
    #[inline(always)]
    pub fn set_pc5_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 5, privileged write enable."]
    #[inline(always)]
    pub const fn pc5_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, privileged write enable."]
    #[inline(always)]
    pub fn set_pc5_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 5, non-secure."]
    #[inline(always)]
    pub const fn pc5_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, non-secure."]
    #[inline(always)]
    pub fn set_pc5_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 6, user read enable."]
    #[inline(always)]
    pub const fn pc6_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, user read enable."]
    #[inline(always)]
    pub fn set_pc6_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 6, user write enable."]
    #[inline(always)]
    pub const fn pc6_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, user write enable."]
    #[inline(always)]
    pub fn set_pc6_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 6, privileged read enable."]
    #[inline(always)]
    pub const fn pc6_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, privileged read enable."]
    #[inline(always)]
    pub fn set_pc6_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 6, privileged write enable."]
    #[inline(always)]
    pub const fn pc6_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, privileged write enable."]
    #[inline(always)]
    pub fn set_pc6_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 6, non-secure."]
    #[inline(always)]
    pub const fn pc6_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, non-secure."]
    #[inline(always)]
    pub fn set_pc6_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 7, user read enable."]
    #[inline(always)]
    pub const fn pc7_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, user read enable."]
    #[inline(always)]
    pub fn set_pc7_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 7, user write enable."]
    #[inline(always)]
    pub const fn pc7_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, user write enable."]
    #[inline(always)]
    pub fn set_pc7_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 7, privileged read enable."]
    #[inline(always)]
    pub const fn pc7_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, privileged read enable."]
    #[inline(always)]
    pub fn set_pc7_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 7, privileged write enable."]
    #[inline(always)]
    pub const fn pc7_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, privileged write enable."]
    #[inline(always)]
    pub fn set_pc7_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 7, non-secure."]
    #[inline(always)]
    pub const fn pc7_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, non-secure."]
    #[inline(always)]
    pub fn set_pc7_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuPrMsAtt1 {
    #[inline(always)]
    fn default() -> PpuPrMsAtt1 {
        PpuPrMsAtt1(0)
    }
}
#[doc = "Master attributes 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrMsAtt2(pub u32);
impl PpuPrMsAtt2 {
    #[doc = "Protection context 8, user read enable."]
    #[inline(always)]
    pub const fn pc8_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, user read enable."]
    #[inline(always)]
    pub fn set_pc8_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 8, user write enable."]
    #[inline(always)]
    pub const fn pc8_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, user write enable."]
    #[inline(always)]
    pub fn set_pc8_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 8, privileged read enable."]
    #[inline(always)]
    pub const fn pc8_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, privileged read enable."]
    #[inline(always)]
    pub fn set_pc8_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 8, privileged write enable."]
    #[inline(always)]
    pub const fn pc8_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, privileged write enable."]
    #[inline(always)]
    pub fn set_pc8_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 8, non-secure."]
    #[inline(always)]
    pub const fn pc8_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, non-secure."]
    #[inline(always)]
    pub fn set_pc8_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 9, user read enable."]
    #[inline(always)]
    pub const fn pc9_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, user read enable."]
    #[inline(always)]
    pub fn set_pc9_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 9, user write enable."]
    #[inline(always)]
    pub const fn pc9_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, user write enable."]
    #[inline(always)]
    pub fn set_pc9_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 9, privileged read enable."]
    #[inline(always)]
    pub const fn pc9_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, privileged read enable."]
    #[inline(always)]
    pub fn set_pc9_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 9, privileged write enable."]
    #[inline(always)]
    pub const fn pc9_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, privileged write enable."]
    #[inline(always)]
    pub fn set_pc9_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 9, non-secure."]
    #[inline(always)]
    pub const fn pc9_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, non-secure."]
    #[inline(always)]
    pub fn set_pc9_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 10, user read enable."]
    #[inline(always)]
    pub const fn pc10_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, user read enable."]
    #[inline(always)]
    pub fn set_pc10_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 10, user write enable."]
    #[inline(always)]
    pub const fn pc10_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, user write enable."]
    #[inline(always)]
    pub fn set_pc10_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 10, privileged read enable."]
    #[inline(always)]
    pub const fn pc10_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, privileged read enable."]
    #[inline(always)]
    pub fn set_pc10_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 10, privileged write enable."]
    #[inline(always)]
    pub const fn pc10_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, privileged write enable."]
    #[inline(always)]
    pub fn set_pc10_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 10, non-secure."]
    #[inline(always)]
    pub const fn pc10_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, non-secure."]
    #[inline(always)]
    pub fn set_pc10_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 11, user read enable."]
    #[inline(always)]
    pub const fn pc11_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, user read enable."]
    #[inline(always)]
    pub fn set_pc11_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 11, user write enable."]
    #[inline(always)]
    pub const fn pc11_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, user write enable."]
    #[inline(always)]
    pub fn set_pc11_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 11, privileged read enable."]
    #[inline(always)]
    pub const fn pc11_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, privileged read enable."]
    #[inline(always)]
    pub fn set_pc11_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 11, privileged write enable."]
    #[inline(always)]
    pub const fn pc11_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, privileged write enable."]
    #[inline(always)]
    pub fn set_pc11_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 11, non-secure."]
    #[inline(always)]
    pub const fn pc11_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, non-secure."]
    #[inline(always)]
    pub fn set_pc11_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuPrMsAtt2 {
    #[inline(always)]
    fn default() -> PpuPrMsAtt2 {
        PpuPrMsAtt2(0)
    }
}
#[doc = "Master attributes 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrMsAtt3(pub u32);
impl PpuPrMsAtt3 {
    #[doc = "Protection context 12, user read enable."]
    #[inline(always)]
    pub const fn pc12_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, user read enable."]
    #[inline(always)]
    pub fn set_pc12_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 12, user write enable."]
    #[inline(always)]
    pub const fn pc12_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, user write enable."]
    #[inline(always)]
    pub fn set_pc12_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 12, privileged read enable."]
    #[inline(always)]
    pub const fn pc12_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, privileged read enable."]
    #[inline(always)]
    pub fn set_pc12_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 12, privileged write enable."]
    #[inline(always)]
    pub const fn pc12_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, privileged write enable."]
    #[inline(always)]
    pub fn set_pc12_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 12, non-secure."]
    #[inline(always)]
    pub const fn pc12_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, non-secure."]
    #[inline(always)]
    pub fn set_pc12_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 13, user read enable."]
    #[inline(always)]
    pub const fn pc13_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, user read enable."]
    #[inline(always)]
    pub fn set_pc13_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 13, user write enable."]
    #[inline(always)]
    pub const fn pc13_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, user write enable."]
    #[inline(always)]
    pub fn set_pc13_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 13, privileged read enable."]
    #[inline(always)]
    pub const fn pc13_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, privileged read enable."]
    #[inline(always)]
    pub fn set_pc13_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 13, privileged write enable."]
    #[inline(always)]
    pub const fn pc13_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, privileged write enable."]
    #[inline(always)]
    pub fn set_pc13_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 13, non-secure."]
    #[inline(always)]
    pub const fn pc13_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, non-secure."]
    #[inline(always)]
    pub fn set_pc13_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 14, user read enable."]
    #[inline(always)]
    pub const fn pc14_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, user read enable."]
    #[inline(always)]
    pub fn set_pc14_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 14, user write enable."]
    #[inline(always)]
    pub const fn pc14_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, user write enable."]
    #[inline(always)]
    pub fn set_pc14_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 14, privileged read enable."]
    #[inline(always)]
    pub const fn pc14_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, privileged read enable."]
    #[inline(always)]
    pub fn set_pc14_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 14, privileged write enable."]
    #[inline(always)]
    pub const fn pc14_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, privileged write enable."]
    #[inline(always)]
    pub fn set_pc14_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 14, non-secure."]
    #[inline(always)]
    pub const fn pc14_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, non-secure."]
    #[inline(always)]
    pub fn set_pc14_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 15, user read enable."]
    #[inline(always)]
    pub const fn pc15_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, user read enable."]
    #[inline(always)]
    pub fn set_pc15_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 15, user write enable."]
    #[inline(always)]
    pub const fn pc15_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, user write enable."]
    #[inline(always)]
    pub fn set_pc15_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 15, privileged read enable."]
    #[inline(always)]
    pub const fn pc15_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, privileged read enable."]
    #[inline(always)]
    pub fn set_pc15_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 15, privileged write enable."]
    #[inline(always)]
    pub const fn pc15_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, privileged write enable."]
    #[inline(always)]
    pub fn set_pc15_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 15, non-secure."]
    #[inline(always)]
    pub const fn pc15_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, non-secure."]
    #[inline(always)]
    pub fn set_pc15_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuPrMsAtt3 {
    #[inline(always)]
    fn default() -> PpuPrMsAtt3 {
        PpuPrMsAtt3(0)
    }
}
#[doc = "Master region, size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrMsSize(pub u32);
impl PpuPrMsSize {
    #[doc = "This field specifies the size of the master region: '5': 64 B region The master region includes the SL_ADDR, SL_SIZE, SL_ATT0, ..., SL_ATT3, MS_ADDR, MS_SIZE, MS_ATT0, ..., MS_ATT3 registers. Therefore, the access privileges for all these registers is determined by MS_ATT0, ..., MS_ATT3."]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "This field specifies the size of the master region: '5': 64 B region The master region includes the SL_ADDR, SL_SIZE, SL_ATT0, ..., SL_ATT3, MS_ADDR, MS_SIZE, MS_ATT0, ..., MS_ATT3 registers. Therefore, the access privileges for all these registers is determined by MS_ATT0, ..., MS_ATT3."]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Master region enable: '1': Enabled."]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master region enable: '1': Enabled."]
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PpuPrMsSize {
    #[inline(always)]
    fn default() -> PpuPrMsSize {
        PpuPrMsSize(0)
    }
}
#[doc = "Slave region, base address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrSlAddr(pub u32);
impl PpuPrSlAddr {
    #[doc = "This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\] must be '0's."]
    #[inline(always)]
    pub const fn addr30(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\] must be '0's."]
    #[inline(always)]
    pub fn set_addr30(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for PpuPrSlAddr {
    #[inline(always)]
    fn default() -> PpuPrSlAddr {
        PpuPrSlAddr(0)
    }
}
#[doc = "Slave attributes 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrSlAtt0(pub u32);
impl PpuPrSlAtt0 {
    #[doc = "Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pc0_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pc0_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub const fn pc0_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn set_pc0_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 1, user read enable."]
    #[inline(always)]
    pub const fn pc1_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, user read enable."]
    #[inline(always)]
    pub fn set_pc1_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 1, user write enable."]
    #[inline(always)]
    pub const fn pc1_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, user write enable."]
    #[inline(always)]
    pub fn set_pc1_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 1, privileged read enable."]
    #[inline(always)]
    pub const fn pc1_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, privileged read enable."]
    #[inline(always)]
    pub fn set_pc1_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 1, privileged write enable."]
    #[inline(always)]
    pub const fn pc1_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, privileged write enable."]
    #[inline(always)]
    pub fn set_pc1_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 1, non-secure."]
    #[inline(always)]
    pub const fn pc1_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 1, non-secure."]
    #[inline(always)]
    pub fn set_pc1_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 2, user read enable."]
    #[inline(always)]
    pub const fn pc2_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, user read enable."]
    #[inline(always)]
    pub fn set_pc2_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 2, user write enable."]
    #[inline(always)]
    pub const fn pc2_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, user write enable."]
    #[inline(always)]
    pub fn set_pc2_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 2, privileged read enable."]
    #[inline(always)]
    pub const fn pc2_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, privileged read enable."]
    #[inline(always)]
    pub fn set_pc2_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 2, privileged write enable."]
    #[inline(always)]
    pub const fn pc2_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, privileged write enable."]
    #[inline(always)]
    pub fn set_pc2_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 2, non-secure."]
    #[inline(always)]
    pub const fn pc2_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 2, non-secure."]
    #[inline(always)]
    pub fn set_pc2_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 3, user read enable."]
    #[inline(always)]
    pub const fn pc3_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, user read enable."]
    #[inline(always)]
    pub fn set_pc3_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 3, user write enable."]
    #[inline(always)]
    pub const fn pc3_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, user write enable."]
    #[inline(always)]
    pub fn set_pc3_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 3, privileged read enable."]
    #[inline(always)]
    pub const fn pc3_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, privileged read enable."]
    #[inline(always)]
    pub fn set_pc3_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 3, privileged write enable."]
    #[inline(always)]
    pub const fn pc3_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, privileged write enable."]
    #[inline(always)]
    pub fn set_pc3_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 3, non-secure."]
    #[inline(always)]
    pub const fn pc3_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 3, non-secure."]
    #[inline(always)]
    pub fn set_pc3_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuPrSlAtt0 {
    #[inline(always)]
    fn default() -> PpuPrSlAtt0 {
        PpuPrSlAtt0(0)
    }
}
#[doc = "Slave attributes 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrSlAtt1(pub u32);
impl PpuPrSlAtt1 {
    #[doc = "Protection context 4, user read enable."]
    #[inline(always)]
    pub const fn pc4_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, user read enable."]
    #[inline(always)]
    pub fn set_pc4_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 4, user write enable."]
    #[inline(always)]
    pub const fn pc4_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, user write enable."]
    #[inline(always)]
    pub fn set_pc4_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 4, privileged read enable."]
    #[inline(always)]
    pub const fn pc4_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, privileged read enable."]
    #[inline(always)]
    pub fn set_pc4_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 4, privileged write enable."]
    #[inline(always)]
    pub const fn pc4_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, privileged write enable."]
    #[inline(always)]
    pub fn set_pc4_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 4, non-secure."]
    #[inline(always)]
    pub const fn pc4_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 4, non-secure."]
    #[inline(always)]
    pub fn set_pc4_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 5, user read enable."]
    #[inline(always)]
    pub const fn pc5_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, user read enable."]
    #[inline(always)]
    pub fn set_pc5_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 5, user write enable."]
    #[inline(always)]
    pub const fn pc5_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, user write enable."]
    #[inline(always)]
    pub fn set_pc5_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 5, privileged read enable."]
    #[inline(always)]
    pub const fn pc5_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, privileged read enable."]
    #[inline(always)]
    pub fn set_pc5_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 5, privileged write enable."]
    #[inline(always)]
    pub const fn pc5_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, privileged write enable."]
    #[inline(always)]
    pub fn set_pc5_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 5, non-secure."]
    #[inline(always)]
    pub const fn pc5_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 5, non-secure."]
    #[inline(always)]
    pub fn set_pc5_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 6, user read enable."]
    #[inline(always)]
    pub const fn pc6_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, user read enable."]
    #[inline(always)]
    pub fn set_pc6_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 6, user write enable."]
    #[inline(always)]
    pub const fn pc6_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, user write enable."]
    #[inline(always)]
    pub fn set_pc6_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 6, privileged read enable."]
    #[inline(always)]
    pub const fn pc6_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, privileged read enable."]
    #[inline(always)]
    pub fn set_pc6_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 6, privileged write enable."]
    #[inline(always)]
    pub const fn pc6_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, privileged write enable."]
    #[inline(always)]
    pub fn set_pc6_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 6, non-secure."]
    #[inline(always)]
    pub const fn pc6_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 6, non-secure."]
    #[inline(always)]
    pub fn set_pc6_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 7, user read enable."]
    #[inline(always)]
    pub const fn pc7_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, user read enable."]
    #[inline(always)]
    pub fn set_pc7_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 7, user write enable."]
    #[inline(always)]
    pub const fn pc7_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, user write enable."]
    #[inline(always)]
    pub fn set_pc7_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 7, privileged read enable."]
    #[inline(always)]
    pub const fn pc7_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, privileged read enable."]
    #[inline(always)]
    pub fn set_pc7_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 7, privileged write enable."]
    #[inline(always)]
    pub const fn pc7_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, privileged write enable."]
    #[inline(always)]
    pub fn set_pc7_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 7, non-secure."]
    #[inline(always)]
    pub const fn pc7_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 7, non-secure."]
    #[inline(always)]
    pub fn set_pc7_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuPrSlAtt1 {
    #[inline(always)]
    fn default() -> PpuPrSlAtt1 {
        PpuPrSlAtt1(0)
    }
}
#[doc = "Slave attributes 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrSlAtt2(pub u32);
impl PpuPrSlAtt2 {
    #[doc = "Protection context 8, user read enable."]
    #[inline(always)]
    pub const fn pc8_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, user read enable."]
    #[inline(always)]
    pub fn set_pc8_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 8, user write enable."]
    #[inline(always)]
    pub const fn pc8_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, user write enable."]
    #[inline(always)]
    pub fn set_pc8_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 8, privileged read enable."]
    #[inline(always)]
    pub const fn pc8_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, privileged read enable."]
    #[inline(always)]
    pub fn set_pc8_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 8, privileged write enable."]
    #[inline(always)]
    pub const fn pc8_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, privileged write enable."]
    #[inline(always)]
    pub fn set_pc8_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 8, non-secure."]
    #[inline(always)]
    pub const fn pc8_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 8, non-secure."]
    #[inline(always)]
    pub fn set_pc8_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 9, user read enable."]
    #[inline(always)]
    pub const fn pc9_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, user read enable."]
    #[inline(always)]
    pub fn set_pc9_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 9, user write enable."]
    #[inline(always)]
    pub const fn pc9_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, user write enable."]
    #[inline(always)]
    pub fn set_pc9_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 9, privileged read enable."]
    #[inline(always)]
    pub const fn pc9_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, privileged read enable."]
    #[inline(always)]
    pub fn set_pc9_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 9, privileged write enable."]
    #[inline(always)]
    pub const fn pc9_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, privileged write enable."]
    #[inline(always)]
    pub fn set_pc9_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 9, non-secure."]
    #[inline(always)]
    pub const fn pc9_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 9, non-secure."]
    #[inline(always)]
    pub fn set_pc9_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 10, user read enable."]
    #[inline(always)]
    pub const fn pc10_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, user read enable."]
    #[inline(always)]
    pub fn set_pc10_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 10, user write enable."]
    #[inline(always)]
    pub const fn pc10_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, user write enable."]
    #[inline(always)]
    pub fn set_pc10_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 10, privileged read enable."]
    #[inline(always)]
    pub const fn pc10_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, privileged read enable."]
    #[inline(always)]
    pub fn set_pc10_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 10, privileged write enable."]
    #[inline(always)]
    pub const fn pc10_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, privileged write enable."]
    #[inline(always)]
    pub fn set_pc10_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 10, non-secure."]
    #[inline(always)]
    pub const fn pc10_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 10, non-secure."]
    #[inline(always)]
    pub fn set_pc10_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 11, user read enable."]
    #[inline(always)]
    pub const fn pc11_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, user read enable."]
    #[inline(always)]
    pub fn set_pc11_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 11, user write enable."]
    #[inline(always)]
    pub const fn pc11_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, user write enable."]
    #[inline(always)]
    pub fn set_pc11_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 11, privileged read enable."]
    #[inline(always)]
    pub const fn pc11_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, privileged read enable."]
    #[inline(always)]
    pub fn set_pc11_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 11, privileged write enable."]
    #[inline(always)]
    pub const fn pc11_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, privileged write enable."]
    #[inline(always)]
    pub fn set_pc11_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 11, non-secure."]
    #[inline(always)]
    pub const fn pc11_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 11, non-secure."]
    #[inline(always)]
    pub fn set_pc11_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuPrSlAtt2 {
    #[inline(always)]
    fn default() -> PpuPrSlAtt2 {
        PpuPrSlAtt2(0)
    }
}
#[doc = "Slave attributes 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrSlAtt3(pub u32);
impl PpuPrSlAtt3 {
    #[doc = "Protection context 12, user read enable."]
    #[inline(always)]
    pub const fn pc12_ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, user read enable."]
    #[inline(always)]
    pub fn set_pc12_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Protection context 12, user write enable."]
    #[inline(always)]
    pub const fn pc12_uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, user write enable."]
    #[inline(always)]
    pub fn set_pc12_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Protection context 12, privileged read enable."]
    #[inline(always)]
    pub const fn pc12_pr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, privileged read enable."]
    #[inline(always)]
    pub fn set_pc12_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context 12, privileged write enable."]
    #[inline(always)]
    pub const fn pc12_pw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, privileged write enable."]
    #[inline(always)]
    pub fn set_pc12_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Protection context 12, non-secure."]
    #[inline(always)]
    pub const fn pc12_ns(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 12, non-secure."]
    #[inline(always)]
    pub fn set_pc12_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Protection context 13, user read enable."]
    #[inline(always)]
    pub const fn pc13_ur(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, user read enable."]
    #[inline(always)]
    pub fn set_pc13_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Protection context 13, user write enable."]
    #[inline(always)]
    pub const fn pc13_uw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, user write enable."]
    #[inline(always)]
    pub fn set_pc13_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protection context 13, privileged read enable."]
    #[inline(always)]
    pub const fn pc13_pr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, privileged read enable."]
    #[inline(always)]
    pub fn set_pc13_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Protection context 13, privileged write enable."]
    #[inline(always)]
    pub const fn pc13_pw(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, privileged write enable."]
    #[inline(always)]
    pub fn set_pc13_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Protection context 13, non-secure."]
    #[inline(always)]
    pub const fn pc13_ns(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 13, non-secure."]
    #[inline(always)]
    pub fn set_pc13_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Protection context 14, user read enable."]
    #[inline(always)]
    pub const fn pc14_ur(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, user read enable."]
    #[inline(always)]
    pub fn set_pc14_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context 14, user write enable."]
    #[inline(always)]
    pub const fn pc14_uw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, user write enable."]
    #[inline(always)]
    pub fn set_pc14_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Protection context 14, privileged read enable."]
    #[inline(always)]
    pub const fn pc14_pr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, privileged read enable."]
    #[inline(always)]
    pub fn set_pc14_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Protection context 14, privileged write enable."]
    #[inline(always)]
    pub const fn pc14_pw(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, privileged write enable."]
    #[inline(always)]
    pub fn set_pc14_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Protection context 14, non-secure."]
    #[inline(always)]
    pub const fn pc14_ns(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 14, non-secure."]
    #[inline(always)]
    pub fn set_pc14_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Protection context 15, user read enable."]
    #[inline(always)]
    pub const fn pc15_ur(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, user read enable."]
    #[inline(always)]
    pub fn set_pc15_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Protection context 15, user write enable."]
    #[inline(always)]
    pub const fn pc15_uw(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, user write enable."]
    #[inline(always)]
    pub fn set_pc15_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Protection context 15, privileged read enable."]
    #[inline(always)]
    pub const fn pc15_pr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, privileged read enable."]
    #[inline(always)]
    pub fn set_pc15_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protection context 15, privileged write enable."]
    #[inline(always)]
    pub const fn pc15_pw(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, privileged write enable."]
    #[inline(always)]
    pub fn set_pc15_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protection context 15, non-secure."]
    #[inline(always)]
    pub const fn pc15_ns(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context 15, non-secure."]
    #[inline(always)]
    pub fn set_pc15_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PpuPrSlAtt3 {
    #[inline(always)]
    fn default() -> PpuPrSlAtt3 {
        PpuPrSlAtt3(0)
    }
}
#[doc = "Slave region, size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PpuPrSlSize(pub u32);
impl PpuPrSlSize {
    #[doc = "This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PpuPrSlSize {
    #[inline(always)]
    fn default() -> PpuPrSlSize {
        PpuPrSlSize(0)
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
