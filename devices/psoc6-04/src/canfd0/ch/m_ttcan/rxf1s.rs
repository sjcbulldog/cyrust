#[doc = "Reader of register RXF1S"]
pub type R = crate::R<u32, super::RXF1S>;
#[doc = "Reader of field `F1FL`"]
pub type F1FL_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1GI`"]
pub type F1GI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1PI`"]
pub type F1PI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1F`"]
pub type F1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1L`"]
pub type RF1L_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMS`"]
pub type DMS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level Number of elements stored in Rx FIFO 1, range 0 to 64."]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 1 Get Index Rx FIFO 1 read index pointer, range 0 to 63. This field is updated by the software writing to RxF1A.FAI"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 1 Put Index Rx FIFO 1 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status 00= Idle state, wait for reception of debug messages, DMA request is cleared 01= Debug message A received 10= Debug messages A, B received 11= Debug messages A, B, C received, DMA request is set"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
