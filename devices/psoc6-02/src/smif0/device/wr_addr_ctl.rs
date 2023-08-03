#[doc = "Reader of register WR_ADDR_CTL"]
pub type R = crate::R<u32, super::WR_ADDR_CTL>;
#[doc = "Writer for register WR_ADDR_CTL"]
pub type W = crate::W<u32, super::WR_ADDR_CTL>;
#[doc = "Register WR_ADDR_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::WR_ADDR_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `WIDTH`"]
pub type WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WIDTH`"]
pub struct WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W { w: self }
    }
}
