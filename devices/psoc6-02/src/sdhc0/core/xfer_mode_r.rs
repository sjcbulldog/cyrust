#[doc = "Reader of register XFER_MODE_R"]
pub type R = crate::R<u16, super::XFER_MODE_R>;
#[doc = "Writer for register XFER_MODE_R"]
pub type W = crate::W<u16, super::XFER_MODE_R>;
#[doc = "Register XFER_MODE_R `reset()`'s with value 0"]
impl crate::ResetValue for super::XFER_MODE_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `DMA_ENABLE`"]
pub type DMA_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ENABLE`"]
pub struct DMA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `BLOCK_COUNT_ENABLE`"]
pub type BLOCK_COUNT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCK_COUNT_ENABLE`"]
pub struct BLOCK_COUNT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_COUNT_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AUTO_CMD_ENABLE`"]
pub type AUTO_CMD_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AUTO_CMD_ENABLE`"]
pub struct AUTO_CMD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CMD_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DATA_XFER_DIR`"]
pub type DATA_XFER_DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_XFER_DIR`"]
pub struct DATA_XFER_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_XFER_DIR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MULTI_BLK_SEL`"]
pub type MULTI_BLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MULTI_BLK_SEL`"]
pub struct MULTI_BLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTI_BLK_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RESP_TYPE`"]
pub type RESP_TYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESP_TYPE`"]
pub struct RESP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_TYPE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RESP_ERR_CHK_ENABLE`"]
pub type RESP_ERR_CHK_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESP_ERR_CHK_ENABLE`"]
pub struct RESP_ERR_CHK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_ERR_CHK_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RESP_INT_DISABLE`"]
pub type RESP_INT_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESP_INT_DISABLE`"]
pub struct RESP_INT_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_INT_DISABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: - 0x1 (ENABLED): DMA Data transfer - 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
    #[inline(always)]
    pub fn dma_enable(&self) -> DMA_ENABLE_R {
        DMA_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. When 16-bit Block Count register is used, the Host Driver can set this bit to 0 in ADMA2 mode to enable larger data transfer than the maximum of 65535 block counts supported by the 16-bit Block Count register."]
    #[inline(always)]
    pub fn block_count_enable(&self) -> BLOCK_COUNT_ENABLE_R {
        BLOCK_COUNT_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: - 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled - 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable - 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable - 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Select"]
    #[inline(always)]
    pub fn auto_cmd_enable(&self) -> AUTO_CMD_ENABLE_R {
        AUTO_CMD_ENABLE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: - 0x1 (READ): Read (Card to Host) - 0x0 (WRITE): Write (Host to Card)"]
    #[inline(always)]
    pub fn data_xfer_dir(&self) -> DATA_XFER_DIR_R {
        DATA_XFER_DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register."]
    #[inline(always)]
    pub fn multi_blk_sel(&self) -> MULTI_BLK_SEL_R {
        MULTI_BLK_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: - OUT_OF_RANGE - ADDRESS_ERROR - BLOCK_LEN_ERROR - WP_VIOLATION - CARD_IS_LOCKED - COM_CRC_ERROR - CARD_ECC_FAILED - CC_ERROR - ERROR Response Flags checked in R5: - COM_CRC_ERROR - ERROR - FUNCTION_NUMBER - OUT_OF_RANGE Values: - 0x0 (RESP_R1): R1 (Memory) - 0x1 (RESP_R5): R5 (SDIO)"]
    #[inline(always)]
    pub fn resp_type(&self) -> RESP_TYPE_R {
        RESP_TYPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
    #[inline(always)]
    pub fn resp_err_chk_enable(&self) -> RESP_ERR_CHK_ENABLE_R {
        RESP_ERR_CHK_ENABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
    #[inline(always)]
    pub fn resp_int_disable(&self) -> RESP_INT_DISABLE_R {
        RESP_INT_DISABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: - 0x1 (ENABLED): DMA Data transfer - 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
    #[inline(always)]
    pub fn dma_enable(&mut self) -> DMA_ENABLE_W {
        DMA_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. When 16-bit Block Count register is used, the Host Driver can set this bit to 0 in ADMA2 mode to enable larger data transfer than the maximum of 65535 block counts supported by the 16-bit Block Count register."]
    #[inline(always)]
    pub fn block_count_enable(&mut self) -> BLOCK_COUNT_ENABLE_W {
        BLOCK_COUNT_ENABLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: - 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled - 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable - 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable - 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Select"]
    #[inline(always)]
    pub fn auto_cmd_enable(&mut self) -> AUTO_CMD_ENABLE_W {
        AUTO_CMD_ENABLE_W { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: - 0x1 (READ): Read (Card to Host) - 0x0 (WRITE): Write (Host to Card)"]
    #[inline(always)]
    pub fn data_xfer_dir(&mut self) -> DATA_XFER_DIR_W {
        DATA_XFER_DIR_W { w: self }
    }
    #[doc = "Bit 5 - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register."]
    #[inline(always)]
    pub fn multi_blk_sel(&mut self) -> MULTI_BLK_SEL_W {
        MULTI_BLK_SEL_W { w: self }
    }
    #[doc = "Bit 6 - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: - OUT_OF_RANGE - ADDRESS_ERROR - BLOCK_LEN_ERROR - WP_VIOLATION - CARD_IS_LOCKED - COM_CRC_ERROR - CARD_ECC_FAILED - CC_ERROR - ERROR Response Flags checked in R5: - COM_CRC_ERROR - ERROR - FUNCTION_NUMBER - OUT_OF_RANGE Values: - 0x0 (RESP_R1): R1 (Memory) - 0x1 (RESP_R5): R5 (SDIO)"]
    #[inline(always)]
    pub fn resp_type(&mut self) -> RESP_TYPE_W {
        RESP_TYPE_W { w: self }
    }
    #[doc = "Bit 7 - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
    #[inline(always)]
    pub fn resp_err_chk_enable(&mut self) -> RESP_ERR_CHK_ENABLE_W {
        RESP_ERR_CHK_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
    #[inline(always)]
    pub fn resp_int_disable(&mut self) -> RESP_INT_DISABLE_W {
        RESP_INT_DISABLE_W { w: self }
    }
}
