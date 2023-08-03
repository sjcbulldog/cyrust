#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPC structure"]
    pub struct_: [STRUCT; 16],
    _reserved1: [u8; 3584usize],
    #[doc = "0x1000 - IPC interrupt structure"]
    pub intr_struct0: INTR_STRUCT,
    _reserved2: [u8; 16usize],
    #[doc = "0x1020 - IPC interrupt structure"]
    pub intr_struct1: INTR_STRUCT,
    _reserved3: [u8; 16usize],
    #[doc = "0x1040 - IPC interrupt structure"]
    pub intr_struct2: INTR_STRUCT,
    _reserved4: [u8; 16usize],
    #[doc = "0x1060 - IPC interrupt structure"]
    pub intr_struct3: INTR_STRUCT,
    _reserved5: [u8; 16usize],
    #[doc = "0x1080 - IPC interrupt structure"]
    pub intr_struct4: INTR_STRUCT,
    _reserved6: [u8; 16usize],
    #[doc = "0x10a0 - IPC interrupt structure"]
    pub intr_struct5: INTR_STRUCT,
    _reserved7: [u8; 16usize],
    #[doc = "0x10c0 - IPC interrupt structure"]
    pub intr_struct6: INTR_STRUCT,
    _reserved8: [u8; 16usize],
    #[doc = "0x10e0 - IPC interrupt structure"]
    pub intr_struct7: INTR_STRUCT,
    _reserved9: [u8; 16usize],
    #[doc = "0x1100 - IPC interrupt structure"]
    pub intr_struct8: INTR_STRUCT,
    _reserved10: [u8; 16usize],
    #[doc = "0x1120 - IPC interrupt structure"]
    pub intr_struct9: INTR_STRUCT,
    _reserved11: [u8; 16usize],
    #[doc = "0x1140 - IPC interrupt structure"]
    pub intr_struct10: INTR_STRUCT,
    _reserved12: [u8; 16usize],
    #[doc = "0x1160 - IPC interrupt structure"]
    pub intr_struct11: INTR_STRUCT,
    _reserved13: [u8; 16usize],
    #[doc = "0x1180 - IPC interrupt structure"]
    pub intr_struct12: INTR_STRUCT,
    _reserved14: [u8; 16usize],
    #[doc = "0x11a0 - IPC interrupt structure"]
    pub intr_struct13: INTR_STRUCT,
    _reserved15: [u8; 16usize],
    #[doc = "0x11c0 - IPC interrupt structure"]
    pub intr_struct14: INTR_STRUCT,
    _reserved16: [u8; 16usize],
    #[doc = "0x11e0 - IPC interrupt structure"]
    pub intr_struct15: INTR_STRUCT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    #[doc = "0x00 - IPC acquire"]
    pub acquire: self::struct_::ACQUIRE,
    #[doc = "0x04 - IPC release"]
    pub release: self::struct_::RELEASE,
    #[doc = "0x08 - IPC notification"]
    pub notify: self::struct_::NOTIFY,
    #[doc = "0x0c - IPC data 0"]
    pub data0: self::struct_::DATA0,
    #[doc = "0x10 - IPC data 1"]
    pub data1: self::struct_::DATA1,
    _reserved5: [u8; 8usize],
    #[doc = "0x1c - IPC lock status"]
    pub lock_status: self::struct_::LOCK_STATUS,
}
#[doc = r"Register block"]
#[doc = "IPC structure"]
pub mod struct_;
#[doc = r"Register block"]
#[repr(C)]
pub struct INTR_STRUCT {
    #[doc = "0x00 - Interrupt"]
    pub intr: self::intr_struct::INTR,
    #[doc = "0x04 - Interrupt set"]
    pub intr_set: self::intr_struct::INTR_SET,
    #[doc = "0x08 - Interrupt mask"]
    pub intr_mask: self::intr_struct::INTR_MASK,
    #[doc = "0x0c - Interrupt masked"]
    pub intr_masked: self::intr_struct::INTR_MASKED,
}
#[doc = r"Register block"]
#[doc = "IPC interrupt structure"]
pub mod intr_struct;
