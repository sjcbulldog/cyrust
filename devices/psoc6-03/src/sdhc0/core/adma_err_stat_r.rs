#[doc = "Reader of register ADMA_ERR_STAT_R"]
pub type R = crate::R<u8, super::ADMA_ERR_STAT_R>;
#[doc = "Reader of field `ADMA_ERR_STATES`"]
pub type ADMA_ERR_STATES_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADMA_LEN_ERR`"]
pub type ADMA_LEN_ERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - ADMA Error States These bits indicate the state of ADMA when an error occurs during ADMA data transfer. Values: - 0x0 (ST_STOP): Stop DMA - SYS_ADR register points to a location next to the error descriptor - 0x1 (ST_FDS): Fetch Descriptor - SYS_ADR register points to the error descriptor - 0x2 (UNUSED): Never set this state - 0x3 (ST_TFR): Transfer Data - SYS_ADR register points to a location next to the error descriptor"]
    #[inline(always)]
    pub fn adma_err_states(&self) -> ADMA_ERR_STATES_R {
        ADMA_ERR_STATES_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error States This error occurs in the following instances: - While the Block Count Enable is being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length - When the total data length cannot be divided by the block length Values: - 0x0 (NO_ERR): No Error - 0x1 (ERROR): Error"]
    #[inline(always)]
    pub fn adma_len_err(&self) -> ADMA_LEN_ERR_R {
        ADMA_LEN_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
