#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0300_000f"]
impl crate::ResetValue for super::CTRL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_000f
    }
}
#[doc = "Reader of field `OVS`"]
pub type OVS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OVS`"]
pub struct OVS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `EC_AM_MODE`"]
pub type EC_AM_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EC_AM_MODE`"]
pub struct EC_AM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EC_AM_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EC_OP_MODE`"]
pub type EC_OP_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EC_OP_MODE`"]
pub struct EC_OP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EC_OP_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `EZ_MODE`"]
pub type EZ_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EZ_MODE`"]
pub struct EZ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EZ_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BYTE_MODE`"]
pub type BYTE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTE_MODE`"]
pub struct BYTE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CMD_RESP_MODE`"]
pub type CMD_RESP_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_RESP_MODE`"]
pub struct CMD_RESP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_RESP_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ADDR_ACCEPT`"]
pub type ADDR_ACCEPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDR_ACCEPT`"]
pub struct ADDR_ACCEPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_ACCEPT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `BLOCK`"]
pub type BLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCK`"]
pub struct BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inter-Integrated Circuits (I2C) mode."]
    I2C = 0,
    #[doc = "1: Serial Peripheral Interface (SPI) mode."]
    SPI = 1,
    #[doc = "2: Universal Asynchronous Receiver/Transmitter (UART) mode."]
    UART = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::I2C),
            1 => Val(MODE_A::SPI),
            2 => Val(MODE_A::UART),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == MODE_A::I2C
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == MODE_A::SPI
    }
    #[doc = "Checks if the value of the field is `UART`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == MODE_A::UART
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Inter-Integrated Circuits (I2C) mode."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(MODE_A::I2C)
    }
    #[doc = "Serial Peripheral Interface (SPI) mode."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(MODE_A::SPI)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter (UART) mode."]
    #[inline(always)]
    pub fn uart(self) -> &'a mut W {
        self.variant(MODE_A::UART)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_am_mode(&self) -> EC_AM_MODE_R {
        EC_AM_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_op_mode(&self) -> EC_OP_MODE_R {
        EC_OP_MODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ez_mode(&self) -> EZ_MODE_R {
        EZ_MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    pub fn byte_mode(&self) -> BYTE_MODE_R {
        BYTE_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    pub fn cmd_resp_mode(&self) -> CMD_RESP_MODE_R {
        CMD_RESP_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    pub fn addr_accept(&self) -> ADDR_ACCEPT_R {
        ADDR_ACCEPT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn ovs(&mut self) -> OVS_W {
        OVS_W { w: self }
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_am_mode(&mut self) -> EC_AM_MODE_W {
        EC_AM_MODE_W { w: self }
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_op_mode(&mut self) -> EC_OP_MODE_W {
        EC_OP_MODE_W { w: self }
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ez_mode(&mut self) -> EZ_MODE_W {
        EZ_MODE_W { w: self }
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    pub fn byte_mode(&mut self) -> BYTE_MODE_W {
        BYTE_MODE_W { w: self }
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    pub fn cmd_resp_mode(&mut self) -> CMD_RESP_MODE_W {
        CMD_RESP_MODE_W { w: self }
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    pub fn addr_accept(&mut self) -> ADDR_ACCEPT_W {
        ADDR_ACCEPT_W { w: self }
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    pub fn block(&mut self) -> BLOCK_W {
        BLOCK_W { w: self }
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
