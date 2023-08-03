#[doc = "Reader of register CURR_CAPABILITIES1_R"]
pub type R = crate::R<u32, super::CURR_CAPABILITIES1_R>;
#[doc = "Reader of field `MAX_CUR_33V`"]
pub type MAX_CUR_33V_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAX_CUR_30V`"]
pub type MAX_CUR_30V_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAX_CUR_18V`"]
pub type MAX_CUR_18V_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V This bit specifies the Maximum Current for 3.3V VDD1 power supply for the card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_33v(&self) -> MAX_CUR_33V_R {
        MAX_CUR_33V_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V This bit specifies the Maximum Current for 3.0V VDD1 power supply for the card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_30v(&self) -> MAX_CUR_30V_R {
        MAX_CUR_30V_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V This bit specifies the Maximum Current for 1.8V VDD1 power supply for the card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_18v(&self) -> MAX_CUR_18V_R {
        MAX_CUR_18V_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
