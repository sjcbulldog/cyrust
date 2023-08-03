#[doc = "Reader of register DESCR_CTL"]
pub type R = crate::R<u32, super::DESCR_CTL>;
#[doc = "Reader of field `WAIT_FOR_DEACT`"]
pub type WAIT_FOR_DEACT_R = crate::R<u8, u8>;
#[doc = "Reader of field `INTR_TYPE`"]
pub type INTR_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `TR_OUT_TYPE`"]
pub type TR_OUT_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `TR_IN_TYPE`"]
pub type TR_IN_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `DATA_PREFETCH`"]
pub type DATA_PREFETCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATA_SIZE`"]
pub type DATA_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `CH_DISABLE`"]
pub type CH_DISABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC_TRANSFER_SIZE`"]
pub type SRC_TRANSFER_SIZE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DST_TRANSFER_SIZE`"]
pub type DST_TRANSFER_SIZE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESCR_TYPE`"]
pub type DESCR_TYPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn wait_for_deact(&self) -> WAIT_FOR_DEACT_R {
        WAIT_FOR_DEACT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - N/A"]
    #[inline(always)]
    pub fn intr_type(&self) -> INTR_TYPE_R {
        INTR_TYPE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - N/A"]
    #[inline(always)]
    pub fn tr_out_type(&self) -> TR_OUT_TYPE_R {
        TR_OUT_TYPE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn tr_in_type(&self) -> TR_IN_TYPE_R {
        TR_IN_TYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn data_prefetch(&self) -> DATA_PREFETCH_R {
        DATA_PREFETCH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn data_size(&self) -> DATA_SIZE_R {
        DATA_SIZE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn ch_disable(&self) -> CH_DISABLE_R {
        CH_DISABLE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn src_transfer_size(&self) -> SRC_TRANSFER_SIZE_R {
        SRC_TRANSFER_SIZE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - N/A"]
    #[inline(always)]
    pub fn dst_transfer_size(&self) -> DST_TRANSFER_SIZE_R {
        DST_TRANSFER_SIZE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - N/A"]
    #[inline(always)]
    pub fn descr_type(&self) -> DESCR_TYPE_R {
        DESCR_TYPE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
