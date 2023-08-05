#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "DW channel structure"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChStruct {
    ptr: *mut u8,
}
unsafe impl Send for ChStruct {}
unsafe impl Sync for ChStruct {}
impl ChStruct {
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
    pub const fn ch_ctl(self) -> crate::common::Reg<ChCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Channel status"]
    #[inline(always)]
    pub const fn ch_status(self) -> crate::common::Reg<ChStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Channel current indices"]
    #[inline(always)]
    pub const fn ch_idx(self) -> crate::common::Reg<ChIdx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Channel current descriptor pointer"]
    #[inline(always)]
    pub const fn ch_curr_ptr(self) -> crate::common::Reg<ChCurrPtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
}
#[doc = "Datawire Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dw0 {
    ptr: *mut u8,
}
unsafe impl Send for Dw0 {}
unsafe impl Sync for Dw0 {}
impl Dw0 {
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
    #[doc = "Status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Pending channels"]
    #[inline(always)]
    pub const fn pending(self) -> crate::common::Reg<Pending, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "System interrupt control"]
    #[inline(always)]
    pub const fn status_intr(self) -> crate::common::Reg<StatusIntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Status of interrupts masked"]
    #[inline(always)]
    pub const fn status_intr_masked(
        self,
    ) -> crate::common::Reg<StatusIntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Active descriptor control"]
    #[inline(always)]
    pub const fn act_descr_ctl(self) -> crate::common::Reg<ActDescrCtl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Active descriptor source"]
    #[inline(always)]
    pub const fn act_descr_src(self) -> crate::common::Reg<ActDescrSrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Active descriptor destination"]
    #[inline(always)]
    pub const fn act_descr_dst(self) -> crate::common::Reg<ActDescrDst, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Active descriptor X loop control"]
    #[inline(always)]
    pub const fn act_descr_x_ctl(self) -> crate::common::Reg<ActDescrXCtl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Active descriptor Y loop control"]
    #[inline(always)]
    pub const fn act_descr_y_ctl(self) -> crate::common::Reg<ActDescrYCtl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Active descriptor next pointer"]
    #[inline(always)]
    pub const fn act_descr_next_ptr(self) -> crate::common::Reg<ActDescrNextPtr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Active source"]
    #[inline(always)]
    pub const fn act_src(self) -> crate::common::Reg<ActSrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Active destination"]
    #[inline(always)]
    pub const fn act_dst(self) -> crate::common::Reg<ActDst, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "DW channel structure"]
    #[inline(always)]
    pub const fn ch_struct(self, n: usize) -> ChStruct {
        assert!(n < 16usize);
        unsafe { ChStruct::from_ptr(self.ptr.add(2048usize + n * 32usize) as _) }
    }
}
#[doc = "Active descriptor control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActDescrCtl(pub u32);
impl ActDescrCtl {
    #[doc = "Copy of DESCR_CTL of the currently active descriptor."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Copy of DESCR_CTL of the currently active descriptor."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActDescrCtl {
    #[inline(always)]
    fn default() -> ActDescrCtl {
        ActDescrCtl(0)
    }
}
#[doc = "Active descriptor destination"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActDescrDst(pub u32);
impl ActDescrDst {
    #[doc = "Copy of DESCR_DST of the currently active descriptor."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Copy of DESCR_DST of the currently active descriptor."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActDescrDst {
    #[inline(always)]
    fn default() -> ActDescrDst {
        ActDescrDst(0)
    }
}
#[doc = "Active descriptor next pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActDescrNextPtr(pub u32);
impl ActDescrNextPtr {
    #[doc = "Copy of DESCR_NEXT_PTR of the currently active descriptor."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Copy of DESCR_NEXT_PTR of the currently active descriptor."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for ActDescrNextPtr {
    #[inline(always)]
    fn default() -> ActDescrNextPtr {
        ActDescrNextPtr(0)
    }
}
#[doc = "Active descriptor source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActDescrSrc(pub u32);
impl ActDescrSrc {
    #[doc = "Copy of DESCR_SRC of the currently active descriptor."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Copy of DESCR_SRC of the currently active descriptor."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActDescrSrc {
    #[inline(always)]
    fn default() -> ActDescrSrc {
        ActDescrSrc(0)
    }
}
#[doc = "Active descriptor X loop control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActDescrXCtl(pub u32);
impl ActDescrXCtl {
    #[doc = "Copy of DESCR_X_CTL of the currently active descriptor."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Copy of DESCR_X_CTL of the currently active descriptor."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActDescrXCtl {
    #[inline(always)]
    fn default() -> ActDescrXCtl {
        ActDescrXCtl(0)
    }
}
#[doc = "Active descriptor Y loop control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActDescrYCtl(pub u32);
impl ActDescrYCtl {
    #[doc = "Copy of DESCR_Y_CTL of the currently active descriptor."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Copy of DESCR_Y_CTL of the currently active descriptor."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActDescrYCtl {
    #[inline(always)]
    fn default() -> ActDescrYCtl {
        ActDescrYCtl(0)
    }
}
#[doc = "Active destination"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActDst(pub u32);
impl ActDst {
    #[doc = "Current address of destination location."]
    #[inline(always)]
    pub const fn dst_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current address of destination location."]
    #[inline(always)]
    pub fn set_dst_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActDst {
    #[inline(always)]
    fn default() -> ActDst {
        ActDst(0)
    }
}
#[doc = "Active source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActSrc(pub u32);
impl ActSrc {
    #[doc = "Current address of source location."]
    #[inline(always)]
    pub const fn src_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current address of source location."]
    #[inline(always)]
    pub fn set_src_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActSrc {
    #[inline(always)]
    fn default() -> ActSrc {
        ActSrc(0)
    }
}
#[doc = "Channel control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChCtl(pub u32);
impl ChCtl {
    #[doc = "User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
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
    #[doc = "Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    pub const fn pc(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    pub fn set_pc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
    #[inline(always)]
    pub const fn preemptable(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
    #[inline(always)]
    pub fn set_preemptable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
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
pub struct ChCurrPtr(pub u32);
impl ChCurrPtr {
    #[doc = "Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for ChCurrPtr {
    #[inline(always)]
    fn default() -> ChCurrPtr {
        ChCurrPtr(0)
    }
}
#[doc = "Channel current indices"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChIdx(pub u32);
impl ChIdx {
    #[doc = "Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub const fn x_idx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn set_x_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub const fn y_idx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn set_y_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for ChIdx {
    #[inline(always)]
    fn default() -> ChIdx {
        ChIdx(0)
    }
}
#[doc = "Channel status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChStatus(pub u32);
impl ChStatus {
    #[doc = "Specifies the source of the interrupt cause: '0': NO_INTR '1': COMPLETION '2': SRC_BUS_ERROR '3': DST_BUS_ERROR '4': SRC_MISAL '5': DST_MISAL '6': CURR_PTR_NULL '7': ACTIVE_CH_DISABLED '8': DESCR_BUS_ERROR '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
    #[inline(always)]
    pub const fn intr_cause(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the source of the interrupt cause: '0': NO_INTR '1': COMPLETION '2': SRC_BUS_ERROR '3': DST_BUS_ERROR '4': SRC_MISAL '5': DST_MISAL '6': CURR_PTR_NULL '7': ACTIVE_CH_DISABLED '8': DESCR_BUS_ERROR '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
    #[inline(always)]
    pub fn set_intr_cause(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for ChStatus {
    #[inline(always)]
    fn default() -> ChStatus {
        ChStatus(0)
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
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
#[doc = "Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
    #[inline(always)]
    pub const fn ch(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
    #[inline(always)]
    pub fn set_ch(&mut self, val: bool) {
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
    #[doc = "Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub const fn ch(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn set_ch(&mut self, val: bool) {
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
    pub const fn ch(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn set_ch(&mut self, val: bool) {
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
    #[doc = "Write INTR_SET field with '1' to set corresponding INTR.CH field (a write of '0' has no effect)."]
    #[inline(always)]
    pub const fn ch(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write INTR_SET field with '1' to set corresponding INTR.CH field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn set_ch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Pending channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pending(pub u32);
impl Pending {
    #[doc = "Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
    #[inline(always)]
    pub const fn ch_pending(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
    #[inline(always)]
    pub fn set_ch_pending(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pending {
    #[inline(always)]
    fn default() -> Pending {
        Pending(0)
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Active channel, user/privileged access control: '0': user mode. '1': privileged mode."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Active channel, user/privileged access control: '0': user mode. '1': privileged mode."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Active channel, secure/non-secure access control: '0': secure. '1': non-secure."]
    #[inline(always)]
    pub const fn ns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Active channel, secure/non-secure access control: '0': secure. '1': non-secure."]
    #[inline(always)]
    pub fn set_ns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Active channel, non-bufferable/bufferable access control: '0': non-bufferable '1': bufferable."]
    #[inline(always)]
    pub const fn b(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Active channel, non-bufferable/bufferable access control: '0': non-bufferable '1': bufferable."]
    #[inline(always)]
    pub fn set_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Active channel protection context."]
    #[inline(always)]
    pub const fn pc(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Active channel protection context."]
    #[inline(always)]
    pub fn set_pc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Active channel index."]
    #[inline(always)]
    pub const fn ch_idx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Active channel index."]
    #[inline(always)]
    pub fn set_ch_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Active channel priority."]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Active channel priority."]
    #[inline(always)]
    pub fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Active channel preemptable."]
    #[inline(always)]
    pub const fn preemptable(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Active channel preemptable."]
    #[inline(always)]
    pub fn set_preemptable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "State of the DW controller. '0': Default/inactive state. '1': Loading descriptor. '2': Loading data element from source location. '3': Storing data element to destination location. '4': Update of active control information (e.g. source and destination addresses). '5': Wait for trigger de-activation."]
    #[inline(always)]
    pub const fn state(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "State of the DW controller. '0': Default/inactive state. '1': Loading descriptor. '2': Loading data element from source location. '3': Storing data element to destination location. '4': Update of active control information (e.g. source and destination addresses). '5': Wait for trigger de-activation."]
    #[inline(always)]
    pub fn set_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Active channel present: '0': No. '1': Yes."]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Active channel present: '0': No. '1': Yes."]
    #[inline(always)]
    pub fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "System interrupt control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatusIntr(pub u32);
impl StatusIntr {
    #[doc = "Reflects the INTR.CH bit fields of all channels."]
    #[inline(always)]
    pub const fn ch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Reflects the INTR.CH bit fields of all channels."]
    #[inline(always)]
    pub fn set_ch(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for StatusIntr {
    #[inline(always)]
    fn default() -> StatusIntr {
        StatusIntr(0)
    }
}
#[doc = "Status of interrupts masked"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatusIntrMasked(pub u32);
impl StatusIntrMasked {
    #[doc = "Reflects the INTR_MASKED.CH bit fields of all channels."]
    #[inline(always)]
    pub const fn ch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Reflects the INTR_MASKED.CH bit fields of all channels."]
    #[inline(always)]
    pub fn set_ch(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for StatusIntrMasked {
    #[inline(always)]
    fn default() -> StatusIntrMasked {
        StatusIntrMasked(0)
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
