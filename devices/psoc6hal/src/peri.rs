pub mod hw ;

use super::sysclk::* ;

pub fn periph_assign_divider(clk: ClockType, dtype: DividerType, index: u8) {
    let peri = hw::get_peri() ;

    peri.clock_ctl[clk as usize].write(|w| unsafe { w.type_sel().bits(dtype as u8).div_sel().bits(index) }) ;
}


pub fn set_divider(dtype: DividerType, index: u8, div : u32) {
    let peri = hw::get_peri() ;

    match dtype {
        DividerType::EightBitDivider => {
            peri.div_8_ctl[index as usize].write(|w| unsafe { w.int8_div().bits(div as u8) }) ;
        },
        DividerType::SixteenBitDivider => {
            peri.div_16_ctl[index as usize].write(|w| unsafe { w.int16_div().bits(div as u16) }) ;
        }
        _ => unreachable!(),
    } ;
}

pub fn enable_divider(dtype: DividerType, index: u8) {
    let peri = hw::get_peri() ;

    match dtype {
        DividerType::EightBitDivider => {

            let dt = dtype as u8 ;
            peri.div_cmd.write(|w| unsafe { 
                w.enable().set_bit().pa_type_sel().bits(3).pa_div_sel().bits(0xff).div_sel().bits(index).type_sel().bits(dt) 
            }) ;

            while !peri.div_8_ctl[index as usize].read().en().bits()
            {
            }
        },
        _ => unreachable!(),
    } ;
}
