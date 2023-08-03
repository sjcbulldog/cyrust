#[doc = "Reader of register SL_ADDR"]
pub type R = crate::R<u32, super::SL_ADDR>;
#[doc = "Writer for register SL_ADDR"]
pub type W = crate::W<u32, super::SL_ADDR>;
#[doc = "Register SL_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SL_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR30`"]
pub type ADDR30_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR30`"]
pub struct ADDR30_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
    #[inline(always)]
    pub fn addr30(&self) -> ADDR30_R {
        ADDR30_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
    #[inline(always)]
    pub fn addr30(&mut self) -> ADDR30_W {
        ADDR30_W { w: self }
    }
}
