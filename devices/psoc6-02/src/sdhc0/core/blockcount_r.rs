#[doc = "Reader of register BLOCKCOUNT_R"]
pub type R = crate::R<u16, super::BLOCKCOUNT_R>;
#[doc = "Writer for register BLOCKCOUNT_R"]
pub type W = crate::W<u16, super::BLOCKCOUNT_R>;
#[doc = "Register BLOCKCOUNT_R `reset()`'s with value 0"]
impl crate::ResetValue for super::BLOCKCOUNT_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `BLOCK_CNT`"]
pub type BLOCK_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLOCK_CNT`"]
pub struct BLOCK_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    pub fn block_cnt(&self) -> BLOCK_CNT_R {
        BLOCK_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    pub fn block_cnt(&mut self) -> BLOCK_CNT_W {
        BLOCK_CNT_W { w: self }
    }
}
