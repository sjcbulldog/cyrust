#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "MPU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpu {
    ptr: *mut u8,
}
unsafe impl Send for Mpu {}
unsafe impl Sync for Mpu {}
impl Mpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master control"]
    #[inline(always)]
    pub const fn ms_ctl(self) -> crate::common::Reg<MsCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Master control read mirror"]
    #[inline(always)]
    pub const fn ms_ctl_read_mir(
        self,
        n: usize,
    ) -> crate::common::Reg<MsCtlReadMir, crate::common::R> {
        assert!(n < 127usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize + n * 4usize) as _) }
    }
    #[doc = "MPU structure"]
    #[inline(always)]
    pub const fn mpu_struct(self, n: usize) -> MpuStruct {
        assert!(n < 8usize);
        unsafe { MpuStruct::from_ptr(self.ptr.add(512usize + n * 32usize) as _) }
    }
}
#[doc = "MPU structure"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuStruct {
    ptr: *mut u8,
}
unsafe impl Send for MpuStruct {}
unsafe impl Sync for MpuStruct {}
impl MpuStruct {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MPU region address"]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "MPU region attrributes"]
    #[inline(always)]
    pub const fn att(self) -> crate::common::Reg<Att, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
}
#[doc = "Protection"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prot {
    ptr: *mut u8,
}
unsafe impl Send for Prot {}
unsafe impl Sync for Prot {}
impl Prot {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SMPU"]
    #[inline(always)]
    pub const fn smpu(self) -> Smpu {
        unsafe { Smpu::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "MPU"]
    #[inline(always)]
    pub const fn mpu(self, n: usize) -> Mpu {
        assert!(n < 16usize);
        unsafe { Mpu::from_ptr(self.ptr.add(16384usize + n * 1024usize) as _) }
    }
}
#[doc = "SMPU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpu {
    ptr: *mut u8,
}
unsafe impl Send for Smpu {}
unsafe impl Sync for Smpu {}
impl Smpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master 0 protection context control"]
    #[inline(always)]
    pub const fn ms0_ctl(self) -> crate::common::Reg<Ms0Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Master 1 protection context control"]
    #[inline(always)]
    pub const fn ms1_ctl(self) -> crate::common::Reg<Ms1Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Master 2 protection context control"]
    #[inline(always)]
    pub const fn ms2_ctl(self) -> crate::common::Reg<Ms2Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Master 3 protection context control"]
    #[inline(always)]
    pub const fn ms3_ctl(self) -> crate::common::Reg<Ms3Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Master 4 protection context control"]
    #[inline(always)]
    pub const fn ms4_ctl(self) -> crate::common::Reg<Ms4Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Master 5 protection context control"]
    #[inline(always)]
    pub const fn ms5_ctl(self) -> crate::common::Reg<Ms5Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Master 6 protection context control"]
    #[inline(always)]
    pub const fn ms6_ctl(self) -> crate::common::Reg<Ms6Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Master 7 protection context control"]
    #[inline(always)]
    pub const fn ms7_ctl(self) -> crate::common::Reg<Ms7Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Master 8 protection context control"]
    #[inline(always)]
    pub const fn ms8_ctl(self) -> crate::common::Reg<Ms8Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Master 9 protection context control"]
    #[inline(always)]
    pub const fn ms9_ctl(self) -> crate::common::Reg<Ms9Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Master 10 protection context control"]
    #[inline(always)]
    pub const fn ms10_ctl(self) -> crate::common::Reg<Ms10Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Master 11 protection context control"]
    #[inline(always)]
    pub const fn ms11_ctl(self) -> crate::common::Reg<Ms11Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Master 12 protection context control"]
    #[inline(always)]
    pub const fn ms12_ctl(self) -> crate::common::Reg<Ms12Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Master 13 protection context control"]
    #[inline(always)]
    pub const fn ms13_ctl(self) -> crate::common::Reg<Ms13Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Master 14 protection context control"]
    #[inline(always)]
    pub const fn ms14_ctl(self) -> crate::common::Reg<Ms14Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Master 15 protection context control"]
    #[inline(always)]
    pub const fn ms15_ctl(self) -> crate::common::Reg<Ms15Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "SMPU structure"]
    #[inline(always)]
    pub const fn smpu_struct(self, n: usize) -> SmpuStruct {
        assert!(n < 16usize);
        unsafe { SmpuStruct::from_ptr(self.ptr.add(8192usize + n * 64usize) as _) }
    }
}
#[doc = "SMPU structure"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmpuStruct {
    ptr: *mut u8,
}
unsafe impl Send for SmpuStruct {}
unsafe impl Sync for SmpuStruct {}
impl SmpuStruct {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SMPU region address 0 (slave structure)"]
    #[inline(always)]
    pub const fn addr0(self) -> crate::common::Reg<Addr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "SMPU region attributes 0 (slave structure)"]
    #[inline(always)]
    pub const fn att0(self) -> crate::common::Reg<Att0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "SMPU region address 1 (master structure)"]
    #[inline(always)]
    pub const fn addr1(self) -> crate::common::Reg<Addr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "SMPU region attributes 1 (master structure)"]
    #[inline(always)]
    pub const fn att1(self) -> crate::common::Reg<Att1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
}
#[doc = "MPU region address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr(pub u32);
impl Addr {
    #[doc = "This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by MPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub const fn subregion_disable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by MPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub fn set_subregion_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\] are ignored."]
    #[inline(always)]
    pub const fn addr24(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\] are ignored."]
    #[inline(always)]
    pub fn set_addr24(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Addr {
    #[inline(always)]
    fn default() -> Addr {
        Addr(0)
    }
}
#[doc = "SMPU region address 0 (slave structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr0(pub u32);
impl Addr0 {
    #[doc = "This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (ATT0.REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by ATT0 applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub const fn subregion_disable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (ATT0.REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by ATT0 applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub fn set_subregion_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT0.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\] are ignored."]
    #[inline(always)]
    pub const fn addr24(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT0.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\] are ignored."]
    #[inline(always)]
    pub fn set_addr24(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Addr0 {
    #[inline(always)]
    fn default() -> Addr0 {
        Addr0(0)
    }
}
#[doc = "SMPU region address 1 (master structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr1(pub u32);
impl Addr1 {
    #[doc = "This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub const fn subregion_disable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub fn set_subregion_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This field specifies the most significant bits of the 32-bit address of an address region. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub const fn addr24(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "This field specifies the most significant bits of the 32-bit address of an address region. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub fn set_addr24(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Addr1 {
    #[inline(always)]
    fn default() -> Addr1 {
        Addr1(0)
    }
}
#[doc = "MPU region attrributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Att(pub u32);
impl Att {
    #[doc = "User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub const fn ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub const fn uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    pub const fn ux(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    pub fn set_ux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pw(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    pub const fn px(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    pub fn set_px(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Att {
    #[inline(always)]
    fn default() -> Att {
        Att(0)
    }
}
#[doc = "SMPU region attributes 0 (slave structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Att0(pub u32);
impl Att0 {
    #[doc = "User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub const fn ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub const fn uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    pub const fn ux(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    pub fn set_ux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pw(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    pub const fn px(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    pub fn set_px(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This field specifies protection context identifier based access control for protection context '0'."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies protection context identifier based access control for protection context '0'."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x7fff;
        val as u16
    }
    #[doc = "This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 9usize)) | (((val as u32) & 0x7fff) << 9usize);
    }
    #[doc = "This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub const fn pc_match(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub fn set_pc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Att0 {
    #[inline(always)]
    fn default() -> Att0 {
        Att0(0)
    }
}
#[doc = "SMPU region attributes 1 (master structure)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Att1(pub u32);
impl Att1 {
    #[doc = "User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed). Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub const fn ur(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed). Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn set_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub const fn uw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_uw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed). Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
    #[inline(always)]
    pub const fn ux(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed). Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn set_ux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed). Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed). Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub const fn pw(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn set_pw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed). Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
    #[inline(always)]
    pub const fn px(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed). Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn set_px(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This field specifies protection context identifier based access control for protection context '0'."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies protection context identifier based access control for protection context '0'."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x7fff;
        val as u16
    }
    #[doc = "This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 9usize)) | (((val as u32) & 0x7fff) << 9usize);
    }
    #[doc = "This field specifies the region size: '7': 256 B region (8 32 B subregions) Note: this field is read-only."]
    #[inline(always)]
    pub const fn region_size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "This field specifies the region size: '7': 256 B region (8 32 B subregions) Note: this field is read-only."]
    #[inline(always)]
    pub fn set_region_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub const fn pc_match(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub fn set_pc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Att1 {
    #[inline(always)]
    fn default() -> Att1 {
        Att1(0)
    }
}
#[doc = "Master 0 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms0Ctl(pub u32);
impl Ms0Ctl {
    #[doc = "Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Protection context mask for protection context '0'. This field is a constant '0': - PC_MASK_0 is '0': MPU MS_CTL.PC\\[3:0\\] can NOT be set to '0' and PC\\[3:0\\] is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\] can be changed."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Protection context mask for protection context '0'. This field is a constant '0': - PC_MASK_0 is '0': MPU MS_CTL.PC\\[3:0\\] can NOT be set to '0' and PC\\[3:0\\] is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\] can be changed."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\] indicates if the MPU MS_CTL.PC\\[3:0\\] protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\] is '0': MPU MS_CTL.PC\\[3:0\\] can NOT be set to 'i+1'; and PC\\[3:0\\] is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\] can be changed. - PC_MASK_15_TO_1\\[i\\] is '1': MPU MS_CTL.PC\\[3:0\\] can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\] is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\] always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\] on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\] indicates if the MPU MS_CTL.PC\\[3:0\\] protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\] is '0': MPU MS_CTL.PC\\[3:0\\] can NOT be set to 'i+1'; and PC\\[3:0\\] is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\] can be changed. - PC_MASK_15_TO_1\\[i\\] is '1': MPU MS_CTL.PC\\[3:0\\] can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\] is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\] always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\] on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms0Ctl {
    #[inline(always)]
    fn default() -> Ms0Ctl {
        Ms0Ctl(0)
    }
}
#[doc = "Master 10 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms10Ctl(pub u32);
impl Ms10Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms10Ctl {
    #[inline(always)]
    fn default() -> Ms10Ctl {
        Ms10Ctl(0)
    }
}
#[doc = "Master 11 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms11Ctl(pub u32);
impl Ms11Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms11Ctl {
    #[inline(always)]
    fn default() -> Ms11Ctl {
        Ms11Ctl(0)
    }
}
#[doc = "Master 12 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms12Ctl(pub u32);
impl Ms12Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms12Ctl {
    #[inline(always)]
    fn default() -> Ms12Ctl {
        Ms12Ctl(0)
    }
}
#[doc = "Master 13 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms13Ctl(pub u32);
impl Ms13Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms13Ctl {
    #[inline(always)]
    fn default() -> Ms13Ctl {
        Ms13Ctl(0)
    }
}
#[doc = "Master 14 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms14Ctl(pub u32);
impl Ms14Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms14Ctl {
    #[inline(always)]
    fn default() -> Ms14Ctl {
        Ms14Ctl(0)
    }
}
#[doc = "Master 15 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms15Ctl(pub u32);
impl Ms15Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms15Ctl {
    #[inline(always)]
    fn default() -> Ms15Ctl {
        Ms15Ctl(0)
    }
}
#[doc = "Master 1 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms1Ctl(pub u32);
impl Ms1Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms1Ctl {
    #[inline(always)]
    fn default() -> Ms1Ctl {
        Ms1Ctl(0)
    }
}
#[doc = "Master 2 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms2Ctl(pub u32);
impl Ms2Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms2Ctl {
    #[inline(always)]
    fn default() -> Ms2Ctl {
        Ms2Ctl(0)
    }
}
#[doc = "Master 3 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms3Ctl(pub u32);
impl Ms3Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms3Ctl {
    #[inline(always)]
    fn default() -> Ms3Ctl {
        Ms3Ctl(0)
    }
}
#[doc = "Master 4 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms4Ctl(pub u32);
impl Ms4Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms4Ctl {
    #[inline(always)]
    fn default() -> Ms4Ctl {
        Ms4Ctl(0)
    }
}
#[doc = "Master 5 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms5Ctl(pub u32);
impl Ms5Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms5Ctl {
    #[inline(always)]
    fn default() -> Ms5Ctl {
        Ms5Ctl(0)
    }
}
#[doc = "Master 6 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms6Ctl(pub u32);
impl Ms6Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms6Ctl {
    #[inline(always)]
    fn default() -> Ms6Ctl {
        Ms6Ctl(0)
    }
}
#[doc = "Master 7 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms7Ctl(pub u32);
impl Ms7Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms7Ctl {
    #[inline(always)]
    fn default() -> Ms7Ctl {
        Ms7Ctl(0)
    }
}
#[doc = "Master 8 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms8Ctl(pub u32);
impl Ms8Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms8Ctl {
    #[inline(always)]
    fn default() -> Ms8Ctl {
        Ms8Ctl(0)
    }
}
#[doc = "Master 9 protection context control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms9Ctl(pub u32);
impl Ms9Ctl {
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.P."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.NS."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub const fn pc_mask_0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn set_pc_mask_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub const fn pc_mask_15_to_1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn set_pc_mask_15_to_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Ms9Ctl {
    #[inline(always)]
    fn default() -> Ms9Ctl {
        Ms9Ctl(0)
    }
}
#[doc = "Master control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MsCtl(pub u32);
impl MsCtl {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn pc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_pc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Saved protection context. Modifications to this field are constrained by the associated MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\] fields."]
    #[inline(always)]
    pub const fn pc_saved(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Saved protection context. Modifications to this field are constrained by the associated MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\] fields."]
    #[inline(always)]
    pub fn set_pc_saved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for MsCtl {
    #[inline(always)]
    fn default() -> MsCtl {
        MsCtl(0)
    }
}
#[doc = "Master control read mirror"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MsCtlReadMir(pub u32);
impl MsCtlReadMir {
    #[doc = "Read-only mirror of MS_CTL.PC"]
    #[inline(always)]
    pub const fn pc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Read-only mirror of MS_CTL.PC"]
    #[inline(always)]
    pub fn set_pc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Read-only mirror of MS_CTL.PC_SAVED"]
    #[inline(always)]
    pub const fn pc_saved(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Read-only mirror of MS_CTL.PC_SAVED"]
    #[inline(always)]
    pub fn set_pc_saved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for MsCtlReadMir {
    #[inline(always)]
    fn default() -> MsCtlReadMir {
        MsCtlReadMir(0)
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