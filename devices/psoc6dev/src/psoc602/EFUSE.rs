#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "EFUSE MXS40 registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Efuse {
    ptr: *mut u8,
}
unsafe impl Send for Efuse {}
unsafe impl Sync for Efuse {}
impl Efuse {
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
    #[doc = "Command"]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Sequencer Default value"]
    #[inline(always)]
    pub const fn seq_default(self) -> crate::common::Reg<SeqDefault, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Sequencer read control 0"]
    #[inline(always)]
    pub const fn seq_read_ctl_0(self) -> crate::common::Reg<SeqReadCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Sequencer read control 1"]
    #[inline(always)]
    pub const fn seq_read_ctl_1(self) -> crate::common::Reg<SeqReadCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Sequencer read control 2"]
    #[inline(always)]
    pub const fn seq_read_ctl_2(self) -> crate::common::Reg<SeqReadCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Sequencer read control 3"]
    #[inline(always)]
    pub const fn seq_read_ctl_3(self) -> crate::common::Reg<SeqReadCtl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Sequencer read control 4"]
    #[inline(always)]
    pub const fn seq_read_ctl_4(self) -> crate::common::Reg<SeqReadCtl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Sequencer read control 5"]
    #[inline(always)]
    pub const fn seq_read_ctl_5(self) -> crate::common::Reg<SeqReadCtl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Sequencer program control 0"]
    #[inline(always)]
    pub const fn seq_program_ctl_0(self) -> crate::common::Reg<SeqProgramCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Sequencer program control 1"]
    #[inline(always)]
    pub const fn seq_program_ctl_1(self) -> crate::common::Reg<SeqProgramCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Sequencer program control 2"]
    #[inline(always)]
    pub const fn seq_program_ctl_2(self) -> crate::common::Reg<SeqProgramCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "Sequencer program control 3"]
    #[inline(always)]
    pub const fn seq_program_ctl_3(self) -> crate::common::Reg<SeqProgramCtl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "Sequencer program control 4"]
    #[inline(always)]
    pub const fn seq_program_ctl_4(self) -> crate::common::Reg<SeqProgramCtl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "Sequencer program control 5"]
    #[inline(always)]
    pub const fn seq_program_ctl_5(self) -> crate::common::Reg<SeqProgramCtl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
}
#[doc = "Command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    pub const fn bit_data(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    pub fn set_bit_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    pub const fn bit_addr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    pub fn set_bit_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
    #[inline(always)]
    pub const fn byte_addr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
    #[inline(always)]
    pub fn set_byte_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Macro address. This field specifies an eFUSE macro."]
    #[inline(always)]
    pub const fn macro_addr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Macro address. This field specifies an eFUSE macro."]
    #[inline(always)]
    pub fn set_macro_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
    #[doc = "IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
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
#[doc = "Sequencer Default value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqDefault(pub u32);
impl SeqDefault {
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for SeqDefault {
    #[inline(always)]
    fn default() -> SeqDefault {
        SeqDefault(0)
    }
}
#[doc = "Sequencer program control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqProgramCtl0(pub u32);
impl SeqProgramCtl0 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqProgramCtl0 {
    #[inline(always)]
    fn default() -> SeqProgramCtl0 {
        SeqProgramCtl0(0)
    }
}
#[doc = "Sequencer program control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqProgramCtl1(pub u32);
impl SeqProgramCtl1 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqProgramCtl1 {
    #[inline(always)]
    fn default() -> SeqProgramCtl1 {
        SeqProgramCtl1(0)
    }
}
#[doc = "Sequencer program control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqProgramCtl2(pub u32);
impl SeqProgramCtl2 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqProgramCtl2 {
    #[inline(always)]
    fn default() -> SeqProgramCtl2 {
        SeqProgramCtl2(0)
    }
}
#[doc = "Sequencer program control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqProgramCtl3(pub u32);
impl SeqProgramCtl3 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqProgramCtl3 {
    #[inline(always)]
    fn default() -> SeqProgramCtl3 {
        SeqProgramCtl3(0)
    }
}
#[doc = "Sequencer program control 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqProgramCtl4(pub u32);
impl SeqProgramCtl4 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqProgramCtl4 {
    #[inline(always)]
    fn default() -> SeqProgramCtl4 {
        SeqProgramCtl4(0)
    }
}
#[doc = "Sequencer program control 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqProgramCtl5(pub u32);
impl SeqProgramCtl5 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqProgramCtl5 {
    #[inline(always)]
    fn default() -> SeqProgramCtl5 {
        SeqProgramCtl5(0)
    }
}
#[doc = "Sequencer read control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqReadCtl0(pub u32);
impl SeqReadCtl0 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqReadCtl0 {
    #[inline(always)]
    fn default() -> SeqReadCtl0 {
        SeqReadCtl0(0)
    }
}
#[doc = "Sequencer read control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqReadCtl1(pub u32);
impl SeqReadCtl1 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqReadCtl1 {
    #[inline(always)]
    fn default() -> SeqReadCtl1 {
        SeqReadCtl1(0)
    }
}
#[doc = "Sequencer read control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqReadCtl2(pub u32);
impl SeqReadCtl2 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqReadCtl2 {
    #[inline(always)]
    fn default() -> SeqReadCtl2 {
        SeqReadCtl2(0)
    }
}
#[doc = "Sequencer read control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqReadCtl3(pub u32);
impl SeqReadCtl3 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqReadCtl3 {
    #[inline(always)]
    fn default() -> SeqReadCtl3 {
        SeqReadCtl3(0)
    }
}
#[doc = "Sequencer read control 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqReadCtl4(pub u32);
impl SeqReadCtl4 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqReadCtl4 {
    #[inline(always)]
    fn default() -> SeqReadCtl4 {
        SeqReadCtl4(0)
    }
}
#[doc = "Sequencer read control 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqReadCtl5(pub u32);
impl SeqReadCtl5 {
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\] IP clock cycles."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_a(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub const fn strobe_b(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn set_strobe_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub const fn strobe_c(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn set_strobe_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub const fn strobe_d(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn set_strobe_d(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub const fn strobe_e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn set_strobe_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub const fn strobe_f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn set_strobe_f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub const fn strobe_g(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn set_strobe_g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SeqReadCtl5 {
    #[inline(always)]
    fn default() -> SeqReadCtl5 {
        SeqReadCtl5(0)
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
