use volatile_register::{RW, RO} ;

const CM4_PWR_CTL : u32 = 0x40201200 ;
const CM4_VECTOR_TABLE_ADDR : u32 = 0x40200200 ;
const CM4_STATUS:u32 = 0x40200004 ;

const CPUSS_CM4_PWR_CTL_VECTKEYSTAT_MSK:u32 = 0xFFFF0000; 
const CPUSS_CM4_PWR_CTL_VECTKEYSTAT_POS:u32 = 16;
const CPUSS_CM4_PWR_CTL_PWR_MODE_MSK:u32 = 3 ;
const CPUSS_CM4_STATUS_PWR_DONE_MSK:u32 = 0x10 ;
const CY_SYS_CM4_PWR_CTL_KEY_OPEN:u32 = 0x05FA ;
const CY_SYS_CM4_STATUS_ENABLED:u32 = 3 ;
const CY_SYS_CM4_STATUS_RESET:u32 = 1 ;

#[repr(C)]
struct CPUSS_CM4_POWER {
    pub cm4_pwr_ctl : RW<u32>,
}

#[repr(C)]
struct CPUSS_CM4_VECTOR {
    pub vector : RW<u32>,
}

#[repr(C)]
struct CPUSS_CM4_STATUS {
    pub status : RO<u32>,
}

pub fn get_cm4_status() -> u32 {
    let ctl = unsafe { &mut *(CM4_PWR_CTL as *mut CPUSS_CM4_POWER) } ;
    return ctl.cm4_pwr_ctl.read() & CPUSS_CM4_PWR_CTL_PWR_MODE_MSK ;
}

pub fn reset_cm4() {
    let ctl = unsafe { &mut *(CM4_PWR_CTL as *mut CPUSS_CM4_POWER) } ;
    let status = unsafe { &mut *(CM4_STATUS as *mut CPUSS_CM4_STATUS) } ;

    let mut v : u32 = 0 ;

    v &= !(CPUSS_CM4_PWR_CTL_VECTKEYSTAT_MSK | CPUSS_CM4_PWR_CTL_PWR_MODE_MSK) ;
    v |= CY_SYS_CM4_PWR_CTL_KEY_OPEN << CPUSS_CM4_PWR_CTL_VECTKEYSTAT_POS ;
    v |= CY_SYS_CM4_STATUS_RESET ;

    unsafe {
        ctl.cm4_pwr_ctl.write(v) ;
    }

    while status.status.read() & CPUSS_CM4_STATUS_PWR_DONE_MSK == 0
    {
    }
}

pub fn enable_cm4(addr: u32) {
    let vect = unsafe { &mut *(CM4_VECTOR_TABLE_ADDR as *mut CPUSS_CM4_VECTOR) } ;
    let ctl = unsafe { &mut *(CM4_PWR_CTL as *mut CPUSS_CM4_POWER) } ;
    let status = unsafe { &mut *(CM4_STATUS as *mut CPUSS_CM4_STATUS) } ;

    let st = get_cm4_status() ;

    if st == CY_SYS_CM4_STATUS_ENABLED
    {
        reset_cm4() ;
    }

    unsafe {
        vect.vector.write(addr) ;
    }

    let mut v : u32 = 0 ;

    v &= !(CPUSS_CM4_PWR_CTL_VECTKEYSTAT_MSK | CPUSS_CM4_PWR_CTL_PWR_MODE_MSK) ;
    v |= CY_SYS_CM4_PWR_CTL_KEY_OPEN << CPUSS_CM4_PWR_CTL_VECTKEYSTAT_POS ;
    v |= CY_SYS_CM4_STATUS_ENABLED ;

    unsafe {
        ctl.cm4_pwr_ctl.write(v) ;
    }

    while status.status.read() & CPUSS_CM4_STATUS_PWR_DONE_MSK == 0
    {
    }
}
