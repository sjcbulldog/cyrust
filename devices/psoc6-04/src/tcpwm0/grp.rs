#[doc = r"Register block"]
#[repr(C)]
pub struct CNT {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: self::cnt::CTRL,
    #[doc = "0x04 - Counter status register"]
    pub status: self::cnt::STATUS,
    #[doc = "0x08 - Counter count register"]
    pub counter: self::cnt::COUNTER,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Counter compare/capture 0 register"]
    pub cc0: self::cnt::CC0,
    #[doc = "0x14 - Counter buffered compare/capture 0 register"]
    pub cc0_buff: self::cnt::CC0_BUFF,
    #[doc = "0x18 - Counter compare/capture 1 register"]
    pub cc1: self::cnt::CC1,
    #[doc = "0x1c - Counter buffered compare/capture 1 register"]
    pub cc1_buff: self::cnt::CC1_BUFF,
    #[doc = "0x20 - Counter period register"]
    pub period: self::cnt::PERIOD,
    #[doc = "0x24 - Counter buffered period register"]
    pub period_buff: self::cnt::PERIOD_BUFF,
    #[doc = "0x28 - Counter line selection register"]
    pub line_sel: self::cnt::LINE_SEL,
    #[doc = "0x2c - Counter buffered line selection register"]
    pub line_sel_buff: self::cnt::LINE_SEL_BUFF,
    #[doc = "0x30 - Counter PWM dead time register"]
    pub dt: self::cnt::DT,
    _reserved12: [u8; 12usize],
    #[doc = "0x40 - Counter trigger command register"]
    pub tr_cmd: self::cnt::TR_CMD,
    #[doc = "0x44 - Counter input trigger selection register 0"]
    pub tr_in_sel0: self::cnt::TR_IN_SEL0,
    #[doc = "0x48 - Counter input trigger selection register 1"]
    pub tr_in_sel1: self::cnt::TR_IN_SEL1,
    #[doc = "0x4c - Counter input trigger edge selection register"]
    pub tr_in_edge_sel: self::cnt::TR_IN_EDGE_SEL,
    #[doc = "0x50 - Counter trigger PWM control register"]
    pub tr_pwm_ctrl: self::cnt::TR_PWM_CTRL,
    #[doc = "0x54 - Counter output trigger selection register"]
    pub tr_out_sel: self::cnt::TR_OUT_SEL,
    _reserved18: [u8; 24usize],
    #[doc = "0x70 - Interrupt request register"]
    pub intr: self::cnt::INTR,
    #[doc = "0x74 - Interrupt set request register"]
    pub intr_set: self::cnt::INTR_SET,
    #[doc = "0x78 - Interrupt mask register"]
    pub intr_mask: self::cnt::INTR_MASK,
    #[doc = "0x7c - Interrupt masked request register"]
    pub intr_masked: self::cnt::INTR_MASKED,
}
#[doc = r"Register block"]
#[doc = "Timer/Counter/PWM Counter Module"]
pub mod cnt;
