#[doc = "Reader of register BLOCKSIZE_R"]
pub type R = crate::R<u16, super::BLOCKSIZE_R>;
#[doc = "Writer for register BLOCKSIZE_R"]
pub type W = crate::W<u16, super::BLOCKSIZE_R>;
#[doc = "Register BLOCKSIZE_R `reset()`'s with value 0"]
impl crate::ResetValue for super::BLOCKSIZE_R {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XFER_BLOCK_SIZE`"]
pub type XFER_BLOCK_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XFER_BLOCK_SIZE`"]
pub struct XFER_BLOCK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XFER_BLOCK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u16) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `SDMA_BUF_BDARY`"]
pub type SDMA_BUF_BDARY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDMA_BUF_BDARY`"]
pub struct SDMA_BUF_BDARY_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_BUF_BDARY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - ...... - 0x1FF: 511 byte - 0x200: 512 bytes - ...... - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
    #[inline(always)]
    pub fn xfer_block_size(&self) -> XFER_BLOCK_SIZE_R {
        XFER_BLOCK_SIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
    #[inline(always)]
    pub fn sdma_buf_bdary(&self) -> SDMA_BUF_BDARY_R {
        SDMA_BUF_BDARY_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - ...... - 0x1FF: 511 byte - 0x200: 512 bytes - ...... - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
    #[inline(always)]
    pub fn xfer_block_size(&mut self) -> XFER_BLOCK_SIZE_W {
        XFER_BLOCK_SIZE_W { w: self }
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
    #[inline(always)]
    pub fn sdma_buf_bdary(&mut self) -> SDMA_BUF_BDARY_W {
        SDMA_BUF_BDARY_W { w: self }
    }
}
