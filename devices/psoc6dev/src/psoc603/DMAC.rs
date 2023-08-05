#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "DMA controller channel"]
use core::prelude::v1::derive;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<ChCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Channel current indices"]
    #[inline(always)]
    pub const fn idx(self) -> crate::common::Reg<Idx, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Channel current source address"]
    #[inline(always)]
    pub const fn src(self) -> crate::common::Reg<Src, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Channel current destination address"]
    #[inline(always)]
    pub const fn dst(self) -> crate::common::Reg<Dst, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Channel current descriptor pointer"]
    #[inline(always)]
    pub const fn curr(self) -> crate::common::Reg<Curr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Channle software trigger"]
    #[inline(always)]
    pub const fn tr_cmd(self) -> crate::common::Reg<TrCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Channel descriptor status"]
    #[inline(always)]
    pub const fn descr_status(self) -> crate::common::Reg<DescrStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Channel descriptor control"]
    #[inline(always)]
    pub const fn descr_ctl(self) -> crate::common::Reg<DescrCtl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Channel descriptor source"]
    #[inline(always)]
    pub const fn descr_src(self) -> crate::common::Reg<DescrSrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Channel descriptor destination"]
    #[inline(always)]
    pub const fn descr_dst(self) -> crate::common::Reg<DescrDst, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "Channel descriptor X size"]
    #[inline(always)]
    pub const fn descr_x_size(self) -> crate::common::Reg<DescrXSize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "Channel descriptor X increment"]
    #[inline(always)]
    pub const fn descr_x_incr(self) -> crate::common::Reg<DescrXIncr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "Channel descriptor Y size"]
    #[inline(always)]
    pub const fn descr_y_size(self) -> crate::common::Reg<DescrYSize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "Channel descriptor Y increment"]
    #[inline(always)]
    pub const fn descr_y_incr(self) -> crate::common::Reg<DescrYIncr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "Channel descriptor next pointer"]
    #[inline(always)]
    pub const fn descr_next(self) -> crate::common::Reg<DescrNext, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
}
#[doc = "DMAC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac {
    ptr: *mut u8,
}
unsafe impl Send for Dmac {}
unsafe impl Sync for Dmac {}
impl Dmac {
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
    pub const fn ctl(self) -> crate::common::Reg<DmacCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Active channels"]
    #[inline(always)]
    pub const fn active(self) -> crate::common::Reg<Active, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "DMA controller channel"]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 2usize);
        unsafe { Ch::from_ptr(self.ptr.add(4096usize + n * 256usize) as _) }
    }
}
#[doc = "Active channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Active(pub u32);
impl Active {
    #[doc = "Specifies active channels; i.e. enabled channels whose trigger got activated."]
    #[inline(always)]
    pub const fn active(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies active channels; i.e. enabled channels whose trigger got activated."]
    #[inline(always)]
    pub fn set_active(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Active {
    #[inline(always)]
    fn default() -> Active {
        Active(0)
    }
}
#[doc = "Channel control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChCtl(pub u32);
impl ChCtl {
    #[doc = "User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the access control is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
    #[inline(always)]
    pub const fn b(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
    #[inline(always)]
    pub fn set_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    pub const fn pc(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the context is inherited from the write transaction and not specified by the transaction write data. All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    pub fn set_pc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group and within this priority group, the following 'roundrobin' arbitration is applied. A 'round' consists of a contiguous sequence of channel activations, within this priority group, without any repetition. Within a round, higher priority is given to the lower channel indices. The notion of a round guarantees that within a group, higher channel indices do not yield to lower indices indefinitely."]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group and within this priority group, the following 'roundrobin' arbitration is applied. A 'round' consists of a contiguous sequence of channel activations, within this priority group, without any repetition. Within a round, higher priority is given to the lower channel indices. The notion of a round guarantees that within a group, higher channel indices do not yield to lower indices indefinitely."]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' when an error interrupt cause is activated."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' when an error interrupt cause is activated."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChCtl {
    #[inline(always)]
    fn default() -> ChCtl {
        ChCtl(0)
    }
}
#[doc = "Channel current descriptor pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Curr(pub u32);
impl Curr {
    #[doc = "Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    pub const fn ptr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    pub fn set_ptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Curr {
    #[inline(always)]
    fn default() -> Curr {
        Curr(0)
    }
}
#[doc = "Channel descriptor control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DescrCtl(pub u32);
impl DescrCtl {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn wait_for_deact(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_wait_for_deact(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn intr_type(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_intr_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn tr_out_type(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_tr_out_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn tr_in_type(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_tr_in_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn data_prefetch(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_data_prefetch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn data_size(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_data_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ch_disable(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ch_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn src_transfer_size(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_src_transfer_size(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dst_transfer_size(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dst_transfer_size(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn descr_type(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_descr_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for DescrCtl {
    #[inline(always)]
    fn default() -> DescrCtl {
        DescrCtl(0)
    }
}
#[doc = "Channel descriptor destination"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DescrDst(pub u32);
impl DescrDst {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DescrDst {
    #[inline(always)]
    fn default() -> DescrDst {
        DescrDst(0)
    }
}
#[doc = "Channel descriptor next pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DescrNext(pub u32);
impl DescrNext {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ptr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DescrNext {
    #[inline(always)]
    fn default() -> DescrNext {
        DescrNext(0)
    }
}
#[doc = "Channel descriptor source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DescrSrc(pub u32);
impl DescrSrc {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DescrSrc {
    #[inline(always)]
    fn default() -> DescrSrc {
        DescrSrc(0)
    }
}
#[doc = "Channel descriptor status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DescrStatus(pub u32);
impl DescrStatus {
    #[doc = "Indicates whether the descriptor information present in DESCR_CTL, DESCR_SRC, DESCR_DST, DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE, DESCR_Y_INCR, DESCR_NEXT status registers is valid or not."]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the descriptor information present in DESCR_CTL, DESCR_SRC, DESCR_DST, DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE, DESCR_Y_INCR, DESCR_NEXT status registers is valid or not."]
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DescrStatus {
    #[inline(always)]
    fn default() -> DescrStatus {
        DescrStatus(0)
    }
}
#[doc = "Channel descriptor X increment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DescrXIncr(pub u32);
impl DescrXIncr {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn src_x(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_src_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dst_x(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dst_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DescrXIncr {
    #[inline(always)]
    fn default() -> DescrXIncr {
        DescrXIncr(0)
    }
}
#[doc = "Channel descriptor X size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DescrXSize(pub u32);
impl DescrXSize {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn x_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_x_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DescrXSize {
    #[inline(always)]
    fn default() -> DescrXSize {
        DescrXSize(0)
    }
}
#[doc = "Channel descriptor Y increment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DescrYIncr(pub u32);
impl DescrYIncr {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn src_y(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_src_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dst_y(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dst_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DescrYIncr {
    #[inline(always)]
    fn default() -> DescrYIncr {
        DescrYIncr(0)
    }
}
#[doc = "Channel descriptor Y size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DescrYSize(pub u32);
impl DescrYSize {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn y_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_y_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DescrYSize {
    #[inline(always)]
    fn default() -> DescrYSize {
        DescrYSize(0)
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmacCtl(pub u32);
impl DmacCtl {
    #[doc = "IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> Enabled {
        let val = (self.0 >> 31usize) & 0x01;
        Enabled::from_bits(val as u8)
    }
    #[doc = "IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: Enabled) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for DmacCtl {
    #[inline(always)]
    fn default() -> DmacCtl {
        DmacCtl(0)
    }
}
#[doc = "Channel current destination address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dst(pub u32);
impl Dst {
    #[doc = "Current address of destination location."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current address of destination location."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dst {
    #[inline(always)]
    fn default() -> Dst {
        Dst(0)
    }
}
#[doc = "Channel current indices"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idx(pub u32);
impl Idx {
    #[doc = "Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor."]
    #[inline(always)]
    pub const fn x(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor."]
    #[inline(always)]
    pub fn set_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Specifies the Y loop index, with Y_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor.."]
    #[inline(always)]
    pub const fn y(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Specifies the Y loop index, with Y_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor.."]
    #[inline(always)]
    pub fn set_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Idx {
    #[inline(always)]
    fn default() -> Idx {
        Idx(0)
    }
}
#[doc = "Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
    #[inline(always)]
    pub const fn completion(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
    #[inline(always)]
    pub fn set_completion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Activated (set to '1') on a bus error for a load from the source."]
    #[inline(always)]
    pub const fn src_bus_error(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Activated (set to '1') on a bus error for a load from the source."]
    #[inline(always)]
    pub fn set_src_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Activated (set to '1') on a bus error for a store to the destination."]
    #[inline(always)]
    pub const fn dst_bus_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Activated (set to '1') on a bus error for a store to the destination."]
    #[inline(always)]
    pub fn set_dst_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Activated (set to '1') on a misalignment of the source address."]
    #[inline(always)]
    pub const fn src_misal(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Activated (set to '1') on a misalignment of the source address."]
    #[inline(always)]
    pub fn set_src_misal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Activated (set to '1') on a misalignment of the destination address."]
    #[inline(always)]
    pub const fn dst_misal(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Activated (set to '1') on a misalignment of the destination address."]
    #[inline(always)]
    pub fn set_dst_misal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
    #[inline(always)]
    pub const fn curr_ptr_null(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
    #[inline(always)]
    pub fn set_curr_ptr_null(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
    #[inline(always)]
    pub const fn active_ch_disabled(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
    #[inline(always)]
    pub fn set_active_ch_disabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Activated (set to '1') on a bus error for a load of the descriptor."]
    #[inline(always)]
    pub const fn descr_bus_error(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Activated (set to '1') on a bus error for a load of the descriptor."]
    #[inline(always)]
    pub fn set_descr_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    #[doc = "Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub const fn completion(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn set_completion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn src_bus_error(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_src_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dst_bus_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dst_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn src_misal(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_src_misal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dst_misal(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dst_misal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn curr_ptr_null(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_curr_ptr_null(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn active_ch_disabled(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_active_ch_disabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn descr_bus_error(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_descr_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    pub const fn completion(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn set_completion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn src_bus_error(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_src_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dst_bus_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dst_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn src_misal(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_src_misal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dst_misal(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dst_misal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn curr_ptr_null(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_curr_ptr_null(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn active_ch_disabled(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_active_ch_disabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn descr_bus_error(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_descr_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    #[doc = "Write this field with '1' to set corresponding INTR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub const fn completion(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write this field with '1' to set corresponding INTR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn set_completion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn src_bus_error(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_src_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dst_bus_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dst_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn src_misal(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_src_misal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn dst_misal(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_dst_misal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn curr_ptr_null(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_curr_ptr_null(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn active_ch_disabled(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_active_ch_disabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn descr_bus_error(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_descr_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Channel current source address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src(pub u32);
impl Src {
    #[doc = "Current address of source location."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current address of source location."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Src {
    #[inline(always)]
    fn default() -> Src {
        Src(0)
    }
}
#[doc = "Channle software trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCmd(pub u32);
impl TrCmd {
    #[doc = "Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    pub const fn activate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    pub fn set_activate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for TrCmd {
    #[inline(always)]
    fn default() -> TrCmd {
        TrCmd(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enabled {
    #[doc = "N/A"]
    DISABLED = 0,
    #[doc = "N/A"]
    ENABLED = 0x01,
}
impl Enabled {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enabled {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enabled {
    #[inline(always)]
    fn from(val: u8) -> Enabled {
        Enabled::from_bits(val)
    }
}
impl From<Enabled> for u8 {
    #[inline(always)]
    fn from(val: Enabled) -> u8 {
        Enabled::to_bits(val)
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
