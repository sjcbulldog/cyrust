#[doc = "Reader of register TTCPT"]
pub type R = crate::R<u32, super::TTCPT>;
#[doc = "Reader of field `CCV`"]
pub type CCV_R = crate::R<u8, u8>;
#[doc = "Reader of field `SWV`"]
pub type SWV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:5 - Cycle Count Value Cycle count value captured together with SWV. 0x00-3F Captured cycle count value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Stop Watch Value On a rising/falling edge (as configured via TTOCN.SWP) at the Stop Watch Trigger pin m_ttcan_swt, when TTOCN.SWS is != '00' and TTIR.SWE is '0', the actual time value as selected by TTOCN.SWS (cycle, local, global) is copied to SWV and TTIR.SWE will be set to '1'. Capturing of the next stop watch value is enabled by resetting TTIR.SWE. 0x0000-FFFF Captured Stop Watch value"]
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
