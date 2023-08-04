#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "CAN Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Canfd0 {
    ptr: *mut u8,
}
unsafe impl Send for Canfd0 {}
unsafe impl Sync for Canfd0 {}
impl Canfd0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FIFO wrapper around M_TTCAN 3PIP, to enable DMA"]
    #[inline(always)]
    pub const fn ch(self) -> Ch {
        unsafe { Ch::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Global CAN control register"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4096usize) as _) }
    }
    #[doc = "Global CAN status register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4100usize) as _) }
    }
    #[doc = "Consolidated interrupt0 cause register"]
    #[inline(always)]
    pub const fn intr0_cause(self) -> crate::common::Reg<Intr0Cause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4112usize) as _) }
    }
    #[doc = "Consolidated interrupt1 cause register"]
    #[inline(always)]
    pub const fn intr1_cause(self) -> crate::common::Reg<Intr1Cause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4116usize) as _) }
    }
    #[doc = "Time Stamp control register"]
    #[inline(always)]
    pub const fn ts_ctl(self) -> crate::common::Reg<TsCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4128usize) as _) }
    }
    #[doc = "Time Stamp counter value"]
    #[inline(always)]
    pub const fn ts_cnt(self) -> crate::common::Reg<TsCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4132usize) as _) }
    }
    #[doc = "ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(self) -> crate::common::Reg<EccCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4224usize) as _) }
    }
    #[doc = "ECC error injection"]
    #[inline(always)]
    pub const fn ecc_err_inj(self) -> crate::common::Reg<EccErrInj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4228usize) as _) }
    }
}
#[doc = "FIFO wrapper around M_TTCAN 3PIP, to enable DMA"]
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
    #[doc = "TTCAN 3PIP, includes FD"]
    #[inline(always)]
    pub const fn m_ttcan(self) -> MTtcan {
        unsafe { MTtcan::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Receive FIFO Top control"]
    #[inline(always)]
    pub const fn rxftop_ctl(self) -> crate::common::Reg<RxftopCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Receive FIFO 0 Top Status"]
    #[inline(always)]
    pub const fn rxftop0_stat(self) -> crate::common::Reg<Rxftop0Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(416usize) as _) }
    }
    #[doc = "Receive FIFO 0 Top Data"]
    #[inline(always)]
    pub const fn rxftop0_data(self) -> crate::common::Reg<Rxftop0Data, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(424usize) as _) }
    }
    #[doc = "Receive FIFO 1 Top Status"]
    #[inline(always)]
    pub const fn rxftop1_stat(self) -> crate::common::Reg<Rxftop1Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(432usize) as _) }
    }
    #[doc = "Receive FIFO 1 Top Data"]
    #[inline(always)]
    pub const fn rxftop1_data(self) -> crate::common::Reg<Rxftop1Data, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(440usize) as _) }
    }
}
#[doc = "TTCAN 3PIP, includes FD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MTtcan {
    ptr: *mut u8,
}
unsafe impl Send for MTtcan {}
unsafe impl Sync for MTtcan {}
impl MTtcan {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Core Release Register"]
    #[inline(always)]
    pub const fn crel(self) -> crate::common::Reg<Crel, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Endian Register"]
    #[inline(always)]
    pub const fn endn(self) -> crate::common::Reg<Endn, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Data Bit Timing & Prescaler Register"]
    #[inline(always)]
    pub const fn dbtp(self) -> crate::common::Reg<Dbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Test Register"]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "RAM Watchdog"]
    #[inline(always)]
    pub const fn rwd(self) -> crate::common::Reg<Rwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "CC Control Register"]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Nominal Bit Timing & Prescaler Register"]
    #[inline(always)]
    pub const fn nbtp(self) -> crate::common::Reg<Nbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Timestamp Counter Configuration"]
    #[inline(always)]
    pub const fn tscc(self) -> crate::common::Reg<Tscc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Timestamp Counter Value"]
    #[inline(always)]
    pub const fn tscv(self) -> crate::common::Reg<Tscv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Timeout Counter Configuration"]
    #[inline(always)]
    pub const fn tocc(self) -> crate::common::Reg<Tocc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Timeout Counter Value"]
    #[inline(always)]
    pub const fn tocv(self) -> crate::common::Reg<Tocv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<Ecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<Psr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Transmitter Delay Compensation Register"]
    #[inline(always)]
    pub const fn tdcr(self) -> crate::common::Reg<Tdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Interrupt Register"]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn ie(self) -> crate::common::Reg<Ie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Interrupt Line Select"]
    #[inline(always)]
    pub const fn ils(self) -> crate::common::Reg<Ils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Interrupt Line Enable"]
    #[inline(always)]
    pub const fn ile(self) -> crate::common::Reg<Ile, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "Global Filter Configuration"]
    #[inline(always)]
    pub const fn gfc(self) -> crate::common::Reg<Gfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Standard ID Filter Configuration"]
    #[inline(always)]
    pub const fn sidfc(self) -> crate::common::Reg<Sidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Extended ID Filter Configuration"]
    #[inline(always)]
    pub const fn xidfc(self) -> crate::common::Reg<Xidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Extended ID AND Mask"]
    #[inline(always)]
    pub const fn xidam(self) -> crate::common::Reg<Xidam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "High Priority Message Status"]
    #[inline(always)]
    pub const fn hpms(self) -> crate::common::Reg<Hpms, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "New Data 1"]
    #[inline(always)]
    pub const fn ndat1(self) -> crate::common::Reg<Ndat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
    #[doc = "New Data 2"]
    #[inline(always)]
    pub const fn ndat2(self) -> crate::common::Reg<Ndat2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(156usize) as _) }
    }
    #[doc = "Rx FIFO 0 Configuration"]
    #[inline(always)]
    pub const fn rxf0c(self) -> crate::common::Reg<Rxf0c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "Rx FIFO 0 Status"]
    #[inline(always)]
    pub const fn rxf0s(self) -> crate::common::Reg<Rxf0s, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "Rx FIFO 0 Acknowledge"]
    #[inline(always)]
    pub const fn rxf0a(self) -> crate::common::Reg<Rxf0a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "Rx Buffer Configuration"]
    #[inline(always)]
    pub const fn rxbc(self) -> crate::common::Reg<Rxbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(172usize) as _) }
    }
    #[doc = "Rx FIFO 1 Configuration"]
    #[inline(always)]
    pub const fn rxf1c(self) -> crate::common::Reg<Rxf1c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "Rx FIFO 1 Status"]
    #[inline(always)]
    pub const fn rxf1s(self) -> crate::common::Reg<Rxf1s, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(180usize) as _) }
    }
    #[doc = "Rx FIFO 1 Acknowledge"]
    #[inline(always)]
    pub const fn rxf1a(self) -> crate::common::Reg<Rxf1a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(184usize) as _) }
    }
    #[doc = "Rx Buffer / FIFO Element Size Configuration"]
    #[inline(always)]
    pub const fn rxesc(self) -> crate::common::Reg<Rxesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(188usize) as _) }
    }
    #[doc = "Tx Buffer Configuration"]
    #[inline(always)]
    pub const fn txbc(self) -> crate::common::Reg<Txbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "Tx FIFO/Queue Status"]
    #[inline(always)]
    pub const fn txfqs(self) -> crate::common::Reg<Txfqs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "Tx Buffer Element Size Configuration"]
    #[inline(always)]
    pub const fn txesc(self) -> crate::common::Reg<Txesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(200usize) as _) }
    }
    #[doc = "Tx Buffer Request Pending"]
    #[inline(always)]
    pub const fn txbrp(self) -> crate::common::Reg<Txbrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(204usize) as _) }
    }
    #[doc = "Tx Buffer Add Request"]
    #[inline(always)]
    pub const fn txbar(self) -> crate::common::Reg<Txbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "Tx Buffer Cancellation Request"]
    #[inline(always)]
    pub const fn txbcr(self) -> crate::common::Reg<Txbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(212usize) as _) }
    }
    #[doc = "Tx Buffer Transmission Occurred"]
    #[inline(always)]
    pub const fn txbto(self) -> crate::common::Reg<Txbto, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(216usize) as _) }
    }
    #[doc = "Tx Buffer Cancellation Finished"]
    #[inline(always)]
    pub const fn txbcf(self) -> crate::common::Reg<Txbcf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(220usize) as _) }
    }
    #[doc = "Tx Buffer Transmission Interrupt Enable"]
    #[inline(always)]
    pub const fn txbtie(self) -> crate::common::Reg<Txbtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn txbcie(self) -> crate::common::Reg<Txbcie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(228usize) as _) }
    }
    #[doc = "Tx Event FIFO Configuration"]
    #[inline(always)]
    pub const fn txefc(self) -> crate::common::Reg<Txefc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "Tx Event FIFO Status"]
    #[inline(always)]
    pub const fn txefs(self) -> crate::common::Reg<Txefs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "Tx Event FIFO Acknowledge"]
    #[inline(always)]
    pub const fn txefa(self) -> crate::common::Reg<Txefa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(248usize) as _) }
    }
    #[doc = "TT Trigger Memory Configuration"]
    #[inline(always)]
    pub const fn tttmc(self) -> crate::common::Reg<Tttmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "TT Reference Message Configuration"]
    #[inline(always)]
    pub const fn ttrmc(self) -> crate::common::Reg<Ttrmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "TT Operation Configuration"]
    #[inline(always)]
    pub const fn ttocf(self) -> crate::common::Reg<Ttocf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "TT Matrix Limits"]
    #[inline(always)]
    pub const fn ttmlm(self) -> crate::common::Reg<Ttmlm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(268usize) as _) }
    }
    #[doc = "TUR Configuration"]
    #[inline(always)]
    pub const fn turcf(self) -> crate::common::Reg<Turcf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(272usize) as _) }
    }
    #[doc = "TT Operation Control"]
    #[inline(always)]
    pub const fn ttocn(self) -> crate::common::Reg<Ttocn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(276usize) as _) }
    }
    #[doc = "TT Global Time Preset"]
    #[inline(always)]
    pub const fn ttgtp(self) -> crate::common::Reg<Ttgtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(280usize) as _) }
    }
    #[doc = "TT Time Mark"]
    #[inline(always)]
    pub const fn tttmk(self) -> crate::common::Reg<Tttmk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(284usize) as _) }
    }
    #[doc = "TT Interrupt Register"]
    #[inline(always)]
    pub const fn ttir(self) -> crate::common::Reg<Ttir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(288usize) as _) }
    }
    #[doc = "TT Interrupt Enable"]
    #[inline(always)]
    pub const fn ttie(self) -> crate::common::Reg<Ttie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(292usize) as _) }
    }
    #[doc = "TT Interrupt Line Select"]
    #[inline(always)]
    pub const fn ttils(self) -> crate::common::Reg<Ttils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(296usize) as _) }
    }
    #[doc = "TT Operation Status"]
    #[inline(always)]
    pub const fn ttost(self) -> crate::common::Reg<Ttost, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(300usize) as _) }
    }
    #[doc = "TUR Numerator Actual"]
    #[inline(always)]
    pub const fn turna(self) -> crate::common::Reg<Turna, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(304usize) as _) }
    }
    #[doc = "TT Local & Global Time"]
    #[inline(always)]
    pub const fn ttlgt(self) -> crate::common::Reg<Ttlgt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(308usize) as _) }
    }
    #[doc = "TT Cycle Time & Count"]
    #[inline(always)]
    pub const fn ttctc(self) -> crate::common::Reg<Ttctc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(312usize) as _) }
    }
    #[doc = "TT Capture Time"]
    #[inline(always)]
    pub const fn ttcpt(self) -> crate::common::Reg<Ttcpt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(316usize) as _) }
    }
    #[doc = "TT Cycle Sync Mark"]
    #[inline(always)]
    pub const fn ttcsm(self) -> crate::common::Reg<Ttcsm, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(320usize) as _) }
    }
}
#[doc = "CC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cccr(pub u32);
impl Cccr {
    #[doc = "Initialization 0= Normal Operation 1= Initialization is started"]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization 0= Normal Operation 1= Initialization is started"]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = '1')"]
    #[inline(always)]
    pub const fn cce(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = '1')"]
    #[inline(always)]
    pub fn set_cce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
    #[inline(always)]
    pub const fn asm(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
    #[inline(always)]
    pub fn set_asm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_TTCAN may be set in power down by stopping m_ttcan_hclk and m_ttcan_cclk"]
    #[inline(always)]
    pub const fn csa(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_TTCAN may be set in power down by stopping m_ttcan_hclk and m_ttcan_cclk"]
    #[inline(always)]
    pub fn set_csa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Stop Request, not supported by M_TTCAN use CTL.STOP_REQ at the group level instead. 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub const fn csr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop Request, not supported by M_TTCAN use CTL.STOP_REQ at the group level instead. 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn set_csr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
    #[inline(always)]
    pub const fn mon_(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
    #[inline(always)]
    pub fn set_mon_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
    #[inline(always)]
    pub const fn dar(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
    #[inline(always)]
    pub fn set_dar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
    #[inline(always)]
    pub const fn fdoe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
    #[inline(always)]
    pub fn set_fdoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled"]
    #[inline(always)]
    pub const fn brse(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled"]
    #[inline(always)]
    pub fn set_brse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled"]
    #[inline(always)]
    pub const fn pxhd(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled"]
    #[inline(always)]
    pub fn set_pxhd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub const fn efbi(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub fn set_efbi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Transmit Pause If this bit is set, the M_TTCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
    #[inline(always)]
    pub const fn txp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Pause If this bit is set, the M_TTCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
    #[inline(always)]
    pub fn set_txp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Non ISO Operation If this bit is set, the M_TTCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 addressing the non-ISO CAN FD"]
    #[inline(always)]
    pub const fn niso(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Non ISO Operation If this bit is set, the M_TTCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 addressing the non-ISO CAN FD"]
    #[inline(always)]
    pub fn set_niso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Cccr {
    #[inline(always)]
    fn default() -> Cccr {
        Cccr(0)
    }
}
#[doc = "Core Release Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crel(pub u32);
impl Crel {
    #[doc = "Time Stamp Day Two digits, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
    #[inline(always)]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Time Stamp Day Two digits, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
    #[inline(always)]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Time Stamp Month Two digits, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
    #[inline(always)]
    pub const fn mon(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Time Stamp Month Two digits, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
    #[inline(always)]
    pub fn set_mon(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Time Stamp Year One digit, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
    #[inline(always)]
    pub const fn year(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Time Stamp Year One digit, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
    #[inline(always)]
    pub fn set_year(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Sub-step of Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub const fn substep(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Sub-step of Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub fn set_substep(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Step of Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub const fn step(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Step of Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub fn set_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Crel {
    #[inline(always)]
    fn default() -> Crel {
        Crel(0)
    }
}
#[doc = "Global CAN control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Clock Stop Request for each TTCAN IP . The m_ttcan_clkstop_req of each TTCAN IP is directly driven by these bits."]
    #[inline(always)]
    pub const fn stop_req(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Stop Request for each TTCAN IP . The m_ttcan_clkstop_req of each TTCAN IP is directly driven by these bits."]
    #[inline(always)]
    pub fn set_stop_req(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "MRAM off 0= Default MRAM on (with MRAM retained in DeepSleep). 1= Switch MRAM off (not retained) to save power. Before setting this bit all the CAN channels have to be powered down using the STOP_REQ/ACK bits. When the MRAM is off any access attempt to it is considered an address error (as if MRAM_SIZE=0). After switching the MRAM on again software needs to allow for a certain power up time before MRAM can be used, i.e. before STOP_REQ can be de-asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register. To meet S8 platform requirements, MRAM_OFF should be set to 0 prior to transitioning to Hibernate mode."]
    #[inline(always)]
    pub const fn mram_off(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "MRAM off 0= Default MRAM on (with MRAM retained in DeepSleep). 1= Switch MRAM off (not retained) to save power. Before setting this bit all the CAN channels have to be powered down using the STOP_REQ/ACK bits. When the MRAM is off any access attempt to it is considered an address error (as if MRAM_SIZE=0). After switching the MRAM on again software needs to allow for a certain power up time before MRAM can be used, i.e. before STOP_REQ can be de-asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register. To meet S8 platform requirements, MRAM_OFF should be set to 0 prior to transitioning to Hibernate mode."]
    #[inline(always)]
    pub fn set_mram_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
#[doc = "Data Bit Timing & Prescaler Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbtp(pub u32);
impl Dbtp {
    #[doc = "Data (Re)Synchronization Jump Width 0x0-0xF Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub const fn dsjw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data (Re)Synchronization Jump Width 0x0-0xF Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn set_dsjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Data time segment after sample point 0x0-0xF Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub const fn dtseg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Data time segment after sample point 0x0-0xF Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn set_dtseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Data time segment before sample point 0x00-0x1F Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub const fn dtseg1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Data time segment before sample point 0x00-0x1F Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn set_dtseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Data Bit Rate Prescaler 0x00-0x1F The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub const fn dbrp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Data Bit Rate Prescaler 0x00-0x1F The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn set_dbrp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Transmitter Delay Compensation 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled"]
    #[inline(always)]
    pub const fn tdc(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Delay Compensation 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled"]
    #[inline(always)]
    pub fn set_tdc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Dbtp {
    #[inline(always)]
    fn default() -> Dbtp {
        Dbtp(0)
    }
}
#[doc = "ECC control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccCtl(pub u32);
impl EccCtl {
    #[doc = "Enable ECC for CANFD SRAM When disabled also all error injection functionality is disabled."]
    #[inline(always)]
    pub const fn ecc_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ECC for CANFD SRAM When disabled also all error injection functionality is disabled."]
    #[inline(always)]
    pub fn set_ecc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for EccCtl {
    #[inline(always)]
    fn default() -> EccCtl {
        EccCtl(0)
    }
}
#[doc = "ECC error injection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccErrInj(pub u32);
impl EccErrInj {
    #[doc = "Specifies the address of the word where an error will be injected on write or an non-correctable error will be suppressed. When the ERR_EN bit is set an error parity (ERR_PAR) is injected when any write, from bus or a CAN channel, is done to this address. When the ERR_EN bit is set and the access address matches ERR_ADDR then a non-correctable ECC error or an Address error will NOT result in a bus error or CAN channel shutdown. Note that error reporting to the fault structure cannot be suppressed."]
    #[inline(always)]
    pub const fn err_addr(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Specifies the address of the word where an error will be injected on write or an non-correctable error will be suppressed. When the ERR_EN bit is set an error parity (ERR_PAR) is injected when any write, from bus or a CAN channel, is done to this address. When the ERR_EN bit is set and the access address matches ERR_ADDR then a non-correctable ECC error or an Address error will NOT result in a bus error or CAN channel shutdown. Note that error reporting to the fault structure cannot be suppressed."]
    #[inline(always)]
    pub fn set_err_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Enable error injection (ECC_EN must be 1). When this bit is set the error parity (ERR_PAR) will be used when an AHB write is done to the ERR_ADDR address. When the error word is read a single or double error will be reported to the fault structure just like for a real ECC error (even if this bit is no longer set). When this bit is set (and ECC_EN=1) a non-correctable error (ECC or address error) for the ERR_ADDR will not be reported back to the CAN channel or AHB bus."]
    #[inline(always)]
    pub const fn err_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable error injection (ECC_EN must be 1). When this bit is set the error parity (ERR_PAR) will be used when an AHB write is done to the ERR_ADDR address. When the error word is read a single or double error will be reported to the fault structure just like for a real ECC error (even if this bit is no longer set). When this bit is set (and ECC_EN=1) a non-correctable error (ECC or address error) for the ERR_ADDR will not be reported back to the CAN channel or AHB bus."]
    #[inline(always)]
    pub fn set_err_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ECC Parity bits to use for ECC error injection at address ERR_ADDR."]
    #[inline(always)]
    pub const fn err_par(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "ECC Parity bits to use for ECC error injection at address ERR_ADDR."]
    #[inline(always)]
    pub fn set_err_par(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for EccErrInj {
    #[inline(always)]
    fn default() -> EccErrInj {
        EccErrInj(0)
    }
}
#[doc = "Error Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc = "Transmit Error Counter Actual state of the Transmit Error Counter, values between 0 and 255"]
    #[inline(always)]
    pub const fn tec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Error Counter Actual state of the Transmit Error Counter, values between 0 and 255"]
    #[inline(always)]
    pub fn set_tec(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Error Counter Actual state of the Receive Error Counter, values between 0 and 127"]
    #[inline(always)]
    pub const fn rec(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Receive Error Counter Actual state of the Receive Error Counter, values between 0 and 127"]
    #[inline(always)]
    pub fn set_rec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Receive Error Passive 0= The Receive Error Counter is below the error passive level of 128 1= The Receive Error Counter has reached the error passive level of 128"]
    #[inline(always)]
    pub const fn rp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Error Passive 0= The Receive Error Counter is below the error passive level of 128 1= The Receive Error Counter has reached the error passive level of 128"]
    #[inline(always)]
    pub fn set_rp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CAN Error Logging The counter is incremented each time when a CAN protocol error causes the Transmit Error Counter or the Receive Error Counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO."]
    #[inline(always)]
    pub const fn cel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "CAN Error Logging The counter is incremented each time when a CAN protocol error causes the Transmit Error Counter or the Receive Error Counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO."]
    #[inline(always)]
    pub fn set_cel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Ecr {
    #[inline(always)]
    fn default() -> Ecr {
        Ecr(0)
    }
}
#[doc = "Endian Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endn(pub u32);
impl Endn {
    #[doc = "Endianness Test Value The endianness test value is 0x87654321."]
    #[inline(always)]
    pub const fn etv(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Endianness Test Value The endianness test value is 0x87654321."]
    #[inline(always)]
    pub fn set_etv(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Endn {
    #[inline(always)]
    fn default() -> Endn {
        Endn(0)
    }
}
#[doc = "Global Filter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gfc(pub u32);
impl Gfc {
    #[doc = "Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs"]
    #[inline(always)]
    pub const fn rrfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs"]
    #[inline(always)]
    pub fn set_rrfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub const fn rrfs(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn set_rrfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub const fn anfe(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub fn set_anfe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub const fn anfs(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub fn set_anfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Gfc {
    #[inline(always)]
    fn default() -> Gfc {
        Gfc(0)
    }
}
#[doc = "High Priority Message Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpms(pub u32);
impl Hpms {
    #[doc = "Buffer Index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\] = '1'."]
    #[inline(always)]
    pub const fn bidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Buffer Index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\] = '1'."]
    #[inline(always)]
    pub fn set_bidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Message Storage Indicator 00= No FIFO selected 01= FIFO message lost 10= Message stored in FIFO 0 11= Message stored in FIFO 1"]
    #[inline(always)]
    pub const fn msi(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Message Storage Indicator 00= No FIFO selected 01= FIFO message lost 10= Message stored in FIFO 0 11= Message stored in FIFO 1"]
    #[inline(always)]
    pub fn set_msi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Filter Index Index of matching filter element. Range is 0 to SIDFC.LSS - 1 resp. XIDFC.LSE - 1."]
    #[inline(always)]
    pub const fn fidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Filter Index Index of matching filter element. Range is 0 to SIDFC.LSS - 1 resp. XIDFC.LSE - 1."]
    #[inline(always)]
    pub fn set_fidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Filter List Indicates the filter list of the matching filter element. 0= Standard Filter List 1= Extended Filter List"]
    #[inline(always)]
    pub const fn flst(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Filter List Indicates the filter list of the matching filter element. 0= Standard Filter List 1= Extended Filter List"]
    #[inline(always)]
    pub fn set_flst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Hpms {
    #[inline(always)]
    fn default() -> Hpms {
        Hpms(0)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ie(pub u32);
impl Ie {
    #[doc = "Rx FIFO 0 New Message Interrupt Enable"]
    #[inline(always)]
    pub const fn rf0ne(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn set_rf0ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub const fn rf0we(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn set_rf0we(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 Full Interrupt Enable"]
    #[inline(always)]
    pub const fn rf0fe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Full Interrupt Enable"]
    #[inline(always)]
    pub fn set_rf0fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub const fn rf0le(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn set_rf0le(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 New Message Interrupt Enable"]
    #[inline(always)]
    pub const fn rf1ne(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn set_rf1ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub const fn rf1we(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn set_rf1we(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 Full Interrupt Enable"]
    #[inline(always)]
    pub const fn rf1fe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Full Interrupt Enable"]
    #[inline(always)]
    pub fn set_rf1fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub const fn rf1le(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn set_rf1le(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High Priority Message Interrupt Enable"]
    #[inline(always)]
    pub const fn hpme(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High Priority Message Interrupt Enable"]
    #[inline(always)]
    pub fn set_hpme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission Completed Interrupt Enable"]
    #[inline(always)]
    pub const fn tce(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Completed Interrupt Enable"]
    #[inline(always)]
    pub fn set_tce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn tcfe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn set_tcfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub const fn tfee(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn set_tfee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx Event FIDO New Entry Interrupt Enable"]
    #[inline(always)]
    pub const fn tefne(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIDO New Entry Interrupt Enable"]
    #[inline(always)]
    pub fn set_tefne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx Event FIFO Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub const fn tefwe(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn set_tefwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx Event FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub const fn teffe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn set_teffe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Event FIFO Event Lost Interrupt Enable"]
    #[inline(always)]
    pub const fn tefle(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Event Lost Interrupt Enable"]
    #[inline(always)]
    pub fn set_tefle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp Wraparound Interrupt Enable"]
    #[inline(always)]
    pub const fn tswe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Wraparound Interrupt Enable"]
    #[inline(always)]
    pub fn set_tswe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM Access Failure Interrupt Enable"]
    #[inline(always)]
    pub const fn mrafe(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM Access Failure Interrupt Enable"]
    #[inline(always)]
    pub fn set_mrafe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout Occurred Interrupt Enable"]
    #[inline(always)]
    pub const fn tooe(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Occurred Interrupt Enable"]
    #[inline(always)]
    pub fn set_tooe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored to Dedicated Rx Buffer Interrupt Enable"]
    #[inline(always)]
    pub const fn drxe(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored to Dedicated Rx Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn set_drxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit Error Corrected Interrupt Enable (not used in M_TTCAN)"]
    #[inline(always)]
    pub const fn bece(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Corrected Interrupt Enable (not used in M_TTCAN)"]
    #[inline(always)]
    pub fn set_bece(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    pub const fn beue(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    pub fn set_beue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error Logging Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn eloe(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error Logging Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn set_eloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error Passive Interrupt Enable"]
    #[inline(always)]
    pub const fn epe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive Interrupt Enable"]
    #[inline(always)]
    pub fn set_epe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning Status Interrupt Enable"]
    #[inline(always)]
    pub const fn ewe(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Status Interrupt Enable"]
    #[inline(always)]
    pub fn set_ewe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status Interrupt Enable"]
    #[inline(always)]
    pub const fn boe(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status Interrupt Enable"]
    #[inline(always)]
    pub fn set_boe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog Interrupt Enable"]
    #[inline(always)]
    pub const fn wdie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn set_wdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub const fn peae(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn set_peae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub const fn pede(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn set_pede(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn arae(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_arae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ie {
    #[inline(always)]
    fn default() -> Ie {
        Ie(0)
    }
}
#[doc = "Interrupt Line Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ile(pub u32);
impl Ile {
    #[doc = "Enable Interrupt Line 0 0= Interrupt line m_ttcan_int0 disabled 1= Interrupt line m_ttcan_int0 enabled"]
    #[inline(always)]
    pub const fn eint0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt Line 0 0= Interrupt line m_ttcan_int0 disabled 1= Interrupt line m_ttcan_int0 enabled"]
    #[inline(always)]
    pub fn set_eint0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Interrupt Line 1 0= Interrupt line m_ttcan_int1 disabled 1= Interrupt line m_ttcan_int1 enabled"]
    #[inline(always)]
    pub const fn eint1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt Line 1 0= Interrupt line m_ttcan_int1 disabled 1= Interrupt line m_ttcan_int1 enabled"]
    #[inline(always)]
    pub fn set_eint1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Ile {
    #[inline(always)]
    fn default() -> Ile {
        Ile(0)
    }
}
#[doc = "Interrupt Line Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ils(pub u32);
impl Ils {
    #[doc = "Rx FIFO 0 New Message Interrupt Line"]
    #[inline(always)]
    pub const fn rf0nl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 New Message Interrupt Line"]
    #[inline(always)]
    pub fn set_rf0nl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub const fn rf0wl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn set_rf0wl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 Full Interrupt Line"]
    #[inline(always)]
    pub const fn rf0fl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Full Interrupt Line"]
    #[inline(always)]
    pub fn set_rf0fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 Message Lost Interrupt Line"]
    #[inline(always)]
    pub const fn rf0ll(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn set_rf0ll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 New Message Interrupt Line"]
    #[inline(always)]
    pub const fn rf1nl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 New Message Interrupt Line"]
    #[inline(always)]
    pub fn set_rf1nl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub const fn rf1wl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn set_rf1wl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 Full Interrupt Line"]
    #[inline(always)]
    pub const fn rf1fl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Full Interrupt Line"]
    #[inline(always)]
    pub fn set_rf1fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 Message Lost Interrupt Line"]
    #[inline(always)]
    pub const fn rf1ll(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn set_rf1ll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High Priority Message Interrupt Line"]
    #[inline(always)]
    pub const fn hpml(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High Priority Message Interrupt Line"]
    #[inline(always)]
    pub fn set_hpml(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission Completed Interrupt Line"]
    #[inline(always)]
    pub const fn tcl(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Completed Interrupt Line"]
    #[inline(always)]
    pub fn set_tcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission Cancellation Finished Interrupt Line"]
    #[inline(always)]
    pub const fn tcfl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Cancellation Finished Interrupt Line"]
    #[inline(always)]
    pub fn set_tcfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO Empty Interrupt Line"]
    #[inline(always)]
    pub const fn tfel(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO Empty Interrupt Line"]
    #[inline(always)]
    pub fn set_tfel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx Event FIFO New Entry Interrupt Line"]
    #[inline(always)]
    pub const fn tefnl(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO New Entry Interrupt Line"]
    #[inline(always)]
    pub fn set_tefnl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx Event FIFO Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub const fn tefwl(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn set_tefwl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx Event FIFO Full Interrupt Line"]
    #[inline(always)]
    pub const fn teffl(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Full Interrupt Line"]
    #[inline(always)]
    pub fn set_teffl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Event FIFO Event Lost Interrupt Line"]
    #[inline(always)]
    pub const fn tefll(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Event Lost Interrupt Line"]
    #[inline(always)]
    pub fn set_tefll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp Wraparound Interrupt Line"]
    #[inline(always)]
    pub const fn tswl(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Wraparound Interrupt Line"]
    #[inline(always)]
    pub fn set_tswl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM Access Failure Interrupt Line"]
    #[inline(always)]
    pub const fn mrafl(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM Access Failure Interrupt Line"]
    #[inline(always)]
    pub fn set_mrafl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout Occurred Interrupt Line"]
    #[inline(always)]
    pub const fn tool(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Occurred Interrupt Line"]
    #[inline(always)]
    pub fn set_tool(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored to Dedicated Rx Buffer Interrupt Line"]
    #[inline(always)]
    pub const fn drxl(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored to Dedicated Rx Buffer Interrupt Line"]
    #[inline(always)]
    pub fn set_drxl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit Error Corrected Interrupt Line (not used in M_TTCAN)"]
    #[inline(always)]
    pub const fn becl(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Corrected Interrupt Line (not used in M_TTCAN)"]
    #[inline(always)]
    pub fn set_becl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit Error Uncorrected Interrupt Line"]
    #[inline(always)]
    pub const fn beul(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Uncorrected Interrupt Line"]
    #[inline(always)]
    pub fn set_beul(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error Logging Overflow Interrupt Line"]
    #[inline(always)]
    pub const fn elol(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error Logging Overflow Interrupt Line"]
    #[inline(always)]
    pub fn set_elol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error Passive Interrupt Line"]
    #[inline(always)]
    pub const fn epl(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive Interrupt Line"]
    #[inline(always)]
    pub fn set_epl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning Status Interrupt Line"]
    #[inline(always)]
    pub const fn ewl(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Status Interrupt Line"]
    #[inline(always)]
    pub fn set_ewl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status Interrupt Line"]
    #[inline(always)]
    pub const fn bol(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status Interrupt Line"]
    #[inline(always)]
    pub fn set_bol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog Interrupt Line"]
    #[inline(always)]
    pub const fn wdil(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn set_wdil(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    pub const fn peal(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    pub fn set_peal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol Error in Data Phase Line"]
    #[inline(always)]
    pub const fn pedl(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Data Phase Line"]
    #[inline(always)]
    pub fn set_pedl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn aral(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_aral(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ils {
    #[inline(always)]
    fn default() -> Ils {
        Ils(0)
    }
}
#[doc = "Consolidated interrupt0 cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr0Cause(pub u32);
impl Intr0Cause {
    #[doc = "Show pending m_ttcan_int0 of each channel"]
    #[inline(always)]
    pub const fn int0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Show pending m_ttcan_int0 of each channel"]
    #[inline(always)]
    pub fn set_int0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Intr0Cause {
    #[inline(always)]
    fn default() -> Intr0Cause {
        Intr0Cause(0)
    }
}
#[doc = "Consolidated interrupt1 cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr1Cause(pub u32);
impl Intr1Cause {
    #[doc = "Show pending m_ttcan_int1 of each channel"]
    #[inline(always)]
    pub const fn int1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Show pending m_ttcan_int1 of each channel"]
    #[inline(always)]
    pub fn set_int1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Intr1Cause {
    #[inline(always)]
    fn default() -> Intr1Cause {
        Intr1Cause(0)
    }
}
#[doc = "Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ir(pub u32);
impl Ir {
    #[doc = "Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0"]
    #[inline(always)]
    pub const fn rf0n(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn set_rf0n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark"]
    #[inline(always)]
    pub const fn rf0w(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark"]
    #[inline(always)]
    pub fn set_rf0w(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    pub const fn rf0f(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    pub fn set_rf0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub const fn rf0l_(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn set_rf0l_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1"]
    #[inline(always)]
    pub const fn rf1n(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn set_rf1n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark"]
    #[inline(always)]
    pub const fn rf1w(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark"]
    #[inline(always)]
    pub fn set_rf1w(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    pub const fn rf1f(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    pub fn set_rf1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub const fn rf1l_(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub fn set_rf1l_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High Priority Message 0= No high priority message received 1= High priority message received"]
    #[inline(always)]
    pub const fn hpm(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High Priority Message 0= No high priority message received 1= High priority message received"]
    #[inline(always)]
    pub fn set_hpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission Completed 0= No transmission completed 1= Transmission completed"]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Completed 0= No transmission completed 1= Transmission completed"]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished"]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished"]
    #[inline(always)]
    pub fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty"]
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty"]
    #[inline(always)]
    pub fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element"]
    #[inline(always)]
    pub const fn tefn(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element"]
    #[inline(always)]
    pub fn set_tefn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark"]
    #[inline(always)]
    pub const fn tefw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark"]
    #[inline(always)]
    pub fn set_tefw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
    #[inline(always)]
    pub const fn teff(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
    #[inline(always)]
    pub fn set_teff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
    #[inline(always)]
    pub const fn tefl_(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
    #[inline(always)]
    pub fn set_tefl_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around"]
    #[inline(always)]
    pub const fn tsw(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn set_tsw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM Access Failure The flag is set, when the Rx Handler - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_TTCAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred"]
    #[inline(always)]
    pub const fn mraf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM Access Failure The flag is set, when the Rx Handler - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_TTCAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred"]
    #[inline(always)]
    pub fn set_mraf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout Occurred 0= No timeout 1= Timeout reached"]
    #[inline(always)]
    pub const fn too(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Occurred 0= No timeout 1= Timeout reached"]
    #[inline(always)]
    pub fn set_too(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into a Rx Buffer"]
    #[inline(always)]
    pub const fn drx(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into a Rx Buffer"]
    #[inline(always)]
    pub fn set_drx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "M_TTCAN reports correctable ECC fault to the generic fault structure, this bit always reads as 0. Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_ttcan_aeim_berr\\[0\\] generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)"]
    #[inline(always)]
    pub const fn bec(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "M_TTCAN reports correctable ECC fault to the generic fault structure, this bit always reads as 0. Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_ttcan_aeim_berr\\[0\\] generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)"]
    #[inline(always)]
    pub fn set_bec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_ttcan_aeim_berr\\[1\\] generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)"]
    #[inline(always)]
    pub const fn beu(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_ttcan_aeim_berr\\[1\\] generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)"]
    #[inline(always)]
    pub fn set_beu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred"]
    #[inline(always)]
    pub const fn elo(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred"]
    #[inline(always)]
    pub fn set_elo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed"]
    #[inline(always)]
    pub const fn ep_(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed"]
    #[inline(always)]
    pub fn set_ep_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed"]
    #[inline(always)]
    pub const fn ew_(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed"]
    #[inline(always)]
    pub fn set_ew_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed"]
    #[inline(always)]
    pub const fn bo_(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed"]
    #[inline(always)]
    pub fn set_bo_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY"]
    #[inline(always)]
    pub const fn wdi(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY"]
    #[inline(always)]
    pub fn set_wdi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC != 0,7)"]
    #[inline(always)]
    pub const fn pea(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC != 0,7)"]
    #[inline(always)]
    pub fn set_pea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC != 0,7)"]
    #[inline(always)]
    pub const fn ped(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC != 0,7)"]
    #[inline(always)]
    pub fn set_ped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ara(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ara(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ir {
    #[inline(always)]
    fn default() -> Ir {
        Ir(0)
    }
}
#[doc = "Nominal Bit Timing & Prescaler Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nbtp(pub u32);
impl Nbtp {
    #[doc = "Nominal Time segment after sample point 0x01-0x7F Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub const fn ntseg2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal Time segment after sample point 0x01-0x7F Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn set_ntseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Nominal Time segment before sample point 0x01-0xFF Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub const fn ntseg1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Nominal Time segment before sample point 0x01-0xFF Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn set_ntseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Nominal Bit Rate Prescaler 0x000-0x1FFThe value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub const fn nbrp(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Nominal Bit Rate Prescaler 0x000-0x1FFThe value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn set_nbrp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Nominal (Re)Synchronization Jump Width 0x00-0x7F Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub const fn nsjw(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal (Re)Synchronization Jump Width 0x00-0x7F Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn set_nsjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Nbtp {
    #[inline(always)]
    fn default() -> Nbtp {
        Nbtp(0)
    }
}
#[doc = "New Data 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat1(pub u32);
impl Ndat1 {
    #[doc = "New Data The register holds the New Data flags of Rx Buffers 0 to 31. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a '1' to the corresponding bit position. Writing a '0' has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
    #[inline(always)]
    pub const fn nd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "New Data The register holds the New Data flags of Rx Buffers 0 to 31. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a '1' to the corresponding bit position. Writing a '0' has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
    #[inline(always)]
    pub fn set_nd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ndat1 {
    #[inline(always)]
    fn default() -> Ndat1 {
        Ndat1(0)
    }
}
#[doc = "New Data 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat2(pub u32);
impl Ndat2 {
    #[doc = "New Data The register holds the New Data flags of Rx Buffers 32 to 63. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a '1' to the corresponding bit position. Writing a '0' has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
    #[inline(always)]
    pub const fn nd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "New Data The register holds the New Data flags of Rx Buffers 32 to 63. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a '1' to the corresponding bit position. Writing a '0' has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message"]
    #[inline(always)]
    pub fn set_nd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ndat2 {
    #[inline(always)]
    fn default() -> Ndat2 {
        Ndat2(0)
    }
}
#[doc = "Protocol Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr(pub u32);
impl Psr {
    #[doc = "Last Error Code, Set on Read0 The LEC indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. 0= No Error: No error occurred since LEC has been reset by successful reception or transmission. 1= Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed. 2= Form Error: A fixed format part of a received frame has the wrong format. 3= AckError: The message transmitted by the M_TTCAN was not acknowledged by another node. 4= Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant. 5= Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0'), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed). 6= CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data. 7= NoChange: Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows the value '7', no CAN bus event was detected since the last CPU read access to the Protocol Status Register."]
    #[inline(always)]
    pub const fn lec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Last Error Code, Set on Read0 The LEC indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. 0= No Error: No error occurred since LEC has been reset by successful reception or transmission. 1= Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed. 2= Form Error: A fixed format part of a received frame has the wrong format. 3= AckError: The message transmitted by the M_TTCAN was not acknowledged by another node. 4= Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant. 5= Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0'), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed). 6= CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data. 7= NoChange: Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows the value '7', no CAN bus event was detected since the last CPU read access to the Protocol Status Register."]
    #[inline(always)]
    pub fn set_lec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Activity Monitors the module's CAN communication state. 00= Synchronizing - node is synchronizing on CAN communication 01= Idle - node is neither receiver nor transmitter 10= Receiver - node is operating as receiver 11= Transmitter - node is operating as transmitter"]
    #[inline(always)]
    pub const fn act(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Activity Monitors the module's CAN communication state. 00= Synchronizing - node is synchronizing on CAN communication 01= Idle - node is neither receiver nor transmitter 10= Receiver - node is operating as receiver 11= Transmitter - node is operating as transmitter"]
    #[inline(always)]
    pub fn set_act(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Error Passive 0= The M_CAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected 1= The M_CAN is in the Error_Passive state"]
    #[inline(always)]
    pub const fn ep(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive 0= The M_CAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected 1= The M_CAN is in the Error_Passive state"]
    #[inline(always)]
    pub fn set_ep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Warning Status 0= Both error counters are below the Error_Warning limit of 96 1= At least one of error counter has reached the Error_Warning limit of 96"]
    #[inline(always)]
    pub const fn ew(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Status 0= Both error counters are below the Error_Warning limit of 96 1= At least one of error counter has reached the Error_Warning limit of 96"]
    #[inline(always)]
    pub fn set_ew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bus_Off Status 0= The M_CAN is not Bus_Off 1= The M_CAN is in Bus_Off state"]
    #[inline(always)]
    pub const fn bo(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status 0= The M_CAN is not Bus_Off 1= The M_CAN is in Bus_Off state"]
    #[inline(always)]
    pub fn set_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Data Phase Last Error Code , Set on Read Type of last error that occurred in the data phase of a CAN FD format frame with its BRS flag set. Coding is the same as for LEC. This field will be cleared to zero when a CAN FD format frame with its BRS flag set has been transferred (reception or transmission) without error."]
    #[inline(always)]
    pub const fn dlec(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Data Phase Last Error Code , Set on Read Type of last error that occurred in the data phase of a CAN FD format frame with its BRS flag set. Coding is the same as for LEC. This field will be cleared to zero when a CAN FD format frame with its BRS flag set has been transferred (reception or transmission) without error."]
    #[inline(always)]
    pub fn set_dlec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "ESI flag of last received CAN FD Message , Reset on Read This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its ESI flag set 1= Last received CAN FD message had its ESI flag set"]
    #[inline(always)]
    pub const fn resi(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "ESI flag of last received CAN FD Message , Reset on Read This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its ESI flag set 1= Last received CAN FD message had its ESI flag set"]
    #[inline(always)]
    pub fn set_resi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "BRS flag of last received CAN FD Message , Reset on Read This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its BRS flag set 1= Last received CAN FD message had its BRS flag set"]
    #[inline(always)]
    pub const fn rbrs(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "BRS flag of last received CAN FD Message , Reset on Read This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its BRS flag set 1= Last received CAN FD message had its BRS flag set"]
    #[inline(always)]
    pub fn set_rbrs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Received a CAN FD Message , Reset on Read This bit is set independent of acceptance filtering. 0= Since this bit was reset by the CPU, no CAN FD message has been received 1= Message in CAN FD format with FDF flag set has been received"]
    #[inline(always)]
    pub const fn rfdf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Received a CAN FD Message , Reset on Read This bit is set independent of acceptance filtering. 0= Since this bit was reset by the CPU, no CAN FD message has been received 1= Message in CAN FD format with FDF flag set has been received"]
    #[inline(always)]
    pub fn set_rfdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Protocol Exception Event , Reset on Read 0= No protocol exception event occurred since last read access 1= Protocol exception event occurred"]
    #[inline(always)]
    pub const fn pxe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Exception Event , Reset on Read 0= No protocol exception event occurred since last read access 1= Protocol exception event occurred"]
    #[inline(always)]
    pub fn set_pxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Transmitter Delay Compensation Value 0x00-0x7F Position of the secondary sample point, defined by the sum of the measured delay from m_can_tx to m_can_rx and TDCR.TDCO. The SSP position is, in the data phase, the number of mtq between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub const fn tdcv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter Delay Compensation Value 0x00-0x7F Position of the secondary sample point, defined by the sum of the measured delay from m_can_tx to m_can_rx and TDCR.TDCO. The SSP position is, in the data phase, the number of mtq between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn set_tdcv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        Psr(0)
    }
}
#[doc = "RAM Watchdog"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwd(pub u32);
impl Rwd {
    #[doc = "Watchdog Configuration Start value of the Message RAM Watchdog Counter. With the reset value of '00' the counter is disabled."]
    #[inline(always)]
    pub const fn wdc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Watchdog Configuration Start value of the Message RAM Watchdog Counter. With the reset value of '00' the counter is disabled."]
    #[inline(always)]
    pub fn set_wdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Watchdog Value Actual Message RAM Watchdog Counter Value."]
    #[inline(always)]
    pub const fn wdv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Watchdog Value Actual Message RAM Watchdog Counter Value."]
    #[inline(always)]
    pub fn set_wdv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Rwd {
    #[inline(always)]
    fn default() -> Rwd {
        Rwd(0)
    }
}
#[doc = "Rx Buffer Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxbc(pub u32);
impl Rxbc {
    #[doc = "Rx Buffer Start Address Configures the start address of the Rx Buffers section in the Message RAM (32-bit word address). Also used to reference debug messages A,B,C."]
    #[inline(always)]
    pub const fn rbsa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx Buffer Start Address Configures the start address of the Rx Buffers section in the Message RAM (32-bit word address). Also used to reference debug messages A,B,C."]
    #[inline(always)]
    pub fn set_rbsa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
}
impl Default for Rxbc {
    #[inline(always)]
    fn default() -> Rxbc {
        Rxbc(0)
    }
}
#[doc = "Rx Buffer / FIFO Element Size Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxesc(pub u32);
impl Rxesc {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn f0ds(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_f0ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Rx FIFO 1 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub const fn f1ds(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Rx FIFO 1 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub fn set_f1ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Rx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub const fn rbds(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Rx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub fn set_rbds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Rxesc {
    #[inline(always)]
    fn default() -> Rxesc {
        Rxesc(0)
    }
}
#[doc = "Rx FIFO 0 Acknowledge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf0a(pub u32);
impl Rxf0a {
    #[doc = "Rx FIFO 0 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI + 1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
    #[inline(always)]
    pub const fn f0ai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI + 1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
    #[inline(always)]
    pub fn set_f0ai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Rxf0a {
    #[inline(always)]
    fn default() -> Rxf0a {
        Rxf0a(0)
    }
}
#[doc = "Rx FIFO 0 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf0c(pub u32);
impl Rxf0c {
    #[doc = "Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub const fn f0sa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn set_f0sa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
    #[inline(always)]
    pub const fn f0s(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1"]
    #[inline(always)]
    pub fn set_f0s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) 64= Watermark interrupt disabled"]
    #[inline(always)]
    pub const fn f0wm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) 64= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn set_f0wm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
    #[inline(always)]
    pub const fn f0om(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode"]
    #[inline(always)]
    pub fn set_f0om(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rxf0c {
    #[inline(always)]
    fn default() -> Rxf0c {
        Rxf0c(0)
    }
}
#[doc = "Rx FIFO 0 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf0s(pub u32);
impl Rxf0s {
    #[doc = "Rx FIFO 0 Fill Level Number of elements stored in Rx FIFO 0, range 0 to 64."]
    #[inline(always)]
    pub const fn f0fl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Fill Level Number of elements stored in Rx FIFO 0, range 0 to 64."]
    #[inline(always)]
    pub fn set_f0fl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Rx FIFO 0 Get Index Rx FIFO 0 read index pointer, range 0 to 63. This field is updated by the software writing to RxF0A.F0AI"]
    #[inline(always)]
    pub const fn f0gi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Get Index Rx FIFO 0 read index pointer, range 0 to 63. This field is updated by the software writing to RxF0A.F0AI"]
    #[inline(always)]
    pub fn set_f0gi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Rx FIFO 0 Put Index Rx FIFO 0 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub const fn f0pi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Put Index Rx FIFO 0 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn set_f0pi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    pub const fn f0f(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    pub fn set_f0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 0 Message Lost This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub const fn rf0l(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Message Lost This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn set_rf0l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Rxf0s {
    #[inline(always)]
    fn default() -> Rxf0s {
        Rxf0s(0)
    }
}
#[doc = "Rx FIFO 1 Acknowledge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf1a(pub u32);
impl Rxf1a {
    #[doc = "Rx FIFO 1 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI + 1 and update the FIFO 1 Fill Level RXF1S.F1FL."]
    #[inline(always)]
    pub const fn f1ai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI + 1 and update the FIFO 1 Fill Level RXF1S.F1FL."]
    #[inline(always)]
    pub fn set_f1ai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Rxf1a {
    #[inline(always)]
    fn default() -> Rxf1a {
        Rxf1a(0)
    }
}
#[doc = "Rx FIFO 1 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf1c(pub u32);
impl Rxf1c {
    #[doc = "Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub const fn f1sa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn set_f1sa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Rx FIFO 1 Size 0= No Rx FIFO 1 1-64= Number of Rx FIFO 1 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 1 elements are indexed from 0 to F1S - 1"]
    #[inline(always)]
    pub const fn f1s(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Size 0= No Rx FIFO 1 1-64= Number of Rx FIFO 1 elements 64= Values greater than 64 are interpreted as 64 The Rx FIFO 1 elements are indexed from 0 to F1S - 1"]
    #[inline(always)]
    pub fn set_f1s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Rx FIFO 1 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 1 watermark interrupt (IR.RF1W) 64= Watermark interrupt disabled"]
    #[inline(always)]
    pub const fn f1wm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 1 watermark interrupt (IR.RF1W) 64= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn set_f1wm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "FIFO 1 Operation Mode FIFO 1 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 1 blocking mode 1= FIFO 1 overwrite mode"]
    #[inline(always)]
    pub const fn f1om(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 1 Operation Mode FIFO 1 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 1 blocking mode 1= FIFO 1 overwrite mode"]
    #[inline(always)]
    pub fn set_f1om(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rxf1c {
    #[inline(always)]
    fn default() -> Rxf1c {
        Rxf1c(0)
    }
}
#[doc = "Rx FIFO 1 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf1s(pub u32);
impl Rxf1s {
    #[doc = "Rx FIFO 1 Fill Level Number of elements stored in Rx FIFO 1, range 0 to 64."]
    #[inline(always)]
    pub const fn f1fl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Fill Level Number of elements stored in Rx FIFO 1, range 0 to 64."]
    #[inline(always)]
    pub fn set_f1fl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Rx FIFO 1 Get Index Rx FIFO 1 read index pointer, range 0 to 63. This field is updated by the software writing to RxF1A.FAI"]
    #[inline(always)]
    pub const fn f1gi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Get Index Rx FIFO 1 read index pointer, range 0 to 63. This field is updated by the software writing to RxF1A.FAI"]
    #[inline(always)]
    pub fn set_f1gi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Rx FIFO 1 Put Index Rx FIFO 1 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub const fn f1pi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Put Index Rx FIFO 1 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn set_f1pi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    pub const fn f1f(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    pub fn set_f1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 1 Message Lost This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub const fn rf1l(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Message Lost This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub fn set_rf1l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Debug Message Status 00= Idle state, wait for reception of debug messages, DMA request is cleared 01= Debug message A received 10= Debug messages A, B received 11= Debug messages A, B, C received, DMA request is set"]
    #[inline(always)]
    pub const fn dms(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Debug Message Status 00= Idle state, wait for reception of debug messages, DMA request is cleared 01= Debug message A received 10= Debug messages A, B received 11= Debug messages A, B, C received, DMA request is set"]
    #[inline(always)]
    pub fn set_dms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Rxf1s {
    #[inline(always)]
    fn default() -> Rxf1s {
        Rxf1s(0)
    }
}
#[doc = "Receive FIFO 0 Top Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxftop0Data(pub u32);
impl Rxftop0Data {
    #[doc = "When enabled (F0TPE=1) read data from MRAM at location FnTA. This register can have a read side effect if the following conditions are met: - M_TTCAN not being reconfigured (CCCR.CCE=0) - FIFO Top Pointer logic is enabled (FnTPE=1) - FIFO is not empty (FnFL!=0) The read side effect is as follows: - if FnMWC pointed to the last word of the message (as indicated by FnDS) then the corresponding message index (FnGI) is automatically acknowledge by a write to FnAI - FnMWC is incremented (or restarted if FnMWC pointed to the last word of the message) - the FIFO top address FnTA is incremented (with FIFO wrap around) When this logic is disabled (F0TPE=0) a Read from this register returns undefined data."]
    #[inline(always)]
    pub const fn f0td(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When enabled (F0TPE=1) read data from MRAM at location FnTA. This register can have a read side effect if the following conditions are met: - M_TTCAN not being reconfigured (CCCR.CCE=0) - FIFO Top Pointer logic is enabled (FnTPE=1) - FIFO is not empty (FnFL!=0) The read side effect is as follows: - if FnMWC pointed to the last word of the message (as indicated by FnDS) then the corresponding message index (FnGI) is automatically acknowledge by a write to FnAI - FnMWC is incremented (or restarted if FnMWC pointed to the last word of the message) - the FIFO top address FnTA is incremented (with FIFO wrap around) When this logic is disabled (F0TPE=0) a Read from this register returns undefined data."]
    #[inline(always)]
    pub fn set_f0td(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxftop0Data {
    #[inline(always)]
    fn default() -> Rxftop0Data {
        Rxftop0Data(0)
    }
}
#[doc = "Receive FIFO 0 Top Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxftop0Stat(pub u32);
impl Rxftop0Stat {
    #[doc = "Current FIFO 0 Top Address. This is a pointer to the next word in the message buffer defined by the FIFO Start Address (FnSA), Get Index (FnGI), the FIFO message size (FnDS) and the message word counter (FnMWC) FnTA = FnSA + FnGI * msg_size\\[FnDS\\] + FnMWC"]
    #[inline(always)]
    pub const fn f0ta(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current FIFO 0 Top Address. This is a pointer to the next word in the message buffer defined by the FIFO Start Address (FnSA), Get Index (FnGI), the FIFO message size (FnDS) and the message word counter (FnMWC) FnTA = FnSA + FnGI * msg_size\\[FnDS\\] + FnMWC"]
    #[inline(always)]
    pub fn set_f0ta(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rxftop0Stat {
    #[inline(always)]
    fn default() -> Rxftop0Stat {
        Rxftop0Stat(0)
    }
}
#[doc = "Receive FIFO 1 Top Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxftop1Data(pub u32);
impl Rxftop1Data {
    #[doc = "See F0TD description"]
    #[inline(always)]
    pub const fn f1td(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "See F0TD description"]
    #[inline(always)]
    pub fn set_f1td(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxftop1Data {
    #[inline(always)]
    fn default() -> Rxftop1Data {
        Rxftop1Data(0)
    }
}
#[doc = "Receive FIFO 1 Top Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxftop1Stat(pub u32);
impl Rxftop1Stat {
    #[doc = "See F0TA description"]
    #[inline(always)]
    pub const fn f1ta(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "See F0TA description"]
    #[inline(always)]
    pub fn set_f1ta(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rxftop1Stat {
    #[inline(always)]
    fn default() -> Rxftop1Stat {
        Rxftop1Stat(0)
    }
}
#[doc = "Receive FIFO Top control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxftopCtl(pub u32);
impl RxftopCtl {
    #[doc = "FIFO 0 Top Pointer Enable. This enables the FIFO top pointer logic to set the FIFO Top Address (FnTA) and message word counter. This logic is also disabled when the IP is being reconfigured (CCCR.CCE=1). When this logic is disabled a Read from RXFTOP0_DATA is undefined."]
    #[inline(always)]
    pub const fn f0tpe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 Top Pointer Enable. This enables the FIFO top pointer logic to set the FIFO Top Address (FnTA) and message word counter. This logic is also disabled when the IP is being reconfigured (CCCR.CCE=1). When this logic is disabled a Read from RXFTOP0_DATA is undefined."]
    #[inline(always)]
    pub fn set_f0tpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO 1 Top Pointer Enable."]
    #[inline(always)]
    pub const fn f1tpe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 1 Top Pointer Enable."]
    #[inline(always)]
    pub fn set_f1tpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RxftopCtl {
    #[inline(always)]
    fn default() -> RxftopCtl {
        RxftopCtl(0)
    }
}
#[doc = "Standard ID Filter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sidfc(pub u32);
impl Sidfc {
    #[doc = "Filter List Standard Start Address Start address of standard Message ID filter list (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub const fn flssa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Filter List Standard Start Address Start address of standard Message ID filter list (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn set_flssa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "List Size Standard 0= No standard Message ID filter 1-128= Number of standard Message ID filter elements 128= Values greater than 128 are interpreted as 128"]
    #[inline(always)]
    pub const fn lss(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "List Size Standard 0= No standard Message ID filter 1-128= Number of standard Message ID filter elements 128= Values greater than 128 are interpreted as 128"]
    #[inline(always)]
    pub fn set_lss(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Sidfc {
    #[inline(always)]
    fn default() -> Sidfc {
        Sidfc(0)
    }
}
#[doc = "Global CAN status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Clock Stop Acknowledge for each TTCAN IP. These bits are directly driven by m_ttcan_clkstop_ack of each TTCAN IP. When this bit is set the corresponding TTCAN IP clocks will be gated off, except HCLK will enabled for each AHB write"]
    #[inline(always)]
    pub const fn stop_ack(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Stop Acknowledge for each TTCAN IP. These bits are directly driven by m_ttcan_clkstop_ack of each TTCAN IP. When this bit is set the corresponding TTCAN IP clocks will be gated off, except HCLK will enabled for each AHB write"]
    #[inline(always)]
    pub fn set_stop_ack(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Transmitter Delay Compensation Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdcr(pub u32);
impl Tdcr {
    #[doc = "Transmitter Delay Compensation Filter Window Length 0x00-0x7F Defines the minimum value for the SSP position, dominant edges on m_ttcan_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq"]
    #[inline(always)]
    pub const fn tdcf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter Delay Compensation Filter Window Length 0x00-0x7F Defines the minimum value for the SSP position, dominant edges on m_ttcan_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq"]
    #[inline(always)]
    pub fn set_tdcf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transmitter Delay Compensation Offset 0x00-0x7F Offset value defining the distance between the measured delay from m_ttcan_tx to m_ttcan_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub const fn tdco(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter Delay Compensation Offset 0x00-0x7F Offset value defining the distance between the measured delay from m_ttcan_tx to m_ttcan_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn set_tdco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for Tdcr {
    #[inline(always)]
    fn default() -> Tdcr {
        Tdcr(0)
    }
}
#[doc = "Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test(pub u32);
impl Test {
    #[doc = "ASC is not supported by M_TTCAN Test ASC Multiplexer Control Controls output pin m_ttcan_ascm in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_ascm controlled by FSE 1= Level at pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub const fn tam(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ASC is not supported by M_TTCAN Test ASC Multiplexer Control Controls output pin m_ttcan_ascm in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_ascm controlled by FSE 1= Level at pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub fn set_tam(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ASC is not supported by M_TTCAN Test ASC Transmit Control Controls output pin m_ttcan_asct in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_asct controlled by FSE 1= Level at pin m_ttcan_asct = '1'"]
    #[inline(always)]
    pub const fn tat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ASC is not supported by M_TTCAN Test ASC Transmit Control Controls output pin m_ttcan_asct in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_asct controlled by FSE 1= Level at pin m_ttcan_asct = '1'"]
    #[inline(always)]
    pub fn set_tat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ASC is not supported by M_TTCAN Check ASC Multiplexer Control Monitors level at output pin m_ttcan_ascm. 0= Output pin m_ttcan_ascm = '0' 1= Output pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub const fn cam(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ASC is not supported by M_TTCAN Check ASC Multiplexer Control Monitors level at output pin m_ttcan_ascm. 0= Output pin m_ttcan_ascm = '0' 1= Output pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub fn set_cam(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ASC is not supported by M_TTCAN Check ASC Transmit Control Monitors level at output pin m_ttcan_asct. 0= Output pin m_ttcan_asct = '0'"]
    #[inline(always)]
    pub const fn cat(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ASC is not supported by M_TTCAN Check ASC Transmit Control Monitors level at output pin m_ttcan_asct. 0= Output pin m_ttcan_asct = '0'"]
    #[inline(always)]
    pub fn set_cat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled (see Section 3.1.9, Test Modes)"]
    #[inline(always)]
    pub const fn lbck(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled (see Section 3.1.9, Test Modes)"]
    #[inline(always)]
    pub fn set_lbck(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Control of Transmit Pin 00 Reset value, m_ttcan_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_ttcan_tx 10 Dominant ('0') level at pin m_ttcan_tx 11 Recessive ('1') at pin m_ttcan_tx"]
    #[inline(always)]
    pub const fn tx(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Control of Transmit Pin 00 Reset value, m_ttcan_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_ttcan_tx 10 Dominant ('0') level at pin m_ttcan_tx 11 Recessive ('1') at pin m_ttcan_tx"]
    #[inline(always)]
    pub fn set_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Receive Pin Monitors the actual value of pin m_ttcan_rx 0= The CAN bus is dominant (m_ttcan_rx = '0') 1= The CAN bus is recessive (m_ttcan_rx = '1')"]
    #[inline(always)]
    pub const fn rx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Pin Monitors the actual value of pin m_ttcan_rx 0= The CAN bus is dominant (m_ttcan_rx = '0') 1= The CAN bus is recessive (m_ttcan_rx = '1')"]
    #[inline(always)]
    pub fn set_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Test {
    #[inline(always)]
    fn default() -> Test {
        Test(0)
    }
}
#[doc = "Timeout Counter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tocc(pub u32);
impl Tocc {
    #[doc = "Enable Timeout Counter 0= Timeout Counter disabled 1= Timeout Counter enabled"]
    #[inline(always)]
    pub const fn etoc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timeout Counter 0= Timeout Counter disabled 1= Timeout Counter enabled"]
    #[inline(always)]
    pub fn set_etoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timeout Select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00= Continuous operation 01= Timeout controlled by Tx Event FIFO 10= Timeout controlled by Rx FIFO 0 11= Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub const fn tos(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Timeout Select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00= Continuous operation 01= Timeout controlled by Tx Event FIFO 10= Timeout controlled by Rx FIFO 0 11= Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn set_tos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Timeout Period Start value of the Timeout Counter (down-counter). Configures the Timeout Period."]
    #[inline(always)]
    pub const fn top(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout Period Start value of the Timeout Counter (down-counter). Configures the Timeout Period."]
    #[inline(always)]
    pub fn set_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tocc {
    #[inline(always)]
    fn default() -> Tocc {
        Tocc(0)
    }
}
#[doc = "Timeout Counter Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tocv(pub u32);
impl Tocv {
    #[doc = "Timeout Counter The Timeout Counter is decremented in multiples of CAN bit times \\[1...16\\] depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
    #[inline(always)]
    pub const fn toc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout Counter The Timeout Counter is decremented in multiples of CAN bit times \\[1...16\\] depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS."]
    #[inline(always)]
    pub fn set_toc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tocv {
    #[inline(always)]
    fn default() -> Tocv {
        Tocv(0)
    }
}
#[doc = "Time Stamp counter value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsCnt(pub u32);
impl TsCnt {
    #[doc = "The counter value of the Time Stamp Counter. When enabled this counter will count Time Stamp clock ticks from the pre-scaler. When written this counter and the pre-scaler will reset to 0 (write data is ignored)."]
    #[inline(always)]
    pub const fn value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The counter value of the Time Stamp Counter. When enabled this counter will count Time Stamp clock ticks from the pre-scaler. When written this counter and the pre-scaler will reset to 0 (write data is ignored)."]
    #[inline(always)]
    pub fn set_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for TsCnt {
    #[inline(always)]
    fn default() -> TsCnt {
        TsCnt(0)
    }
}
#[doc = "Time Stamp control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsCtl(pub u32);
impl TsCtl {
    #[doc = "Time Stamp counter prescale value. When enabled divide the Host clock (HCLK) by PRESCALE+1 to create Time Stamp clock ticks."]
    #[inline(always)]
    pub const fn prescale(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Time Stamp counter prescale value. When enabled divide the Host clock (HCLK) by PRESCALE+1 to create Time Stamp clock ticks."]
    #[inline(always)]
    pub fn set_prescale(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter enable bit 0 = Count disabled. Stop counting up and keep the counter value 1 = Count enabled. Start counting up from the current value"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Counter enable bit 0 = Count disabled. Stop counting up and keep the counter value 1 = Count enabled. Start counting up from the current value"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TsCtl {
    #[inline(always)]
    fn default() -> TsCtl {
        TsCtl(0)
    }
}
#[doc = "Timestamp Counter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscc(pub u32);
impl Tscc {
    #[doc = "Timestamp Select, should always be set to external timestamp counter 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as '00'"]
    #[inline(always)]
    pub const fn tss(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Timestamp Select, should always be set to external timestamp counter 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as '00'"]
    #[inline(always)]
    pub fn set_tss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Timestamp Counter Prescaler (still used for TOCC) 0x0-0xF Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1...16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub const fn tcp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Timestamp Counter Prescaler (still used for TOCC) 0x0-0xF Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1...16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn set_tcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Tscc {
    #[inline(always)]
    fn default() -> Tscc {
        Tscc(0)
    }
}
#[doc = "Timestamp Counter Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscv(pub u32);
impl Tscv {
    #[doc = "Timestamp Counter, not used for M_TTCAN The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx). When TSCC.TSS = '01', the Timestamp Counter is incremented in multiples of CAN bit times \\[1...16\\] depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = '10', TSC reflects the external Timestamp Counter value. A write access has no impact."]
    #[inline(always)]
    pub const fn tsc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timestamp Counter, not used for M_TTCAN The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx). When TSCC.TSS = '01', the Timestamp Counter is incremented in multiples of CAN bit times \\[1...16\\] depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = '10', TSC reflects the external Timestamp Counter value. A write access has no impact."]
    #[inline(always)]
    pub fn set_tsc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tscv {
    #[inline(always)]
    fn default() -> Tscv {
        Tscv(0)
    }
}
#[doc = "TT Capture Time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttcpt(pub u32);
impl Ttcpt {
    #[doc = "Cycle Count Value Cycle count value captured together with SWV. 0x00-3F Captured cycle count value"]
    #[inline(always)]
    pub const fn ccv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Cycle Count Value Cycle count value captured together with SWV. 0x00-3F Captured cycle count value"]
    #[inline(always)]
    pub fn set_ccv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Stop Watch Value On a rising/falling edge (as configured via TTOCN.SWP) at the Stop Watch Trigger pin m_ttcan_swt, when TTOCN.SWS is != '00' and TTIR.SWE is '0', the actual time value as selected by TTOCN.SWS (cycle, local, global) is copied to SWV and TTIR.SWE will be set to '1'. Capturing of the next stop watch value is enabled by resetting TTIR.SWE. 0x0000-FFFF Captured Stop Watch value"]
    #[inline(always)]
    pub const fn swv(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop Watch Value On a rising/falling edge (as configured via TTOCN.SWP) at the Stop Watch Trigger pin m_ttcan_swt, when TTOCN.SWS is != '00' and TTIR.SWE is '0', the actual time value as selected by TTOCN.SWS (cycle, local, global) is copied to SWV and TTIR.SWE will be set to '1'. Capturing of the next stop watch value is enabled by resetting TTIR.SWE. 0x0000-FFFF Captured Stop Watch value"]
    #[inline(always)]
    pub fn set_swv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ttcpt {
    #[inline(always)]
    fn default() -> Ttcpt {
        Ttcpt(0)
    }
}
#[doc = "TT Cycle Sync Mark"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttcsm(pub u32);
impl Ttcsm {
    #[doc = "Cycle Sync Mark The Cycle Sync Mark is measured"]
    #[inline(always)]
    pub const fn csm(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Cycle Sync Mark The Cycle Sync Mark is measured"]
    #[inline(always)]
    pub fn set_csm(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ttcsm {
    #[inline(always)]
    fn default() -> Ttcsm {
        Ttcsm(0)
    }
}
#[doc = "TT Cycle Time & Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttctc(pub u32);
impl Ttctc {
    #[doc = "Cycle Time Non-fractional part of the difference of the node's local time and Ref_Mark (see Section 4.5). 0x0000-FFFF Cycle time value of TTCAN Basic Cycle"]
    #[inline(always)]
    pub const fn ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Cycle Time Non-fractional part of the difference of the node's local time and Ref_Mark (see Section 4.5). 0x0000-FFFF Cycle time value of TTCAN Basic Cycle"]
    #[inline(always)]
    pub fn set_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Cycle Count 0x00-3F Number of actual Basic Cycle in the System Matrix"]
    #[inline(always)]
    pub const fn cc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Cycle Count 0x00-3F Number of actual Basic Cycle in the System Matrix"]
    #[inline(always)]
    pub fn set_cc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Ttctc {
    #[inline(always)]
    fn default() -> Ttctc {
        Ttctc(0)
    }
}
#[doc = "TT Global Time Preset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttgtp(pub u32);
impl Ttgtp {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn tp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_tp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Cycle Time Target Phase CTP is write-protected while TTOCN.ESCN or TTOST.SPL are set (see Section 4.11). 0x0000-FFFF Defines target value of cycle time when a rising edge of m_ttcan_evt is expected"]
    #[inline(always)]
    pub const fn ctp(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Cycle Time Target Phase CTP is write-protected while TTOCN.ESCN or TTOST.SPL are set (see Section 4.11). 0x0000-FFFF Defines target value of cycle time when a rising edge of m_ttcan_evt is expected"]
    #[inline(always)]
    pub fn set_ctp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ttgtp {
    #[inline(always)]
    fn default() -> Ttgtp {
        Ttgtp(0)
    }
}
#[doc = "TT Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttie(pub u32);
impl Ttie {
    #[doc = "Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    pub const fn sbce(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn set_sbce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    pub const fn smce(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn set_smce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    pub const fn csme(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    pub fn set_csme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Start of Gap Interrupt Enable"]
    #[inline(always)]
    pub const fn soge(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Gap Interrupt Enable"]
    #[inline(always)]
    pub fn set_soge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Register Time Mark Interrupt Enable"]
    #[inline(always)]
    pub const fn rtmie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Register Time Mark Interrupt Enable"]
    #[inline(always)]
    pub fn set_rtmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Trigger Time Mark Event Internal Enable"]
    #[inline(always)]
    pub const fn ttmie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Time Mark Event Internal Enable"]
    #[inline(always)]
    pub fn set_ttmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub const fn swee(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub fn set_swee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    pub const fn gtwe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    pub fn set_gtwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    pub const fn gtde(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    pub fn set_gtde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Global Time Error Interrupt Enable"]
    #[inline(always)]
    pub const fn gtee(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Global Time Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_gtee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn txue(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn set_txue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn txoe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn set_txoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn se1e(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn set_se1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn se2e(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn set_se2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Change Error Level Interrupt Enable"]
    #[inline(always)]
    pub const fn elce(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Change Error Level Interrupt Enable"]
    #[inline(always)]
    pub fn set_elce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub const fn iwte(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn set_iwte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub const fn wte(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn set_wte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Application Watchdog Interrupt Enable"]
    #[inline(always)]
    pub const fn awe_(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Application Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn set_awe_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub const fn cere(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_cere(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Ttie {
    #[inline(always)]
    fn default() -> Ttie {
        Ttie(0)
    }
}
#[doc = "TT Interrupt Line Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttils(pub u32);
impl Ttils {
    #[doc = "Start of Basic Cycle Interrupt Line"]
    #[inline(always)]
    pub const fn sbcl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Basic Cycle Interrupt Line"]
    #[inline(always)]
    pub fn set_sbcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start of Matrix Cycle Interrupt Line"]
    #[inline(always)]
    pub const fn smcl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Matrix Cycle Interrupt Line"]
    #[inline(always)]
    pub fn set_smcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Change of Synchronization Mode Interrupt Line"]
    #[inline(always)]
    pub const fn csml(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Change of Synchronization Mode Interrupt Line"]
    #[inline(always)]
    pub fn set_csml(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Start of Gap Interrupt Line"]
    #[inline(always)]
    pub const fn sogl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Gap Interrupt Line"]
    #[inline(always)]
    pub fn set_sogl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Register Time Mark Interrupt Line"]
    #[inline(always)]
    pub const fn rtmil(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Register Time Mark Interrupt Line"]
    #[inline(always)]
    pub fn set_rtmil(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Trigger Time Mark Event Internal Line"]
    #[inline(always)]
    pub const fn ttmil(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Time Mark Event Internal Line"]
    #[inline(always)]
    pub fn set_ttmil(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Stop Watch Event Interrupt Line"]
    #[inline(always)]
    pub const fn swel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Watch Event Interrupt Line"]
    #[inline(always)]
    pub fn set_swel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Time Wrap Interrupt Line"]
    #[inline(always)]
    pub const fn gtwl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global Time Wrap Interrupt Line"]
    #[inline(always)]
    pub fn set_gtwl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Global Time Discontinuity Interrupt Line"]
    #[inline(always)]
    pub const fn gtdl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Global Time Discontinuity Interrupt Line"]
    #[inline(always)]
    pub fn set_gtdl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Global Time Error Interrupt Line"]
    #[inline(always)]
    pub const fn gtel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Global Time Error Interrupt Line"]
    #[inline(always)]
    pub fn set_gtel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Tx Count Underflow Interrupt Line"]
    #[inline(always)]
    pub const fn txul(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Count Underflow Interrupt Line"]
    #[inline(always)]
    pub fn set_txul(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx Count Overflow Interrupt Line"]
    #[inline(always)]
    pub const fn txol(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Count Overflow Interrupt Line"]
    #[inline(always)]
    pub fn set_txol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Scheduling Error 1 Interrupt Line"]
    #[inline(always)]
    pub const fn se1l(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Error 1 Interrupt Line"]
    #[inline(always)]
    pub fn set_se1l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Scheduling Error 2 Interrupt Line"]
    #[inline(always)]
    pub const fn se2l(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Error 2 Interrupt Line"]
    #[inline(always)]
    pub fn set_se2l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Change Error Level Interrupt Line"]
    #[inline(always)]
    pub const fn elcl(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Change Error Level Interrupt Line"]
    #[inline(always)]
    pub fn set_elcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Initialization Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub const fn iwtl(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn set_iwtl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub const fn wtl(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn set_wtl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Application Watchdog Interrupt Line"]
    #[inline(always)]
    pub const fn awl_(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Application Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn set_awl_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Configuration Error Interrupt Line"]
    #[inline(always)]
    pub const fn cerl(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration Error Interrupt Line"]
    #[inline(always)]
    pub fn set_cerl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Ttils {
    #[inline(always)]
    fn default() -> Ttils {
        Ttils(0)
    }
}
#[doc = "TT Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttir(pub u32);
impl Ttir {
    #[doc = "Start of Basic Cycle 0= No Basic Cycle started since bit has been reset 1= Basic Cycle started"]
    #[inline(always)]
    pub const fn sbc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Basic Cycle 0= No Basic Cycle started since bit has been reset 1= Basic Cycle started"]
    #[inline(always)]
    pub fn set_sbc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start of Matrix Cycle 0= No Matrix Cycle started since bit has been reset 1= Matrix Cycle started"]
    #[inline(always)]
    pub const fn smc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Matrix Cycle 0= No Matrix Cycle started since bit has been reset 1= Matrix Cycle started"]
    #[inline(always)]
    pub fn set_smc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Change of Synchronization Mode 0= No change in master to slave relation or schedule synchronization 1= Master to slave relation or schedule synchronization changed, also set when TTOST.SPL is reset"]
    #[inline(always)]
    pub const fn csm_(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Change of Synchronization Mode 0= No change in master to slave relation or schedule synchronization 1= Master to slave relation or schedule synchronization changed, also set when TTOST.SPL is reset"]
    #[inline(always)]
    pub fn set_csm_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Start of Gap 0= No reference message seen with Next_is_Gap bit set 1= Reference message with Next_is_Gap bit set becomes valid"]
    #[inline(always)]
    pub const fn sog(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Gap 0= No reference message seen with Next_is_Gap bit set 1= Reference message with Next_is_Gap bit set becomes valid"]
    #[inline(always)]
    pub fn set_sog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Register Time Mark Interrupt Set when time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Time mark not reached 1= Time mark reached"]
    #[inline(always)]
    pub const fn rtmi(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Register Time Mark Interrupt Set when time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Time mark not reached 1= Time mark reached"]
    #[inline(always)]
    pub fn set_rtmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Trigger Time Mark Event Internal Internal time mark events are configured by trigger memory element TMIN (see Section 2.4.7). Set when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Gap or In_Schedule. 0= Time mark not reached 1= Time mark reached (Level 0: cycle time TTOCF.IRTO * 0x200)"]
    #[inline(always)]
    pub const fn ttmi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Time Mark Event Internal Internal time mark events are configured by trigger memory element TMIN (see Section 2.4.7). Set when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Gap or In_Schedule. 0= Time mark not reached 1= Time mark reached (Level 0: cycle time TTOCF.IRTO * 0x200)"]
    #[inline(always)]
    pub fn set_ttmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Stop Watch Event 0= No rising/falling edge at stop watch trigger pin m_ttcan_swt detected 1= Rising/falling edge at stop watch trigger pin m_ttcan_swt detected"]
    #[inline(always)]
    pub const fn swe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Watch Event 0= No rising/falling edge at stop watch trigger pin m_ttcan_swt detected 1= Rising/falling edge at stop watch trigger pin m_ttcan_swt detected"]
    #[inline(always)]
    pub fn set_swe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Time Wrap 0= No global time wrap occurred 1= Global time wrap from 0xFFFF to 0x0000 occurred"]
    #[inline(always)]
    pub const fn gtw(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global Time Wrap 0= No global time wrap occurred 1= Global time wrap from 0xFFFF to 0x0000 occurred"]
    #[inline(always)]
    pub fn set_gtw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Global Time Discontinuity 0= No discontinuity of global time 1= Discontinuity of global time"]
    #[inline(always)]
    pub const fn gtd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Global Time Discontinuity 0= No discontinuity of global time 1= Discontinuity of global time"]
    #[inline(always)]
    pub fn set_gtd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Global Time Error Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL, TTCAN Level 0,2 only. 0= Synchronization deviation within limit 1= Synchronization deviation exceeded limit"]
    #[inline(always)]
    pub const fn gte(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Global Time Error Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL, TTCAN Level 0,2 only. 0= Synchronization deviation within limit 1= Synchronization deviation exceeded limit"]
    #[inline(always)]
    pub fn set_gte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Tx Count Underflow 0= Number of Tx Trigger as expected 1= Less Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub const fn txu(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Count Underflow 0= Number of Tx Trigger as expected 1= Less Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub fn set_txu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx Count Overflow 0= Number of Tx Trigger as expected 1= More Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub const fn txo(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Count Overflow 0= Number of Tx Trigger as expected 1= More Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub fn set_txo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Scheduling Error 1 0= No scheduling error 1 1= Scheduling error 1 occurred"]
    #[inline(always)]
    pub const fn se1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Error 1 0= No scheduling error 1 1= Scheduling error 1 occurred"]
    #[inline(always)]
    pub fn set_se1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Scheduling Error 2 0= No scheduling error 2 1= Scheduling error 2 occurred"]
    #[inline(always)]
    pub const fn se2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Error 2 0= No scheduling error 2 1= Scheduling error 2 occurred"]
    #[inline(always)]
    pub fn set_se2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error Level Changed Not set when error level changed during initialization. 0= No change in error level 1= Error level changed"]
    #[inline(always)]
    pub const fn elc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Error Level Changed Not set when error level changed during initialization. 0= No change in error level 1= Error level changed"]
    #[inline(always)]
    pub fn set_elc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Initialization Watch Trigger The initialization is restarted by resetting IWT. 0= No missing reference message during system startup 1= No system startup due to missing reference message"]
    #[inline(always)]
    pub const fn iwt(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization Watch Trigger The initialization is restarted by resetting IWT. 0= No missing reference message during system startup 1= No system startup due to missing reference message"]
    #[inline(always)]
    pub fn set_iwt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Watch Trigger 0= No missing reference message 1= Missing reference message (Level 0: cycle time 0xFF00)"]
    #[inline(always)]
    pub const fn wt(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Watch Trigger 0= No missing reference message 1= Missing reference message (Level 0: cycle time 0xFF00)"]
    #[inline(always)]
    pub fn set_wt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Application Watchdog 0= Application watchdog served in time 1= Application watchdog not served in time"]
    #[inline(always)]
    pub const fn aw(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Application Watchdog 0= Application watchdog served in time 1= Application watchdog not served in time"]
    #[inline(always)]
    pub fn set_aw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Configuration Error Trigger out of order. 0= No error found in trigger list 1= Error found in trigger list"]
    #[inline(always)]
    pub const fn cer(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration Error Trigger out of order. 0= No error found in trigger list 1= Error found in trigger list"]
    #[inline(always)]
    pub fn set_cer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Ttir {
    #[inline(always)]
    fn default() -> Ttir {
        Ttir(0)
    }
}
#[doc = "TT Local & Global Time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttlgt(pub u32);
impl Ttlgt {
    #[doc = "Local Time Non-fractional part of local time, incremented once each local NTU (see Section 4.5). 0x0000-FFFF Local time value of TTCAN node"]
    #[inline(always)]
    pub const fn lt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Local Time Non-fractional part of local time, incremented once each local NTU (see Section 4.5). 0x0000-FFFF Local time value of TTCAN node"]
    #[inline(always)]
    pub fn set_lt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Time Non-fractional part of the sum of the node's local time and its local offset (see Section 4.5). 0x0000-FFFF Global time value of TTCAN network"]
    #[inline(always)]
    pub const fn gt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Time Non-fractional part of the sum of the node's local time and its local offset (see Section 4.5). 0x0000-FFFF Global time value of TTCAN network"]
    #[inline(always)]
    pub fn set_gt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ttlgt {
    #[inline(always)]
    fn default() -> Ttlgt {
        Ttlgt(0)
    }
}
#[doc = "TT Matrix Limits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttmlm(pub u32);
impl Ttmlm {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ccm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ccm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn css(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_css(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Tx Enable Window 0x0-F Length of Tx enable window, 1-16 NTU cycles"]
    #[inline(always)]
    pub const fn txew(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Tx Enable Window 0x0-F Length of Tx enable window, 1-16 NTU cycles"]
    #[inline(always)]
    pub fn set_txew(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Expected Number of Tx Triggers 0x000-FFF Expected number of Tx Triggers in one Matrix Cycle"]
    #[inline(always)]
    pub const fn entt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Expected Number of Tx Triggers 0x000-FFF Expected number of Tx Triggers in one Matrix Cycle"]
    #[inline(always)]
    pub fn set_entt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Ttmlm {
    #[inline(always)]
    fn default() -> Ttmlm {
        Ttmlm(0)
    }
}
#[doc = "TT Operation Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttocf(pub u32);
impl Ttocf {
    #[doc = "Operation Mode 00= Event-driven CAN communication, default 01= TTCAN level 1 10= TTCAN level 2 11= TTCAN level 0"]
    #[inline(always)]
    pub const fn om(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Operation Mode 00= Event-driven CAN communication, default 01= TTCAN level 1 10= TTCAN level 2 11= TTCAN level 0"]
    #[inline(always)]
    pub fn set_om(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Gap Enable 0= Strictly time-triggered operation 1= External event-synchronized time-triggered operation"]
    #[inline(always)]
    pub const fn gen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Gap Enable 0= Strictly time-triggered operation 1= External event-synchronized time-triggered operation"]
    #[inline(always)]
    pub fn set_gen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Time Master 0= Time Master function disabled 1= Potential Time Master"]
    #[inline(always)]
    pub const fn tm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Time Master 0= Time Master function disabled 1= Potential Time Master"]
    #[inline(always)]
    pub fn set_tm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LD of Synchronization Deviation Limit The Synchronization Deviation Limit SDL is configured by its dual logarithm LDSDL with SDL = 2(LDSDL + 5). It should not exceed the clock tolerance given by the CAN bit timing configuration. 0x0-7 LD of Synchronization Deviation Limit (SDL <= 32...4096)"]
    #[inline(always)]
    pub const fn ldsdl(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "LD of Synchronization Deviation Limit The Synchronization Deviation Limit SDL is configured by its dual logarithm LDSDL with SDL = 2(LDSDL + 5). It should not exceed the clock tolerance given by the CAN bit timing configuration. 0x0-7 LD of Synchronization Deviation Limit (SDL <= 32...4096)"]
    #[inline(always)]
    pub fn set_ldsdl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Initial Reference Trigger Offset 0x00-7F Positive offset, range from 0 to 127"]
    #[inline(always)]
    pub const fn irto(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Initial Reference Trigger Offset 0x00-7F Positive offset, range from 0 to 127"]
    #[inline(always)]
    pub fn set_irto(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Enable External Clock Synchronization If enabled, TUR configuration (TURCF.NCL only) may be updated during TTCAN operation. 0= External clock synchronization in TTCAN Level 0,2 disabled 1= External clock synchronization in TTCAN Level 0,2 enabled"]
    #[inline(always)]
    pub const fn eecs(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable External Clock Synchronization If enabled, TUR configuration (TURCF.NCL only) may be updated during TTCAN operation. 0= External clock synchronization in TTCAN Level 0,2 disabled 1= External clock synchronization in TTCAN Level 0,2 enabled"]
    #[inline(always)]
    pub fn set_eecs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Application Watchdog Limit The application watchdog can be disabled by programming AWL to 0x00. 0x00-FF Maximum time after which the application has to serve the application watchdog. The application watchdog is incremented once each 256 NTUs."]
    #[inline(always)]
    pub const fn awl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Application Watchdog Limit The application watchdog can be disabled by programming AWL to 0x00. 0x00-FF Maximum time after which the application has to serve the application watchdog. The application watchdog is incremented once each 256 NTUs."]
    #[inline(always)]
    pub fn set_awl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Enable Global Time Filtering 0= Global time filtering in TTCAN Level 0,2 is disabled 1= Global time filtering in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub const fn egtf(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Global Time Filtering 0= Global time filtering in TTCAN Level 0,2 is disabled 1= Global time filtering in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub fn set_egtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable Clock Calibration 0= Automatic clock calibration in TTCAN Level 0,2 is disabled 1= Automatic clock calibration in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub const fn ecc(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Clock Calibration 0= Automatic clock calibration in TTCAN Level 0,2 is disabled 1= Automatic clock calibration in TTCAN Level 0,2 is enabled"]
    #[inline(always)]
    pub fn set_ecc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Event Trigger Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub const fn evtp(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub fn set_evtp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Ttocf {
    #[inline(always)]
    fn default() -> Ttocf {
        Ttocf(0)
    }
}
#[doc = "TT Operation Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttocn(pub u32);
impl Ttocn {
    #[doc = "Set Global time Writing a '1' to SGT sets TTOST.WGDT if the node is the actual Time Master. SGT is reset after one Host clock period. The global time preset takes effect when the node transmits the next reference message with the Master_Ref_Mark modified by the preset value written to TTGTP."]
    #[inline(always)]
    pub const fn sgt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set Global time Writing a '1' to SGT sets TTOST.WGDT if the node is the actual Time Master. SGT is reset after one Host clock period. The global time preset takes effect when the node transmits the next reference message with the Master_Ref_Mark modified by the preset value written to TTGTP."]
    #[inline(always)]
    pub fn set_sgt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "External Clock Synchronization Writing a '1' to ECS sets TTOST.WECS if the node is the actual Time Master. ECS is reset after one Host clock period. The external clock synchronization takes effect at the start of the next basic cycle."]
    #[inline(always)]
    pub const fn ecs(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "External Clock Synchronization Writing a '1' to ECS sets TTOST.WECS if the node is the actual Time Master. ECS is reset after one Host clock period. The external clock synchronization takes effect at the start of the next basic cycle."]
    #[inline(always)]
    pub fn set_ecs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Stop Watch Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub const fn swp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Watch Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub fn set_swp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Stop Watch Source 00= Stop Watch disabled 01= Actual value of cycle time is copied to TTCPT.SWV 10= Actual value of local time is copied to TTCPT.SWV 11= Actual value of global time is copied to TTCPT.SWV"]
    #[inline(always)]
    pub const fn sws(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Stop Watch Source 00= Stop Watch disabled 01= Actual value of cycle time is copied to TTCPT.SWV 10= Actual value of local time is copied to TTCPT.SWV 11= Actual value of global time is copied to TTCPT.SWV"]
    #[inline(always)]
    pub fn set_sws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Register Time Mark Interrupt Pulse Enable Register time mark interrupts are configured by register TTTMK. A register time mark interrupt pulse with the length of one NTU is generated when the time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Register Time Mark Interrupt output m_ttcan_rtp disabled 1= Register Time Mark Interrupt output m_ttcan_rtp enabled"]
    #[inline(always)]
    pub const fn rtie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Register Time Mark Interrupt Pulse Enable Register time mark interrupts are configured by register TTTMK. A register time mark interrupt pulse with the length of one NTU is generated when the time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Register Time Mark Interrupt output m_ttcan_rtp disabled 1= Register Time Mark Interrupt output m_ttcan_rtp enabled"]
    #[inline(always)]
    pub fn set_rtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Register Time Mark Compare 00= No Register Time Mark Interrupt generated 01= Register Time Mark Interrupt if Time Mark = cycle time 10= Register Time Mark Interrupt if Time Mark = local time 11= Register Time Mark Interrupt if Time Mark = global time"]
    #[inline(always)]
    pub const fn tmc(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Register Time Mark Compare 00= No Register Time Mark Interrupt generated 01= Register Time Mark Interrupt if Time Mark = cycle time 10= Register Time Mark Interrupt if Time Mark = local time 11= Register Time Mark Interrupt if Time Mark = global time"]
    #[inline(always)]
    pub fn set_tmc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Trigger Time Mark Interrupt Pulse Enable External time mark events are configured by trigger memory element TMEX (see Section 2.4.7). A trigger time mark interrupt pulse is generated when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Schedule or In_Gap. 0= Trigger Time Mark Interrupt output m_ttcan_tmp disabled 1= Trigger Time Mark Interrupt output m_ttcan_tmp enabled"]
    #[inline(always)]
    pub const fn ttie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Time Mark Interrupt Pulse Enable External time mark events are configured by trigger memory element TMEX (see Section 2.4.7). A trigger time mark interrupt pulse is generated when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Schedule or In_Gap. 0= Trigger Time Mark Interrupt output m_ttcan_tmp disabled 1= Trigger Time Mark Interrupt output m_ttcan_tmp enabled"]
    #[inline(always)]
    pub fn set_ttie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Gap Control Select 0= Gap control independent from m_ttcan_evt 1= Gap control by input pin m_ttcan_evt"]
    #[inline(always)]
    pub const fn gcs(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Gap Control Select 0= Gap control independent from m_ttcan_evt 1= Gap control by input pin m_ttcan_evt"]
    #[inline(always)]
    pub fn set_gcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Finish Gap Set by the CPU, reset by each reference message 0= No reference message requested 1= Application requested start of reference message"]
    #[inline(always)]
    pub const fn fgp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Finish Gap Set by the CPU, reset by each reference message 0= No reference message requested 1= Application requested start of reference message"]
    #[inline(always)]
    pub fn set_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Time Mark Gap 0= Reset by each reference message 1= Next reference message started when Register Time Mark interrupt TTIR.RTMI is activated"]
    #[inline(always)]
    pub const fn tmg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Time Mark Gap 0= Reset by each reference message 1= Next reference message started when Register Time Mark interrupt TTIR.RTMI is activated"]
    #[inline(always)]
    pub fn set_tmg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Next is Gap This bit can only be set when the M_TTCAN is the actual Time Master and when it is configured for external event-synchronized time-triggered operation (TTOCF.GEN = '1') 0= No action, reset by reception of any reference message 1= Transmit next reference message with Next_is_Gap = '1'"]
    #[inline(always)]
    pub const fn nig(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Next is Gap This bit can only be set when the M_TTCAN is the actual Time Master and when it is configured for external event-synchronized time-triggered operation (TTOCF.GEN = '1') 0= No action, reset by reception of any reference message 1= Transmit next reference message with Next_is_Gap = '1'"]
    #[inline(always)]
    pub fn set_nig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External Synchronization Control If enabled the M_TTCAN synchronizes its cycle time phase to an external event signaled by a rising edge at pin m_ttcan_evt (see Section 4.11). 0= External synchronization disabled 1= External synchronization enabled"]
    #[inline(always)]
    pub const fn escn(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External Synchronization Control If enabled the M_TTCAN synchronizes its cycle time phase to an external event signaled by a rising edge at pin m_ttcan_evt (see Section 4.11). 0= External synchronization disabled 1= External synchronization enabled"]
    #[inline(always)]
    pub fn set_escn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TT Operation Control Register Locked Set by a write access to register TTOCN. Reset when the updated configuration has been synchronized into the CAN clock domain. 0= Write access to TTOCN enabled 1= Write access to TTOCN locked"]
    #[inline(always)]
    pub const fn lckc(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TT Operation Control Register Locked Set by a write access to register TTOCN. Reset when the updated configuration has been synchronized into the CAN clock domain. 0= Write access to TTOCN enabled 1= Write access to TTOCN locked"]
    #[inline(always)]
    pub fn set_lckc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ttocn {
    #[inline(always)]
    fn default() -> Ttocn {
        Ttocn(0)
    }
}
#[doc = "TT Operation Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttost(pub u32);
impl Ttost {
    #[doc = "Error Level 00= Severity 0 - No Error 01= Severity 1 - Warning 10= Severity 2 - Error 11= Severity 3 - Severe Error"]
    #[inline(always)]
    pub const fn el(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Error Level 00= Severity 0 - No Error 01= Severity 1 - Warning 10= Severity 2 - Error 11= Severity 3 - Severe Error"]
    #[inline(always)]
    pub fn set_el(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Master State 00= Master_Off, no master properties relevant 01= Operating as Time Slave 10= Operating as Backup Time Master 11= Operating as current Time Master"]
    #[inline(always)]
    pub const fn ms(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Master State 00= Master_Off, no master properties relevant 01= Operating as Time Slave 10= Operating as Backup Time Master 11= Operating as current Time Master"]
    #[inline(always)]
    pub fn set_ms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Synchronization State 00= Out of Synchronization 01= Synchronizing to TTCAN communication 10= Schedule suspended by Gap (In_Gap) 11= Synchronized to schedule (In_Schedule)"]
    #[inline(always)]
    pub const fn sys(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Synchronization State 00= Out of Synchronization 01= Synchronizing to TTCAN communication 10= Schedule suspended by Gap (In_Gap) 11= Synchronized to schedule (In_Schedule)"]
    #[inline(always)]
    pub fn set_sys(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Quality of Global Time Phase Only relevant in TTCAN Level 0 and Level 2, otherwise fixed to '0'. 0= Global time not valid 1= Global time in phase with Time Master"]
    #[inline(always)]
    pub const fn qgtp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Quality of Global Time Phase Only relevant in TTCAN Level 0 and Level 2, otherwise fixed to '0'. 0= Global time not valid 1= Global time in phase with Time Master"]
    #[inline(always)]
    pub fn set_qgtp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Quality of Clock Speed Only relevant in TTCAN Level 0 and Level 2, otherwise fixed to '1'. 0= Local clock speed not synchronized to Time Master clock speed 1= Synchronization Deviation <= SDL"]
    #[inline(always)]
    pub const fn qcs(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Quality of Clock Speed Only relevant in TTCAN Level 0 and Level 2, otherwise fixed to '1'. 0= Local clock speed not synchronized to Time Master clock speed 1= Synchronization Deviation <= SDL"]
    #[inline(always)]
    pub fn set_qcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Reference Trigger Offset The Reference Trigger Offset value is a signed integer with a range from -127 (0x81) to 127 (0x7F). There is no notification when the lower limit of -127 is reached. In case the M_TTCAN becomes Time Master (MS\\[1:0\\] = '11'), the reset of RTO is delayed due to synchronization between Host and CAN clock domain. For time slaves the value configured by TTOCF.IRTO is read. 0x00-FF Actual Reference Trigger offset value"]
    #[inline(always)]
    pub const fn rto(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Reference Trigger Offset The Reference Trigger Offset value is a signed integer with a range from -127 (0x81) to 127 (0x7F). There is no notification when the lower limit of -127 is reached. In case the M_TTCAN becomes Time Master (MS\\[1:0\\] = '11'), the reset of RTO is delayed due to synchronization between Host and CAN clock domain. For time slaves the value configured by TTOCF.IRTO is read. 0x00-FF Actual Reference Trigger offset value"]
    #[inline(always)]
    pub fn set_rto(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Wait for Global Time Discontinuity 0= No global time preset pending 1= Node waits for the global time preset to take effect. The bit is reset when the node has transmitted a reference message with Disc_Bit = '1' or after it received a reference message."]
    #[inline(always)]
    pub const fn wgtd(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Global Time Discontinuity 0= No global time preset pending 1= Node waits for the global time preset to take effect. The bit is reset when the node has transmitted a reference message with Disc_Bit = '1' or after it received a reference message."]
    #[inline(always)]
    pub fn set_wgtd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Gap Finished Indicator Set when the CPU writes TTOCN.FGP, or by a time mark interrupt if TMG = '1', or via input pin m_ttcan_evt if TTOCN.GCS = '1'. Not set by Ref_Trigger_Gap or when Gap is finished by another node sending a reference message. 0= Reset at the end of each reference message 1= Gap finished by M_TTCAN"]
    #[inline(always)]
    pub const fn gfi(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Gap Finished Indicator Set when the CPU writes TTOCN.FGP, or by a time mark interrupt if TMG = '1', or via input pin m_ttcan_evt if TTOCN.GCS = '1'. Not set by Ref_Trigger_Gap or when Gap is finished by another node sending a reference message. 0= Reset at the end of each reference message 1= Gap finished by M_TTCAN"]
    #[inline(always)]
    pub fn set_gfi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Time Master Priority 0x0-7 Priority of actual Time Master"]
    #[inline(always)]
    pub const fn tmp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Time Master Priority 0x0-7 Priority of actual Time Master"]
    #[inline(always)]
    pub fn set_tmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Gap Started Indicator 0= No Gap in schedule, reset by each reference message and for all time slaves 1= Gap time after Basic Cycle has started"]
    #[inline(always)]
    pub const fn gsi(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Gap Started Indicator 0= No Gap in schedule, reset by each reference message and for all time slaves 1= Gap time after Basic Cycle has started"]
    #[inline(always)]
    pub fn set_gsi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Wait for Event 0= No Gap announced, reset by a reference message with Next_is_Gap = '0' 1= Reference message with Next_is_Gap = '1' received"]
    #[inline(always)]
    pub const fn wfe(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Event 0= No Gap announced, reset by a reference message with Next_is_Gap = '0' 1= Reference message with Next_is_Gap = '1' received"]
    #[inline(always)]
    pub fn set_wfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Application Watchdog Event The application watchdog is served by reading TTOST. When the watchdog is not served in time, bit AWE is set, all TTCAN communication is stopped, and the M_TTCAN is set into Bus Monitoring Mode. 0= Application Watchdog served in time 1= Failed to serve Application Watchdog in time"]
    #[inline(always)]
    pub const fn awe(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Application Watchdog Event The application watchdog is served by reading TTOST. When the watchdog is not served in time, bit AWE is set, all TTCAN communication is stopped, and the M_TTCAN is set into Bus Monitoring Mode. 0= Application Watchdog served in time 1= Failed to serve Application Watchdog in time"]
    #[inline(always)]
    pub fn set_awe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Wait for External Clock Synchronization 0= No external clock synchronization pending 1= Node waits for external clock synchronization to take effect. The bit is reset at the start of the next basic cycle."]
    #[inline(always)]
    pub const fn wecs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for External Clock Synchronization 0= No external clock synchronization pending 1= Node waits for external clock synchronization to take effect. The bit is reset at the start of the next basic cycle."]
    #[inline(always)]
    pub fn set_wecs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Schedule Phase Lock The bit is valid only when external synchronization is enabled (TTOCN.ESCN = '1'). In this case it signals that the difference between cycle time configured by TTGTP.CTP and the cycle time at the rising edge at pin m_ttcan_evt is less or equal 9 NTU (see Section 4.11). 0= Phase outside range 1= Phase inside range"]
    #[inline(always)]
    pub const fn spl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Schedule Phase Lock The bit is valid only when external synchronization is enabled (TTOCN.ESCN = '1'). In this case it signals that the difference between cycle time configured by TTGTP.CTP and the cycle time at the rising edge at pin m_ttcan_evt is less or equal 9 NTU (see Section 4.11). 0= Phase outside range 1= Phase inside range"]
    #[inline(always)]
    pub fn set_spl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ttost {
    #[inline(always)]
    fn default() -> Ttost {
        Ttost(0)
    }
}
#[doc = "TT Reference Message Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ttrmc(pub u32);
impl Ttrmc {
    #[doc = "Reference Identifier Identifier transmitted with reference message and used for reference message filtering. Standard or extended reference identifier depending on bit XTD. A standard identifier has to be written to ID\\[28:18\\]."]
    #[inline(always)]
    pub const fn rid(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Reference Identifier Identifier transmitted with reference message and used for reference message filtering. Standard or extended reference identifier depending on bit XTD. A standard identifier has to be written to ID\\[28:18\\]."]
    #[inline(always)]
    pub fn set_rid(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
    #[doc = "Extended Identifier 0= 11-bit standard identifier 1= 29-bit extended identifier"]
    #[inline(always)]
    pub const fn xtd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Identifier 0= 11-bit standard identifier 1= 29-bit extended identifier"]
    #[inline(always)]
    pub fn set_xtd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reference Message Payload Select Ignored in case of time slaves. 0= Reference message has no additional payload 1= The following elements are taken from Tx Buffer 0: Message Marker MM, Event FIFO Control EFC, Data Length Code DLC, Data Bytes DB Level 1: bytes 2-8, Level 0,2: bytes 5-8)"]
    #[inline(always)]
    pub const fn rmps(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Message Payload Select Ignored in case of time slaves. 0= Reference message has no additional payload 1= The following elements are taken from Tx Buffer 0: Message Marker MM, Event FIFO Control EFC, Data Length Code DLC, Data Bytes DB Level 1: bytes 2-8, Level 0,2: bytes 5-8)"]
    #[inline(always)]
    pub fn set_rmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ttrmc {
    #[inline(always)]
    fn default() -> Ttrmc {
        Ttrmc(0)
    }
}
#[doc = "TT Trigger Memory Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tttmc(pub u32);
impl Tttmc {
    #[doc = "Trigger Memory Start Address Start address of Trigger Memory in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub const fn tmsa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Trigger Memory Start Address Start address of Trigger Memory in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn set_tmsa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Trigger Memory Elements 0= No Trigger Memory 1-64= Number of Trigger Memory elements 64= Values greater than 64 are interpreted as 64"]
    #[inline(always)]
    pub const fn tme(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Trigger Memory Elements 0= No Trigger Memory 1-64= Number of Trigger Memory elements 64= Values greater than 64 are interpreted as 64"]
    #[inline(always)]
    pub fn set_tme(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Tttmc {
    #[inline(always)]
    fn default() -> Tttmc {
        Tttmc(0)
    }
}
#[doc = "TT Time Mark"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tttmk(pub u32);
impl Tttmk {
    #[doc = "Time Mark 0x0000-FFFF Time Mark"]
    #[inline(always)]
    pub const fn tm_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Time Mark 0x0000-FFFF Time Mark"]
    #[inline(always)]
    pub fn set_tm_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Time Mark Cycle Code Cycle count for which the time mark is valid. 0b000000x valid for all cycles 0b000001c valid every second cycle at cycle count mod2 = c 0b00001cc valid every fourth cycle at cycle count mod4 = cc 0b0001ccc valid every eighth cycle at cycle count mod8 = ccc 0b001cccc valid every sixteenth cycle at cycle count mod16 = cccc 0b01ccccc valid every thirty-second cycle at cycle count mod32 = ccccc 0b1cccccc valid every sixty-fourth cycle at cycle count mod64 = cccccc"]
    #[inline(always)]
    pub const fn ticc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Time Mark Cycle Code Cycle count for which the time mark is valid. 0b000000x valid for all cycles 0b000001c valid every second cycle at cycle count mod2 = c 0b00001cc valid every fourth cycle at cycle count mod4 = cc 0b0001ccc valid every eighth cycle at cycle count mod8 = ccc 0b001cccc valid every sixteenth cycle at cycle count mod16 = cccc 0b01ccccc valid every thirty-second cycle at cycle count mod32 = ccccc 0b1cccccc valid every sixty-fourth cycle at cycle count mod64 = cccccc"]
    #[inline(always)]
    pub fn set_ticc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "TT Time Mark Register Locked Always set by a write access to registers TTOCN. Set by write access to register TTTMK when TTOCN.TMC != '00'. Reset when the registers have been synchronized into the CAN clock domain. 0= Write access to TTTMK enabled 1= Write access to TTTMK locked"]
    #[inline(always)]
    pub const fn lckm(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TT Time Mark Register Locked Always set by a write access to registers TTOCN. Set by write access to register TTTMK when TTOCN.TMC != '00'. Reset when the registers have been synchronized into the CAN clock domain. 0= Write access to TTTMK enabled 1= Write access to TTTMK locked"]
    #[inline(always)]
    pub fn set_lckm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tttmk {
    #[inline(always)]
    fn default() -> Tttmk {
        Tttmk(0)
    }
}
#[doc = "TUR Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Turcf(pub u32);
impl Turcf {
    #[doc = "Numerator Configuration Low Write access to the TUR Numerator Configuration Low is only possible during configuration with TURCF.ELT = '0' or if TTOCF.EECS (external clock synchronization enabled) is set. When a new value for NCL is written outside TT Configuration Mode, the new value takes effect when TTOST.WECS is cleared to '0'. NCL is locked TTOST.WECS is '1'. 0x0000-FFFF Numerator Configuration Low"]
    #[inline(always)]
    pub const fn ncl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Numerator Configuration Low Write access to the TUR Numerator Configuration Low is only possible during configuration with TURCF.ELT = '0' or if TTOCF.EECS (external clock synchronization enabled) is set. When a new value for NCL is written outside TT Configuration Mode, the new value takes effect when TTOST.WECS is cleared to '0'. NCL is locked TTOST.WECS is '1'. 0x0000-FFFF Numerator Configuration Low"]
    #[inline(always)]
    pub fn set_ncl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Denominator Configuration 0x0000 Illegal value 0x0001-3FFF Denominator Configuration"]
    #[inline(always)]
    pub const fn dc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Denominator Configuration 0x0000 Illegal value 0x0001-3FFF Denominator Configuration"]
    #[inline(always)]
    pub fn set_dc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
    #[doc = "Enable Local Time 0= Local time is stopped, default 1= Local time is enabled"]
    #[inline(always)]
    pub const fn elt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local Time 0= Local time is stopped, default 1= Local time is enabled"]
    #[inline(always)]
    pub fn set_elt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Turcf {
    #[inline(always)]
    fn default() -> Turcf {
        Turcf(0)
    }
}
#[doc = "TUR Numerator Actual"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Turna(pub u32);
impl Turna {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn nav(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_nav(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for Turna {
    #[inline(always)]
    fn default() -> Turna {
        Turna(0)
    }
}
#[doc = "Tx Buffer Add Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbar(pub u32);
impl Txbar {
    #[doc = "Add Request Each Tx Buffer has its own Add Request bit. Writing a '1' will set the corresponding Add Request bit; writing a '0' has no impact. This enables the Host to set transmission requests for multiple Tx Buffers with one write to TXBAR. TXBAR bits are set only for those Tx Buffers configured via TXBC. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed. 0= No transmission request added 1= Transmission requested added"]
    #[inline(always)]
    pub const fn ar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Add Request Each Tx Buffer has its own Add Request bit. Writing a '1' will set the corresponding Add Request bit; writing a '0' has no impact. This enables the Host to set transmission requests for multiple Tx Buffers with one write to TXBAR. TXBAR bits are set only for those Tx Buffers configured via TXBC. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed. 0= No transmission request added 1= Transmission requested added"]
    #[inline(always)]
    pub fn set_ar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbar {
    #[inline(always)]
    fn default() -> Txbar {
        Txbar(0)
    }
}
#[doc = "Tx Buffer Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbc(pub u32);
impl Txbc {
    #[doc = "Tx Buffers Start Address Start address of Tx Buffers section in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub const fn tbsa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Tx Buffers Start Address Start address of Tx Buffers section in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn set_tbsa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Number of Dedicated Transmit Buffers 0= No Dedicated Tx Buffers 1-32= Number of Dedicated Tx Buffers 32= Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    pub const fn ndtb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of Dedicated Transmit Buffers 0= No Dedicated Tx Buffers 1-32= Number of Dedicated Tx Buffers 32= Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    pub fn set_ndtb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Transmit FIFO/Queue Size 0= No Tx FIFO/Queue 1-32= Number of Tx Buffers used for Tx FIFO/Queue 32= Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    pub const fn tfqs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Transmit FIFO/Queue Size 0= No Tx FIFO/Queue 1-32= Number of Tx Buffers used for Tx FIFO/Queue 32= Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    pub fn set_tfqs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Tx FIFO/Queue Mode 0= Tx FIFO operation 1= Tx Queue operation"]
    #[inline(always)]
    pub const fn tfqm(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO/Queue Mode 0= Tx FIFO operation 1= Tx Queue operation"]
    #[inline(always)]
    pub fn set_tfqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Txbc {
    #[inline(always)]
    fn default() -> Txbc {
        Txbc(0)
    }
}
#[doc = "Tx Buffer Cancellation Finished"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbcf(pub u32);
impl Txbcf {
    #[doc = "Cancellation Finished Each Tx Buffer has its own Cancellation Finished bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0= No transmit buffer cancellation 1= Transmit buffer cancellation finished"]
    #[inline(always)]
    pub const fn cf(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation Finished Each Tx Buffer has its own Cancellation Finished bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0= No transmit buffer cancellation 1= Transmit buffer cancellation finished"]
    #[inline(always)]
    pub fn set_cf(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbcf {
    #[inline(always)]
    fn default() -> Txbcf {
        Txbcf(0)
    }
}
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbcie(pub u32);
impl Txbcie {
    #[doc = "Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub const fn cfie(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub fn set_cfie(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbcie {
    #[inline(always)]
    fn default() -> Txbcie {
        Txbcie(0)
    }
}
#[doc = "Tx Buffer Cancellation Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbcr(pub u32);
impl Txbcr {
    #[doc = "Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a '1' will set the corresponding Cancellation Request bit; writing a '0' has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending"]
    #[inline(always)]
    pub const fn cr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a '1' will set the corresponding Cancellation Request bit; writing a '0' has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending"]
    #[inline(always)]
    pub fn set_cr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbcr {
    #[inline(always)]
    fn default() -> Txbcr {
        Txbcr(0)
    }
}
#[doc = "Tx Buffer Request Pending"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbrp(pub u32);
impl Txbrp {
    #[doc = "Transmission Request Pending Each Tx Buffer has its own Transmission Request Pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been cancelled via register TXBCR. TXBRP bits are set only for those Tx Buffers configured via TXBC. After a TXBRP bit has been set, a Tx scan (see Section 3.5, Tx Handling) is started to check for the pending Tx request with the highest priority (Tx Buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically cancelled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions. 0= No transmission request pending 1= Transmission request pending"]
    #[inline(always)]
    pub const fn trp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission Request Pending Each Tx Buffer has its own Transmission Request Pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been cancelled via register TXBCR. TXBRP bits are set only for those Tx Buffers configured via TXBC. After a TXBRP bit has been set, a Tx scan (see Section 3.5, Tx Handling) is started to check for the pending Tx request with the highest priority (Tx Buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically cancelled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions. 0= No transmission request pending 1= Transmission request pending"]
    #[inline(always)]
    pub fn set_trp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbrp {
    #[inline(always)]
    fn default() -> Txbrp {
        Txbrp(0)
    }
}
#[doc = "Tx Buffer Transmission Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbtie(pub u32);
impl Txbtie {
    #[doc = "Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable"]
    #[inline(always)]
    pub const fn tie(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable"]
    #[inline(always)]
    pub fn set_tie(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbtie {
    #[inline(always)]
    fn default() -> Txbtie {
        Txbtie(0)
    }
}
#[doc = "Tx Buffer Transmission Occurred"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbto(pub u32);
impl Txbto {
    #[doc = "Transmission Occurred Each Tx Buffer has its own Transmission Occurred bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0= No transmission occurred 1= Transmission occurred"]
    #[inline(always)]
    pub const fn to(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission Occurred Each Tx Buffer has its own Transmission Occurred bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0= No transmission occurred 1= Transmission occurred"]
    #[inline(always)]
    pub fn set_to(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbto {
    #[inline(always)]
    fn default() -> Txbto {
        Txbto(0)
    }
}
#[doc = "Tx Event FIFO Acknowledge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txefa(pub u32);
impl Txefa {
    #[doc = "Event FIFO Acknowledge Index After the Host has read an element or a sequence of elements from the Tx Event FIFO it has to write the index of the last element read from Tx Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI to EFAI + 1 and update the Event FIFO Fill Level TXEFS.EFFL."]
    #[inline(always)]
    pub const fn efai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO Acknowledge Index After the Host has read an element or a sequence of elements from the Tx Event FIFO it has to write the index of the last element read from Tx Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI to EFAI + 1 and update the Event FIFO Fill Level TXEFS.EFFL."]
    #[inline(always)]
    pub fn set_efai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Txefa {
    #[inline(always)]
    fn default() -> Txefa {
        Txefa(0)
    }
}
#[doc = "Tx Event FIFO Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txefc(pub u32);
impl Txefc {
    #[doc = "Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub const fn efsa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn set_efsa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements 32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS-1"]
    #[inline(always)]
    pub const fn efs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements 32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS-1"]
    #[inline(always)]
    pub fn set_efs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) 32= Watermark interrupt disabled"]
    #[inline(always)]
    pub const fn efwm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) 32= Watermark interrupt disabled"]
    #[inline(always)]
    pub fn set_efwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Txefc {
    #[inline(always)]
    fn default() -> Txefc {
        Txefc(0)
    }
}
#[doc = "Tx Event FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txefs(pub u32);
impl Txefs {
    #[doc = "Event FIFO Fill Level Number of elements stored in Tx Event FIFO, range 0 to 32."]
    #[inline(always)]
    pub const fn effl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO Fill Level Number of elements stored in Tx Event FIFO, range 0 to 32."]
    #[inline(always)]
    pub fn set_effl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Event FIFO Get Index Tx Event FIFO read index pointer, range 0 to 31."]
    #[inline(always)]
    pub const fn efgi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO Get Index Tx Event FIFO read index pointer, range 0 to 31."]
    #[inline(always)]
    pub fn set_efgi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Event FIFO Put Index Tx Event FIFO write index pointer, range 0 to 31."]
    #[inline(always)]
    pub const fn efpi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO Put Index Tx Event FIFO write index pointer, range 0 to 31."]
    #[inline(always)]
    pub fn set_efpi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
    #[inline(always)]
    pub const fn eff(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
    #[inline(always)]
    pub fn set_eff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Tx Event FIFO Element Lost This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset, this bit is also reset. 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
    #[inline(always)]
    pub const fn tefl(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Element Lost This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset, this bit is also reset. 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
    #[inline(always)]
    pub fn set_tefl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Txefs {
    #[inline(always)]
    fn default() -> Txefs {
        Txefs(0)
    }
}
#[doc = "Tx Buffer Element Size Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txesc(pub u32);
impl Txesc {
    #[doc = "Tx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub const fn tbds(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Tx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field"]
    #[inline(always)]
    pub fn set_tbds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Txesc {
    #[inline(always)]
    fn default() -> Txesc {
        Txesc(0)
    }
}
#[doc = "Tx FIFO/Queue Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txfqs(pub u32);
impl Txfqs {
    #[doc = "Tx FIFO Free Level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = '1')"]
    #[inline(always)]
    pub const fn tffl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Tx FIFO Free Level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = '1')"]
    #[inline(always)]
    pub fn set_tffl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Tx FIFO Get Index Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured TXBC.TFQM = '1')."]
    #[inline(always)]
    pub const fn tfgi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Tx FIFO Get Index Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured TXBC.TFQM = '1')."]
    #[inline(always)]
    pub fn set_tfgi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Tx FIFO/Queue Put Index Tx FIFO/Queue write index pointer, range 0 to 31."]
    #[inline(always)]
    pub const fn tfqpi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Tx FIFO/Queue Put Index Tx FIFO/Queue write index pointer, range 0 to 31."]
    #[inline(always)]
    pub fn set_tfqpi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Tx FIFO/Queue Full 0= Tx FIFO/Queue not full 1= Tx FIFO/Queue full"]
    #[inline(always)]
    pub const fn tfqf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO/Queue Full 0= Tx FIFO/Queue not full 1= Tx FIFO/Queue full"]
    #[inline(always)]
    pub fn set_tfqf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Txfqs {
    #[inline(always)]
    fn default() -> Txfqs {
        Txfqs(0)
    }
}
#[doc = "Extended ID AND Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xidam(pub u32);
impl Xidam {
    #[doc = "Extended ID Mask For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active."]
    #[inline(always)]
    pub const fn eidm(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Extended ID Mask For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active."]
    #[inline(always)]
    pub fn set_eidm(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for Xidam {
    #[inline(always)]
    fn default() -> Xidam {
        Xidam(0)
    }
}
#[doc = "Extended ID Filter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xidfc(pub u32);
impl Xidfc {
    #[doc = "Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub const fn flesa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address, see Figure 2)."]
    #[inline(always)]
    pub fn set_flesa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements 64= Values greater than 64 are interpreted as 64"]
    #[inline(always)]
    pub const fn lse(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements 64= Values greater than 64 are interpreted as 64"]
    #[inline(always)]
    pub fn set_lse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Xidfc {
    #[inline(always)]
    fn default() -> Xidfc {
        Xidfc(0)
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
