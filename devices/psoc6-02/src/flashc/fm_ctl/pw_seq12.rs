#[doc = "Reader of register PW_SEQ12"]
pub type R = crate::R<u32, super::PW_SEQ12>;
#[doc = "Writer for register PW_SEQ12"]
pub type W = crate::W<u32, super::PW_SEQ12>;
#[doc = "Register PW_SEQ12 `reset()`'s with value 0"]
impl crate::ResetValue for super::PW_SEQ12 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `PW_SEQ1`"]
pub type PW_SEQ1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PW_SEQ1`"]
pub struct PW_SEQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PW_SEQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PW_SEQ2_PRE`"]
pub type PW_SEQ2_PRE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PW_SEQ2_PRE`"]
pub struct PW_SEQ2_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PW_SEQ2_PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Seq1 delay"]
    #[inline(always)]
    pub fn pw_seq1(&self) -> PW_SEQ1_R {
        PW_SEQ1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Seq2 pre delay"]
    #[inline(always)]
    pub fn pw_seq2_pre(&self) -> PW_SEQ2_PRE_R {
        PW_SEQ2_PRE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Seq1 delay"]
    #[inline(always)]
    pub fn pw_seq1(&mut self) -> PW_SEQ1_W {
        PW_SEQ1_W { w: self }
    }
    #[doc = "Bits 16:31 - Seq2 pre delay"]
    #[inline(always)]
    pub fn pw_seq2_pre(&mut self) -> PW_SEQ2_PRE_W {
        PW_SEQ2_PRE_W { w: self }
    }
}
