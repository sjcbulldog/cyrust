#[doc = "Reader of register CQCRI"]
pub type R = crate::R<u32, super::CQCRI>;
#[doc = "Reader of field `CMD_RESP_INDX`"]
pub type CMD_RESP_INDX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Last Command Response index This field stores the index of the last received command response. Controller updates the value every time a command response is received."]
    #[inline(always)]
    pub fn cmd_resp_indx(&self) -> CMD_RESP_INDX_R {
        CMD_RESP_INDX_R::new((self.bits & 0x3f) as u8)
    }
}
