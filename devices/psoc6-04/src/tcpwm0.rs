use grp::CNT ; 

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Group of counters"]
    pub grp0: GRP,
    _reserved1: [u8; 32640usize],
    #[doc = "0x8000 - Group of counters"]
    pub grp1: GRP,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GRP {
    #[doc = "0x00 - Timer/Counter/PWM Counter Module"]
    pub cnt: [CNT; 8],
}
#[doc = r"Register block"]
#[doc = "Group of counters"]
pub mod grp;
