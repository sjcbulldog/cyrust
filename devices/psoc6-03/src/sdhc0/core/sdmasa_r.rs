#[doc = "Reader of register SDMASA_R"]
pub type R = crate::R<u32, super::SDMASA_R>;
#[doc = "Writer for register SDMASA_R"]
pub type W = crate::W<u32, super::SDMASA_R>;
#[doc = "Register SDMASA_R `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMASA_R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLOCKCNT_SDMASA`"]
pub type BLOCKCNT_SDMASA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BLOCKCNT_SDMASA`"]
pub struct BLOCKCNT_SDMASA_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKCNT_SDMASA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 32-bit Block Count (SDMA System Address) - SDMA System Address (Host Version 4 Enable = 0): This register contains the system memory address for an SDMA transfer in the 32-bit addressing mode. When the Host Controller stops an SDMA transfer, this register points to the system address of the next contiguous data position. It can be accessed only if no transaction is executing. Reading this register during data transfers may return an invalid value. - 32-bit Block Count (Host Version 4 Enable = 1): From the Host Controller Version 4.10 specification, this register is redefined as 32-bit Block Count. The Host Controller decrements the block count of this register for every block transfer and the data transfer stops when the count reaches zero. This register must be accessed when no transaction is executing. Reading this register during data transfers may return invalid value. Following are the values for BLOCKCNT_SDMASA: - 0xFFFF_FFFF - 4G - 1 Block - ...... - 0x0000_0002 - 2 Blocks - 0x0000_0001 - 1 Block - 0x0000_0000 - Stop Count Note: - When Host Version 4 Enable = 0, SDMA uses this register as system address and hence Auto CMD23 cannot be used with SDMA since this register is assigned for Auto CMD23 as 32-bit Block Count register. -When Host Version 4 Enable = 1, SDMA uses ADMA system address register and this register is reassigned to 32-bit Block Count. This register must be programmed with a non-zero value for data transfer if the 32-bit Block count register is used instead of the 16-bit Block count register. SDMA may use Auto CMD23 if 32-bit Block Count register is used."]
    #[inline(always)]
    pub fn blockcnt_sdmasa(&self) -> BLOCKCNT_SDMASA_R {
        BLOCKCNT_SDMASA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit Block Count (SDMA System Address) - SDMA System Address (Host Version 4 Enable = 0): This register contains the system memory address for an SDMA transfer in the 32-bit addressing mode. When the Host Controller stops an SDMA transfer, this register points to the system address of the next contiguous data position. It can be accessed only if no transaction is executing. Reading this register during data transfers may return an invalid value. - 32-bit Block Count (Host Version 4 Enable = 1): From the Host Controller Version 4.10 specification, this register is redefined as 32-bit Block Count. The Host Controller decrements the block count of this register for every block transfer and the data transfer stops when the count reaches zero. This register must be accessed when no transaction is executing. Reading this register during data transfers may return invalid value. Following are the values for BLOCKCNT_SDMASA: - 0xFFFF_FFFF - 4G - 1 Block - ...... - 0x0000_0002 - 2 Blocks - 0x0000_0001 - 1 Block - 0x0000_0000 - Stop Count Note: - When Host Version 4 Enable = 0, SDMA uses this register as system address and hence Auto CMD23 cannot be used with SDMA since this register is assigned for Auto CMD23 as 32-bit Block Count register. -When Host Version 4 Enable = 1, SDMA uses ADMA system address register and this register is reassigned to 32-bit Block Count. This register must be programmed with a non-zero value for data transfer if the 32-bit Block count register is used instead of the 16-bit Block count register. SDMA may use Auto CMD23 if 32-bit Block Count register is used."]
    #[inline(always)]
    pub fn blockcnt_sdmasa(&mut self) -> BLOCKCNT_SDMASA_W {
        BLOCKCNT_SDMASA_W { w: self }
    }
}
