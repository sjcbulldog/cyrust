#[doc = "Reader of register PW_SEQ23"]
pub type R = crate::R<u32, super::PW_SEQ23>;
#[doc = "Writer for register PW_SEQ23"]
pub type W = crate::W<u32, super::PW_SEQ23>;
#[doc = "Register PW_SEQ23 `reset()`'s with value 0"]
impl crate::ResetValue for super::PW_SEQ23 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `PW_SEQ2_POST`"]
pub type PW_SEQ2_POST_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PW_SEQ2_POST`"]
pub struct PW_SEQ2_POST_W<'a> {
    w: &'a mut W,
}
impl<'a> PW_SEQ2_POST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PW_SEQ3`"]
pub type PW_SEQ3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PW_SEQ3`"]
pub struct PW_SEQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> PW_SEQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Seq2 post delay"]
    #[inline(always)]
    pub fn pw_seq2_post(&self) -> PW_SEQ2_POST_R {
        PW_SEQ2_POST_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Seq3 delay"]
    #[inline(always)]
    pub fn pw_seq3(&self) -> PW_SEQ3_R {
        PW_SEQ3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Seq2 post delay"]
    #[inline(always)]
    pub fn pw_seq2_post(&mut self) -> PW_SEQ2_POST_W {
        PW_SEQ2_POST_W { w: self }
    }
    #[doc = "Bits 16:31 - Seq3 delay"]
    #[inline(always)]
    pub fn pw_seq3(&mut self) -> PW_SEQ3_W {
        PW_SEQ3_W { w: self }
    }
}
