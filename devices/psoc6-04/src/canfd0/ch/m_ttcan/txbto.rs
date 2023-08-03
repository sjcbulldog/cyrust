#[doc = "Reader of register TXBTO"]
pub type R = crate::R<u32, super::TXBTO>;
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Occurred Each Tx Buffer has its own Transmission Occurred bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0= No transmission occurred 1= Transmission occurred"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
