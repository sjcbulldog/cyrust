#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "USB Device"]
use core::prelude::v1::derive;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbdev {
    ptr: *mut u8,
}
unsafe impl Send for Usbdev {}
unsafe impl Sync for Usbdev {}
impl Usbdev {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control End point EP0 Data Register"]
    #[inline(always)]
    pub const fn ep0_dr(self, n: usize) -> crate::common::Reg<Ep0Dr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize + n * 4usize) as _) }
    }
    #[doc = "USB control 0 Register"]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "USB control 1 Register"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "USB SIE Data Endpoints Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sie_ep_int_en(self) -> crate::common::Reg<SieEpIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "USB SIE Data Endpoint Interrupt Status"]
    #[inline(always)]
    pub const fn sie_ep_int_sr(self) -> crate::common::Reg<SieEpIntSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep1_cnt0(self) -> crate::common::Reg<SieEp1Cnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep1_cnt1(self) -> crate::common::Reg<SieEp1Cnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep1_cr0(self) -> crate::common::Reg<SieEp1Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "USBIO Control 0 Register"]
    #[inline(always)]
    pub const fn usbio_cr0(self) -> crate::common::Reg<UsbioCr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "USBIO control 2 Register"]
    #[inline(always)]
    pub const fn usbio_cr2(self) -> crate::common::Reg<UsbioCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "USBIO control 1 Register"]
    #[inline(always)]
    pub const fn usbio_cr1(self) -> crate::common::Reg<UsbioCr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "USB Dynamic reconfiguration register"]
    #[inline(always)]
    pub const fn dyn_reconfig(self) -> crate::common::Reg<DynReconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Start Of Frame Register"]
    #[inline(always)]
    pub const fn sof0(self) -> crate::common::Reg<Sof0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Start Of Frame Register"]
    #[inline(always)]
    pub const fn sof1(self) -> crate::common::Reg<Sof1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep2_cnt0(self) -> crate::common::Reg<SieEp2Cnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep2_cnt1(self) -> crate::common::Reg<SieEp2Cnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep2_cr0(self) -> crate::common::Reg<SieEp2Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "Oscillator lock data register 0"]
    #[inline(always)]
    pub const fn osclk_dr0(self) -> crate::common::Reg<OsclkDr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Oscillator lock data register 1"]
    #[inline(always)]
    pub const fn osclk_dr1(self) -> crate::common::Reg<OsclkDr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Endpoint0 control Register"]
    #[inline(always)]
    pub const fn ep0_cr(self) -> crate::common::Reg<Ep0Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "Endpoint0 count Register"]
    #[inline(always)]
    pub const fn ep0_cnt(self) -> crate::common::Reg<Ep0Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep3_cnt0(self) -> crate::common::Reg<SieEp3Cnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep3_cnt1(self) -> crate::common::Reg<SieEp3Cnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(180usize) as _) }
    }
    #[doc = "Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep3_cr0(self) -> crate::common::Reg<SieEp3Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(184usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep4_cnt0(self) -> crate::common::Reg<SieEp4Cnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep4_cnt1(self) -> crate::common::Reg<SieEp4Cnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep4_cr0(self) -> crate::common::Reg<SieEp4Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(248usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep5_cnt0(self) -> crate::common::Reg<SieEp5Cnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(304usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep5_cnt1(self) -> crate::common::Reg<SieEp5Cnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(308usize) as _) }
    }
    #[doc = "Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep5_cr0(self) -> crate::common::Reg<SieEp5Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(312usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep6_cnt0(self) -> crate::common::Reg<SieEp6Cnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep6_cnt1(self) -> crate::common::Reg<SieEp6Cnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(372usize) as _) }
    }
    #[doc = "Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep6_cr0(self) -> crate::common::Reg<SieEp6Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(376usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep7_cnt0(self) -> crate::common::Reg<SieEp7Cnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(432usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep7_cnt1(self) -> crate::common::Reg<SieEp7Cnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(436usize) as _) }
    }
    #[doc = "Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep7_cr0(self) -> crate::common::Reg<SieEp7Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(440usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep8_cnt0(self) -> crate::common::Reg<SieEp8Cnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(496usize) as _) }
    }
    #[doc = "Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep8_cnt1(self) -> crate::common::Reg<SieEp8Cnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(500usize) as _) }
    }
    #[doc = "Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep8_cr0(self) -> crate::common::Reg<SieEp8Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(504usize) as _) }
    }
    #[doc = "Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep1_cfg(self) -> crate::common::Reg<ArbEp1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep1_int_en(self) -> crate::common::Reg<ArbEp1IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep1_sr(self) -> crate::common::Reg<ArbEp1Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw1_wa(self) -> crate::common::Reg<ArbRw1Wa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(528usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw1_wa_msb(self) -> crate::common::Reg<ArbRw1WaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(532usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw1_ra(self) -> crate::common::Reg<ArbRw1Ra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(536usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw1_ra_msb(self) -> crate::common::Reg<ArbRw1RaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(540usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw1_dr(self) -> crate::common::Reg<ArbRw1Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize) as _) }
    }
    #[doc = "Dedicated Endpoint Buffer Size Register *1"]
    #[inline(always)]
    pub const fn buf_size(self) -> crate::common::Reg<BufSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(560usize) as _) }
    }
    #[doc = "Endpoint Active Indication Register *1"]
    #[inline(always)]
    pub const fn ep_active(self) -> crate::common::Reg<EpActive, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(568usize) as _) }
    }
    #[doc = "Endpoint Type (IN/OUT) Indication *1"]
    #[inline(always)]
    pub const fn ep_type(self) -> crate::common::Reg<EpType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(572usize) as _) }
    }
    #[doc = "Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep2_cfg(self) -> crate::common::Reg<ArbEp2Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(576usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep2_int_en(self) -> crate::common::Reg<ArbEp2IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(580usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep2_sr(self) -> crate::common::Reg<ArbEp2Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(584usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw2_wa(self) -> crate::common::Reg<ArbRw2Wa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(592usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw2_wa_msb(self) -> crate::common::Reg<ArbRw2WaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(596usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw2_ra(self) -> crate::common::Reg<ArbRw2Ra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(600usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw2_ra_msb(self) -> crate::common::Reg<ArbRw2RaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(604usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw2_dr(self) -> crate::common::Reg<ArbRw2Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(608usize) as _) }
    }
    #[doc = "Arbiter Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_cfg(self) -> crate::common::Reg<ArbCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(624usize) as _) }
    }
    #[doc = "USB Block Clock Enable Register"]
    #[inline(always)]
    pub const fn usb_clk_en(self) -> crate::common::Reg<UsbClkEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(628usize) as _) }
    }
    #[doc = "Arbiter Interrupt Enable *1"]
    #[inline(always)]
    pub const fn arb_int_en(self) -> crate::common::Reg<ArbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(632usize) as _) }
    }
    #[doc = "Arbiter Interrupt Status *1"]
    #[inline(always)]
    pub const fn arb_int_sr(self) -> crate::common::Reg<ArbIntSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(636usize) as _) }
    }
    #[doc = "Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep3_cfg(self) -> crate::common::Reg<ArbEp3Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(640usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep3_int_en(self) -> crate::common::Reg<ArbEp3IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(644usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep3_sr(self) -> crate::common::Reg<ArbEp3Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(648usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw3_wa(self) -> crate::common::Reg<ArbRw3Wa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(656usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw3_wa_msb(self) -> crate::common::Reg<ArbRw3WaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(660usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw3_ra(self) -> crate::common::Reg<ArbRw3Ra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(664usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw3_ra_msb(self) -> crate::common::Reg<ArbRw3RaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(668usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw3_dr(self) -> crate::common::Reg<ArbRw3Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(672usize) as _) }
    }
    #[doc = "Common Area Write Address *1"]
    #[inline(always)]
    pub const fn cwa(self) -> crate::common::Reg<Cwa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(688usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn cwa_msb(self) -> crate::common::Reg<CwaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(692usize) as _) }
    }
    #[doc = "Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep4_cfg(self) -> crate::common::Reg<ArbEp4Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(704usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep4_int_en(self) -> crate::common::Reg<ArbEp4IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(708usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep4_sr(self) -> crate::common::Reg<ArbEp4Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(712usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw4_wa(self) -> crate::common::Reg<ArbRw4Wa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(720usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw4_wa_msb(self) -> crate::common::Reg<ArbRw4WaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(724usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw4_ra(self) -> crate::common::Reg<ArbRw4Ra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(728usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw4_ra_msb(self) -> crate::common::Reg<ArbRw4RaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(732usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw4_dr(self) -> crate::common::Reg<ArbRw4Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(736usize) as _) }
    }
    #[doc = "DMA Burst / Threshold Configuration"]
    #[inline(always)]
    pub const fn dma_thres(self) -> crate::common::Reg<DmaThres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(752usize) as _) }
    }
    #[doc = "DMA Burst / Threshold Configuration"]
    #[inline(always)]
    pub const fn dma_thres_msb(self) -> crate::common::Reg<DmaThresMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(756usize) as _) }
    }
    #[doc = "Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep5_cfg(self) -> crate::common::Reg<ArbEp5Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep5_int_en(self) -> crate::common::Reg<ArbEp5IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(772usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep5_sr(self) -> crate::common::Reg<ArbEp5Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(776usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw5_wa(self) -> crate::common::Reg<ArbRw5Wa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(784usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw5_wa_msb(self) -> crate::common::Reg<ArbRw5WaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(788usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw5_ra(self) -> crate::common::Reg<ArbRw5Ra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(792usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw5_ra_msb(self) -> crate::common::Reg<ArbRw5RaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(796usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw5_dr(self) -> crate::common::Reg<ArbRw5Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(800usize) as _) }
    }
    #[doc = "Bus Reset Count Register"]
    #[inline(always)]
    pub const fn bus_rst_cnt(self) -> crate::common::Reg<BusRstCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(816usize) as _) }
    }
    #[doc = "Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep6_cfg(self) -> crate::common::Reg<ArbEp6Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(832usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep6_int_en(self) -> crate::common::Reg<ArbEp6IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(836usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep6_sr(self) -> crate::common::Reg<ArbEp6Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(840usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw6_wa(self) -> crate::common::Reg<ArbRw6Wa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(848usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw6_wa_msb(self) -> crate::common::Reg<ArbRw6WaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(852usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw6_ra(self) -> crate::common::Reg<ArbRw6Ra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(856usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw6_ra_msb(self) -> crate::common::Reg<ArbRw6RaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(860usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw6_dr(self) -> crate::common::Reg<ArbRw6Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(864usize) as _) }
    }
    #[doc = "Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep7_cfg(self) -> crate::common::Reg<ArbEp7Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(896usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep7_int_en(self) -> crate::common::Reg<ArbEp7IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(900usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep7_sr(self) -> crate::common::Reg<ArbEp7Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(904usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw7_wa(self) -> crate::common::Reg<ArbRw7Wa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(912usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw7_wa_msb(self) -> crate::common::Reg<ArbRw7WaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(916usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw7_ra(self) -> crate::common::Reg<ArbRw7Ra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(920usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw7_ra_msb(self) -> crate::common::Reg<ArbRw7RaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(924usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw7_dr(self) -> crate::common::Reg<ArbRw7Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(928usize) as _) }
    }
    #[doc = "Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep8_cfg(self) -> crate::common::Reg<ArbEp8Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(960usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep8_int_en(self) -> crate::common::Reg<ArbEp8IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(964usize) as _) }
    }
    #[doc = "Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep8_sr(self) -> crate::common::Reg<ArbEp8Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(968usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw8_wa(self) -> crate::common::Reg<ArbRw8Wa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(976usize) as _) }
    }
    #[doc = "Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw8_wa_msb(self) -> crate::common::Reg<ArbRw8WaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(980usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw8_ra(self) -> crate::common::Reg<ArbRw8Ra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(984usize) as _) }
    }
    #[doc = "Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw8_ra_msb(self) -> crate::common::Reg<ArbRw8RaMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(988usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw8_dr(self) -> crate::common::Reg<ArbRw8Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(992usize) as _) }
    }
    #[doc = "DATA"]
    #[inline(always)]
    pub const fn mem_data(self, n: usize) -> crate::common::Reg<MemData, crate::common::RW> {
        assert!(n < 512usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize + n * 4usize) as _) }
    }
    #[doc = "Start Of Frame Register"]
    #[inline(always)]
    pub const fn sof16(self) -> crate::common::Reg<Sof16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4192usize) as _) }
    }
    #[doc = "Oscillator lock data register"]
    #[inline(always)]
    pub const fn osclk_dr16(self) -> crate::common::Reg<OsclkDr16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4224usize) as _) }
    }
    #[doc = "Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw1_wa16(self) -> crate::common::Reg<ArbRw1Wa16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4624usize) as _) }
    }
    #[doc = "Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw1_ra16(self) -> crate::common::Reg<ArbRw1Ra16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4632usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw1_dr16(self) -> crate::common::Reg<ArbRw1Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4640usize) as _) }
    }
    #[doc = "Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw2_wa16(self) -> crate::common::Reg<ArbRw2Wa16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4688usize) as _) }
    }
    #[doc = "Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw2_ra16(self) -> crate::common::Reg<ArbRw2Ra16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4696usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw2_dr16(self) -> crate::common::Reg<ArbRw2Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4704usize) as _) }
    }
    #[doc = "Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw3_wa16(self) -> crate::common::Reg<ArbRw3Wa16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4752usize) as _) }
    }
    #[doc = "Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw3_ra16(self) -> crate::common::Reg<ArbRw3Ra16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4760usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw3_dr16(self) -> crate::common::Reg<ArbRw3Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4768usize) as _) }
    }
    #[doc = "Common Area Write Address"]
    #[inline(always)]
    pub const fn cwa16(self) -> crate::common::Reg<Cwa16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4784usize) as _) }
    }
    #[doc = "Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw4_wa16(self) -> crate::common::Reg<ArbRw4Wa16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4816usize) as _) }
    }
    #[doc = "Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw4_ra16(self) -> crate::common::Reg<ArbRw4Ra16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4824usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw4_dr16(self) -> crate::common::Reg<ArbRw4Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4832usize) as _) }
    }
    #[doc = "DMA Burst / Threshold Configuration"]
    #[inline(always)]
    pub const fn dma_thres16(self) -> crate::common::Reg<DmaThres16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4848usize) as _) }
    }
    #[doc = "Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw5_wa16(self) -> crate::common::Reg<ArbRw5Wa16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4880usize) as _) }
    }
    #[doc = "Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw5_ra16(self) -> crate::common::Reg<ArbRw5Ra16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4888usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw5_dr16(self) -> crate::common::Reg<ArbRw5Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4896usize) as _) }
    }
    #[doc = "Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw6_wa16(self) -> crate::common::Reg<ArbRw6Wa16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4944usize) as _) }
    }
    #[doc = "Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw6_ra16(self) -> crate::common::Reg<ArbRw6Ra16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4952usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw6_dr16(self) -> crate::common::Reg<ArbRw6Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4960usize) as _) }
    }
    #[doc = "Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw7_wa16(self) -> crate::common::Reg<ArbRw7Wa16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(5008usize) as _) }
    }
    #[doc = "Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw7_ra16(self) -> crate::common::Reg<ArbRw7Ra16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(5016usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw7_dr16(self) -> crate::common::Reg<ArbRw7Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(5024usize) as _) }
    }
    #[doc = "Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw8_wa16(self) -> crate::common::Reg<ArbRw8Wa16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(5072usize) as _) }
    }
    #[doc = "Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw8_ra16(self) -> crate::common::Reg<ArbRw8Ra16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(5080usize) as _) }
    }
    #[doc = "Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw8_dr16(self) -> crate::common::Reg<ArbRw8Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(5088usize) as _) }
    }
}
#[doc = "USB Host and Device Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfs0 {
    ptr: *mut u8,
}
unsafe impl Send for Usbfs0 {}
unsafe impl Sync for Usbfs0 {}
impl Usbfs0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB Device"]
    #[inline(always)]
    pub const fn usbdev(self) -> Usbdev {
        unsafe { Usbdev::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "USB Device LPM and PHY Test"]
    #[inline(always)]
    pub const fn usblpm(self) -> Usblpm {
        unsafe { Usblpm::from_ptr(self.ptr.add(8192usize) as _) }
    }
    #[doc = "USB Host Controller"]
    #[inline(always)]
    pub const fn usbhost(self) -> Usbhost {
        unsafe { Usbhost::from_ptr(self.ptr.add(16384usize) as _) }
    }
}
#[doc = "USB Host Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhost {
    ptr: *mut u8,
}
unsafe impl Send for Usbhost {}
unsafe impl Sync for Usbhost {}
impl Usbhost {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Host Control 0 Register."]
    #[inline(always)]
    pub const fn host_ctl0(self) -> crate::common::Reg<HostCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Host Control 1 Register."]
    #[inline(always)]
    pub const fn host_ctl1(self) -> crate::common::Reg<HostCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Host Control 2 Register."]
    #[inline(always)]
    pub const fn host_ctl2(self) -> crate::common::Reg<HostCtl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "Host Error Status Register."]
    #[inline(always)]
    pub const fn host_err(self) -> crate::common::Reg<HostErr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "Host Status Register."]
    #[inline(always)]
    pub const fn host_status(self) -> crate::common::Reg<HostStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "Host SOF Interrupt Frame Compare Register"]
    #[inline(always)]
    pub const fn host_fcomp(self) -> crate::common::Reg<HostFcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(268usize) as _) }
    }
    #[doc = "Host Retry Timer Setup Register"]
    #[inline(always)]
    pub const fn host_rtimer(self) -> crate::common::Reg<HostRtimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(272usize) as _) }
    }
    #[doc = "Host Address Register"]
    #[inline(always)]
    pub const fn host_addr(self) -> crate::common::Reg<HostAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(276usize) as _) }
    }
    #[doc = "Host EOF Setup Register"]
    #[inline(always)]
    pub const fn host_eof(self) -> crate::common::Reg<HostEof, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(280usize) as _) }
    }
    #[doc = "Host Frame Setup Register"]
    #[inline(always)]
    pub const fn host_frame(self) -> crate::common::Reg<HostFrame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(284usize) as _) }
    }
    #[doc = "Host Token Endpoint Register"]
    #[inline(always)]
    pub const fn host_token(self) -> crate::common::Reg<HostToken, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(288usize) as _) }
    }
    #[doc = "Host Endpoint 1 Control Register"]
    #[inline(always)]
    pub const fn host_ep1_ctl(self) -> crate::common::Reg<HostEp1Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "Host Endpoint 1 Status Register"]
    #[inline(always)]
    pub const fn host_ep1_status(self) -> crate::common::Reg<HostEp1Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1028usize) as _) }
    }
    #[doc = "Host Endpoint 1 Data 1-Byte Register"]
    #[inline(always)]
    pub const fn host_ep1_rw1_dr(self) -> crate::common::Reg<HostEp1Rw1Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1032usize) as _) }
    }
    #[doc = "Host Endpoint 1 Data 2-Byte Register"]
    #[inline(always)]
    pub const fn host_ep1_rw2_dr(self) -> crate::common::Reg<HostEp1Rw2Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1036usize) as _) }
    }
    #[doc = "Host Endpoint 2 Control Register"]
    #[inline(always)]
    pub const fn host_ep2_ctl(self) -> crate::common::Reg<HostEp2Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1280usize) as _) }
    }
    #[doc = "Host Endpoint 2 Status Register"]
    #[inline(always)]
    pub const fn host_ep2_status(self) -> crate::common::Reg<HostEp2Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1284usize) as _) }
    }
    #[doc = "Host Endpoint 2 Data 1-Byte Register"]
    #[inline(always)]
    pub const fn host_ep2_rw1_dr(self) -> crate::common::Reg<HostEp2Rw1Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1288usize) as _) }
    }
    #[doc = "Host Endpoint 2 Data 2-Byte Register"]
    #[inline(always)]
    pub const fn host_ep2_rw2_dr(self) -> crate::common::Reg<HostEp2Rw2Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1292usize) as _) }
    }
    #[doc = "Host Interrupt Level 1 Selection Register"]
    #[inline(always)]
    pub const fn host_lvl1_sel(self) -> crate::common::Reg<HostLvl1Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2048usize) as _) }
    }
    #[doc = "Host Interrupt Level 2 Selection Register"]
    #[inline(always)]
    pub const fn host_lvl2_sel(self) -> crate::common::Reg<HostLvl2Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2052usize) as _) }
    }
    #[doc = "Interrupt USB Host Cause High Register"]
    #[inline(always)]
    pub const fn intr_usbhost_cause_hi(
        self,
    ) -> crate::common::Reg<IntrUsbhostCauseHi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2304usize) as _) }
    }
    #[doc = "Interrupt USB Host Cause Medium Register"]
    #[inline(always)]
    pub const fn intr_usbhost_cause_med(
        self,
    ) -> crate::common::Reg<IntrUsbhostCauseMed, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2308usize) as _) }
    }
    #[doc = "Interrupt USB Host Cause Low Register"]
    #[inline(always)]
    pub const fn intr_usbhost_cause_lo(
        self,
    ) -> crate::common::Reg<IntrUsbhostCauseLo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2312usize) as _) }
    }
    #[doc = "Interrupt USB Host Endpoint Cause High Register"]
    #[inline(always)]
    pub const fn intr_host_ep_cause_hi(
        self,
    ) -> crate::common::Reg<IntrHostEpCauseHi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2336usize) as _) }
    }
    #[doc = "Interrupt USB Host Endpoint Cause Medium Register"]
    #[inline(always)]
    pub const fn intr_host_ep_cause_med(
        self,
    ) -> crate::common::Reg<IntrHostEpCauseMed, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2340usize) as _) }
    }
    #[doc = "Interrupt USB Host Endpoint Cause Low Register"]
    #[inline(always)]
    pub const fn intr_host_ep_cause_lo(
        self,
    ) -> crate::common::Reg<IntrHostEpCauseLo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2344usize) as _) }
    }
    #[doc = "Interrupt USB Host Register"]
    #[inline(always)]
    pub const fn intr_usbhost(self) -> crate::common::Reg<IntrUsbhost, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2368usize) as _) }
    }
    #[doc = "Interrupt USB Host Set Register"]
    #[inline(always)]
    pub const fn intr_usbhost_set(self) -> crate::common::Reg<IntrUsbhostSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2372usize) as _) }
    }
    #[doc = "Interrupt USB Host Mask Register"]
    #[inline(always)]
    pub const fn intr_usbhost_mask(self) -> crate::common::Reg<IntrUsbhostMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2376usize) as _) }
    }
    #[doc = "Interrupt USB Host Masked Register"]
    #[inline(always)]
    pub const fn intr_usbhost_masked(
        self,
    ) -> crate::common::Reg<IntrUsbhostMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2380usize) as _) }
    }
    #[doc = "Interrupt USB Host Endpoint Register"]
    #[inline(always)]
    pub const fn intr_host_ep(self) -> crate::common::Reg<IntrHostEp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2560usize) as _) }
    }
    #[doc = "Interrupt USB Host Endpoint Set Register"]
    #[inline(always)]
    pub const fn intr_host_ep_set(self) -> crate::common::Reg<IntrHostEpSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2564usize) as _) }
    }
    #[doc = "Interrupt USB Host Endpoint Mask Register"]
    #[inline(always)]
    pub const fn intr_host_ep_mask(self) -> crate::common::Reg<IntrHostEpMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2568usize) as _) }
    }
    #[doc = "Interrupt USB Host Endpoint Masked Register"]
    #[inline(always)]
    pub const fn intr_host_ep_masked(
        self,
    ) -> crate::common::Reg<IntrHostEpMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2572usize) as _) }
    }
    #[doc = "Host DMA Enable Register"]
    #[inline(always)]
    pub const fn host_dma_enbl(self) -> crate::common::Reg<HostDmaEnbl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2816usize) as _) }
    }
    #[doc = "Host Endpoint 1 Block Register"]
    #[inline(always)]
    pub const fn host_ep1_blk(self) -> crate::common::Reg<HostEp1Blk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2848usize) as _) }
    }
    #[doc = "Host Endpoint 2 Block Register"]
    #[inline(always)]
    pub const fn host_ep2_blk(self) -> crate::common::Reg<HostEp2Blk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2864usize) as _) }
    }
}
#[doc = "USB Device LPM and PHY Test"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usblpm {
    ptr: *mut u8,
}
unsafe impl Send for Usblpm {}
unsafe impl Sync for Usblpm {}
impl Usblpm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power Control Register"]
    #[inline(always)]
    pub const fn power_ctl(self) -> crate::common::Reg<PowerCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "USB IO Control Register"]
    #[inline(always)]
    pub const fn usbio_ctl(self) -> crate::common::Reg<UsbioCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Flow Control Register"]
    #[inline(always)]
    pub const fn flow_ctl(self) -> crate::common::Reg<FlowCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "LPM Control Register"]
    #[inline(always)]
    pub const fn lpm_ctl(self) -> crate::common::Reg<LpmCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "LPM Status register"]
    #[inline(always)]
    pub const fn lpm_stat(self) -> crate::common::Reg<LpmStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "USB SOF, BUS RESET and EP0 Interrupt Status"]
    #[inline(always)]
    pub const fn intr_sie(self) -> crate::common::Reg<IntrSie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "USB SOF, BUS RESET and EP0 Interrupt Set"]
    #[inline(always)]
    pub const fn intr_sie_set(self) -> crate::common::Reg<IntrSieSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask"]
    #[inline(always)]
    pub const fn intr_sie_mask(self) -> crate::common::Reg<IntrSieMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "USB SOF, BUS RESET and EP0 Interrupt Masked"]
    #[inline(always)]
    pub const fn intr_sie_masked(self) -> crate::common::Reg<IntrSieMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Select interrupt level for each interrupt source"]
    #[inline(always)]
    pub const fn intr_lvl_sel(self) -> crate::common::Reg<IntrLvlSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "High priority interrupt Cause register"]
    #[inline(always)]
    pub const fn intr_cause_hi(self) -> crate::common::Reg<IntrCauseHi, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Medium priority interrupt Cause register"]
    #[inline(always)]
    pub const fn intr_cause_med(self) -> crate::common::Reg<IntrCauseMed, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Low priority interrupt Cause register"]
    #[inline(always)]
    pub const fn intr_cause_lo(self) -> crate::common::Reg<IntrCauseLo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "DFT control"]
    #[inline(always)]
    pub const fn dft_ctl(self) -> crate::common::Reg<DftCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
}
#[doc = "Arbiter Configuration Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbCfg(pub u32);
impl ArbCfg {
    #[doc = "Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    pub const fn auto_mem(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    pub fn set_auto_mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA Access Configuration."]
    #[inline(always)]
    pub const fn dma_cfg(&self) -> DmaCfg {
        let val = (self.0 >> 5usize) & 0x03;
        DmaCfg::from_bits(val as u8)
    }
    #[doc = "DMA Access Configuration."]
    #[inline(always)]
    pub fn set_dma_cfg(&mut self, val: DmaCfg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    pub const fn cfg_cmp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    pub fn set_cfg_cmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ArbCfg {
    #[inline(always)]
    fn default() -> ArbCfg {
        ArbCfg(0)
    }
}
#[doc = "Endpoint Configuration Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp1Cfg(pub u32);
impl ArbEp1Cfg {
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub const fn in_data_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn set_in_data_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub const fn crc_bypass(&self) -> ArbEp1CfgCrcBypass {
        let val = (self.0 >> 2usize) & 0x01;
        ArbEp1CfgCrcBypass::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn set_crc_bypass(&mut self, val: ArbEp1CfgCrcBypass) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub const fn reset_ptr(&self) -> ArbEp1CfgResetPtr {
        let val = (self.0 >> 3usize) & 0x01;
        ArbEp1CfgResetPtr::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn set_reset_ptr(&mut self, val: ArbEp1CfgResetPtr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for ArbEp1Cfg {
    #[inline(always)]
    fn default() -> ArbEp1Cfg {
        ArbEp1Cfg(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp1IntEn(pub u32);
impl ArbEp1IntEn {
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub const fn in_buf_full_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn set_in_buf_full_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub const fn dma_gnt_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn set_dma_gnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub const fn buf_over_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn set_buf_over_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub const fn buf_under_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn set_buf_under_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub const fn err_int_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn set_err_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub const fn dma_termin_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn set_dma_termin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp1IntEn {
    #[inline(always)]
    fn default() -> ArbEp1IntEn {
        ArbEp1IntEn(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp1Sr(pub u32);
impl ArbEp1Sr {
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub const fn in_buf_full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn set_in_buf_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub const fn dma_gnt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn set_dma_gnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub const fn buf_over(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub const fn buf_under(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub const fn dma_termin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn set_dma_termin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp1Sr {
    #[inline(always)]
    fn default() -> ArbEp1Sr {
        ArbEp1Sr(0)
    }
}
#[doc = "Endpoint Configuration Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp2Cfg(pub u32);
impl ArbEp2Cfg {
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub const fn in_data_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn set_in_data_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub const fn crc_bypass(&self) -> ArbEp2CfgCrcBypass {
        let val = (self.0 >> 2usize) & 0x01;
        ArbEp2CfgCrcBypass::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn set_crc_bypass(&mut self, val: ArbEp2CfgCrcBypass) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub const fn reset_ptr(&self) -> ArbEp2CfgResetPtr {
        let val = (self.0 >> 3usize) & 0x01;
        ArbEp2CfgResetPtr::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn set_reset_ptr(&mut self, val: ArbEp2CfgResetPtr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for ArbEp2Cfg {
    #[inline(always)]
    fn default() -> ArbEp2Cfg {
        ArbEp2Cfg(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp2IntEn(pub u32);
impl ArbEp2IntEn {
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub const fn in_buf_full_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn set_in_buf_full_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub const fn dma_gnt_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn set_dma_gnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub const fn buf_over_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn set_buf_over_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub const fn buf_under_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn set_buf_under_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub const fn err_int_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn set_err_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub const fn dma_termin_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn set_dma_termin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp2IntEn {
    #[inline(always)]
    fn default() -> ArbEp2IntEn {
        ArbEp2IntEn(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp2Sr(pub u32);
impl ArbEp2Sr {
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub const fn in_buf_full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn set_in_buf_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub const fn dma_gnt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn set_dma_gnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub const fn buf_over(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub const fn buf_under(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub const fn dma_termin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn set_dma_termin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp2Sr {
    #[inline(always)]
    fn default() -> ArbEp2Sr {
        ArbEp2Sr(0)
    }
}
#[doc = "Endpoint Configuration Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp3Cfg(pub u32);
impl ArbEp3Cfg {
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub const fn in_data_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn set_in_data_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub const fn crc_bypass(&self) -> ArbEp3CfgCrcBypass {
        let val = (self.0 >> 2usize) & 0x01;
        ArbEp3CfgCrcBypass::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn set_crc_bypass(&mut self, val: ArbEp3CfgCrcBypass) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub const fn reset_ptr(&self) -> ArbEp3CfgResetPtr {
        let val = (self.0 >> 3usize) & 0x01;
        ArbEp3CfgResetPtr::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn set_reset_ptr(&mut self, val: ArbEp3CfgResetPtr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for ArbEp3Cfg {
    #[inline(always)]
    fn default() -> ArbEp3Cfg {
        ArbEp3Cfg(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp3IntEn(pub u32);
impl ArbEp3IntEn {
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub const fn in_buf_full_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn set_in_buf_full_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub const fn dma_gnt_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn set_dma_gnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub const fn buf_over_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn set_buf_over_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub const fn buf_under_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn set_buf_under_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub const fn err_int_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn set_err_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub const fn dma_termin_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn set_dma_termin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp3IntEn {
    #[inline(always)]
    fn default() -> ArbEp3IntEn {
        ArbEp3IntEn(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp3Sr(pub u32);
impl ArbEp3Sr {
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub const fn in_buf_full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn set_in_buf_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub const fn dma_gnt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn set_dma_gnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub const fn buf_over(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub const fn buf_under(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub const fn dma_termin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn set_dma_termin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp3Sr {
    #[inline(always)]
    fn default() -> ArbEp3Sr {
        ArbEp3Sr(0)
    }
}
#[doc = "Endpoint Configuration Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp4Cfg(pub u32);
impl ArbEp4Cfg {
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub const fn in_data_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn set_in_data_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub const fn crc_bypass(&self) -> ArbEp4CfgCrcBypass {
        let val = (self.0 >> 2usize) & 0x01;
        ArbEp4CfgCrcBypass::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn set_crc_bypass(&mut self, val: ArbEp4CfgCrcBypass) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub const fn reset_ptr(&self) -> ArbEp4CfgResetPtr {
        let val = (self.0 >> 3usize) & 0x01;
        ArbEp4CfgResetPtr::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn set_reset_ptr(&mut self, val: ArbEp4CfgResetPtr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for ArbEp4Cfg {
    #[inline(always)]
    fn default() -> ArbEp4Cfg {
        ArbEp4Cfg(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp4IntEn(pub u32);
impl ArbEp4IntEn {
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub const fn in_buf_full_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn set_in_buf_full_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub const fn dma_gnt_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn set_dma_gnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub const fn buf_over_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn set_buf_over_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub const fn buf_under_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn set_buf_under_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub const fn err_int_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn set_err_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub const fn dma_termin_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn set_dma_termin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp4IntEn {
    #[inline(always)]
    fn default() -> ArbEp4IntEn {
        ArbEp4IntEn(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp4Sr(pub u32);
impl ArbEp4Sr {
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub const fn in_buf_full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn set_in_buf_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub const fn dma_gnt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn set_dma_gnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub const fn buf_over(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub const fn buf_under(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub const fn dma_termin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn set_dma_termin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp4Sr {
    #[inline(always)]
    fn default() -> ArbEp4Sr {
        ArbEp4Sr(0)
    }
}
#[doc = "Endpoint Configuration Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp5Cfg(pub u32);
impl ArbEp5Cfg {
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub const fn in_data_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn set_in_data_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub const fn crc_bypass(&self) -> ArbEp5CfgCrcBypass {
        let val = (self.0 >> 2usize) & 0x01;
        ArbEp5CfgCrcBypass::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn set_crc_bypass(&mut self, val: ArbEp5CfgCrcBypass) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub const fn reset_ptr(&self) -> ArbEp5CfgResetPtr {
        let val = (self.0 >> 3usize) & 0x01;
        ArbEp5CfgResetPtr::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn set_reset_ptr(&mut self, val: ArbEp5CfgResetPtr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for ArbEp5Cfg {
    #[inline(always)]
    fn default() -> ArbEp5Cfg {
        ArbEp5Cfg(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp5IntEn(pub u32);
impl ArbEp5IntEn {
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub const fn in_buf_full_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn set_in_buf_full_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub const fn dma_gnt_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn set_dma_gnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub const fn buf_over_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn set_buf_over_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub const fn buf_under_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn set_buf_under_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub const fn err_int_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn set_err_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub const fn dma_termin_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn set_dma_termin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp5IntEn {
    #[inline(always)]
    fn default() -> ArbEp5IntEn {
        ArbEp5IntEn(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp5Sr(pub u32);
impl ArbEp5Sr {
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub const fn in_buf_full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn set_in_buf_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub const fn dma_gnt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn set_dma_gnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub const fn buf_over(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub const fn buf_under(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub const fn dma_termin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn set_dma_termin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp5Sr {
    #[inline(always)]
    fn default() -> ArbEp5Sr {
        ArbEp5Sr(0)
    }
}
#[doc = "Endpoint Configuration Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp6Cfg(pub u32);
impl ArbEp6Cfg {
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub const fn in_data_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn set_in_data_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub const fn crc_bypass(&self) -> ArbEp6CfgCrcBypass {
        let val = (self.0 >> 2usize) & 0x01;
        ArbEp6CfgCrcBypass::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn set_crc_bypass(&mut self, val: ArbEp6CfgCrcBypass) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub const fn reset_ptr(&self) -> ArbEp6CfgResetPtr {
        let val = (self.0 >> 3usize) & 0x01;
        ArbEp6CfgResetPtr::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn set_reset_ptr(&mut self, val: ArbEp6CfgResetPtr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for ArbEp6Cfg {
    #[inline(always)]
    fn default() -> ArbEp6Cfg {
        ArbEp6Cfg(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp6IntEn(pub u32);
impl ArbEp6IntEn {
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub const fn in_buf_full_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn set_in_buf_full_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub const fn dma_gnt_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn set_dma_gnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub const fn buf_over_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn set_buf_over_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub const fn buf_under_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn set_buf_under_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub const fn err_int_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn set_err_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub const fn dma_termin_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn set_dma_termin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp6IntEn {
    #[inline(always)]
    fn default() -> ArbEp6IntEn {
        ArbEp6IntEn(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp6Sr(pub u32);
impl ArbEp6Sr {
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub const fn in_buf_full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn set_in_buf_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub const fn dma_gnt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn set_dma_gnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub const fn buf_over(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub const fn buf_under(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub const fn dma_termin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn set_dma_termin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp6Sr {
    #[inline(always)]
    fn default() -> ArbEp6Sr {
        ArbEp6Sr(0)
    }
}
#[doc = "Endpoint Configuration Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp7Cfg(pub u32);
impl ArbEp7Cfg {
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub const fn in_data_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn set_in_data_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub const fn crc_bypass(&self) -> ArbEp7CfgCrcBypass {
        let val = (self.0 >> 2usize) & 0x01;
        ArbEp7CfgCrcBypass::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn set_crc_bypass(&mut self, val: ArbEp7CfgCrcBypass) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub const fn reset_ptr(&self) -> ArbEp7CfgResetPtr {
        let val = (self.0 >> 3usize) & 0x01;
        ArbEp7CfgResetPtr::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn set_reset_ptr(&mut self, val: ArbEp7CfgResetPtr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for ArbEp7Cfg {
    #[inline(always)]
    fn default() -> ArbEp7Cfg {
        ArbEp7Cfg(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp7IntEn(pub u32);
impl ArbEp7IntEn {
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub const fn in_buf_full_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn set_in_buf_full_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub const fn dma_gnt_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn set_dma_gnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub const fn buf_over_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn set_buf_over_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub const fn buf_under_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn set_buf_under_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub const fn err_int_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn set_err_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub const fn dma_termin_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn set_dma_termin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp7IntEn {
    #[inline(always)]
    fn default() -> ArbEp7IntEn {
        ArbEp7IntEn(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp7Sr(pub u32);
impl ArbEp7Sr {
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub const fn in_buf_full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn set_in_buf_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub const fn dma_gnt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn set_dma_gnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub const fn buf_over(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub const fn buf_under(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub const fn dma_termin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn set_dma_termin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp7Sr {
    #[inline(always)]
    fn default() -> ArbEp7Sr {
        ArbEp7Sr(0)
    }
}
#[doc = "Endpoint Configuration Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp8Cfg(pub u32);
impl ArbEp8Cfg {
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub const fn in_data_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn set_in_data_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub const fn dma_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn set_dma_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub const fn crc_bypass(&self) -> ArbEp8CfgCrcBypass {
        let val = (self.0 >> 2usize) & 0x01;
        ArbEp8CfgCrcBypass::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn set_crc_bypass(&mut self, val: ArbEp8CfgCrcBypass) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub const fn reset_ptr(&self) -> ArbEp8CfgResetPtr {
        let val = (self.0 >> 3usize) & 0x01;
        ArbEp8CfgResetPtr::from_bits(val as u8)
    }
    #[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn set_reset_ptr(&mut self, val: ArbEp8CfgResetPtr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for ArbEp8Cfg {
    #[inline(always)]
    fn default() -> ArbEp8Cfg {
        ArbEp8Cfg(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp8IntEn(pub u32);
impl ArbEp8IntEn {
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub const fn in_buf_full_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn set_in_buf_full_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub const fn dma_gnt_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn set_dma_gnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub const fn buf_over_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn set_buf_over_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub const fn buf_under_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn set_buf_under_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub const fn err_int_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn set_err_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub const fn dma_termin_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn set_dma_termin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp8IntEn {
    #[inline(always)]
    fn default() -> ArbEp8IntEn {
        ArbEp8IntEn(0)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbEp8Sr(pub u32);
impl ArbEp8Sr {
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub const fn in_buf_full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn set_in_buf_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub const fn dma_gnt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn set_dma_gnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub const fn buf_over(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub const fn buf_under(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn set_buf_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub const fn dma_termin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn set_dma_termin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for ArbEp8Sr {
    #[inline(always)]
    fn default() -> ArbEp8Sr {
        ArbEp8Sr(0)
    }
}
#[doc = "Arbiter Interrupt Enable *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbIntEn(pub u32);
impl ArbIntEn {
    #[doc = "Enables interrupt for EP1"]
    #[inline(always)]
    pub const fn ep1_intr_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP1"]
    #[inline(always)]
    pub fn set_ep1_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables interrupt for EP2"]
    #[inline(always)]
    pub const fn ep2_intr_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP2"]
    #[inline(always)]
    pub fn set_ep2_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for EP3"]
    #[inline(always)]
    pub const fn ep3_intr_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP3"]
    #[inline(always)]
    pub fn set_ep3_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables interrupt for EP4"]
    #[inline(always)]
    pub const fn ep4_intr_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP4"]
    #[inline(always)]
    pub fn set_ep4_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables interrupt for EP5"]
    #[inline(always)]
    pub const fn ep5_intr_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP5"]
    #[inline(always)]
    pub fn set_ep5_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables interrupt for EP6"]
    #[inline(always)]
    pub const fn ep6_intr_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP6"]
    #[inline(always)]
    pub fn set_ep6_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables interrupt for EP7"]
    #[inline(always)]
    pub const fn ep7_intr_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP7"]
    #[inline(always)]
    pub fn set_ep7_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables interrupt for EP8"]
    #[inline(always)]
    pub const fn ep8_intr_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP8"]
    #[inline(always)]
    pub fn set_ep8_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ArbIntEn {
    #[inline(always)]
    fn default() -> ArbIntEn {
        ArbIntEn(0)
    }
}
#[doc = "Arbiter Interrupt Status *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbIntSr(pub u32);
impl ArbIntSr {
    #[doc = "Interrupt status for EP1"]
    #[inline(always)]
    pub const fn ep1_intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP1"]
    #[inline(always)]
    pub fn set_ep1_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt status for EP2"]
    #[inline(always)]
    pub const fn ep2_intr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP2"]
    #[inline(always)]
    pub fn set_ep2_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt status for EP3"]
    #[inline(always)]
    pub const fn ep3_intr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP3"]
    #[inline(always)]
    pub fn set_ep3_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt status for EP4"]
    #[inline(always)]
    pub const fn ep4_intr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP4"]
    #[inline(always)]
    pub fn set_ep4_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt status for EP5"]
    #[inline(always)]
    pub const fn ep5_intr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP5"]
    #[inline(always)]
    pub fn set_ep5_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt status for EP6"]
    #[inline(always)]
    pub const fn ep6_intr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP6"]
    #[inline(always)]
    pub fn set_ep6_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt status for EP7"]
    #[inline(always)]
    pub const fn ep7_intr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP7"]
    #[inline(always)]
    pub fn set_ep7_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt status for EP8"]
    #[inline(always)]
    pub const fn ep8_intr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP8"]
    #[inline(always)]
    pub fn set_ep8_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ArbIntSr {
    #[inline(always)]
    fn default() -> ArbIntSr {
        ArbIntSr(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw1Dr(pub u32);
impl ArbRw1Dr {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw1Dr {
    #[inline(always)]
    fn default() -> ArbRw1Dr {
        ArbRw1Dr(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw1Dr16(pub u32);
impl ArbRw1Dr16 {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ArbRw1Dr16 {
    #[inline(always)]
    fn default() -> ArbRw1Dr16 {
        ArbRw1Dr16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw1Ra(pub u32);
impl ArbRw1Ra {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw1Ra {
    #[inline(always)]
    fn default() -> ArbRw1Ra {
        ArbRw1Ra(0)
    }
}
#[doc = "Endpoint Read Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw1Ra16(pub u32);
impl ArbRw1Ra16 {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw1Ra16 {
    #[inline(always)]
    fn default() -> ArbRw1Ra16 {
        ArbRw1Ra16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw1RaMsb(pub u32);
impl ArbRw1RaMsb {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw1RaMsb {
    #[inline(always)]
    fn default() -> ArbRw1RaMsb {
        ArbRw1RaMsb(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw1Wa(pub u32);
impl ArbRw1Wa {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw1Wa {
    #[inline(always)]
    fn default() -> ArbRw1Wa {
        ArbRw1Wa(0)
    }
}
#[doc = "Endpoint Write Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw1Wa16(pub u32);
impl ArbRw1Wa16 {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw1Wa16 {
    #[inline(always)]
    fn default() -> ArbRw1Wa16 {
        ArbRw1Wa16(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw1WaMsb(pub u32);
impl ArbRw1WaMsb {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw1WaMsb {
    #[inline(always)]
    fn default() -> ArbRw1WaMsb {
        ArbRw1WaMsb(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw2Dr(pub u32);
impl ArbRw2Dr {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw2Dr {
    #[inline(always)]
    fn default() -> ArbRw2Dr {
        ArbRw2Dr(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw2Dr16(pub u32);
impl ArbRw2Dr16 {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ArbRw2Dr16 {
    #[inline(always)]
    fn default() -> ArbRw2Dr16 {
        ArbRw2Dr16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw2Ra(pub u32);
impl ArbRw2Ra {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw2Ra {
    #[inline(always)]
    fn default() -> ArbRw2Ra {
        ArbRw2Ra(0)
    }
}
#[doc = "Endpoint Read Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw2Ra16(pub u32);
impl ArbRw2Ra16 {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw2Ra16 {
    #[inline(always)]
    fn default() -> ArbRw2Ra16 {
        ArbRw2Ra16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw2RaMsb(pub u32);
impl ArbRw2RaMsb {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw2RaMsb {
    #[inline(always)]
    fn default() -> ArbRw2RaMsb {
        ArbRw2RaMsb(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw2Wa(pub u32);
impl ArbRw2Wa {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw2Wa {
    #[inline(always)]
    fn default() -> ArbRw2Wa {
        ArbRw2Wa(0)
    }
}
#[doc = "Endpoint Write Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw2Wa16(pub u32);
impl ArbRw2Wa16 {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw2Wa16 {
    #[inline(always)]
    fn default() -> ArbRw2Wa16 {
        ArbRw2Wa16(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw2WaMsb(pub u32);
impl ArbRw2WaMsb {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw2WaMsb {
    #[inline(always)]
    fn default() -> ArbRw2WaMsb {
        ArbRw2WaMsb(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw3Dr(pub u32);
impl ArbRw3Dr {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw3Dr {
    #[inline(always)]
    fn default() -> ArbRw3Dr {
        ArbRw3Dr(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw3Dr16(pub u32);
impl ArbRw3Dr16 {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ArbRw3Dr16 {
    #[inline(always)]
    fn default() -> ArbRw3Dr16 {
        ArbRw3Dr16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw3Ra(pub u32);
impl ArbRw3Ra {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw3Ra {
    #[inline(always)]
    fn default() -> ArbRw3Ra {
        ArbRw3Ra(0)
    }
}
#[doc = "Endpoint Read Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw3Ra16(pub u32);
impl ArbRw3Ra16 {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw3Ra16 {
    #[inline(always)]
    fn default() -> ArbRw3Ra16 {
        ArbRw3Ra16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw3RaMsb(pub u32);
impl ArbRw3RaMsb {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw3RaMsb {
    #[inline(always)]
    fn default() -> ArbRw3RaMsb {
        ArbRw3RaMsb(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw3Wa(pub u32);
impl ArbRw3Wa {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw3Wa {
    #[inline(always)]
    fn default() -> ArbRw3Wa {
        ArbRw3Wa(0)
    }
}
#[doc = "Endpoint Write Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw3Wa16(pub u32);
impl ArbRw3Wa16 {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw3Wa16 {
    #[inline(always)]
    fn default() -> ArbRw3Wa16 {
        ArbRw3Wa16(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw3WaMsb(pub u32);
impl ArbRw3WaMsb {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw3WaMsb {
    #[inline(always)]
    fn default() -> ArbRw3WaMsb {
        ArbRw3WaMsb(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw4Dr(pub u32);
impl ArbRw4Dr {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw4Dr {
    #[inline(always)]
    fn default() -> ArbRw4Dr {
        ArbRw4Dr(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw4Dr16(pub u32);
impl ArbRw4Dr16 {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ArbRw4Dr16 {
    #[inline(always)]
    fn default() -> ArbRw4Dr16 {
        ArbRw4Dr16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw4Ra(pub u32);
impl ArbRw4Ra {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw4Ra {
    #[inline(always)]
    fn default() -> ArbRw4Ra {
        ArbRw4Ra(0)
    }
}
#[doc = "Endpoint Read Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw4Ra16(pub u32);
impl ArbRw4Ra16 {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw4Ra16 {
    #[inline(always)]
    fn default() -> ArbRw4Ra16 {
        ArbRw4Ra16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw4RaMsb(pub u32);
impl ArbRw4RaMsb {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw4RaMsb {
    #[inline(always)]
    fn default() -> ArbRw4RaMsb {
        ArbRw4RaMsb(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw4Wa(pub u32);
impl ArbRw4Wa {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw4Wa {
    #[inline(always)]
    fn default() -> ArbRw4Wa {
        ArbRw4Wa(0)
    }
}
#[doc = "Endpoint Write Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw4Wa16(pub u32);
impl ArbRw4Wa16 {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw4Wa16 {
    #[inline(always)]
    fn default() -> ArbRw4Wa16 {
        ArbRw4Wa16(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw4WaMsb(pub u32);
impl ArbRw4WaMsb {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw4WaMsb {
    #[inline(always)]
    fn default() -> ArbRw4WaMsb {
        ArbRw4WaMsb(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw5Dr(pub u32);
impl ArbRw5Dr {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw5Dr {
    #[inline(always)]
    fn default() -> ArbRw5Dr {
        ArbRw5Dr(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw5Dr16(pub u32);
impl ArbRw5Dr16 {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ArbRw5Dr16 {
    #[inline(always)]
    fn default() -> ArbRw5Dr16 {
        ArbRw5Dr16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw5Ra(pub u32);
impl ArbRw5Ra {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw5Ra {
    #[inline(always)]
    fn default() -> ArbRw5Ra {
        ArbRw5Ra(0)
    }
}
#[doc = "Endpoint Read Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw5Ra16(pub u32);
impl ArbRw5Ra16 {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw5Ra16 {
    #[inline(always)]
    fn default() -> ArbRw5Ra16 {
        ArbRw5Ra16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw5RaMsb(pub u32);
impl ArbRw5RaMsb {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw5RaMsb {
    #[inline(always)]
    fn default() -> ArbRw5RaMsb {
        ArbRw5RaMsb(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw5Wa(pub u32);
impl ArbRw5Wa {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw5Wa {
    #[inline(always)]
    fn default() -> ArbRw5Wa {
        ArbRw5Wa(0)
    }
}
#[doc = "Endpoint Write Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw5Wa16(pub u32);
impl ArbRw5Wa16 {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw5Wa16 {
    #[inline(always)]
    fn default() -> ArbRw5Wa16 {
        ArbRw5Wa16(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw5WaMsb(pub u32);
impl ArbRw5WaMsb {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw5WaMsb {
    #[inline(always)]
    fn default() -> ArbRw5WaMsb {
        ArbRw5WaMsb(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw6Dr(pub u32);
impl ArbRw6Dr {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw6Dr {
    #[inline(always)]
    fn default() -> ArbRw6Dr {
        ArbRw6Dr(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw6Dr16(pub u32);
impl ArbRw6Dr16 {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ArbRw6Dr16 {
    #[inline(always)]
    fn default() -> ArbRw6Dr16 {
        ArbRw6Dr16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw6Ra(pub u32);
impl ArbRw6Ra {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw6Ra {
    #[inline(always)]
    fn default() -> ArbRw6Ra {
        ArbRw6Ra(0)
    }
}
#[doc = "Endpoint Read Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw6Ra16(pub u32);
impl ArbRw6Ra16 {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw6Ra16 {
    #[inline(always)]
    fn default() -> ArbRw6Ra16 {
        ArbRw6Ra16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw6RaMsb(pub u32);
impl ArbRw6RaMsb {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw6RaMsb {
    #[inline(always)]
    fn default() -> ArbRw6RaMsb {
        ArbRw6RaMsb(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw6Wa(pub u32);
impl ArbRw6Wa {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw6Wa {
    #[inline(always)]
    fn default() -> ArbRw6Wa {
        ArbRw6Wa(0)
    }
}
#[doc = "Endpoint Write Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw6Wa16(pub u32);
impl ArbRw6Wa16 {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw6Wa16 {
    #[inline(always)]
    fn default() -> ArbRw6Wa16 {
        ArbRw6Wa16(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw6WaMsb(pub u32);
impl ArbRw6WaMsb {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw6WaMsb {
    #[inline(always)]
    fn default() -> ArbRw6WaMsb {
        ArbRw6WaMsb(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw7Dr(pub u32);
impl ArbRw7Dr {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw7Dr {
    #[inline(always)]
    fn default() -> ArbRw7Dr {
        ArbRw7Dr(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw7Dr16(pub u32);
impl ArbRw7Dr16 {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ArbRw7Dr16 {
    #[inline(always)]
    fn default() -> ArbRw7Dr16 {
        ArbRw7Dr16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw7Ra(pub u32);
impl ArbRw7Ra {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw7Ra {
    #[inline(always)]
    fn default() -> ArbRw7Ra {
        ArbRw7Ra(0)
    }
}
#[doc = "Endpoint Read Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw7Ra16(pub u32);
impl ArbRw7Ra16 {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw7Ra16 {
    #[inline(always)]
    fn default() -> ArbRw7Ra16 {
        ArbRw7Ra16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw7RaMsb(pub u32);
impl ArbRw7RaMsb {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw7RaMsb {
    #[inline(always)]
    fn default() -> ArbRw7RaMsb {
        ArbRw7RaMsb(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw7Wa(pub u32);
impl ArbRw7Wa {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw7Wa {
    #[inline(always)]
    fn default() -> ArbRw7Wa {
        ArbRw7Wa(0)
    }
}
#[doc = "Endpoint Write Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw7Wa16(pub u32);
impl ArbRw7Wa16 {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw7Wa16 {
    #[inline(always)]
    fn default() -> ArbRw7Wa16 {
        ArbRw7Wa16(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw7WaMsb(pub u32);
impl ArbRw7WaMsb {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw7WaMsb {
    #[inline(always)]
    fn default() -> ArbRw7WaMsb {
        ArbRw7WaMsb(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw8Dr(pub u32);
impl ArbRw8Dr {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw8Dr {
    #[inline(always)]
    fn default() -> ArbRw8Dr {
        ArbRw8Dr(0)
    }
}
#[doc = "Endpoint Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw8Dr16(pub u32);
impl ArbRw8Dr16 {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ArbRw8Dr16 {
    #[inline(always)]
    fn default() -> ArbRw8Dr16 {
        ArbRw8Dr16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw8Ra(pub u32);
impl ArbRw8Ra {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw8Ra {
    #[inline(always)]
    fn default() -> ArbRw8Ra {
        ArbRw8Ra(0)
    }
}
#[doc = "Endpoint Read Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw8Ra16(pub u32);
impl ArbRw8Ra16 {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw8Ra16 {
    #[inline(always)]
    fn default() -> ArbRw8Ra16 {
        ArbRw8Ra16(0)
    }
}
#[doc = "Endpoint Read Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw8RaMsb(pub u32);
impl ArbRw8RaMsb {
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub const fn ra_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read Address for EP"]
    #[inline(always)]
    pub fn set_ra_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw8RaMsb {
    #[inline(always)]
    fn default() -> ArbRw8RaMsb {
        ArbRw8RaMsb(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw8Wa(pub u32);
impl ArbRw8Wa {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ArbRw8Wa {
    #[inline(always)]
    fn default() -> ArbRw8Wa {
        ArbRw8Wa(0)
    }
}
#[doc = "Endpoint Write Address value *3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw8Wa16(pub u32);
impl ArbRw8Wa16 {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for ArbRw8Wa16 {
    #[inline(always)]
    fn default() -> ArbRw8Wa16 {
        ArbRw8Wa16(0)
    }
}
#[doc = "Endpoint Write Address value *1, *2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArbRw8WaMsb(pub u32);
impl ArbRw8WaMsb {
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub const fn wa_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Address for EP"]
    #[inline(always)]
    pub fn set_wa_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ArbRw8WaMsb {
    #[inline(always)]
    fn default() -> ArbRw8WaMsb {
        ArbRw8WaMsb(0)
    }
}
#[doc = "Dedicated Endpoint Buffer Size Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BufSize(pub u32);
impl BufSize {
    #[doc = "Buffer size for IN Endpoints."]
    #[inline(always)]
    pub const fn in_buf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer size for IN Endpoints."]
    #[inline(always)]
    pub fn set_in_buf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Buffer size for OUT Endpoints."]
    #[inline(always)]
    pub const fn out_buf(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer size for OUT Endpoints."]
    #[inline(always)]
    pub fn set_out_buf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for BufSize {
    #[inline(always)]
    fn default() -> BufSize {
        BufSize(0)
    }
}
#[doc = "Bus Reset Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BusRstCnt(pub u32);
impl BusRstCnt {
    #[doc = "Bus Reset Count Length"]
    #[inline(always)]
    pub const fn bus_rst_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bus Reset Count Length"]
    #[inline(always)]
    pub fn set_bus_rst_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for BusRstCnt {
    #[inline(always)]
    fn default() -> BusRstCnt {
        BusRstCnt(0)
    }
}
#[doc = "USB control 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr0(pub u32);
impl Cr0 {
    #[doc = "These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
    #[inline(always)]
    pub const fn device_address(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
    #[inline(always)]
    pub fn set_device_address(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
    #[inline(always)]
    pub const fn usb_enable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
    #[inline(always)]
    pub fn set_usb_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Cr0 {
    #[inline(always)]
    fn default() -> Cr0 {
        Cr0(0)
    }
}
#[doc = "USB control 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
    #[inline(always)]
    pub const fn reg_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
    #[inline(always)]
    pub fn set_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
    #[inline(always)]
    pub const fn enable_lock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
    #[inline(always)]
    pub fn set_enable_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
    #[inline(always)]
    pub const fn bus_activity(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
    #[inline(always)]
    pub fn set_bus_activity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
#[doc = "Common Area Write Address *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cwa(pub u32);
impl Cwa {
    #[doc = "Write Address for Common Area"]
    #[inline(always)]
    pub const fn cwa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write Address for Common Area"]
    #[inline(always)]
    pub fn set_cwa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cwa {
    #[inline(always)]
    fn default() -> Cwa {
        Cwa(0)
    }
}
#[doc = "Common Area Write Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cwa16(pub u32);
impl Cwa16 {
    #[doc = "Write Address for Common Area"]
    #[inline(always)]
    pub const fn cwa16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write Address for Common Area"]
    #[inline(always)]
    pub fn set_cwa16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Cwa16 {
    #[inline(always)]
    fn default() -> Cwa16 {
        Cwa16(0)
    }
}
#[doc = "Endpoint Read Address value *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CwaMsb(pub u32);
impl CwaMsb {
    #[doc = "Write Address for Common Area"]
    #[inline(always)]
    pub const fn cwa_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Address for Common Area"]
    #[inline(always)]
    pub fn set_cwa_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CwaMsb {
    #[inline(always)]
    fn default() -> CwaMsb {
        CwaMsb(0)
    }
}
#[doc = "DFT control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DftCtl(pub u32);
impl DftCtl {
    #[doc = "DDFT output select signal"]
    #[inline(always)]
    pub const fn ddft_out_sel(&self) -> DdftOutSel {
        let val = (self.0 >> 0usize) & 0x07;
        DdftOutSel::from_bits(val as u8)
    }
    #[doc = "DDFT output select signal"]
    #[inline(always)]
    pub fn set_ddft_out_sel(&mut self, val: DdftOutSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "DDFT input select signal"]
    #[inline(always)]
    pub const fn ddft_in_sel(&self) -> DdftInSel {
        let val = (self.0 >> 3usize) & 0x03;
        DdftInSel::from_bits(val as u8)
    }
    #[doc = "DDFT input select signal"]
    #[inline(always)]
    pub fn set_ddft_in_sel(&mut self, val: DdftInSel) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
}
impl Default for DftCtl {
    #[inline(always)]
    fn default() -> DftCtl {
        DftCtl(0)
    }
}
#[doc = "DMA Burst / Threshold Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaThres(pub u32);
impl DmaThres {
    #[doc = "DMA Threshold count"]
    #[inline(always)]
    pub const fn dma_ths(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DMA Threshold count"]
    #[inline(always)]
    pub fn set_dma_ths(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DmaThres {
    #[inline(always)]
    fn default() -> DmaThres {
        DmaThres(0)
    }
}
#[doc = "DMA Burst / Threshold Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaThres16(pub u32);
impl DmaThres16 {
    #[doc = "DMA Threshold count"]
    #[inline(always)]
    pub const fn dma_ths16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "DMA Threshold count"]
    #[inline(always)]
    pub fn set_dma_ths16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for DmaThres16 {
    #[inline(always)]
    fn default() -> DmaThres16 {
        DmaThres16(0)
    }
}
#[doc = "DMA Burst / Threshold Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaThresMsb(pub u32);
impl DmaThresMsb {
    #[doc = "DMA Threshold count"]
    #[inline(always)]
    pub const fn dma_ths_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Threshold count"]
    #[inline(always)]
    pub fn set_dma_ths_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for DmaThresMsb {
    #[inline(always)]
    fn default() -> DmaThresMsb {
        DmaThresMsb(0)
    }
}
#[doc = "USB Dynamic reconfiguration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DynReconfig(pub u32);
impl DynReconfig {
    #[doc = "This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    pub const fn dyn_config_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    pub fn set_dyn_config_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    pub const fn dyn_reconfig_epno(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    pub fn set_dyn_reconfig_epno(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "This bit indicates the ready status for the dynamic reconfiguration, when set to 1, indicates the block is ready for reconfiguration."]
    #[inline(always)]
    pub const fn dyn_reconfig_rdy_sts(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the ready status for the dynamic reconfiguration, when set to 1, indicates the block is ready for reconfiguration."]
    #[inline(always)]
    pub fn set_dyn_reconfig_rdy_sts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for DynReconfig {
    #[inline(always)]
    fn default() -> DynReconfig {
        DynReconfig(0)
    }
}
#[doc = "Endpoint0 count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep0Cnt(pub u32);
impl Ep0Cnt {
    #[doc = "These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
    #[inline(always)]
    pub const fn byte_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
    #[inline(always)]
    pub fn set_byte_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub const fn data_valid(&self) -> Ep0CntDataValid {
        let val = (self.0 >> 6usize) & 0x01;
        Ep0CntDataValid::from_bits(val as u8)
    }
    #[doc = "This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn set_data_valid(&mut self, val: Ep0CntDataValid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub const fn data_toggle(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn set_data_toggle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ep0Cnt {
    #[inline(always)]
    fn default() -> Ep0Cnt {
        Ep0Cnt(0)
    }
}
#[doc = "Endpoint0 control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep0Cr(pub u32);
impl Ep0Cr {
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub const fn mode(&self) -> Ep0CrMode {
        let val = (self.0 >> 0usize) & 0x0f;
        Ep0CrMode::from_bits(val as u8)
    }
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: Ep0CrMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn acked_txn(&self) -> Ep0CrAckedTxn {
        let val = (self.0 >> 4usize) & 0x01;
        Ep0CrAckedTxn::from_bits(val as u8)
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_acked_txn(&mut self, val: Ep0CrAckedTxn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn out_rcvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_out_rcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn in_rcvd(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_in_rcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
    #[inline(always)]
    pub const fn setup_rcvd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
    #[inline(always)]
    pub fn set_setup_rcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ep0Cr {
    #[inline(always)]
    fn default() -> Ep0Cr {
        Ep0Cr(0)
    }
}
#[doc = "Control End point EP0 Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep0Dr(pub u32);
impl Ep0Dr {
    #[doc = "This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    pub const fn data_byte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    pub fn set_data_byte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Ep0Dr {
    #[inline(always)]
    fn default() -> Ep0Dr {
        Ep0Dr(0)
    }
}
#[doc = "Endpoint Active Indication Register *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpActive(pub u32);
impl EpActive {
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub const fn ep1_act(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn set_ep1_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub const fn ep2_act(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn set_ep2_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub const fn ep3_act(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn set_ep3_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub const fn ep4_act(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn set_ep4_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub const fn ep5_act(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn set_ep5_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub const fn ep6_act(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn set_ep6_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub const fn ep7_act(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn set_ep7_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub const fn ep8_act(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn set_ep8_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for EpActive {
    #[inline(always)]
    fn default() -> EpActive {
        EpActive(0)
    }
}
#[doc = "Endpoint Type (IN/OUT) Indication *1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpType(pub u32);
impl EpType {
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub const fn ep1_typ(&self) -> Ep1Typ {
        let val = (self.0 >> 0usize) & 0x01;
        Ep1Typ::from_bits(val as u8)
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub fn set_ep1_typ(&mut self, val: Ep1Typ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub const fn ep2_typ(&self) -> Ep2Typ {
        let val = (self.0 >> 1usize) & 0x01;
        Ep2Typ::from_bits(val as u8)
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub fn set_ep2_typ(&mut self, val: Ep2Typ) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub const fn ep3_typ(&self) -> Ep3Typ {
        let val = (self.0 >> 2usize) & 0x01;
        Ep3Typ::from_bits(val as u8)
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub fn set_ep3_typ(&mut self, val: Ep3Typ) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub const fn ep4_typ(&self) -> Ep4Typ {
        let val = (self.0 >> 3usize) & 0x01;
        Ep4Typ::from_bits(val as u8)
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub fn set_ep4_typ(&mut self, val: Ep4Typ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub const fn ep5_typ(&self) -> Ep5Typ {
        let val = (self.0 >> 4usize) & 0x01;
        Ep5Typ::from_bits(val as u8)
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub fn set_ep5_typ(&mut self, val: Ep5Typ) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub const fn ep6_typ(&self) -> Ep6Typ {
        let val = (self.0 >> 5usize) & 0x01;
        Ep6Typ::from_bits(val as u8)
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub fn set_ep6_typ(&mut self, val: Ep6Typ) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub const fn ep7_typ(&self) -> Ep7Typ {
        let val = (self.0 >> 6usize) & 0x01;
        Ep7Typ::from_bits(val as u8)
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub fn set_ep7_typ(&mut self, val: Ep7Typ) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub const fn ep8_typ(&self) -> Ep8Typ {
        let val = (self.0 >> 7usize) & 0x01;
        Ep8Typ::from_bits(val as u8)
    }
    #[doc = "Endpoint Type Indication."]
    #[inline(always)]
    pub fn set_ep8_typ(&mut self, val: Ep8Typ) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for EpType {
    #[inline(always)]
    fn default() -> EpType {
        EpType(0)
    }
}
#[doc = "Flow Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlowCtl(pub u32);
impl FlowCtl {
    #[doc = "End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub const fn ep1_err_resp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub fn set_ep1_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "End Point 2 error response"]
    #[inline(always)]
    pub const fn ep2_err_resp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 2 error response"]
    #[inline(always)]
    pub fn set_ep2_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "End Point 3 error response"]
    #[inline(always)]
    pub const fn ep3_err_resp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 3 error response"]
    #[inline(always)]
    pub fn set_ep3_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "End Point 4 error response"]
    #[inline(always)]
    pub const fn ep4_err_resp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 4 error response"]
    #[inline(always)]
    pub fn set_ep4_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "End Point 5 error response"]
    #[inline(always)]
    pub const fn ep5_err_resp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 5 error response"]
    #[inline(always)]
    pub fn set_ep5_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "End Point 6 error response"]
    #[inline(always)]
    pub const fn ep6_err_resp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 6 error response"]
    #[inline(always)]
    pub fn set_ep6_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "End Point 7 error response"]
    #[inline(always)]
    pub const fn ep7_err_resp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 7 error response"]
    #[inline(always)]
    pub fn set_ep7_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "End Point 8 error response"]
    #[inline(always)]
    pub const fn ep8_err_resp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 8 error response"]
    #[inline(always)]
    pub fn set_ep8_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for FlowCtl {
    #[inline(always)]
    fn default() -> FlowCtl {
        FlowCtl(0)
    }
}
#[doc = "Host Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostAddr(pub u32);
impl HostAddr {
    #[doc = "These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn address(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_address(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for HostAddr {
    #[inline(always)]
    fn default() -> HostAddr {
        HostAddr(0)
    }
}
#[doc = "Host Control 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCtl0(pub u32);
impl HostCtl0 {
    #[doc = "This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The mode of operation mode does not transition immediately after setting this bit. Read this bit to confirm that the operation mode has changed. - This bit is reset to '0' if the ENABLE bit in this register changes from '1' to '0'. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
    #[inline(always)]
    pub const fn host(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The mode of operation mode does not transition immediately after setting this bit. Read this bit to confirm that the operation mode has changed. - This bit is reset to '0' if the ENABLE bit in this register changes from '1' to '0'. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
    #[inline(always)]
    pub fn set_host(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn't affect the USB Device."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn't affect the USB Device."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HostCtl0 {
    #[inline(always)]
    fn default() -> HostCtl0 {
        HostCtl0(0)
    }
}
#[doc = "Host Control 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCtl1(pub u32);
impl HostCtl1 {
    #[doc = "This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is set to it's default vaulue '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    pub const fn clksel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is set to it's default vaulue '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    pub fn set_clksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal operating mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    pub const fn ustp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal operating mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    pub fn set_ustp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit resets the USB Host. '0' : Normal operating mode. '1' : USB Host is reset. Notes: - This bit is to it's default value '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit resets the USB Host. '0' : Normal operating mode. '1' : USB Host is reset. Notes: - This bit is to it's default value '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    pub fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for HostCtl1 {
    #[inline(always)]
    fn default() -> HostCtl1 {
        HostCtl1(0)
    }
}
#[doc = "Host Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCtl2(pub u32);
impl HostCtl2 {
    #[doc = "If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed after the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn retry(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed after the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_retry(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
    #[inline(always)]
    pub const fn cancel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
    #[inline(always)]
    pub fn set_cancel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
    #[inline(always)]
    pub const fn sofstep(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
    #[inline(always)]
    pub fn set_sofstep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is only effective when the CLKSEL bit is '0'. If the CLKSEL bit is '1' (Full-Speed mode), SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
    #[inline(always)]
    pub const fn alive(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is only effective when the CLKSEL bit is '0'. If the CLKSEL bit is '1' (Full-Speed mode), SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
    #[inline(always)]
    pub fn set_alive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ttest(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ttest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for HostCtl2 {
    #[inline(always)]
    fn default() -> HostCtl2 {
        HostCtl2(0)
    }
}
#[doc = "Host DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostDmaEnbl(pub u32);
impl HostDmaEnbl {
    #[doc = "This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub const fn dm_ep1drqe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn set_dm_ep1drqe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub const fn dm_ep2drqe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn set_dm_ep2drqe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for HostDmaEnbl {
    #[inline(always)]
    fn default() -> HostDmaEnbl {
        HostDmaEnbl(0)
    }
}
#[doc = "Host EOF Setup Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEof(pub u32);
impl HostEof {
    #[doc = "These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn eof(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_eof(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for HostEof {
    #[inline(always)]
    fn default() -> HostEof {
        HostEof(0)
    }
}
#[doc = "Host Endpoint 1 Block Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp1Blk(pub u32);
impl HostEp1Blk {
    #[doc = "Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
    #[inline(always)]
    pub const fn blk_num(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
    #[inline(always)]
    pub fn set_blk_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HostEp1Blk {
    #[inline(always)]
    fn default() -> HostEp1Blk {
        HostEp1Blk(0)
    }
}
#[doc = "Host Endpoint 1 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp1Ctl(pub u32);
impl HostEp1Ctl {
    #[doc = "This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x100. - If automatic buffer transfer mode (DMAE='1') is used, Endpoint 0,1, or 2 cannot be used,"]
    #[inline(always)]
    pub const fn pks1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x100. - If automatic buffer transfer mode (DMAE='1') is used, Endpoint 0,1, or 2 cannot be used,"]
    #[inline(always)]
    pub fn set_pks1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    pub const fn nulle(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    pub fn set_nulle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the packet transfer mode. '1' : Sets the packet transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS1 bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    pub const fn dmae(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the packet transfer mode. '1' : Sets the packet transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS1 bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    pub fn set_dmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 1 Status Register (HOST_EP1_STATUS) is '1'."]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 1 Status Register (HOST_EP1_STATUS) is '1'."]
    #[inline(always)]
    pub fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP1 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP1DRQ and EP1SPK bits."]
    #[inline(always)]
    pub const fn bfini(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP1 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP1DRQ and EP1SPK bits."]
    #[inline(always)]
    pub fn set_bfini(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for HostEp1Ctl {
    #[inline(always)]
    fn default() -> HostEp1Ctl {
        HostEp1Ctl(0)
    }
}
#[doc = "Host Endpoint 1 Data 1-Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp1Rw1Dr(pub u32);
impl HostEp1Rw1Dr {
    #[doc = "Data Register for EP1 for 1-byte data"]
    #[inline(always)]
    pub const fn bfdt8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP1 for 1-byte data"]
    #[inline(always)]
    pub fn set_bfdt8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for HostEp1Rw1Dr {
    #[inline(always)]
    fn default() -> HostEp1Rw1Dr {
        HostEp1Rw1Dr(0)
    }
}
#[doc = "Host Endpoint 1 Data 2-Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp1Rw2Dr(pub u32);
impl HostEp1Rw2Dr {
    #[doc = "Data Register for EP1 for 2-byte data"]
    #[inline(always)]
    pub const fn bfdt16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP1 for 2-byte data"]
    #[inline(always)]
    pub fn set_bfdt16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HostEp1Rw2Dr {
    #[inline(always)]
    fn default() -> HostEp1Rw2Dr {
        HostEp1Rw2Dr(0)
    }
}
#[doc = "Host Endpoint 1 Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp1Status(pub u32);
impl HostEp1Status {
    #[doc = "These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP1 has finished. The indication range is from 0x000 to 0x100. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
    #[inline(always)]
    pub const fn size1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP1 has finished. The indication range is from 0x000 to 0x100. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
    #[inline(always)]
    pub fn set_size1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This bit shows that there is valid data in the EP1 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
    #[inline(always)]
    pub const fn val_data(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit shows that there is valid data in the EP1 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
    #[inline(always)]
    pub fn set_val_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit shows that EP1 is initialized. If the init bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' and EP1 is initialized, this bit is to '1'. '0' : Not initiatialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately even if BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '0' or '1'. Read this bit to confirm the transition."]
    #[inline(always)]
    pub const fn ini_st(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This bit shows that EP1 is initialized. If the init bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' and EP1 is initialized, this bit is to '1'. '0' : Not initiatialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately even if BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '0' or '1'. Read this bit to confirm the transition."]
    #[inline(always)]
    pub fn set_ini_st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for HostEp1Status {
    #[inline(always)]
    fn default() -> HostEp1Status {
        HostEp1Status(0)
    }
}
#[doc = "Host Endpoint 2 Block Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp2Blk(pub u32);
impl HostEp2Blk {
    #[doc = "Set the total byte number for DMA transfer. If HOST_EP2_RW1_DR or HOST_EP2_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP2DRQE='1')"]
    #[inline(always)]
    pub const fn blk_num(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Set the total byte number for DMA transfer. If HOST_EP2_RW1_DR or HOST_EP2_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP2DRQE='1')"]
    #[inline(always)]
    pub fn set_blk_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HostEp2Blk {
    #[inline(always)]
    fn default() -> HostEp2Blk {
        HostEp2Blk(0)
    }
}
#[doc = "Host Endpoint 2 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp2Ctl(pub u32);
impl HostEp2Ctl {
    #[doc = "This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x40. - If automatic buffer transfer mode (DMAE='1') is used, this Endpoint must not set from 0 to 2."]
    #[inline(always)]
    pub const fn pks2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x40. - If automatic buffer transfer mode (DMAE='1') is used, this Endpoint must not set from 0 to 2."]
    #[inline(always)]
    pub fn set_pks2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "When a data transfer request in the OUT direction transmitted while packet transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    pub const fn nulle(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "When a data transfer request in the OUT direction transmitted while packet transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    pub fn set_nulle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the automatic buffer transfer mode. '1' : Sets the automatic buffer transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    pub const fn dmae(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the automatic buffer transfer mode. '1' : Sets the automatic buffer transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    pub fn set_dmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 2 Status Register (HOST_EP2_STATUS) is '1'."]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 2 Status Register (HOST_EP2_STATUS) is '1'."]
    #[inline(always)]
    pub fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP2 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP2DRQ and EP2SPK bits."]
    #[inline(always)]
    pub const fn bfini(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP2 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP2DRQ and EP2SPK bits."]
    #[inline(always)]
    pub fn set_bfini(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for HostEp2Ctl {
    #[inline(always)]
    fn default() -> HostEp2Ctl {
        HostEp2Ctl(0)
    }
}
#[doc = "Host Endpoint 2 Data 1-Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp2Rw1Dr(pub u32);
impl HostEp2Rw1Dr {
    #[doc = "Data Register for EP2 for 1-byte data."]
    #[inline(always)]
    pub const fn bfdt8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP2 for 1-byte data."]
    #[inline(always)]
    pub fn set_bfdt8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for HostEp2Rw1Dr {
    #[inline(always)]
    fn default() -> HostEp2Rw1Dr {
        HostEp2Rw1Dr(0)
    }
}
#[doc = "Host Endpoint 2 Data 2-Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp2Rw2Dr(pub u32);
impl HostEp2Rw2Dr {
    #[doc = "Data Register for EP2 for 2 byte data."]
    #[inline(always)]
    pub const fn bfdt16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Register for EP2 for 2 byte data."]
    #[inline(always)]
    pub fn set_bfdt16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HostEp2Rw2Dr {
    #[inline(always)]
    fn default() -> HostEp2Rw2Dr {
        HostEp2Rw2Dr(0)
    }
}
#[doc = "Host Endpoint 2 Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostEp2Status(pub u32);
impl HostEp2Status {
    #[doc = "These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP2 has finished. The indication range is from 0x000 to 0x40. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
    #[inline(always)]
    pub const fn size2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP2 has finished. The indication range is from 0x000 to 0x40. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
    #[inline(always)]
    pub fn set_size2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "This bit shows that there is valid data in the EP2 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
    #[inline(always)]
    pub const fn val_data(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit shows that there is valid data in the EP2 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
    #[inline(always)]
    pub fn set_val_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit shows that EP2 is initialized. If the BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' and EP2 is initialized, this bit is to '1'. '0' : Not Initialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately evne if BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '0' or '1'."]
    #[inline(always)]
    pub const fn ini_st(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This bit shows that EP2 is initialized. If the BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' and EP2 is initialized, this bit is to '1'. '0' : Not Initialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately evne if BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '0' or '1'."]
    #[inline(always)]
    pub fn set_ini_st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for HostEp2Status {
    #[inline(always)]
    fn default() -> HostEp2Status {
        HostEp2Status(0)
    }
}
#[doc = "Host Error Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostErr(pub u32);
impl HostErr {
    #[doc = "These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn hs(&self) -> Hs {
        let val = (self.0 >> 0usize) & 0x03;
        Hs::from_bits(val as u8)
    }
    #[doc = "These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_hs(&mut self, val: Hs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn stuff(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_stuff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn tgerr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_tgerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn crc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn tout(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_tout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn rerr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_rerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn lstsof(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_lstsof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for HostErr {
    #[inline(always)]
    fn default() -> HostErr {
        HostErr(0)
    }
}
#[doc = "Host SOF Interrupt Frame Compare Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostFcomp(pub u32);
impl HostFcomp {
    #[doc = "These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn framecomp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_framecomp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for HostFcomp {
    #[inline(always)]
    fn default() -> HostFcomp {
        HostFcomp(0)
    }
}
#[doc = "Host Frame Setup Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostFrame(pub u32);
impl HostFrame {
    #[doc = "These bits are used to specify a frame number of SOF. Notes: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    pub const fn frame(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "These bits are used to specify a frame number of SOF. Notes: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    pub fn set_frame(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for HostFrame {
    #[inline(always)]
    fn default() -> HostFrame {
        HostFrame(0)
    }
}
#[doc = "Host Interrupt Level 1 Selection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostLvl1Sel(pub u32);
impl HostLvl1Sel {
    #[doc = "These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn sofirq_sel(&self) -> SofirqSel {
        let val = (self.0 >> 0usize) & 0x03;
        SofirqSel::from_bits(val as u8)
    }
    #[doc = "These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_sofirq_sel(&mut self, val: SofirqSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "These bits assign DIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn dirq_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "These bits assign DIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_dirq_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn cnnirq_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_cnnirq_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn cmpirq_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_cmpirq_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn urirq_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_urirq_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn rwkirq_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_rwkirq_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_13_12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_13_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "These bits assign TCAN interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn tcan_sel(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "These bits assign TCAN interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_tcan_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for HostLvl1Sel {
    #[inline(always)]
    fn default() -> HostLvl1Sel {
        HostLvl1Sel(0)
    }
}
#[doc = "Host Interrupt Level 2 Selection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostLvl2Sel(pub u32);
impl HostLvl2Sel {
    #[doc = "These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn ep1_drq_sel(&self) -> Ep1DrqSel {
        let val = (self.0 >> 4usize) & 0x03;
        Ep1DrqSel::from_bits(val as u8)
    }
    #[doc = "These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_ep1_drq_sel(&mut self, val: Ep1DrqSel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn ep1_spk_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_ep1_spk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn ep2_drq_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_ep2_drq_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub const fn ep2_spk_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn set_ep2_spk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
}
impl Default for HostLvl2Sel {
    #[inline(always)]
    fn default() -> HostLvl2Sel {
        HostLvl2Sel(0)
    }
}
#[doc = "Host Retry Timer Setup Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostRtimer(pub u32);
impl HostRtimer {
    #[doc = "These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing ends. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    pub const fn rtimer(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing ends. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    pub fn set_rtimer(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
}
impl Default for HostRtimer {
    #[inline(always)]
    fn default() -> HostRtimer {
        HostRtimer(0)
    }
}
#[doc = "Host Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostStatus(pub u32);
impl HostStatus {
    #[doc = "When this bit is '1', it means that the device is connected. When this bit is '0', it means that the device is disconnected. '0' : Device is disconnected. '1' : Device is connected. Notes: - This bit is set to the default value if the RST bit of the Host Control 1 Register (Host_CTL1) is set to '1'. - The transition to disconnected on RST isn't immediate. Read this bit to confirm the transition is complete."]
    #[inline(always)]
    pub const fn cstat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is '1', it means that the device is connected. When this bit is '0', it means that the device is disconnected. '0' : Device is disconnected. '1' : Device is connected. Notes: - This bit is set to the default value if the RST bit of the Host Control 1 Register (Host_CTL1) is set to '1'. - The transition to disconnected on RST isn't immediate. Read this bit to confirm the transition is complete."]
    #[inline(always)]
    pub fn set_cstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If this bit is '1', it means that the device is connected in the full-speed mode. When this bit is '0', it means that the device is connected in the low-speed mode. This bit is valid when the CSTAT bit of the Host Status Register (HOST_STATUS) is '1'. '0' : Low-speed. '1' : Full-speed. Notes: - This bit is set to the default value if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The transition to disconnected on RST isn't immediate. Read this bit to confirm the transition is complete."]
    #[inline(always)]
    pub const fn tmode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is '1', it means that the device is connected in the full-speed mode. When this bit is '0', it means that the device is connected in the low-speed mode. This bit is valid when the CSTAT bit of the Host Status Register (HOST_STATUS) is '1'. '0' : Low-speed. '1' : Full-speed. Notes: - This bit is set to the default value if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The transition to disconnected on RST isn't immediate. Read this bit to confirm the transition is complete."]
    #[inline(always)]
    pub fn set_tmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If this bit is set to '1', the USB Host is placed into the suspend state. If this bit is set to '0' while it is '1' or the USB bus is placed into the k-state mode, then suspend state is released, and the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. Set to '1' : Suspend. Set '0' when this bit is '1' : Resume. Other conditions : Holds the status. Notes: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The transition to disconnected on RST isn't immediate. Read this bit to confirm the transition is complete. - If this bit is set to '1', this bit must not be set to '1' until the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. - Do not set this bit to '1' while the USB is active (during USB bus resetting, data transfer, or SOF timer running). - If the value of this bit is changed, it is not immediately reflected on the state of the USB bus. To check whether or not the state is updated, read this bit."]
    #[inline(always)]
    pub const fn susp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', the USB Host is placed into the suspend state. If this bit is set to '0' while it is '1' or the USB bus is placed into the k-state mode, then suspend state is released, and the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. Set to '1' : Suspend. Set '0' when this bit is '1' : Resume. Other conditions : Holds the status. Notes: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The transition to disconnected on RST isn't immediate. Read this bit to confirm the transition is complete. - If this bit is set to '1', this bit must not be set to '1' until the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. - Do not set this bit to '1' while the USB is active (during USB bus resetting, data transfer, or SOF timer running). - If the value of this bit is changed, it is not immediately reflected on the state of the USB bus. To check whether or not the state is updated, read this bit."]
    #[inline(always)]
    pub fn set_susp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When a SOF token is sent using the Host Token Endpoint Register (HOST_TOKEN), this bit is set to '1', which means that the SOF timer is active. When this bit is '0', it means that the SOF timer is under suspension. To stop the active SOF timer, write '0' to this bit. However, if this bit is written with '1', its value is ignored. '0' : The SOF timer is stopped. '1' : The SOF timer is active. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - The SOF timer does not stop immediately after this bit has been set to '0' to stop the SOF timer. To check whether or not the SOF timer is stopped, read this bit."]
    #[inline(always)]
    pub const fn sofbusy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a SOF token is sent using the Host Token Endpoint Register (HOST_TOKEN), this bit is set to '1', which means that the SOF timer is active. When this bit is '0', it means that the SOF timer is under suspension. To stop the active SOF timer, write '0' to this bit. However, if this bit is written with '1', its value is ignored. '0' : The SOF timer is stopped. '1' : The SOF timer is active. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - The SOF timer does not stop immediately after this bit has been set to '0' to stop the SOF timer. To check whether or not the SOF timer is stopped, read this bit."]
    #[inline(always)]
    pub fn set_sofbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "When this bit is set to '1', the USB bus is reset. This bit remains a '1' during USB bus resetting, and changes to '0' when USB bus resetting is ended. If this bit is set to '0', the USB bus reset is complete"]
    #[inline(always)]
    pub const fn urst(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set to '1', the USB bus is reset. This bit remains a '1' during USB bus resetting, and changes to '0' when USB bus resetting is ended. If this bit is set to '0', the USB bus reset is complete"]
    #[inline(always)]
    pub fn set_urst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit shows that USB Host is being reset internally. If the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. If the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0', this bit is set to '0'. '0' : USB Host isn't being reset. '1' : USB Host is being reset. Notes: - If this bit is '1', the a token must not be executed. - This bit isn't set to '0' or '1' immediately even if the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0' or '1'. Read this bit to confirm the operation is complete."]
    #[inline(always)]
    pub const fn rstbusy(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit shows that USB Host is being reset internally. If the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. If the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0', this bit is set to '0'. '0' : USB Host isn't being reset. '1' : USB Host is being reset. Notes: - If this bit is '1', the a token must not be executed. - This bit isn't set to '0' or '1' immediately even if the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0' or '1'. Read this bit to confirm the operation is complete."]
    #[inline(always)]
    pub fn set_rstbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit shows whether it is full-speed or not. If the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. '0' : Low speed '1' : Full speed Note: - If this bit is different from the CLKSEL bit, The execution of the token and bus reset must wait these bits match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). Read this bit to confirm the operation is complete."]
    #[inline(always)]
    pub const fn clksel_st(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit shows whether it is full-speed or not. If the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. '0' : Low speed '1' : Full speed Note: - If this bit is different from the CLKSEL bit, The execution of the token and bus reset must wait these bits match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). Read this bit to confirm the operation is complete."]
    #[inline(always)]
    pub fn set_clksel_st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit shows whether the device is in USB Host mode. If the HOST bit of the Host Control Register (HOST_CTL0) is set to '1', this bit is set to '1'. '0' : USB Device '1' : USB Host Notes: - If this bit is different from the HOST bit, The execution of a token must wait these bits match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). Read this bit to confirm the operation is complete."]
    #[inline(always)]
    pub const fn host_st(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit shows whether the device is in USB Host mode. If the HOST bit of the Host Control Register (HOST_CTL0) is set to '1', this bit is set to '1'. '0' : USB Device '1' : USB Host Notes: - If this bit is different from the HOST bit, The execution of a token must wait these bits match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). Read this bit to confirm the operation is complete."]
    #[inline(always)]
    pub fn set_host_st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for HostStatus {
    #[inline(always)]
    fn default() -> HostStatus {
        HostStatus(0)
    }
}
#[doc = "Host Token Endpoint Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostToken(pub u32);
impl HostToken {
    #[doc = "These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn endpt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_endpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "These bits send a token according to the current settings. After operation is complete, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Mode should be USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub const fn tknen(&self) -> Tknen {
        let val = (self.0 >> 4usize) & 0x07;
        Tknen::from_bits(val as u8)
    }
    #[doc = "These bits send a token according to the current settings. After operation is complete, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Mode should be USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub fn set_tknen(&mut self, val: Tknen) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't reset to the default value even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
    #[inline(always)]
    pub const fn tggl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't reset to the default value even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
    #[inline(always)]
    pub fn set_tggl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for HostToken {
    #[inline(always)]
    fn default() -> HostToken {
        HostToken(0)
    }
}
#[doc = "High priority interrupt Cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCauseHi(pub u32);
impl IntrCauseHi {
    #[doc = "USB SOF Interrupt"]
    #[inline(always)]
    pub const fn sof_intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB SOF Interrupt"]
    #[inline(always)]
    pub fn set_sof_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BUS RESET Interrupt"]
    #[inline(always)]
    pub const fn bus_reset_intr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BUS RESET Interrupt"]
    #[inline(always)]
    pub fn set_bus_reset_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EP0 Interrupt"]
    #[inline(always)]
    pub const fn ep0_intr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP0 Interrupt"]
    #[inline(always)]
    pub fn set_ep0_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPM Interrupt"]
    #[inline(always)]
    pub const fn lpm_intr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPM Interrupt"]
    #[inline(always)]
    pub fn set_lpm_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub const fn resume_intr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub fn set_resume_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Arbiter Endpoint Interrupt"]
    #[inline(always)]
    pub const fn arb_ep_intr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Arbiter Endpoint Interrupt"]
    #[inline(always)]
    pub fn set_arb_ep_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP1 Interrupt"]
    #[inline(always)]
    pub const fn ep1_intr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP1 Interrupt"]
    #[inline(always)]
    pub fn set_ep1_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP2 Interrupt"]
    #[inline(always)]
    pub const fn ep2_intr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "EP2 Interrupt"]
    #[inline(always)]
    pub fn set_ep2_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "EP3 Interrupt"]
    #[inline(always)]
    pub const fn ep3_intr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "EP3 Interrupt"]
    #[inline(always)]
    pub fn set_ep3_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "EP4 Interrupt"]
    #[inline(always)]
    pub const fn ep4_intr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "EP4 Interrupt"]
    #[inline(always)]
    pub fn set_ep4_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "EP5 Interrupt"]
    #[inline(always)]
    pub const fn ep5_intr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "EP5 Interrupt"]
    #[inline(always)]
    pub fn set_ep5_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EP6 Interrupt"]
    #[inline(always)]
    pub const fn ep6_intr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EP6 Interrupt"]
    #[inline(always)]
    pub fn set_ep6_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EP7 Interrupt"]
    #[inline(always)]
    pub const fn ep7_intr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EP7 Interrupt"]
    #[inline(always)]
    pub fn set_ep7_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "EP8 Interrupt"]
    #[inline(always)]
    pub const fn ep8_intr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "EP8 Interrupt"]
    #[inline(always)]
    pub fn set_ep8_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for IntrCauseHi {
    #[inline(always)]
    fn default() -> IntrCauseHi {
        IntrCauseHi(0)
    }
}
#[doc = "Low priority interrupt Cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCauseLo(pub u32);
impl IntrCauseLo {
    #[doc = "USB SOF Interrupt"]
    #[inline(always)]
    pub const fn sof_intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB SOF Interrupt"]
    #[inline(always)]
    pub fn set_sof_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BUS RESET Interrupt"]
    #[inline(always)]
    pub const fn bus_reset_intr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BUS RESET Interrupt"]
    #[inline(always)]
    pub fn set_bus_reset_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EP0 Interrupt"]
    #[inline(always)]
    pub const fn ep0_intr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP0 Interrupt"]
    #[inline(always)]
    pub fn set_ep0_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPM Interrupt"]
    #[inline(always)]
    pub const fn lpm_intr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPM Interrupt"]
    #[inline(always)]
    pub fn set_lpm_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub const fn resume_intr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub fn set_resume_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Arbiter Endpoint Interrupt"]
    #[inline(always)]
    pub const fn arb_ep_intr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Arbiter Endpoint Interrupt"]
    #[inline(always)]
    pub fn set_arb_ep_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP1 Interrupt"]
    #[inline(always)]
    pub const fn ep1_intr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP1 Interrupt"]
    #[inline(always)]
    pub fn set_ep1_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP2 Interrupt"]
    #[inline(always)]
    pub const fn ep2_intr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "EP2 Interrupt"]
    #[inline(always)]
    pub fn set_ep2_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "EP3 Interrupt"]
    #[inline(always)]
    pub const fn ep3_intr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "EP3 Interrupt"]
    #[inline(always)]
    pub fn set_ep3_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "EP4 Interrupt"]
    #[inline(always)]
    pub const fn ep4_intr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "EP4 Interrupt"]
    #[inline(always)]
    pub fn set_ep4_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "EP5 Interrupt"]
    #[inline(always)]
    pub const fn ep5_intr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "EP5 Interrupt"]
    #[inline(always)]
    pub fn set_ep5_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EP6 Interrupt"]
    #[inline(always)]
    pub const fn ep6_intr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EP6 Interrupt"]
    #[inline(always)]
    pub fn set_ep6_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EP7 Interrupt"]
    #[inline(always)]
    pub const fn ep7_intr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EP7 Interrupt"]
    #[inline(always)]
    pub fn set_ep7_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "EP8 Interrupt"]
    #[inline(always)]
    pub const fn ep8_intr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "EP8 Interrupt"]
    #[inline(always)]
    pub fn set_ep8_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for IntrCauseLo {
    #[inline(always)]
    fn default() -> IntrCauseLo {
        IntrCauseLo(0)
    }
}
#[doc = "Medium priority interrupt Cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCauseMed(pub u32);
impl IntrCauseMed {
    #[doc = "USB SOF Interrupt"]
    #[inline(always)]
    pub const fn sof_intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB SOF Interrupt"]
    #[inline(always)]
    pub fn set_sof_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BUS RESET Interrupt"]
    #[inline(always)]
    pub const fn bus_reset_intr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BUS RESET Interrupt"]
    #[inline(always)]
    pub fn set_bus_reset_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "EP0 Interrupt"]
    #[inline(always)]
    pub const fn ep0_intr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP0 Interrupt"]
    #[inline(always)]
    pub fn set_ep0_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPM Interrupt"]
    #[inline(always)]
    pub const fn lpm_intr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPM Interrupt"]
    #[inline(always)]
    pub fn set_lpm_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub const fn resume_intr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub fn set_resume_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Arbiter Endpoint Interrupt"]
    #[inline(always)]
    pub const fn arb_ep_intr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Arbiter Endpoint Interrupt"]
    #[inline(always)]
    pub fn set_arb_ep_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP1 Interrupt"]
    #[inline(always)]
    pub const fn ep1_intr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP1 Interrupt"]
    #[inline(always)]
    pub fn set_ep1_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP2 Interrupt"]
    #[inline(always)]
    pub const fn ep2_intr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "EP2 Interrupt"]
    #[inline(always)]
    pub fn set_ep2_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "EP3 Interrupt"]
    #[inline(always)]
    pub const fn ep3_intr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "EP3 Interrupt"]
    #[inline(always)]
    pub fn set_ep3_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "EP4 Interrupt"]
    #[inline(always)]
    pub const fn ep4_intr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "EP4 Interrupt"]
    #[inline(always)]
    pub fn set_ep4_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "EP5 Interrupt"]
    #[inline(always)]
    pub const fn ep5_intr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "EP5 Interrupt"]
    #[inline(always)]
    pub fn set_ep5_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EP6 Interrupt"]
    #[inline(always)]
    pub const fn ep6_intr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EP6 Interrupt"]
    #[inline(always)]
    pub fn set_ep6_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EP7 Interrupt"]
    #[inline(always)]
    pub const fn ep7_intr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EP7 Interrupt"]
    #[inline(always)]
    pub fn set_ep7_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "EP8 Interrupt"]
    #[inline(always)]
    pub const fn ep8_intr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "EP8 Interrupt"]
    #[inline(always)]
    pub fn set_ep8_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for IntrCauseMed {
    #[inline(always)]
    fn default() -> IntrCauseMed {
        IntrCauseMed(0)
    }
}
#[doc = "Interrupt USB Host Endpoint Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrHostEp(pub u32);
impl IntrHostEp {
    #[doc = "This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub const fn ep1drq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn set_ep1drq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The EP1SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub const fn ep1spk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The EP1SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn set_ep1spk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If packet transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub const fn ep2drq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If packet transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn set_ep2drq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS1 in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub const fn ep2spk(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS1 in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn set_ep2spk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrHostEp {
    #[inline(always)]
    fn default() -> IntrHostEp {
        IntrHostEp(0)
    }
}
#[doc = "Interrupt USB Host Endpoint Cause High Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrHostEpCauseHi(pub u32);
impl IntrHostEpCauseHi {
    #[doc = "EP1DRQ interrupt"]
    #[inline(always)]
    pub const fn ep1drq_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP1DRQ interrupt"]
    #[inline(always)]
    pub fn set_ep1drq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EP1SPK interrupt"]
    #[inline(always)]
    pub const fn ep1spk_int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EP1SPK interrupt"]
    #[inline(always)]
    pub fn set_ep1spk_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EP2DRQ interrupt"]
    #[inline(always)]
    pub const fn ep2drq_int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EP2DRQ interrupt"]
    #[inline(always)]
    pub fn set_ep2drq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EP2SPK interrupt"]
    #[inline(always)]
    pub const fn ep2spk_int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "EP2SPK interrupt"]
    #[inline(always)]
    pub fn set_ep2spk_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrHostEpCauseHi {
    #[inline(always)]
    fn default() -> IntrHostEpCauseHi {
        IntrHostEpCauseHi(0)
    }
}
#[doc = "Interrupt USB Host Endpoint Cause Low Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrHostEpCauseLo(pub u32);
impl IntrHostEpCauseLo {
    #[doc = "EP1DRQ interrupt"]
    #[inline(always)]
    pub const fn ep1drq_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP1DRQ interrupt"]
    #[inline(always)]
    pub fn set_ep1drq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EP1SPK interrupt"]
    #[inline(always)]
    pub const fn ep1spk_int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EP1SPK interrupt"]
    #[inline(always)]
    pub fn set_ep1spk_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EP2DRQ interrupt"]
    #[inline(always)]
    pub const fn ep2drq_int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EP2DRQ interrupt"]
    #[inline(always)]
    pub fn set_ep2drq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EP2SPK interrupt"]
    #[inline(always)]
    pub const fn ep2spk_int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "EP2SPK interrupt"]
    #[inline(always)]
    pub fn set_ep2spk_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrHostEpCauseLo {
    #[inline(always)]
    fn default() -> IntrHostEpCauseLo {
        IntrHostEpCauseLo(0)
    }
}
#[doc = "Interrupt USB Host Endpoint Cause Medium Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrHostEpCauseMed(pub u32);
impl IntrHostEpCauseMed {
    #[doc = "EP1DRQ interrupt"]
    #[inline(always)]
    pub const fn ep1drq_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP1DRQ interrupt"]
    #[inline(always)]
    pub fn set_ep1drq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EP1SPK interrupt"]
    #[inline(always)]
    pub const fn ep1spk_int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EP1SPK interrupt"]
    #[inline(always)]
    pub fn set_ep1spk_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "EP2DRQ interrupt"]
    #[inline(always)]
    pub const fn ep2drq_int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EP2DRQ interrupt"]
    #[inline(always)]
    pub fn set_ep2drq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "EP2SPK interrupt"]
    #[inline(always)]
    pub const fn ep2spk_int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "EP2SPK interrupt"]
    #[inline(always)]
    pub fn set_ep2spk_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrHostEpCauseMed {
    #[inline(always)]
    fn default() -> IntrHostEpCauseMed {
        IntrHostEpCauseMed(0)
    }
}
#[doc = "Interrupt USB Host Endpoint Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrHostEpMask(pub u32);
impl IntrHostEpMask {
    #[doc = "This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn ep1drqm(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_ep1drqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn ep1spkm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_ep1spkm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn ep2drqm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_ep2drqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn ep2spkm(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_ep2spkm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrHostEpMask {
    #[inline(always)]
    fn default() -> IntrHostEpMask {
        IntrHostEpMask(0)
    }
}
#[doc = "Interrupt USB Host Endpoint Masked Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrHostEpMasked(pub u32);
impl IntrHostEpMasked {
    #[doc = "This bit indicates the interrupt by EP1DRQ flag. '0' : Doesn't request the interrupt by EP1DRQ '1' : Request the interrupt by EP1DRQ"]
    #[inline(always)]
    pub const fn ep1drqed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by EP1DRQ flag. '0' : Doesn't request the interrupt by EP1DRQ '1' : Request the interrupt by EP1DRQ"]
    #[inline(always)]
    pub fn set_ep1drqed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates the interrupt by EP1SPK flag. '0' : Doesn't request the interrupt by EP1SPK '1' : Request the interrupt by EP1SPK"]
    #[inline(always)]
    pub const fn ep1spked(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by EP1SPK flag. '0' : Doesn't request the interrupt by EP1SPK '1' : Request the interrupt by EP1SPK"]
    #[inline(always)]
    pub fn set_ep1spked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates the interrupt by EP2DRQ flag. '0' : Doesn't request the interrupt by EP2DRQ '1' : Request the interrupt by EP2DRQ"]
    #[inline(always)]
    pub const fn ep2drqed(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by EP2DRQ flag. '0' : Doesn't request the interrupt by EP2DRQ '1' : Request the interrupt by EP2DRQ"]
    #[inline(always)]
    pub fn set_ep2drqed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates the interrupt by EP2SPK flag. '0' : Doesn't request the interrupt by EP2SPK '1' : Request the interrupt by EP2SPK"]
    #[inline(always)]
    pub const fn ep2spked(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by EP2SPK flag. '0' : Doesn't request the interrupt by EP2SPK '1' : Request the interrupt by EP2SPK"]
    #[inline(always)]
    pub fn set_ep2spked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrHostEpMasked {
    #[inline(always)]
    fn default() -> IntrHostEpMasked {
        IntrHostEpMasked(0)
    }
}
#[doc = "Interrupt USB Host Endpoint Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrHostEpSet(pub u32);
impl IntrHostEpSet {
    #[doc = "This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    pub const fn ep1drqs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn set_ep1drqs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    pub const fn ep1spks(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    pub fn set_ep1spks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    pub const fn ep2drqs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn set_ep2drqs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    pub const fn ep2spks(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    pub fn set_ep2spks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrHostEpSet {
    #[inline(always)]
    fn default() -> IntrHostEpSet {
        IntrHostEpSet(0)
    }
}
#[doc = "Select interrupt level for each interrupt source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrLvlSel(pub u32);
impl IntrLvlSel {
    #[doc = "USB SOF Interrupt level select"]
    #[inline(always)]
    pub const fn sof_lvl_sel(&self) -> SofLvlSel {
        let val = (self.0 >> 0usize) & 0x03;
        SofLvlSel::from_bits(val as u8)
    }
    #[doc = "USB SOF Interrupt level select"]
    #[inline(always)]
    pub fn set_sof_lvl_sel(&mut self, val: SofLvlSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "BUS RESET Interrupt level select"]
    #[inline(always)]
    pub const fn bus_reset_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "BUS RESET Interrupt level select"]
    #[inline(always)]
    pub fn set_bus_reset_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "EP0 Interrupt level select"]
    #[inline(always)]
    pub const fn ep0_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "EP0 Interrupt level select"]
    #[inline(always)]
    pub fn set_ep0_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "LPM Interrupt level select"]
    #[inline(always)]
    pub const fn lpm_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "LPM Interrupt level select"]
    #[inline(always)]
    pub fn set_lpm_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Resume Interrupt level select"]
    #[inline(always)]
    pub const fn resume_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Resume Interrupt level select"]
    #[inline(always)]
    pub fn set_resume_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    pub const fn arb_ep_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    pub fn set_arb_ep_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "EP1 Interrupt level select"]
    #[inline(always)]
    pub const fn ep1_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "EP1 Interrupt level select"]
    #[inline(always)]
    pub fn set_ep1_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "EP2 Interrupt level select"]
    #[inline(always)]
    pub const fn ep2_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "EP2 Interrupt level select"]
    #[inline(always)]
    pub fn set_ep2_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "EP3 Interrupt level select"]
    #[inline(always)]
    pub const fn ep3_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "EP3 Interrupt level select"]
    #[inline(always)]
    pub fn set_ep3_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "EP4 Interrupt level select"]
    #[inline(always)]
    pub const fn ep4_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "EP4 Interrupt level select"]
    #[inline(always)]
    pub fn set_ep4_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "EP5 Interrupt level select"]
    #[inline(always)]
    pub const fn ep5_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "EP5 Interrupt level select"]
    #[inline(always)]
    pub fn set_ep5_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "EP6 Interrupt level select"]
    #[inline(always)]
    pub const fn ep6_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "EP6 Interrupt level select"]
    #[inline(always)]
    pub fn set_ep6_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "EP7 Interrupt level select"]
    #[inline(always)]
    pub const fn ep7_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "EP7 Interrupt level select"]
    #[inline(always)]
    pub fn set_ep7_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "EP8 Interrupt level select"]
    #[inline(always)]
    pub const fn ep8_lvl_sel(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "EP8 Interrupt level select"]
    #[inline(always)]
    pub fn set_ep8_lvl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for IntrLvlSel {
    #[inline(always)]
    fn default() -> IntrLvlSel {
        IntrLvlSel(0)
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSie(pub u32);
impl IntrSie {
    #[doc = "Interrupt status for USB SOF"]
    #[inline(always)]
    pub const fn sof_intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for USB SOF"]
    #[inline(always)]
    pub fn set_sof_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt status for BUS RESET"]
    #[inline(always)]
    pub const fn bus_reset_intr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for BUS RESET"]
    #[inline(always)]
    pub fn set_bus_reset_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt status for EP0"]
    #[inline(always)]
    pub const fn ep0_intr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP0"]
    #[inline(always)]
    pub fn set_ep0_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    pub const fn lpm_intr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    pub fn set_lpm_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt status for Resume"]
    #[inline(always)]
    pub const fn resume_intr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for Resume"]
    #[inline(always)]
    pub fn set_resume_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for IntrSie {
    #[inline(always)]
    fn default() -> IntrSie {
        IntrSie(0)
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSieMask(pub u32);
impl IntrSieMask {
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub const fn sof_intr_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn set_sof_intr_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub const fn bus_reset_intr_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn set_bus_reset_intr_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub const fn ep0_intr_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn set_ep0_intr_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub const fn lpm_intr_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn set_lpm_intr_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub const fn resume_intr_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn set_resume_intr_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for IntrSieMask {
    #[inline(always)]
    fn default() -> IntrSieMask {
        IntrSieMask(0)
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Masked"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSieMasked(pub u32);
impl IntrSieMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn sof_intr_masked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_sof_intr_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn bus_reset_intr_masked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_bus_reset_intr_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn ep0_intr_masked(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_ep0_intr_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn lpm_intr_masked(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_lpm_intr_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn resume_intr_masked(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_resume_intr_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for IntrSieMasked {
    #[inline(always)]
    fn default() -> IntrSieMasked {
        IntrSieMasked(0)
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSieSet(pub u32);
impl IntrSieSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn sof_intr_set(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_sof_intr_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn bus_reset_intr_set(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_bus_reset_intr_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn ep0_intr_set(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_ep0_intr_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn lpm_intr_set(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_lpm_intr_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn resume_intr_set(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_resume_intr_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for IntrSieSet {
    #[inline(always)]
    fn default() -> IntrSieSet {
        IntrSieSet(0)
    }
}
#[doc = "Interrupt USB Host Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrUsbhost(pub u32);
impl IntrUsbhost {
    #[doc = "If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn sofirq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_sofirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn dirq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_dirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn cnnirq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_cnnirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub const fn cmpirq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub fn set_cmpirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn urirq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_urirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn rwkirq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. Write '1' to clear, a write of '0' is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_rwkirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. Write '1' to clear, a write of '0' is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub const fn tcan(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. Write '1' to clear, a write of '0' is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn set_tcan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrUsbhost {
    #[inline(always)]
    fn default() -> IntrUsbhost {
        IntrUsbhost(0)
    }
}
#[doc = "Interrupt USB Host Cause High Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrUsbhostCauseHi(pub u32);
impl IntrUsbhostCauseHi {
    #[doc = "SOFIRQ interrupt"]
    #[inline(always)]
    pub const fn sofirq_int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SOFIRQ interrupt"]
    #[inline(always)]
    pub fn set_sofirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DIRQ interrupt"]
    #[inline(always)]
    pub const fn dirq_int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DIRQ interrupt"]
    #[inline(always)]
    pub fn set_dirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CNNIRQ interrupt"]
    #[inline(always)]
    pub const fn cnnirq_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CNNIRQ interrupt"]
    #[inline(always)]
    pub fn set_cnnirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMPIRQ interrupt"]
    #[inline(always)]
    pub const fn cmpirq_int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CMPIRQ interrupt"]
    #[inline(always)]
    pub fn set_cmpirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "URIRQ interrupt"]
    #[inline(always)]
    pub const fn urirq_int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "URIRQ interrupt"]
    #[inline(always)]
    pub fn set_urirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RWKIRQ interrupt"]
    #[inline(always)]
    pub const fn rwkirq_int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RWKIRQ interrupt"]
    #[inline(always)]
    pub fn set_rwkirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TCAN interrupt"]
    #[inline(always)]
    pub const fn tcan_int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TCAN interrupt"]
    #[inline(always)]
    pub fn set_tcan_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrUsbhostCauseHi {
    #[inline(always)]
    fn default() -> IntrUsbhostCauseHi {
        IntrUsbhostCauseHi(0)
    }
}
#[doc = "Interrupt USB Host Cause Low Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrUsbhostCauseLo(pub u32);
impl IntrUsbhostCauseLo {
    #[doc = "SOFIRQ interrupt"]
    #[inline(always)]
    pub const fn sofirq_int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SOFIRQ interrupt"]
    #[inline(always)]
    pub fn set_sofirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DIRQ interrupt"]
    #[inline(always)]
    pub const fn dirq_int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DIRQ interrupt"]
    #[inline(always)]
    pub fn set_dirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CNNIRQ interrupt"]
    #[inline(always)]
    pub const fn cnnirq_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CNNIRQ interrupt"]
    #[inline(always)]
    pub fn set_cnnirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMPIRQ interrupt"]
    #[inline(always)]
    pub const fn cmpirq_int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CMPIRQ interrupt"]
    #[inline(always)]
    pub fn set_cmpirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "URIRQ interrupt"]
    #[inline(always)]
    pub const fn urirq_int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "URIRQ interrupt"]
    #[inline(always)]
    pub fn set_urirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RWKIRQ interrupt"]
    #[inline(always)]
    pub const fn rwkirq_int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RWKIRQ interrupt"]
    #[inline(always)]
    pub fn set_rwkirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TCAN interrupt"]
    #[inline(always)]
    pub const fn tcan_int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TCAN interrupt"]
    #[inline(always)]
    pub fn set_tcan_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrUsbhostCauseLo {
    #[inline(always)]
    fn default() -> IntrUsbhostCauseLo {
        IntrUsbhostCauseLo(0)
    }
}
#[doc = "Interrupt USB Host Cause Medium Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrUsbhostCauseMed(pub u32);
impl IntrUsbhostCauseMed {
    #[doc = "SOFIRQ interrupt"]
    #[inline(always)]
    pub const fn sofirq_int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SOFIRQ interrupt"]
    #[inline(always)]
    pub fn set_sofirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DIRQ interrupt"]
    #[inline(always)]
    pub const fn dirq_int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DIRQ interrupt"]
    #[inline(always)]
    pub fn set_dirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CNNIRQ interrupt"]
    #[inline(always)]
    pub const fn cnnirq_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CNNIRQ interrupt"]
    #[inline(always)]
    pub fn set_cnnirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMPIRQ interrupt"]
    #[inline(always)]
    pub const fn cmpirq_int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CMPIRQ interrupt"]
    #[inline(always)]
    pub fn set_cmpirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "URIRQ interrupt"]
    #[inline(always)]
    pub const fn urirq_int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "URIRQ interrupt"]
    #[inline(always)]
    pub fn set_urirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RWKIRQ interrupt"]
    #[inline(always)]
    pub const fn rwkirq_int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RWKIRQ interrupt"]
    #[inline(always)]
    pub fn set_rwkirq_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TCAN interrupt"]
    #[inline(always)]
    pub const fn tcan_int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TCAN interrupt"]
    #[inline(always)]
    pub fn set_tcan_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrUsbhostCauseMed {
    #[inline(always)]
    fn default() -> IntrUsbhostCauseMed {
        IntrUsbhostCauseMed(0)
    }
}
#[doc = "Interrupt USB Host Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrUsbhostMask(pub u32);
impl IntrUsbhostMask {
    #[doc = "This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn sofirqm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_sofirqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn dirqm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_dirqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn cnnirqm(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_cnnirqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn cmpirqm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_cmpirqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn urirqm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_urirqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn rwkirqm(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_rwkirqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub const fn tcanm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn set_tcanm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrUsbhostMask {
    #[inline(always)]
    fn default() -> IntrUsbhostMask {
        IntrUsbhostMask(0)
    }
}
#[doc = "Interrupt USB Host Masked Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrUsbhostMasked(pub u32);
impl IntrUsbhostMasked {
    #[doc = "This bit indicates the interrupt by SOF flag. '0' : Doesn't request the interrupt by SOF '1' : Request the interrupt by SOF"]
    #[inline(always)]
    pub const fn sofirqed(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by SOF flag. '0' : Doesn't request the interrupt by SOF '1' : Request the interrupt by SOF"]
    #[inline(always)]
    pub fn set_sofirqed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit indicates the interrupt by DIRQ flag. '0' : Doesn't request the interrupt by DIRQ '1' : Request the interrupt by DIRQ"]
    #[inline(always)]
    pub const fn dirqed(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by DIRQ flag. '0' : Doesn't request the interrupt by DIRQ '1' : Request the interrupt by DIRQ"]
    #[inline(always)]
    pub fn set_dirqed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit indicates the interrupt by CNNIRQ flag. '0' : Doesn't request the interrupt by CNNIRQ '1' : Request the interrupt by CNNIRQ"]
    #[inline(always)]
    pub const fn cnnirqed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by CNNIRQ flag. '0' : Doesn't request the interrupt by CNNIRQ '1' : Request the interrupt by CNNIRQ"]
    #[inline(always)]
    pub fn set_cnnirqed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates the interrupt by CMPIRQ flag. '0' : Doesn't request the interrupt by CMPIRQ '1' : Request the interrupt by CMPIRQ"]
    #[inline(always)]
    pub const fn cmpirqed(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by CMPIRQ flag. '0' : Doesn't request the interrupt by CMPIRQ '1' : Request the interrupt by CMPIRQ"]
    #[inline(always)]
    pub fn set_cmpirqed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit indicates the interrupt by URIRQ flag. '0' : Doesn't request the interrupt by URIRQ '1' : Request the interrupt by URIRQ"]
    #[inline(always)]
    pub const fn urirqed(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by URIRQ flag. '0' : Doesn't request the interrupt by URIRQ '1' : Request the interrupt by URIRQ"]
    #[inline(always)]
    pub fn set_urirqed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit indicates the interrupt by RWKIRQ flag. '0' : Doesn't request the interrupt by RWKIRQ '1' : Request the interrupt by RWKIRQ"]
    #[inline(always)]
    pub const fn rwkirqed(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by RWKIRQ flag. '0' : Doesn't request the interrupt by RWKIRQ '1' : Request the interrupt by RWKIRQ"]
    #[inline(always)]
    pub fn set_rwkirqed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit indicates the interrupt by TCAN flag. '0' : Doesn't request the interrupt by TCAN '1' : Request the interrupt by TCAN"]
    #[inline(always)]
    pub const fn tcaned(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the interrupt by TCAN flag. '0' : Doesn't request the interrupt by TCAN '1' : Request the interrupt by TCAN"]
    #[inline(always)]
    pub fn set_tcaned(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrUsbhostMasked {
    #[inline(always)]
    fn default() -> IntrUsbhostMasked {
        IntrUsbhostMasked(0)
    }
}
#[doc = "Interrupt USB Host Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrUsbhostSet(pub u32);
impl IntrUsbhostSet {
    #[doc = "This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub const fn sofirqs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn set_sofirqs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub const fn dirqs(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn set_dirqs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub const fn cnnirqs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn set_cnnirqs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub const fn cmpirqs(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn set_cmpirqs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub const fn urirqs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn set_urirqs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub const fn rwkirqs(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn set_rwkirqs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub const fn tcans(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn set_tcans(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrUsbhostSet {
    #[inline(always)]
    fn default() -> IntrUsbhostSet {
        IntrUsbhostSet(0)
    }
}
#[doc = "LPM Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpmCtl(pub u32);
impl LpmCtl {
    #[doc = "LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    pub const fn lpm_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    pub fn set_lpm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    pub const fn lpm_ack_resp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    pub fn set_lpm_ack_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    pub const fn nyet_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    pub fn set_nyet_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    pub const fn sub_resp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    pub fn set_sub_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for LpmCtl {
    #[inline(always)]
    fn default() -> LpmCtl {
        LpmCtl(0)
    }
}
#[doc = "LPM Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpmStat(pub u32);
impl LpmStat {
    #[doc = "Best Effort Service Latency This value should match either the Baseline (DeepSleep) or Deep (Hibernate) BESL in the BOS descriptor."]
    #[inline(always)]
    pub const fn lpm_besl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Best Effort Service Latency This value should match either the Baseline (DeepSleep) or Deep (Hibernate) BESL in the BOS descriptor."]
    #[inline(always)]
    pub fn set_lpm_besl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "0: Device is prohibited from initiating a remote wake 1: Device is allow to wake the host"]
    #[inline(always)]
    pub const fn lpm_remotewake(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0: Device is prohibited from initiating a remote wake 1: Device is allow to wake the host"]
    #[inline(always)]
    pub fn set_lpm_remotewake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for LpmStat {
    #[inline(always)]
    fn default() -> LpmStat {
        LpmStat(0)
    }
}
#[doc = "DATA"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemData(pub u32);
impl MemData {
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for MemData {
    #[inline(always)]
    fn default() -> MemData {
        MemData(0)
    }
}
#[doc = "Oscillator lock data register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OsclkDr0(pub u32);
impl OsclkDr0 {
    #[doc = "These bits return the lower 8 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub const fn adder(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits return the lower 8 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn set_adder(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for OsclkDr0 {
    #[inline(always)]
    fn default() -> OsclkDr0 {
        OsclkDr0(0)
    }
}
#[doc = "Oscillator lock data register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OsclkDr1(pub u32);
impl OsclkDr1 {
    #[doc = "These bits return the upper 7 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub const fn adder_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "These bits return the upper 7 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn set_adder_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for OsclkDr1 {
    #[inline(always)]
    fn default() -> OsclkDr1 {
        OsclkDr1(0)
    }
}
#[doc = "Oscillator lock data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OsclkDr16(pub u32);
impl OsclkDr16 {
    #[doc = "These bits return the oscillator locking circuits adder output."]
    #[inline(always)]
    pub const fn adder16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "These bits return the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn set_adder16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for OsclkDr16 {
    #[inline(always)]
    fn default() -> OsclkDr16 {
        OsclkDr16(0)
    }
}
#[doc = "Power Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PowerCtl(pub u32);
impl PowerCtl {
    #[doc = "Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    pub const fn suspend(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    pub fn set_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub const fn dp_up_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn set_dp_up_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
    #[inline(always)]
    pub const fn dp_big(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
    #[inline(always)]
    pub fn set_dp_big(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub const fn dp_down_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn set_dp_down_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub const fn dm_up_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn set_dm_up_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
    #[inline(always)]
    pub const fn dm_big(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
    #[inline(always)]
    pub fn set_dm_big(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub const fn dm_down_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn set_dm_down_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the single ended receiver on D+."]
    #[inline(always)]
    pub const fn enable_dpo(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the single ended receiver on D+."]
    #[inline(always)]
    pub fn set_enable_dpo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enables the signle ended receiver on D-."]
    #[inline(always)]
    pub const fn enable_dmo(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the signle ended receiver on D-."]
    #[inline(always)]
    pub fn set_enable_dmo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for PowerCtl {
    #[inline(always)]
    fn default() -> PowerCtl {
        PowerCtl(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp1Cnt0(pub u32);
impl SieEp1Cnt0 {
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub const fn data_count_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn set_data_count_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub const fn data_valid(&self) -> SieEp1Cnt0DataValid {
        let val = (self.0 >> 6usize) & 0x01;
        SieEp1Cnt0DataValid::from_bits(val as u8)
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn set_data_valid(&mut self, val: SieEp1Cnt0DataValid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub const fn data_toggle(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn set_data_toggle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp1Cnt0 {
    #[inline(always)]
    fn default() -> SieEp1Cnt0 {
        SieEp1Cnt0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp1Cnt1(pub u32);
impl SieEp1Cnt1 {
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub const fn data_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn set_data_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SieEp1Cnt1 {
    #[inline(always)]
    fn default() -> SieEp1Cnt1 {
        SieEp1Cnt1(0)
    }
}
#[doc = "Non-control endpoint's control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp1Cr0(pub u32);
impl SieEp1Cr0 {
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub const fn mode(&self) -> SieEp1Cr0Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        SieEp1Cr0Mode::from_bits(val as u8)
    }
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: SieEp1Cr0Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn acked_txn(&self) -> SieEp1Cr0AckedTxn {
        let val = (self.0 >> 4usize) & 0x01;
        SieEp1Cr0AckedTxn::from_bits(val as u8)
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_acked_txn(&mut self, val: SieEp1Cr0AckedTxn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub const fn nak_int_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn set_nak_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn err_in_txn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_err_in_txn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp1Cr0 {
    #[inline(always)]
    fn default() -> SieEp1Cr0 {
        SieEp1Cr0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp2Cnt0(pub u32);
impl SieEp2Cnt0 {
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub const fn data_count_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn set_data_count_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub const fn data_valid(&self) -> SieEp2Cnt0DataValid {
        let val = (self.0 >> 6usize) & 0x01;
        SieEp2Cnt0DataValid::from_bits(val as u8)
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn set_data_valid(&mut self, val: SieEp2Cnt0DataValid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub const fn data_toggle(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn set_data_toggle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp2Cnt0 {
    #[inline(always)]
    fn default() -> SieEp2Cnt0 {
        SieEp2Cnt0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp2Cnt1(pub u32);
impl SieEp2Cnt1 {
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub const fn data_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn set_data_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SieEp2Cnt1 {
    #[inline(always)]
    fn default() -> SieEp2Cnt1 {
        SieEp2Cnt1(0)
    }
}
#[doc = "Non-control endpoint's control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp2Cr0(pub u32);
impl SieEp2Cr0 {
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub const fn mode(&self) -> SieEp2Cr0Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        SieEp2Cr0Mode::from_bits(val as u8)
    }
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: SieEp2Cr0Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn acked_txn(&self) -> SieEp2Cr0AckedTxn {
        let val = (self.0 >> 4usize) & 0x01;
        SieEp2Cr0AckedTxn::from_bits(val as u8)
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_acked_txn(&mut self, val: SieEp2Cr0AckedTxn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub const fn nak_int_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn set_nak_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn err_in_txn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_err_in_txn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp2Cr0 {
    #[inline(always)]
    fn default() -> SieEp2Cr0 {
        SieEp2Cr0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp3Cnt0(pub u32);
impl SieEp3Cnt0 {
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub const fn data_count_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn set_data_count_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub const fn data_valid(&self) -> SieEp3Cnt0DataValid {
        let val = (self.0 >> 6usize) & 0x01;
        SieEp3Cnt0DataValid::from_bits(val as u8)
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn set_data_valid(&mut self, val: SieEp3Cnt0DataValid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub const fn data_toggle(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn set_data_toggle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp3Cnt0 {
    #[inline(always)]
    fn default() -> SieEp3Cnt0 {
        SieEp3Cnt0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp3Cnt1(pub u32);
impl SieEp3Cnt1 {
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub const fn data_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn set_data_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SieEp3Cnt1 {
    #[inline(always)]
    fn default() -> SieEp3Cnt1 {
        SieEp3Cnt1(0)
    }
}
#[doc = "Non-control endpoint's control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp3Cr0(pub u32);
impl SieEp3Cr0 {
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub const fn mode(&self) -> SieEp3Cr0Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        SieEp3Cr0Mode::from_bits(val as u8)
    }
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: SieEp3Cr0Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn acked_txn(&self) -> SieEp3Cr0AckedTxn {
        let val = (self.0 >> 4usize) & 0x01;
        SieEp3Cr0AckedTxn::from_bits(val as u8)
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_acked_txn(&mut self, val: SieEp3Cr0AckedTxn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub const fn nak_int_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn set_nak_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn err_in_txn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_err_in_txn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp3Cr0 {
    #[inline(always)]
    fn default() -> SieEp3Cr0 {
        SieEp3Cr0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp4Cnt0(pub u32);
impl SieEp4Cnt0 {
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub const fn data_count_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn set_data_count_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub const fn data_valid(&self) -> SieEp4Cnt0DataValid {
        let val = (self.0 >> 6usize) & 0x01;
        SieEp4Cnt0DataValid::from_bits(val as u8)
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn set_data_valid(&mut self, val: SieEp4Cnt0DataValid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub const fn data_toggle(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn set_data_toggle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp4Cnt0 {
    #[inline(always)]
    fn default() -> SieEp4Cnt0 {
        SieEp4Cnt0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp4Cnt1(pub u32);
impl SieEp4Cnt1 {
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub const fn data_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn set_data_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SieEp4Cnt1 {
    #[inline(always)]
    fn default() -> SieEp4Cnt1 {
        SieEp4Cnt1(0)
    }
}
#[doc = "Non-control endpoint's control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp4Cr0(pub u32);
impl SieEp4Cr0 {
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub const fn mode(&self) -> SieEp4Cr0Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        SieEp4Cr0Mode::from_bits(val as u8)
    }
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: SieEp4Cr0Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn acked_txn(&self) -> SieEp4Cr0AckedTxn {
        let val = (self.0 >> 4usize) & 0x01;
        SieEp4Cr0AckedTxn::from_bits(val as u8)
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_acked_txn(&mut self, val: SieEp4Cr0AckedTxn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub const fn nak_int_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn set_nak_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn err_in_txn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_err_in_txn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp4Cr0 {
    #[inline(always)]
    fn default() -> SieEp4Cr0 {
        SieEp4Cr0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp5Cnt0(pub u32);
impl SieEp5Cnt0 {
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub const fn data_count_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn set_data_count_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub const fn data_valid(&self) -> SieEp5Cnt0DataValid {
        let val = (self.0 >> 6usize) & 0x01;
        SieEp5Cnt0DataValid::from_bits(val as u8)
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn set_data_valid(&mut self, val: SieEp5Cnt0DataValid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub const fn data_toggle(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn set_data_toggle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp5Cnt0 {
    #[inline(always)]
    fn default() -> SieEp5Cnt0 {
        SieEp5Cnt0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp5Cnt1(pub u32);
impl SieEp5Cnt1 {
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub const fn data_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn set_data_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SieEp5Cnt1 {
    #[inline(always)]
    fn default() -> SieEp5Cnt1 {
        SieEp5Cnt1(0)
    }
}
#[doc = "Non-control endpoint's control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp5Cr0(pub u32);
impl SieEp5Cr0 {
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub const fn mode(&self) -> SieEp5Cr0Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        SieEp5Cr0Mode::from_bits(val as u8)
    }
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: SieEp5Cr0Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn acked_txn(&self) -> SieEp5Cr0AckedTxn {
        let val = (self.0 >> 4usize) & 0x01;
        SieEp5Cr0AckedTxn::from_bits(val as u8)
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_acked_txn(&mut self, val: SieEp5Cr0AckedTxn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub const fn nak_int_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn set_nak_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn err_in_txn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_err_in_txn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp5Cr0 {
    #[inline(always)]
    fn default() -> SieEp5Cr0 {
        SieEp5Cr0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp6Cnt0(pub u32);
impl SieEp6Cnt0 {
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub const fn data_count_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn set_data_count_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub const fn data_valid(&self) -> SieEp6Cnt0DataValid {
        let val = (self.0 >> 6usize) & 0x01;
        SieEp6Cnt0DataValid::from_bits(val as u8)
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn set_data_valid(&mut self, val: SieEp6Cnt0DataValid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub const fn data_toggle(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn set_data_toggle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp6Cnt0 {
    #[inline(always)]
    fn default() -> SieEp6Cnt0 {
        SieEp6Cnt0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp6Cnt1(pub u32);
impl SieEp6Cnt1 {
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub const fn data_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn set_data_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SieEp6Cnt1 {
    #[inline(always)]
    fn default() -> SieEp6Cnt1 {
        SieEp6Cnt1(0)
    }
}
#[doc = "Non-control endpoint's control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp6Cr0(pub u32);
impl SieEp6Cr0 {
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub const fn mode(&self) -> SieEp6Cr0Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        SieEp6Cr0Mode::from_bits(val as u8)
    }
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: SieEp6Cr0Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn acked_txn(&self) -> SieEp6Cr0AckedTxn {
        let val = (self.0 >> 4usize) & 0x01;
        SieEp6Cr0AckedTxn::from_bits(val as u8)
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_acked_txn(&mut self, val: SieEp6Cr0AckedTxn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub const fn nak_int_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn set_nak_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn err_in_txn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_err_in_txn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp6Cr0 {
    #[inline(always)]
    fn default() -> SieEp6Cr0 {
        SieEp6Cr0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp7Cnt0(pub u32);
impl SieEp7Cnt0 {
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub const fn data_count_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn set_data_count_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub const fn data_valid(&self) -> SieEp7Cnt0DataValid {
        let val = (self.0 >> 6usize) & 0x01;
        SieEp7Cnt0DataValid::from_bits(val as u8)
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn set_data_valid(&mut self, val: SieEp7Cnt0DataValid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub const fn data_toggle(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn set_data_toggle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp7Cnt0 {
    #[inline(always)]
    fn default() -> SieEp7Cnt0 {
        SieEp7Cnt0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp7Cnt1(pub u32);
impl SieEp7Cnt1 {
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub const fn data_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn set_data_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SieEp7Cnt1 {
    #[inline(always)]
    fn default() -> SieEp7Cnt1 {
        SieEp7Cnt1(0)
    }
}
#[doc = "Non-control endpoint's control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp7Cr0(pub u32);
impl SieEp7Cr0 {
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub const fn mode(&self) -> SieEp7Cr0Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        SieEp7Cr0Mode::from_bits(val as u8)
    }
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: SieEp7Cr0Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn acked_txn(&self) -> SieEp7Cr0AckedTxn {
        let val = (self.0 >> 4usize) & 0x01;
        SieEp7Cr0AckedTxn::from_bits(val as u8)
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_acked_txn(&mut self, val: SieEp7Cr0AckedTxn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub const fn nak_int_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn set_nak_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn err_in_txn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_err_in_txn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp7Cr0 {
    #[inline(always)]
    fn default() -> SieEp7Cr0 {
        SieEp7Cr0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp8Cnt0(pub u32);
impl SieEp8Cnt0 {
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub const fn data_count_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\] bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn set_data_count_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub const fn data_valid(&self) -> SieEp8Cnt0DataValid {
        let val = (self.0 >> 6usize) & 0x01;
        SieEp8Cnt0DataValid::from_bits(val as u8)
    }
    #[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn set_data_valid(&mut self, val: SieEp8Cnt0DataValid) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub const fn data_toggle(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn set_data_toggle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp8Cnt0 {
    #[inline(always)]
    fn default() -> SieEp8Cnt0 {
        SieEp8Cnt0(0)
    }
}
#[doc = "Non-control endpoint count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp8Cnt1(pub u32);
impl SieEp8Cnt1 {
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub const fn data_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn set_data_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SieEp8Cnt1 {
    #[inline(always)]
    fn default() -> SieEp8Cnt1 {
        SieEp8Cnt1(0)
    }
}
#[doc = "Non-control endpoint's control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEp8Cr0(pub u32);
impl SieEp8Cr0 {
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub const fn mode(&self) -> SieEp8Cr0Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        SieEp8Cr0Mode::from_bits(val as u8)
    }
    #[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: SieEp8Cr0Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn acked_txn(&self) -> SieEp8Cr0AckedTxn {
        let val = (self.0 >> 4usize) & 0x01;
        SieEp8Cr0AckedTxn::from_bits(val as u8)
    }
    #[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_acked_txn(&mut self, val: SieEp8Cr0AckedTxn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub const fn nak_int_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn set_nak_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub const fn err_in_txn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn set_err_in_txn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEp8Cr0 {
    #[inline(always)]
    fn default() -> SieEp8Cr0 {
        SieEp8Cr0(0)
    }
}
#[doc = "USB SIE Data Endpoints Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEpIntEn(pub u32);
impl SieEpIntEn {
    #[doc = "Enables interrupt for EP1"]
    #[inline(always)]
    pub const fn ep1_intr_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP1"]
    #[inline(always)]
    pub fn set_ep1_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables interrupt for EP2"]
    #[inline(always)]
    pub const fn ep2_intr_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP2"]
    #[inline(always)]
    pub fn set_ep2_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for EP3"]
    #[inline(always)]
    pub const fn ep3_intr_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP3"]
    #[inline(always)]
    pub fn set_ep3_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables interrupt for EP4"]
    #[inline(always)]
    pub const fn ep4_intr_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP4"]
    #[inline(always)]
    pub fn set_ep4_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables interrupt for EP5"]
    #[inline(always)]
    pub const fn ep5_intr_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP5"]
    #[inline(always)]
    pub fn set_ep5_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables interrupt for EP6"]
    #[inline(always)]
    pub const fn ep6_intr_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP6"]
    #[inline(always)]
    pub fn set_ep6_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables interrupt for EP7"]
    #[inline(always)]
    pub const fn ep7_intr_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP7"]
    #[inline(always)]
    pub fn set_ep7_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables interrupt for EP8"]
    #[inline(always)]
    pub const fn ep8_intr_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for EP8"]
    #[inline(always)]
    pub fn set_ep8_intr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEpIntEn {
    #[inline(always)]
    fn default() -> SieEpIntEn {
        SieEpIntEn(0)
    }
}
#[doc = "USB SIE Data Endpoint Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieEpIntSr(pub u32);
impl SieEpIntSr {
    #[doc = "Interrupt status for EP1"]
    #[inline(always)]
    pub const fn ep1_intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP1"]
    #[inline(always)]
    pub fn set_ep1_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt status for EP2"]
    #[inline(always)]
    pub const fn ep2_intr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP2"]
    #[inline(always)]
    pub fn set_ep2_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt status for EP3"]
    #[inline(always)]
    pub const fn ep3_intr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP3"]
    #[inline(always)]
    pub fn set_ep3_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt status for EP4"]
    #[inline(always)]
    pub const fn ep4_intr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP4"]
    #[inline(always)]
    pub fn set_ep4_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt status for EP5"]
    #[inline(always)]
    pub const fn ep5_intr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP5"]
    #[inline(always)]
    pub fn set_ep5_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt status for EP6"]
    #[inline(always)]
    pub const fn ep6_intr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP6"]
    #[inline(always)]
    pub fn set_ep6_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt status for EP7"]
    #[inline(always)]
    pub const fn ep7_intr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP7"]
    #[inline(always)]
    pub fn set_ep7_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt status for EP8"]
    #[inline(always)]
    pub const fn ep8_intr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status for EP8"]
    #[inline(always)]
    pub fn set_ep8_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SieEpIntSr {
    #[inline(always)]
    fn default() -> SieEpIntSr {
        SieEpIntSr(0)
    }
}
#[doc = "Start Of Frame Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sof0(pub u32);
impl Sof0 {
    #[doc = "It has the lower 8 bits \\[7:0\\] of the SOF frame number."]
    #[inline(always)]
    pub const fn frame_number(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "It has the lower 8 bits \\[7:0\\] of the SOF frame number."]
    #[inline(always)]
    pub fn set_frame_number(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Sof0 {
    #[inline(always)]
    fn default() -> Sof0 {
        Sof0(0)
    }
}
#[doc = "Start Of Frame Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sof1(pub u32);
impl Sof1 {
    #[doc = "It has the upper 3 bits \\[10:8\\] of the SOF frame number."]
    #[inline(always)]
    pub const fn frame_number_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "It has the upper 3 bits \\[10:8\\] of the SOF frame number."]
    #[inline(always)]
    pub fn set_frame_number_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Sof1 {
    #[inline(always)]
    fn default() -> Sof1 {
        Sof1(0)
    }
}
#[doc = "Start Of Frame Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sof16(pub u32);
impl Sof16 {
    #[doc = "The frame number (11b)"]
    #[inline(always)]
    pub const fn frame_number16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "The frame number (11b)"]
    #[inline(always)]
    pub fn set_frame_number16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for Sof16 {
    #[inline(always)]
    fn default() -> Sof16 {
        Sof16(0)
    }
}
#[doc = "USB Block Clock Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbClkEn(pub u32);
impl UsbClkEn {
    #[doc = "Clock Enable for Core Logic clocked by AHB bus clock"]
    #[inline(always)]
    pub const fn csr_clk_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Enable for Core Logic clocked by AHB bus clock"]
    #[inline(always)]
    pub fn set_csr_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbClkEn {
    #[inline(always)]
    fn default() -> UsbClkEn {
        UsbClkEn(0)
    }
}
#[doc = "USBIO Control 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbioCr0(pub u32);
impl UsbioCr0 {
    #[doc = "Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined."]
    #[inline(always)]
    pub const fn rd(&self) -> Rd {
        let val = (self.0 >> 0usize) & 0x01;
        Rd::from_bits(val as u8)
    }
    #[doc = "Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined."]
    #[inline(always)]
    pub fn set_rd(&mut self, val: Rd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
    #[inline(always)]
    pub const fn td(&self) -> Td {
        let val = (self.0 >> 5usize) & 0x01;
        Td::from_bits(val as u8)
    }
    #[doc = "Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
    #[inline(always)]
    pub fn set_td(&mut self, val: Td) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
    #[inline(always)]
    pub const fn tse0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
    #[inline(always)]
    pub fn set_tse0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
    #[inline(always)]
    pub const fn ten(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
    #[inline(always)]
    pub fn set_ten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for UsbioCr0 {
    #[inline(always)]
    fn default() -> UsbioCr0 {
        UsbioCr0(0)
    }
}
#[doc = "USBIO control 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbioCr1(pub u32);
impl UsbioCr1 {
    #[doc = "This read only bit gives the state of the D- pin when IOMODE bit is '0' and USB doesn't transmit. This bit is '0' when USB transmits SE0, and this bit is '1' when USB transmits other than SE0. This bit is valid if USB Device."]
    #[inline(always)]
    pub const fn dmo(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This read only bit gives the state of the D- pin when IOMODE bit is '0' and USB doesn't transmit. This bit is '0' when USB transmits SE0, and this bit is '1' when USB transmits other than SE0. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn set_dmo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This read only bit gives the state of the D+ pin when IOMODE bit is '0' and USB doesn't transmit. This bit displays the output value of D+ pin when USB transmits SE0 or data. This bit is valid if USB Device."]
    #[inline(always)]
    pub const fn dpo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This read only bit gives the state of the D+ pin when IOMODE bit is '0' and USB doesn't transmit. This bit displays the output value of D+ pin when USB transmits SE0 or data. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn set_dpo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    pub const fn iomode(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    pub fn set_iomode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for UsbioCr1 {
    #[inline(always)]
    fn default() -> UsbioCr1 {
        UsbioCr1(0)
    }
}
#[doc = "USBIO control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbioCr2(pub u32);
impl UsbioCr2 {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_5_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_5_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub const fn test_pkt(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub fn set_test_pkt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn rsvd_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_rsvd_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for UsbioCr2 {
    #[inline(always)]
    fn default() -> UsbioCr2 {
        UsbioCr2(0)
    }
}
#[doc = "USB IO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbioCtl(pub u32);
impl UsbioCtl {
    #[doc = "The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    pub const fn dm_p(&self) -> DmP {
        let val = (self.0 >> 0usize) & 0x07;
        DmP::from_bits(val as u8)
    }
    #[doc = "The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    pub fn set_dm_p(&mut self, val: DmP) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    pub const fn dm_m(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    pub fn set_dm_m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
}
impl Default for UsbioCtl {
    #[inline(always)]
    fn default() -> UsbioCtl {
        UsbioCtl(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp1CfgCrcBypass {
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 0x01,
}
impl ArbEp1CfgCrcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp1CfgCrcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp1CfgCrcBypass {
    #[inline(always)]
    fn from(val: u8) -> ArbEp1CfgCrcBypass {
        ArbEp1CfgCrcBypass::from_bits(val)
    }
}
impl From<ArbEp1CfgCrcBypass> for u8 {
    #[inline(always)]
    fn from(val: ArbEp1CfgCrcBypass) -> u8 {
        ArbEp1CfgCrcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp1CfgResetPtr {
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 0x01,
}
impl ArbEp1CfgResetPtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp1CfgResetPtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp1CfgResetPtr {
    #[inline(always)]
    fn from(val: u8) -> ArbEp1CfgResetPtr {
        ArbEp1CfgResetPtr::from_bits(val)
    }
}
impl From<ArbEp1CfgResetPtr> for u8 {
    #[inline(always)]
    fn from(val: ArbEp1CfgResetPtr) -> u8 {
        ArbEp1CfgResetPtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp2CfgCrcBypass {
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 0x01,
}
impl ArbEp2CfgCrcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp2CfgCrcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp2CfgCrcBypass {
    #[inline(always)]
    fn from(val: u8) -> ArbEp2CfgCrcBypass {
        ArbEp2CfgCrcBypass::from_bits(val)
    }
}
impl From<ArbEp2CfgCrcBypass> for u8 {
    #[inline(always)]
    fn from(val: ArbEp2CfgCrcBypass) -> u8 {
        ArbEp2CfgCrcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp2CfgResetPtr {
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 0x01,
}
impl ArbEp2CfgResetPtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp2CfgResetPtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp2CfgResetPtr {
    #[inline(always)]
    fn from(val: u8) -> ArbEp2CfgResetPtr {
        ArbEp2CfgResetPtr::from_bits(val)
    }
}
impl From<ArbEp2CfgResetPtr> for u8 {
    #[inline(always)]
    fn from(val: ArbEp2CfgResetPtr) -> u8 {
        ArbEp2CfgResetPtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp3CfgCrcBypass {
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 0x01,
}
impl ArbEp3CfgCrcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp3CfgCrcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp3CfgCrcBypass {
    #[inline(always)]
    fn from(val: u8) -> ArbEp3CfgCrcBypass {
        ArbEp3CfgCrcBypass::from_bits(val)
    }
}
impl From<ArbEp3CfgCrcBypass> for u8 {
    #[inline(always)]
    fn from(val: ArbEp3CfgCrcBypass) -> u8 {
        ArbEp3CfgCrcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp3CfgResetPtr {
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 0x01,
}
impl ArbEp3CfgResetPtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp3CfgResetPtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp3CfgResetPtr {
    #[inline(always)]
    fn from(val: u8) -> ArbEp3CfgResetPtr {
        ArbEp3CfgResetPtr::from_bits(val)
    }
}
impl From<ArbEp3CfgResetPtr> for u8 {
    #[inline(always)]
    fn from(val: ArbEp3CfgResetPtr) -> u8 {
        ArbEp3CfgResetPtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp4CfgCrcBypass {
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 0x01,
}
impl ArbEp4CfgCrcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp4CfgCrcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp4CfgCrcBypass {
    #[inline(always)]
    fn from(val: u8) -> ArbEp4CfgCrcBypass {
        ArbEp4CfgCrcBypass::from_bits(val)
    }
}
impl From<ArbEp4CfgCrcBypass> for u8 {
    #[inline(always)]
    fn from(val: ArbEp4CfgCrcBypass) -> u8 {
        ArbEp4CfgCrcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp4CfgResetPtr {
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 0x01,
}
impl ArbEp4CfgResetPtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp4CfgResetPtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp4CfgResetPtr {
    #[inline(always)]
    fn from(val: u8) -> ArbEp4CfgResetPtr {
        ArbEp4CfgResetPtr::from_bits(val)
    }
}
impl From<ArbEp4CfgResetPtr> for u8 {
    #[inline(always)]
    fn from(val: ArbEp4CfgResetPtr) -> u8 {
        ArbEp4CfgResetPtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp5CfgCrcBypass {
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 0x01,
}
impl ArbEp5CfgCrcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp5CfgCrcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp5CfgCrcBypass {
    #[inline(always)]
    fn from(val: u8) -> ArbEp5CfgCrcBypass {
        ArbEp5CfgCrcBypass::from_bits(val)
    }
}
impl From<ArbEp5CfgCrcBypass> for u8 {
    #[inline(always)]
    fn from(val: ArbEp5CfgCrcBypass) -> u8 {
        ArbEp5CfgCrcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp5CfgResetPtr {
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 0x01,
}
impl ArbEp5CfgResetPtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp5CfgResetPtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp5CfgResetPtr {
    #[inline(always)]
    fn from(val: u8) -> ArbEp5CfgResetPtr {
        ArbEp5CfgResetPtr::from_bits(val)
    }
}
impl From<ArbEp5CfgResetPtr> for u8 {
    #[inline(always)]
    fn from(val: ArbEp5CfgResetPtr) -> u8 {
        ArbEp5CfgResetPtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp6CfgCrcBypass {
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 0x01,
}
impl ArbEp6CfgCrcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp6CfgCrcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp6CfgCrcBypass {
    #[inline(always)]
    fn from(val: u8) -> ArbEp6CfgCrcBypass {
        ArbEp6CfgCrcBypass::from_bits(val)
    }
}
impl From<ArbEp6CfgCrcBypass> for u8 {
    #[inline(always)]
    fn from(val: ArbEp6CfgCrcBypass) -> u8 {
        ArbEp6CfgCrcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp6CfgResetPtr {
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 0x01,
}
impl ArbEp6CfgResetPtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp6CfgResetPtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp6CfgResetPtr {
    #[inline(always)]
    fn from(val: u8) -> ArbEp6CfgResetPtr {
        ArbEp6CfgResetPtr::from_bits(val)
    }
}
impl From<ArbEp6CfgResetPtr> for u8 {
    #[inline(always)]
    fn from(val: ArbEp6CfgResetPtr) -> u8 {
        ArbEp6CfgResetPtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp7CfgCrcBypass {
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 0x01,
}
impl ArbEp7CfgCrcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp7CfgCrcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp7CfgCrcBypass {
    #[inline(always)]
    fn from(val: u8) -> ArbEp7CfgCrcBypass {
        ArbEp7CfgCrcBypass::from_bits(val)
    }
}
impl From<ArbEp7CfgCrcBypass> for u8 {
    #[inline(always)]
    fn from(val: ArbEp7CfgCrcBypass) -> u8 {
        ArbEp7CfgCrcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp7CfgResetPtr {
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 0x01,
}
impl ArbEp7CfgResetPtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp7CfgResetPtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp7CfgResetPtr {
    #[inline(always)]
    fn from(val: u8) -> ArbEp7CfgResetPtr {
        ArbEp7CfgResetPtr::from_bits(val)
    }
}
impl From<ArbEp7CfgResetPtr> for u8 {
    #[inline(always)]
    fn from(val: ArbEp7CfgResetPtr) -> u8 {
        ArbEp7CfgResetPtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp8CfgCrcBypass {
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 0x01,
}
impl ArbEp8CfgCrcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp8CfgCrcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp8CfgCrcBypass {
    #[inline(always)]
    fn from(val: u8) -> ArbEp8CfgCrcBypass {
        ArbEp8CfgCrcBypass::from_bits(val)
    }
}
impl From<ArbEp8CfgCrcBypass> for u8 {
    #[inline(always)]
    fn from(val: ArbEp8CfgCrcBypass) -> u8 {
        ArbEp8CfgCrcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArbEp8CfgResetPtr {
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 0x01,
}
impl ArbEp8CfgResetPtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArbEp8CfgResetPtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArbEp8CfgResetPtr {
    #[inline(always)]
    fn from(val: u8) -> ArbEp8CfgResetPtr {
        ArbEp8CfgResetPtr::from_bits(val)
    }
}
impl From<ArbEp8CfgResetPtr> for u8 {
    #[inline(always)]
    fn from(val: ArbEp8CfgResetPtr) -> u8 {
        ArbEp8CfgResetPtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DdftInSel {
    #[doc = "Nothing connected, output 0"]
    OFF = 0,
    #[doc = "GPIO input of DP"]
    GPIO_DP_IN = 0x01,
    #[doc = "GPIO input of DM"]
    GPIO_DM_IN = 0x02,
    _RESERVED_3 = 0x03,
}
impl DdftInSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DdftInSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DdftInSel {
    #[inline(always)]
    fn from(val: u8) -> DdftInSel {
        DdftInSel::from_bits(val)
    }
}
impl From<DdftInSel> for u8 {
    #[inline(always)]
    fn from(val: DdftInSel) -> u8 {
        DdftInSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DdftOutSel {
    #[doc = "Nothing connected, output 0"]
    OFF = 0,
    #[doc = "Single Ended output of DP"]
    DP_SE = 0x01,
    #[doc = "Single Ended output of DM"]
    DM_SE = 0x02,
    #[doc = "Output Enable"]
    TXOE = 0x03,
    #[doc = "Differential Receiver output"]
    RCV_DF = 0x04,
    #[doc = "GPIO output of DP"]
    GPIO_DP_OUT = 0x05,
    #[doc = "GPIO output of DM"]
    GPIO_DM_OUT = 0x06,
    _RESERVED_7 = 0x07,
}
impl DdftOutSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DdftOutSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DdftOutSel {
    #[inline(always)]
    fn from(val: u8) -> DdftOutSel {
        DdftOutSel::from_bits(val)
    }
}
impl From<DdftOutSel> for u8 {
    #[inline(always)]
    fn from(val: DdftOutSel) -> u8 {
        DdftOutSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DmP {
    #[doc = "Mode 0: Output buffer off (high Z). Input buffer off."]
    OFF = 0,
    #[doc = "Mode 1: Output buffer off (high Z). Input buffer on. Other values, not supported."]
    INPUT = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl DmP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmP {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmP {
    #[inline(always)]
    fn from(val: u8) -> DmP {
        DmP::from_bits(val)
    }
}
impl From<DmP> for u8 {
    #[inline(always)]
    fn from(val: DmP) -> u8 {
        DmP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DmaCfg {
    #[doc = "No DMA"]
    DMA_NONE = 0,
    #[doc = "Manual DMA"]
    DMA_MANUAL = 0x01,
    #[doc = "Auto DMA"]
    DMA_AUTO = 0x02,
    _RESERVED_3 = 0x03,
}
impl DmaCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCfg {
    #[inline(always)]
    fn from(val: u8) -> DmaCfg {
        DmaCfg::from_bits(val)
    }
}
impl From<DmaCfg> for u8 {
    #[inline(always)]
    fn from(val: DmaCfg) -> u8 {
        DmaCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep0CntDataValid {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    DATA_VALID = 0x01,
}
impl Ep0CntDataValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep0CntDataValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep0CntDataValid {
    #[inline(always)]
    fn from(val: u8) -> Ep0CntDataValid {
        Ep0CntDataValid::from_bits(val)
    }
}
impl From<Ep0CntDataValid> for u8 {
    #[inline(always)]
    fn from(val: Ep0CntDataValid) -> u8 {
        Ep0CntDataValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep0CrAckedTxn {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    ACKED_YES = 0x01,
}
impl Ep0CrAckedTxn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep0CrAckedTxn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep0CrAckedTxn {
    #[inline(always)]
    fn from(val: u8) -> Ep0CrAckedTxn {
        Ep0CrAckedTxn::from_bits(val)
    }
}
impl From<Ep0CrAckedTxn> for u8 {
    #[inline(always)]
    fn from(val: Ep0CrAckedTxn) -> u8 {
        Ep0CrAckedTxn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep0CrMode {
    #[doc = "Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 0x01,
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 0x02,
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 0x05,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 0x06,
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 0x07,
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 0x08,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 0x0b,
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 0x0c,
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 0x0f,
}
impl Ep0CrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep0CrMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep0CrMode {
    #[inline(always)]
    fn from(val: u8) -> Ep0CrMode {
        Ep0CrMode::from_bits(val)
    }
}
impl From<Ep0CrMode> for u8 {
    #[inline(always)]
    fn from(val: Ep0CrMode) -> u8 {
        Ep0CrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep1DrqSel {
    #[doc = "High priority interrupt"]
    HI = 0,
    #[doc = "Medium priority interrupt"]
    MED = 0x01,
    #[doc = "Low priority interrupt"]
    LO = 0x02,
    #[doc = "N/A"]
    RSVD = 0x03,
}
impl Ep1DrqSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep1DrqSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep1DrqSel {
    #[inline(always)]
    fn from(val: u8) -> Ep1DrqSel {
        Ep1DrqSel::from_bits(val)
    }
}
impl From<Ep1DrqSel> for u8 {
    #[inline(always)]
    fn from(val: Ep1DrqSel) -> u8 {
        Ep1DrqSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep1Typ {
    #[doc = "IN outpoint"]
    EP_IN = 0,
    #[doc = "OUT outpoint"]
    EP_OUT = 0x01,
}
impl Ep1Typ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep1Typ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep1Typ {
    #[inline(always)]
    fn from(val: u8) -> Ep1Typ {
        Ep1Typ::from_bits(val)
    }
}
impl From<Ep1Typ> for u8 {
    #[inline(always)]
    fn from(val: Ep1Typ) -> u8 {
        Ep1Typ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep2Typ {
    #[doc = "IN outpoint"]
    EP_IN = 0,
    #[doc = "OUT outpoint"]
    EP_OUT = 0x01,
}
impl Ep2Typ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep2Typ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep2Typ {
    #[inline(always)]
    fn from(val: u8) -> Ep2Typ {
        Ep2Typ::from_bits(val)
    }
}
impl From<Ep2Typ> for u8 {
    #[inline(always)]
    fn from(val: Ep2Typ) -> u8 {
        Ep2Typ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep3Typ {
    #[doc = "IN outpoint"]
    EP_IN = 0,
    #[doc = "OUT outpoint"]
    EP_OUT = 0x01,
}
impl Ep3Typ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep3Typ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep3Typ {
    #[inline(always)]
    fn from(val: u8) -> Ep3Typ {
        Ep3Typ::from_bits(val)
    }
}
impl From<Ep3Typ> for u8 {
    #[inline(always)]
    fn from(val: Ep3Typ) -> u8 {
        Ep3Typ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep4Typ {
    #[doc = "IN outpoint"]
    EP_IN = 0,
    #[doc = "OUT outpoint"]
    EP_OUT = 0x01,
}
impl Ep4Typ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep4Typ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep4Typ {
    #[inline(always)]
    fn from(val: u8) -> Ep4Typ {
        Ep4Typ::from_bits(val)
    }
}
impl From<Ep4Typ> for u8 {
    #[inline(always)]
    fn from(val: Ep4Typ) -> u8 {
        Ep4Typ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep5Typ {
    #[doc = "IN outpoint"]
    EP_IN = 0,
    #[doc = "OUT outpoint"]
    EP_OUT = 0x01,
}
impl Ep5Typ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep5Typ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep5Typ {
    #[inline(always)]
    fn from(val: u8) -> Ep5Typ {
        Ep5Typ::from_bits(val)
    }
}
impl From<Ep5Typ> for u8 {
    #[inline(always)]
    fn from(val: Ep5Typ) -> u8 {
        Ep5Typ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep6Typ {
    #[doc = "IN outpoint"]
    EP_IN = 0,
    #[doc = "OUT outpoint"]
    EP_OUT = 0x01,
}
impl Ep6Typ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep6Typ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep6Typ {
    #[inline(always)]
    fn from(val: u8) -> Ep6Typ {
        Ep6Typ::from_bits(val)
    }
}
impl From<Ep6Typ> for u8 {
    #[inline(always)]
    fn from(val: Ep6Typ) -> u8 {
        Ep6Typ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep7Typ {
    #[doc = "IN outpoint"]
    EP_IN = 0,
    #[doc = "OUT outpoint"]
    EP_OUT = 0x01,
}
impl Ep7Typ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep7Typ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep7Typ {
    #[inline(always)]
    fn from(val: u8) -> Ep7Typ {
        Ep7Typ::from_bits(val)
    }
}
impl From<Ep7Typ> for u8 {
    #[inline(always)]
    fn from(val: Ep7Typ) -> u8 {
        Ep7Typ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ep8Typ {
    #[doc = "IN outpoint"]
    EP_IN = 0,
    #[doc = "OUT outpoint"]
    EP_OUT = 0x01,
}
impl Ep8Typ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ep8Typ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ep8Typ {
    #[inline(always)]
    fn from(val: u8) -> Ep8Typ {
        Ep8Typ::from_bits(val)
    }
}
impl From<Ep8Typ> for u8 {
    #[inline(always)]
    fn from(val: Ep8Typ) -> u8 {
        Ep8Typ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hs {
    #[doc = "Acknowledge Packet"]
    ACK = 0,
    #[doc = "Non-Acknowledge Packet"]
    NAK = 0x01,
    #[doc = "Stall Packet"]
    STALL = 0x02,
    #[doc = "Null Packet"]
    NULL = 0x03,
}
impl Hs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hs {
    #[inline(always)]
    fn from(val: u8) -> Hs {
        Hs::from_bits(val)
    }
}
impl From<Hs> for u8 {
    #[inline(always)]
    fn from(val: Hs) -> u8 {
        Hs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rd {
    #[doc = "D+ < D- (K state)"]
    DIFF_LOW = 0,
    #[doc = "D+ > D- (J state)"]
    DIFF_HIGH = 0x01,
}
impl Rd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rd {
    #[inline(always)]
    fn from(val: u8) -> Rd {
        Rd::from_bits(val)
    }
}
impl From<Rd> for u8 {
    #[inline(always)]
    fn from(val: Rd) -> u8 {
        Rd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp1Cnt0DataValid {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    DATA_VALID = 0x01,
}
impl SieEp1Cnt0DataValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp1Cnt0DataValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp1Cnt0DataValid {
    #[inline(always)]
    fn from(val: u8) -> SieEp1Cnt0DataValid {
        SieEp1Cnt0DataValid::from_bits(val)
    }
}
impl From<SieEp1Cnt0DataValid> for u8 {
    #[inline(always)]
    fn from(val: SieEp1Cnt0DataValid) -> u8 {
        SieEp1Cnt0DataValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp1Cr0AckedTxn {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    ACKED_YES = 0x01,
}
impl SieEp1Cr0AckedTxn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp1Cr0AckedTxn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp1Cr0AckedTxn {
    #[inline(always)]
    fn from(val: u8) -> SieEp1Cr0AckedTxn {
        SieEp1Cr0AckedTxn::from_bits(val)
    }
}
impl From<SieEp1Cr0AckedTxn> for u8 {
    #[inline(always)]
    fn from(val: SieEp1Cr0AckedTxn) -> u8 {
        SieEp1Cr0AckedTxn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp1Cr0Mode {
    #[doc = "Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 0x01,
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 0x02,
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 0x05,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 0x06,
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 0x07,
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 0x08,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 0x0b,
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 0x0c,
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 0x0f,
}
impl SieEp1Cr0Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp1Cr0Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp1Cr0Mode {
    #[inline(always)]
    fn from(val: u8) -> SieEp1Cr0Mode {
        SieEp1Cr0Mode::from_bits(val)
    }
}
impl From<SieEp1Cr0Mode> for u8 {
    #[inline(always)]
    fn from(val: SieEp1Cr0Mode) -> u8 {
        SieEp1Cr0Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp2Cnt0DataValid {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    DATA_VALID = 0x01,
}
impl SieEp2Cnt0DataValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp2Cnt0DataValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp2Cnt0DataValid {
    #[inline(always)]
    fn from(val: u8) -> SieEp2Cnt0DataValid {
        SieEp2Cnt0DataValid::from_bits(val)
    }
}
impl From<SieEp2Cnt0DataValid> for u8 {
    #[inline(always)]
    fn from(val: SieEp2Cnt0DataValid) -> u8 {
        SieEp2Cnt0DataValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp2Cr0AckedTxn {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    ACKED_YES = 0x01,
}
impl SieEp2Cr0AckedTxn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp2Cr0AckedTxn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp2Cr0AckedTxn {
    #[inline(always)]
    fn from(val: u8) -> SieEp2Cr0AckedTxn {
        SieEp2Cr0AckedTxn::from_bits(val)
    }
}
impl From<SieEp2Cr0AckedTxn> for u8 {
    #[inline(always)]
    fn from(val: SieEp2Cr0AckedTxn) -> u8 {
        SieEp2Cr0AckedTxn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp2Cr0Mode {
    #[doc = "Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 0x01,
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 0x02,
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 0x05,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 0x06,
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 0x07,
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 0x08,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 0x0b,
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 0x0c,
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 0x0f,
}
impl SieEp2Cr0Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp2Cr0Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp2Cr0Mode {
    #[inline(always)]
    fn from(val: u8) -> SieEp2Cr0Mode {
        SieEp2Cr0Mode::from_bits(val)
    }
}
impl From<SieEp2Cr0Mode> for u8 {
    #[inline(always)]
    fn from(val: SieEp2Cr0Mode) -> u8 {
        SieEp2Cr0Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp3Cnt0DataValid {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    DATA_VALID = 0x01,
}
impl SieEp3Cnt0DataValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp3Cnt0DataValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp3Cnt0DataValid {
    #[inline(always)]
    fn from(val: u8) -> SieEp3Cnt0DataValid {
        SieEp3Cnt0DataValid::from_bits(val)
    }
}
impl From<SieEp3Cnt0DataValid> for u8 {
    #[inline(always)]
    fn from(val: SieEp3Cnt0DataValid) -> u8 {
        SieEp3Cnt0DataValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp3Cr0AckedTxn {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    ACKED_YES = 0x01,
}
impl SieEp3Cr0AckedTxn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp3Cr0AckedTxn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp3Cr0AckedTxn {
    #[inline(always)]
    fn from(val: u8) -> SieEp3Cr0AckedTxn {
        SieEp3Cr0AckedTxn::from_bits(val)
    }
}
impl From<SieEp3Cr0AckedTxn> for u8 {
    #[inline(always)]
    fn from(val: SieEp3Cr0AckedTxn) -> u8 {
        SieEp3Cr0AckedTxn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp3Cr0Mode {
    #[doc = "Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 0x01,
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 0x02,
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 0x05,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 0x06,
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 0x07,
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 0x08,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 0x0b,
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 0x0c,
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 0x0f,
}
impl SieEp3Cr0Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp3Cr0Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp3Cr0Mode {
    #[inline(always)]
    fn from(val: u8) -> SieEp3Cr0Mode {
        SieEp3Cr0Mode::from_bits(val)
    }
}
impl From<SieEp3Cr0Mode> for u8 {
    #[inline(always)]
    fn from(val: SieEp3Cr0Mode) -> u8 {
        SieEp3Cr0Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp4Cnt0DataValid {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    DATA_VALID = 0x01,
}
impl SieEp4Cnt0DataValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp4Cnt0DataValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp4Cnt0DataValid {
    #[inline(always)]
    fn from(val: u8) -> SieEp4Cnt0DataValid {
        SieEp4Cnt0DataValid::from_bits(val)
    }
}
impl From<SieEp4Cnt0DataValid> for u8 {
    #[inline(always)]
    fn from(val: SieEp4Cnt0DataValid) -> u8 {
        SieEp4Cnt0DataValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp4Cr0AckedTxn {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    ACKED_YES = 0x01,
}
impl SieEp4Cr0AckedTxn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp4Cr0AckedTxn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp4Cr0AckedTxn {
    #[inline(always)]
    fn from(val: u8) -> SieEp4Cr0AckedTxn {
        SieEp4Cr0AckedTxn::from_bits(val)
    }
}
impl From<SieEp4Cr0AckedTxn> for u8 {
    #[inline(always)]
    fn from(val: SieEp4Cr0AckedTxn) -> u8 {
        SieEp4Cr0AckedTxn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp4Cr0Mode {
    #[doc = "Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 0x01,
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 0x02,
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 0x05,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 0x06,
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 0x07,
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 0x08,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 0x0b,
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 0x0c,
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 0x0f,
}
impl SieEp4Cr0Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp4Cr0Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp4Cr0Mode {
    #[inline(always)]
    fn from(val: u8) -> SieEp4Cr0Mode {
        SieEp4Cr0Mode::from_bits(val)
    }
}
impl From<SieEp4Cr0Mode> for u8 {
    #[inline(always)]
    fn from(val: SieEp4Cr0Mode) -> u8 {
        SieEp4Cr0Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp5Cnt0DataValid {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    DATA_VALID = 0x01,
}
impl SieEp5Cnt0DataValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp5Cnt0DataValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp5Cnt0DataValid {
    #[inline(always)]
    fn from(val: u8) -> SieEp5Cnt0DataValid {
        SieEp5Cnt0DataValid::from_bits(val)
    }
}
impl From<SieEp5Cnt0DataValid> for u8 {
    #[inline(always)]
    fn from(val: SieEp5Cnt0DataValid) -> u8 {
        SieEp5Cnt0DataValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp5Cr0AckedTxn {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    ACKED_YES = 0x01,
}
impl SieEp5Cr0AckedTxn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp5Cr0AckedTxn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp5Cr0AckedTxn {
    #[inline(always)]
    fn from(val: u8) -> SieEp5Cr0AckedTxn {
        SieEp5Cr0AckedTxn::from_bits(val)
    }
}
impl From<SieEp5Cr0AckedTxn> for u8 {
    #[inline(always)]
    fn from(val: SieEp5Cr0AckedTxn) -> u8 {
        SieEp5Cr0AckedTxn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp5Cr0Mode {
    #[doc = "Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 0x01,
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 0x02,
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 0x05,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 0x06,
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 0x07,
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 0x08,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 0x0b,
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 0x0c,
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 0x0f,
}
impl SieEp5Cr0Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp5Cr0Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp5Cr0Mode {
    #[inline(always)]
    fn from(val: u8) -> SieEp5Cr0Mode {
        SieEp5Cr0Mode::from_bits(val)
    }
}
impl From<SieEp5Cr0Mode> for u8 {
    #[inline(always)]
    fn from(val: SieEp5Cr0Mode) -> u8 {
        SieEp5Cr0Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp6Cnt0DataValid {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    DATA_VALID = 0x01,
}
impl SieEp6Cnt0DataValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp6Cnt0DataValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp6Cnt0DataValid {
    #[inline(always)]
    fn from(val: u8) -> SieEp6Cnt0DataValid {
        SieEp6Cnt0DataValid::from_bits(val)
    }
}
impl From<SieEp6Cnt0DataValid> for u8 {
    #[inline(always)]
    fn from(val: SieEp6Cnt0DataValid) -> u8 {
        SieEp6Cnt0DataValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp6Cr0AckedTxn {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    ACKED_YES = 0x01,
}
impl SieEp6Cr0AckedTxn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp6Cr0AckedTxn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp6Cr0AckedTxn {
    #[inline(always)]
    fn from(val: u8) -> SieEp6Cr0AckedTxn {
        SieEp6Cr0AckedTxn::from_bits(val)
    }
}
impl From<SieEp6Cr0AckedTxn> for u8 {
    #[inline(always)]
    fn from(val: SieEp6Cr0AckedTxn) -> u8 {
        SieEp6Cr0AckedTxn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp6Cr0Mode {
    #[doc = "Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 0x01,
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 0x02,
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 0x05,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 0x06,
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 0x07,
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 0x08,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 0x0b,
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 0x0c,
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 0x0f,
}
impl SieEp6Cr0Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp6Cr0Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp6Cr0Mode {
    #[inline(always)]
    fn from(val: u8) -> SieEp6Cr0Mode {
        SieEp6Cr0Mode::from_bits(val)
    }
}
impl From<SieEp6Cr0Mode> for u8 {
    #[inline(always)]
    fn from(val: SieEp6Cr0Mode) -> u8 {
        SieEp6Cr0Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp7Cnt0DataValid {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    DATA_VALID = 0x01,
}
impl SieEp7Cnt0DataValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp7Cnt0DataValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp7Cnt0DataValid {
    #[inline(always)]
    fn from(val: u8) -> SieEp7Cnt0DataValid {
        SieEp7Cnt0DataValid::from_bits(val)
    }
}
impl From<SieEp7Cnt0DataValid> for u8 {
    #[inline(always)]
    fn from(val: SieEp7Cnt0DataValid) -> u8 {
        SieEp7Cnt0DataValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp7Cr0AckedTxn {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    ACKED_YES = 0x01,
}
impl SieEp7Cr0AckedTxn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp7Cr0AckedTxn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp7Cr0AckedTxn {
    #[inline(always)]
    fn from(val: u8) -> SieEp7Cr0AckedTxn {
        SieEp7Cr0AckedTxn::from_bits(val)
    }
}
impl From<SieEp7Cr0AckedTxn> for u8 {
    #[inline(always)]
    fn from(val: SieEp7Cr0AckedTxn) -> u8 {
        SieEp7Cr0AckedTxn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp7Cr0Mode {
    #[doc = "Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 0x01,
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 0x02,
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 0x05,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 0x06,
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 0x07,
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 0x08,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 0x0b,
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 0x0c,
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 0x0f,
}
impl SieEp7Cr0Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp7Cr0Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp7Cr0Mode {
    #[inline(always)]
    fn from(val: u8) -> SieEp7Cr0Mode {
        SieEp7Cr0Mode::from_bits(val)
    }
}
impl From<SieEp7Cr0Mode> for u8 {
    #[inline(always)]
    fn from(val: SieEp7Cr0Mode) -> u8 {
        SieEp7Cr0Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp8Cnt0DataValid {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    DATA_VALID = 0x01,
}
impl SieEp8Cnt0DataValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp8Cnt0DataValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp8Cnt0DataValid {
    #[inline(always)]
    fn from(val: u8) -> SieEp8Cnt0DataValid {
        SieEp8Cnt0DataValid::from_bits(val)
    }
}
impl From<SieEp8Cnt0DataValid> for u8 {
    #[inline(always)]
    fn from(val: SieEp8Cnt0DataValid) -> u8 {
        SieEp8Cnt0DataValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp8Cr0AckedTxn {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "Indicates a transaction ended with an ACK."]
    ACKED_YES = 0x01,
}
impl SieEp8Cr0AckedTxn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp8Cr0AckedTxn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp8Cr0AckedTxn {
    #[inline(always)]
    fn from(val: u8) -> SieEp8Cr0AckedTxn {
        SieEp8Cr0AckedTxn::from_bits(val)
    }
}
impl From<SieEp8Cr0AckedTxn> for u8 {
    #[inline(always)]
    fn from(val: SieEp8Cr0AckedTxn) -> u8 {
        SieEp8Cr0AckedTxn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SieEp8Cr0Mode {
    #[doc = "Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 0x01,
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 0x02,
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 0x05,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 0x06,
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 0x07,
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 0x08,
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 0x0b,
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 0x0c,
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 0x0f,
}
impl SieEp8Cr0Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SieEp8Cr0Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SieEp8Cr0Mode {
    #[inline(always)]
    fn from(val: u8) -> SieEp8Cr0Mode {
        SieEp8Cr0Mode::from_bits(val)
    }
}
impl From<SieEp8Cr0Mode> for u8 {
    #[inline(always)]
    fn from(val: SieEp8Cr0Mode) -> u8 {
        SieEp8Cr0Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SofLvlSel {
    #[doc = "High priority interrupt"]
    HI = 0,
    #[doc = "Medium priority interrupt"]
    MED = 0x01,
    #[doc = "Low priority interrupt"]
    LO = 0x02,
    #[doc = "illegal"]
    RSVD = 0x03,
}
impl SofLvlSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SofLvlSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SofLvlSel {
    #[inline(always)]
    fn from(val: u8) -> SofLvlSel {
        SofLvlSel::from_bits(val)
    }
}
impl From<SofLvlSel> for u8 {
    #[inline(always)]
    fn from(val: SofLvlSel) -> u8 {
        SofLvlSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SofirqSel {
    #[doc = "High priority interrupt"]
    HI = 0,
    #[doc = "Medium priority interrupt"]
    MED = 0x01,
    #[doc = "Low priority interrupt"]
    LO = 0x02,
    #[doc = "N/A"]
    RSVD = 0x03,
}
impl SofirqSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SofirqSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SofirqSel {
    #[inline(always)]
    fn from(val: u8) -> SofirqSel {
        SofirqSel::from_bits(val)
    }
}
impl From<SofirqSel> for u8 {
    #[inline(always)]
    fn from(val: SofirqSel) -> u8 {
        SofirqSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Td {
    #[doc = "Force USB K state (D+ is low D- is high)."]
    DIFF_K = 0,
    #[doc = "Force USB J state (D+ is high D- is low)."]
    DIFF_J = 0x01,
}
impl Td {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Td {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Td {
    #[inline(always)]
    fn from(val: u8) -> Td {
        Td::from_bits(val)
    }
}
impl From<Td> for u8 {
    #[inline(always)]
    fn from(val: Td) -> u8 {
        Td::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tknen {
    #[doc = "Sends no data."]
    NONE = 0,
    #[doc = "Sends SETUP token."]
    SETUP = 0x01,
    #[doc = "Sends IN token."]
    IN = 0x02,
    #[doc = "Sends OUT token."]
    OUT = 0x03,
    #[doc = "Sends SOF token."]
    SOF = 0x04,
    #[doc = "Sends Isochronous IN."]
    ISO_IN = 0x05,
    #[doc = "Sends Isochronous OUT."]
    ISO_OUT = 0x06,
    #[doc = "N/A"]
    RSV = 0x07,
}
impl Tknen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tknen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tknen {
    #[inline(always)]
    fn from(val: u8) -> Tknen {
        Tknen::from_bits(val)
    }
}
impl From<Tknen> for u8 {
    #[inline(always)]
    fn from(val: Tknen) -> u8 {
        Tknen::to_bits(val)
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
