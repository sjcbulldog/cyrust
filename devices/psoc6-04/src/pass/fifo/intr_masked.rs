#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `FIFO_LEVEL`"]
pub type FIFO_LEVEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFO_OVERFLOW`"]
pub type FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFO_UNDERFLOW`"]
pub type FIFO_UNDERFLOW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn fifo_level(&self) -> FIFO_LEVEL_R {
        FIFO_LEVEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn fifo_overflow(&self) -> FIFO_OVERFLOW_R {
        FIFO_OVERFLOW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn fifo_underflow(&self) -> FIFO_UNDERFLOW_R {
        FIFO_UNDERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
