#[doc = "Reader of register ECR"]
pub type R = crate::R<u32, super::ECR>;
#[doc = "Reader of field `TEC`"]
pub type TEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `REC`"]
pub type REC_R = crate::R<u8, u8>;
#[doc = "Reader of field `RP`"]
pub type RP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CEL`"]
pub type CEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter Actual state of the Transmit Error Counter, values between 0 and 255"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter Actual state of the Receive Error Counter, values between 0 and 127"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive 0= The Receive Error Counter is below the error passive level of 128 1= The Receive Error Counter has reached the error passive level of 128"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - CAN Error Logging The counter is incremented each time when a CAN protocol error causes the Transmit Error Counter or the Receive Error Counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO."]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
