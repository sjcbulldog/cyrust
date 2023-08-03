#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Programmable protection structure pair"]
    pub ppu_pr: [PPU_PR; 8],
    _reserved1: [u8; 1536usize],
    #[doc = "0x800 - Fixed protection structure pair"]
    pub ppu_fx: [PPU_FX; 226],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_PR {
    #[doc = "0x00 - Slave region, base address"]
    pub sl_addr: self::ppu_pr::SL_ADDR,
    #[doc = "0x04 - Slave region, size"]
    pub sl_size: self::ppu_pr::SL_SIZE,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Slave attributes 0"]
    pub sl_att0: self::ppu_pr::SL_ATT0,
    #[doc = "0x14 - Slave attributes 1"]
    pub sl_att1: self::ppu_pr::SL_ATT1,
    #[doc = "0x18 - Slave attributes 2"]
    pub sl_att2: self::ppu_pr::SL_ATT2,
    #[doc = "0x1c - Slave attributes 3"]
    pub sl_att3: self::ppu_pr::SL_ATT3,
    #[doc = "0x20 - Master region, base address"]
    pub ms_addr: self::ppu_pr::MS_ADDR,
    #[doc = "0x24 - Master region, size"]
    pub ms_size: self::ppu_pr::MS_SIZE,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - Master attributes 0"]
    pub ms_att0: self::ppu_pr::MS_ATT0,
    #[doc = "0x34 - Master attributes 1"]
    pub ms_att1: self::ppu_pr::MS_ATT1,
    #[doc = "0x38 - Master attributes 2"]
    pub ms_att2: self::ppu_pr::MS_ATT2,
    #[doc = "0x3c - Master attributes 3"]
    pub ms_att3: self::ppu_pr::MS_ATT3,
}
#[doc = r"Register block"]
#[doc = "Programmable protection structure pair"]
pub mod ppu_pr;
#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_FX {
    #[doc = "0x00 - Slave region, base address"]
    pub sl_addr: self::ppu_fx::SL_ADDR,
    #[doc = "0x04 - Slave region, size"]
    pub sl_size: self::ppu_fx::SL_SIZE,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Slave attributes 0"]
    pub sl_att0: self::ppu_fx::SL_ATT0,
    #[doc = "0x14 - Slave attributes 1"]
    pub sl_att1: self::ppu_fx::SL_ATT1,
    #[doc = "0x18 - Slave attributes 2"]
    pub sl_att2: self::ppu_fx::SL_ATT2,
    #[doc = "0x1c - Slave attributes 3"]
    pub sl_att3: self::ppu_fx::SL_ATT3,
    #[doc = "0x20 - Master region, base address"]
    pub ms_addr: self::ppu_fx::MS_ADDR,
    #[doc = "0x24 - Master region, size"]
    pub ms_size: self::ppu_fx::MS_SIZE,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - Master attributes 0"]
    pub ms_att0: self::ppu_fx::MS_ATT0,
    #[doc = "0x34 - Master attributes 1"]
    pub ms_att1: self::ppu_fx::MS_ATT1,
    #[doc = "0x38 - Master attributes 2"]
    pub ms_att2: self::ppu_fx::MS_ATT2,
    #[doc = "0x3c - Master attributes 3"]
    pub ms_att3: self::ppu_fx::MS_ATT3,
}
#[doc = r"Register block"]
#[doc = "Fixed protection structure pair"]
pub mod ppu_fx;
