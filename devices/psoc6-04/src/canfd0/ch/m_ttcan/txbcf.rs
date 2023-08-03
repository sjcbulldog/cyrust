#[doc = "Reader of register TXBCF"]
pub type R = crate::R<u32, super::TXBCF>;
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Finished Each Tx Buffer has its own Cancellation Finished bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0= No transmit buffer cancellation 1= Transmit buffer cancellation finished"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
