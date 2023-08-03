#[doc = "Reader of register RXF0S"]
pub type R = crate::R<u32, super::RXF0S>;
#[doc = "Reader of field `F0FL`"]
pub type F0FL_R = crate::R<u8, u8>;
#[doc = "Reader of field `F0GI`"]
pub type F0GI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F0PI`"]
pub type F0PI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F0F`"]
pub type F0F_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0L`"]
pub type RF0L_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level Number of elements stored in Rx FIFO 0, range 0 to 64."]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index Rx FIFO 0 read index pointer, range 0 to 63. This field is updated by the software writing to RxF0A.F0AI"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index Rx FIFO 0 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
