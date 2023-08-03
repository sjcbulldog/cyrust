#[doc = "Reader of register TX_FIFO_STATUS"]
pub type R = crate::R<u32, super::TX_FIFO_STATUS>;
#[doc = "Reader of field `USED`"]
pub type USED_R = crate::R<u16, u16>;
#[doc = "Reader of field `SR_VALID`"]
pub type SR_VALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `RD_PTR`"]
pub type RD_PTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `WR_PTR`"]
pub type WR_PTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:8 - Amount of entries in the transmitter FIFO. The value of this field ranges from 0 to FF_DATA_NR (EZ_DATA_NR/2)."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Indicates whether the TX shift registers holds a valid data frame ('1') or not ('0'). The shift register can be considered the top of the TX FIFO (the data frame is not included in the USED field of the TX FIFO). The shift register is a working register and holds the data frame that is currently transmitted (when the protocol state machine is transmitting a data frame) or the data frame that is transmitted next (when the protocol state machine is not transmitting a data frame)."]
    #[inline(always)]
    pub fn sr_valid(&self) -> SR_VALID_R {
        SR_VALID_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - FIFO read pointer: FIFO location from which a data frame is read by the hardware."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FIFO write pointer: FIFO location at which a new data frame is written."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
