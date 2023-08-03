#[doc = "Reader of register TR_IN_SEL1"]
pub type R = crate::R<u32, super::TR_IN_SEL1>;
#[doc = "Writer for register TR_IN_SEL1"]
pub type W = crate::W<u32, super::TR_IN_SEL1>;
#[doc = "Register TR_IN_SEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TR_IN_SEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START_SEL`"]
pub type START_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `START_SEL`"]
pub struct START_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> START_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CAPTURE1_SEL`"]
pub type CAPTURE1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPTURE1_SEL`"]
pub struct CAPTURE1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects one of the 256 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn start_sel(&self) -> START_SEL_R {
        START_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Selects one of the 256 input triggers as a capture 1 trigger."]
    #[inline(always)]
    pub fn capture1_sel(&self) -> CAPTURE1_SEL_R {
        CAPTURE1_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects one of the 256 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn start_sel(&mut self) -> START_SEL_W {
        START_SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - Selects one of the 256 input triggers as a capture 1 trigger."]
    #[inline(always)]
    pub fn capture1_sel(&mut self) -> CAPTURE1_SEL_W {
        CAPTURE1_SEL_W { w: self }
    }
}
