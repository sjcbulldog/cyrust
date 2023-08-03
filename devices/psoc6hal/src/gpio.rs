pub mod hw ;

use super::* ;

pub struct Gpio {
    pub port: u8,
    pub pin: u8
}

pub enum DriveMode {
    Analog = 0,
    PullupInOff = 2,
    PullDownInOff = 3,
    OpenDrainDrivesLowInOff = 4,
    OpenDrainDrivesHighInOff = 5,
    StrongInOff = 6,
    PullUpDownInOff = 7,
    HighZ = 8,
    PullUp = 10,
    PullDown = 11,
    OpenDrainDrivesLow = 12,
    OpenDrainDrivesHigh = 13,
    Strong = 14,
    PullUpDown = 15
}

pub enum Trigger {
    Disable = 0,
    Rising = 1,
    Falling = 2,
    Both = 3
}

impl Gpio {
    pub fn set_drive_mode(&mut self, dm: DriveMode) {
        let prt = hw::get_hw(self.port) ;
        let dmval = dm as u8 ;

        let mval: u8 = (dmval as u8) & 0x07 ;
        let inp: bool = (((dmval as u8) >> 3) & 1) == 1 ;

        #[allow(unused_unsafe)]
        match self.pin {
            0 => prt.cfg.modify(|_, w| unsafe { w.in_en0().bit(inp).drive_mode0().bits(mval)}), 
            1 => prt.cfg.modify(|_, w| unsafe { w.in_en1().bit(inp).drive_mode1().bits(mval)}), 
            2 => prt.cfg.modify(|_, w| unsafe { w.in_en2().bit(inp).drive_mode2().bits(mval)}), 
            3 => prt.cfg.modify(|_, w| unsafe { w.in_en3().bit(inp).drive_mode3().bits(mval)}), 
            4 => prt.cfg.modify(|_, w| unsafe { w.in_en4().bit(inp).drive_mode4().bits(mval)}), 
            5 => prt.cfg.modify(|_, w| unsafe { w.in_en5().bit(inp).drive_mode5().bits(mval)}), 
            6 => prt.cfg.modify(|_, w| unsafe { w.in_en6().bit(inp).drive_mode6().bits(mval)}), 
            7 => prt.cfg.modify(|_, w| unsafe { w.in_en7().bit(inp).drive_mode7().bits(mval)}),                                                                                     
            _ => unreachable!(),
        }
    }

    pub fn set_output(&mut self, out: bool) {
        let prt = hw::get_hw(self.port) ;

        let mut v:u32 = prt.out.read().bits() ;

        if out
        {
            v |= 1 << self.pin ;
        }
        else
        {
            v &= !(1 << self.pin) ;
        }

        prt.out.write(|w| unsafe { w.bits(v) }) ;
    }

    pub fn get_output(&mut self) -> bool {
        let prt = hw::get_hw(self.port) ;

        let mut v:u32 = prt.out.read().bits() ;
        v &= 1 << self.pin ;
        return v != 0 ;
    }

    pub fn get_input(&mut self) -> bool {
        let prt = hw::get_hw(self.port) ;

        let mut v:u32 = prt.in_.read().bits() ;
        v &= 1 << self.pin ;
        return v != 0 ;
    }

    pub fn set_hsioim(&mut self, value: u8) -> u32 {
        let prt = hsiom::hw::get_hsiom(self.port) ;

        let mut u:u32 = 0 ;
        let v:u32  ;

        match self.pin {
            #[allow(unused_unsafe)]
            0 => prt.port_sel0.modify(|_, w| unsafe { w.io0_sel().bits(value) }),
            1 => prt.port_sel0.modify(|_, w| unsafe { w.io1_sel().bits(value) }),
            2 => prt.port_sel0.modify(|_, w| unsafe { w.io2_sel().bits(value) }),
            3 => prt.port_sel0.modify(|_, w| unsafe { w.io3_sel().bits(value) }),
            4 => prt.port_sel1.modify(|_, w| unsafe { w.io4_sel().bits(value) }),
            5 => prt.port_sel1.modify(|_, w| unsafe { w.io5_sel().bits(value) }),
            6 => prt.port_sel1.modify(|_, w| unsafe { w.io6_sel().bits(value) }),
            7 => prt.port_sel1.modify(|_, w| unsafe { w.io7_sel().bits(value) }),
            _ => unreachable!(),
        } ;

        v = prt.port_sel0.read().bits() ;

        for _ in 0 .. 3
        {
            u += v ;
        }

        return u ;
    }

    pub fn set_interrrupt_edge(&mut self, edge: Trigger) {
        let prt = hw::get_hw(self.port) ;
 
        let mut v = prt.intr_cfg.read().bits() ;
        v &= 0x03 << (self.pin * 2) ;
        v |= (edge as u32) << (self.pin * 2) ;
        prt.intr_cfg.write(|w| unsafe { w.bits(v) }) ;
    }

    pub fn set_interrupt_mask(&mut self, enable: bool) {
        let prt = hw::get_hw(self.port) ;

        let mut v = prt.intr_mask.read().bits() ;
        v &= 0x01 << self.pin ;
        v |= (enable as u32) << self.pin ;
        prt.intr_mask.write(|w| unsafe { w.bits(v) }) ;
    }

    pub fn get_interrupt_mask(&mut self) -> bool {
        let prt = hw::get_hw(self.port) ;

        let mut v = prt.intr_mask.read().bits() ;
        v &= 0x01 << self.pin ;
        return v != 0 ;
    }

    pub fn clear_interrupt(&mut self) {
        let prt = hw::get_hw(self.port) ;

        let mut v = prt.intr.read().bits() ;
        v |= 1 << self.pin ;
        prt.intr.write(|w| unsafe { w.bits(v)}) ;
    }
}
