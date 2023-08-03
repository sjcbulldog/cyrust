#[doc = "Reader of register BUF_DATA_R"]
pub type R = crate::R<u32, super::BUF_DATA_R>;
#[doc = "Writer for register BUF_DATA_R"]
pub type W = crate::W<u32, super::BUF_DATA_R>;
#[doc = "Register BUF_DATA_R `reset()`'s with value 0"]
impl crate::ResetValue for super::BUF_DATA_R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF_DATA`"]
pub type BUF_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUF_DATA`"]
pub struct BUF_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer Data These bits enable access to the Host Controller packet buffer."]
    #[inline(always)]
    pub fn buf_data(&self) -> BUF_DATA_R {
        BUF_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data These bits enable access to the Host Controller packet buffer."]
    #[inline(always)]
    pub fn buf_data(&mut self) -> BUF_DATA_W {
        BUF_DATA_W { w: self }
    }
}
