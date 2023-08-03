#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `COMPLETION`"]
pub type COMPLETION_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC_BUS_ERROR`"]
pub type SRC_BUS_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DST_BUS_ERROR`"]
pub type DST_BUS_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRC_MISAL`"]
pub type SRC_MISAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DST_MISAL`"]
pub type DST_MISAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CURR_PTR_NULL`"]
pub type CURR_PTR_NULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE_CH_DISABLED`"]
pub type ACTIVE_CH_DISABLED_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESCR_BUS_ERROR`"]
pub type DESCR_BUS_ERROR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn completion(&self) -> COMPLETION_R {
        COMPLETION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SRC_BUS_ERROR_R {
        SRC_BUS_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DST_BUS_ERROR_R {
        DST_BUS_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn src_misal(&self) -> SRC_MISAL_R {
        SRC_MISAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn dst_misal(&self) -> DST_MISAL_R {
        DST_MISAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CURR_PTR_NULL_R {
        CURR_PTR_NULL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ACTIVE_CH_DISABLED_R {
        ACTIVE_CH_DISABLED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DESCR_BUS_ERROR_R {
        DESCR_BUS_ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
