#[doc = "Reader of register SAR_TR_SCAN_CNT_STATUS[%s]"]
pub type R = crate::R<u32, super::SAR_TR_SCAN_CNT_STATUS>;
#[doc = "Reader of field `SCAN_CNT_STATUS`"]
pub type SCAN_CNT_STATUS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - A read from this register returns the current sample count (possible values are 1 through SCAN_TR_SCAN_CNT.SCAN_CNT+1). This field cannot be read if SAR_CLOCK_SEL.CLOCK_SEL =1."]
    #[inline(always)]
    pub fn scan_cnt_status(&self) -> SCAN_CNT_STATUS_R {
        SCAN_CNT_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
