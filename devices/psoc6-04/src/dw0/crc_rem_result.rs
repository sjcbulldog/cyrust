#[doc = "Reader of register CRC_REM_RESULT"]
pub type R = crate::R<u32, super::CRC_REM_RESULT>;
#[doc = "Reader of field `REM`"]
pub type REM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remainder value. The alignment of the remainder depends on CRC_REM_CTL0.REM_REVERSE: '0': the more significant bits (bit 31 and down) contain the remainder. '1': the less significant bits (bit 0 and up) contain the remainder. Note: This field is combinatorially derived from CRC_LFSR_CTL.LFSR32, CRC_CTL.REM_REVERSE and CRC_REM_CTL.REM_XOR."]
    #[inline(always)]
    pub fn rem(&self) -> REM_R {
        REM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
