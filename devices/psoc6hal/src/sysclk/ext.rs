use super::HalStatus ;

const MAX_FREQ:u32 = 100000000 ;
static mut PRIV_EXT_CLK_FREQ_HZ:u32 = 0 ;

pub fn set_freq(freq: u32) -> HalStatus  {
    if freq > MAX_FREQ
    {
        return Err(super::INVALID_EXT_FREQ) ;
    }

    unsafe {
        PRIV_EXT_CLK_FREQ_HZ = freq ;
    }

    return Ok(0) ;
}

pub fn get_frequency() -> u32 {
    let ret:u32 ;

    unsafe {
        ret = PRIV_EXT_CLK_FREQ_HZ ;
    }

    return ret ;
}