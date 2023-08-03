#[doc = "Reader of register LINE_SEL_BUFF"]
pub type R = crate::R<u32, super::LINE_SEL_BUFF>;
#[doc = "Writer for register LINE_SEL_BUFF"]
pub type W = crate::W<u32, super::LINE_SEL_BUFF>;
#[doc = "Register LINE_SEL_BUFF `reset()`'s with value 0x32"]
impl crate::ResetValue for super::LINE_SEL_BUFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x32
    }
}
#[doc = "Reader of field `OUT_SEL`"]
pub type OUT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT_SEL`"]
pub struct OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `COMPL_OUT_SEL`"]
pub type COMPL_OUT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPL_OUT_SEL`"]
pub struct COMPL_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPL_OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Buffer for LINE_SEL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Buffer for LINE_SEL.COMPL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_COMPL_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
    #[inline(always)]
    pub fn compl_out_sel(&self) -> COMPL_OUT_SEL_R {
        COMPL_OUT_SEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Buffer for LINE_SEL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OUT_SEL_W {
        OUT_SEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Buffer for LINE_SEL.COMPL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_COMPL_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
    #[inline(always)]
    pub fn compl_out_sel(&mut self) -> COMPL_OUT_SEL_W {
        COMPL_OUT_SEL_W { w: self }
    }
}
