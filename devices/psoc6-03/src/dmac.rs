#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Active channels"]
    pub active: ACTIVE,
    _reserved2: [u8; 4084usize],
    #[doc = "0x1000 - DMA controller channel"]
    pub ch0: CH,
    _reserved3: [u8; 112usize],
    #[doc = "0x1100 - DMA controller channel"]
    pub ch1: CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Channel control"]
    pub ctl: self::ch::CTL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Channel current indices"]
    pub idx: self::ch::IDX,
    #[doc = "0x14 - Channel current source address"]
    pub src: self::ch::SRC,
    #[doc = "0x18 - Channel current destination address"]
    pub dst: self::ch::DST,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - Channel current descriptor pointer"]
    pub curr: self::ch::CURR,
    _reserved5: [u8; 4usize],
    #[doc = "0x28 - Channle software trigger"]
    pub tr_cmd: self::ch::TR_CMD,
    _reserved6: [u8; 20usize],
    #[doc = "0x40 - Channel descriptor status"]
    pub descr_status: self::ch::DESCR_STATUS,
    _reserved7: [u8; 28usize],
    #[doc = "0x60 - Channel descriptor control"]
    pub descr_ctl: self::ch::DESCR_CTL,
    #[doc = "0x64 - Channel descriptor source"]
    pub descr_src: self::ch::DESCR_SRC,
    #[doc = "0x68 - Channel descriptor destination"]
    pub descr_dst: self::ch::DESCR_DST,
    #[doc = "0x6c - Channel descriptor X size"]
    pub descr_x_size: self::ch::DESCR_X_SIZE,
    #[doc = "0x70 - Channel descriptor X increment"]
    pub descr_x_incr: self::ch::DESCR_X_INCR,
    #[doc = "0x74 - Channel descriptor Y size"]
    pub descr_y_size: self::ch::DESCR_Y_SIZE,
    #[doc = "0x78 - Channel descriptor Y increment"]
    pub descr_y_incr: self::ch::DESCR_Y_INCR,
    #[doc = "0x7c - Channel descriptor next pointer"]
    pub descr_next: self::ch::DESCR_NEXT,
    #[doc = "0x80 - Interrupt"]
    pub intr: self::ch::INTR,
    #[doc = "0x84 - Interrupt set"]
    pub intr_set: self::ch::INTR_SET,
    #[doc = "0x88 - Interrupt mask"]
    pub intr_mask: self::ch::INTR_MASK,
    #[doc = "0x8c - Interrupt masked"]
    pub intr_masked: self::ch::INTR_MASKED,
}
#[doc = r"Register block"]
#[doc = "DMA controller channel"]
pub mod ch;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control"]
pub mod ctl;
#[doc = "Active channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active](active) module"]
pub type ACTIVE = crate::Reg<u32, _ACTIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTIVE;
#[doc = "`read()` method returns [active::R](active::R) reader structure"]
impl crate::Readable for ACTIVE {}
#[doc = "Active channels"]
pub mod active;
