#[doc = "Reader of register CQCRA"]
pub type R = crate::R<u32, super::CQCRA>;
#[doc = "Reader of field `CMD_RESP_ARG`"]
pub type CMD_RESP_ARG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last Command Response argument This field stores the argument of the last received command response. Controller updates the value every time a command response is received."]
    #[inline(always)]
    pub fn cmd_resp_arg(&self) -> CMD_RESP_ARG_R {
        CMD_RESP_ARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
