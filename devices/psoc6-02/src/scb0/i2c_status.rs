#[doc = "Reader of register I2C_STATUS"]
pub type R = crate::R<u32, super::I2C_STATUS>;
#[doc = "Reader of field `BUS_BUSY`"]
pub type BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_EC_BUSY`"]
pub type I2C_EC_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `S_READ`"]
pub type S_READ_R = crate::R<bool, bool>;
#[doc = "Reader of field `M_READ`"]
pub type M_READ_R = crate::R<bool, bool>;
#[doc = "Reader of field `CURR_EZ_ADDR`"]
pub type CURR_EZ_ADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `BASE_EZ_ADDR`"]
pub type BASE_EZ_ADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - I2C bus is busy. The bus is considered busy ('1'), from the time a START is detected or from the time the SCL line is '0'. The bus is considered idle ('0'), from the time a STOP is detected. If the IP is disabled, BUS_BUSY is '0'. After enabling the IP, it takes time for the BUS_BUSY to detect a busy bus. This time is the maximum high time of the SCL line. For a 100 kHz interface frequency, this maximum high time may last roughly 5 us (half a bit period). For single master systems, BUS_BUSY does not have to be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START (no bus collisions). For multi-master systems, BUS_BUSY can be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START_ON_IDLE (to prevent bus collisions)."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_EZ_ADDR or CURR_EZ_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_EZ_ADDR and CURR_EZ_ADDR are reliable."]
    #[inline(always)]
    pub fn i2c_ec_busy(&self) -> I2C_EC_BUSY_R {
        I2C_EC_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C slave read transfer ('1') or I2C slave write transfer ('0'). When the I2C slave is inactive/idle or receiving START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub fn s_read(&self) -> S_READ_R {
        S_READ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C master read transfer ('1') or I2C master write transfer ('0'). When the I2C master is inactive/idle or transmitting START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub fn m_read(&self) -> M_READ_R {
        M_READ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - I2C slave current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when I2C_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn curr_ez_addr(&self) -> CURR_EZ_ADDR_R {
        CURR_EZ_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C slave base EZ address. Address as provided by an I2C write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn base_ez_addr(&self) -> BASE_EZ_ADDR_R {
        BASE_EZ_ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
