#[doc = "Reader of register CURR_CAPABILITIES2_R"]
pub type R = crate::R<u32, super::CURR_CAPABILITIES2_R>;
#[doc = "Reader of field `MAX_CUR_VDD2_18V`"]
pub type MAX_CUR_VDD2_18V_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 1.8V VDD2 This bit specifies the Maximum Current for 1.8V VDD2 power supply for the UHS-II card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_vdd2_18v(&self) -> MAX_CUR_VDD2_18V_R {
        MAX_CUR_VDD2_18V_R::new((self.bits & 0xff) as u8)
    }
}
