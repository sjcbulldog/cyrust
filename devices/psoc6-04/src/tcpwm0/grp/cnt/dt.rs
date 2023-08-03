#[doc = "Reader of register DT"]
pub type R = crate::R<u32, super::DT>;
#[doc = "Writer for register DT"]
pub type W = crate::W<u32, super::DT>;
#[doc = "Register DT `reset()`'s with value 0"]
impl crate::ResetValue for super::DT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DT_LINE_OUT_L`"]
pub type DT_LINE_OUT_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT_LINE_OUT_L`"]
pub struct DT_LINE_OUT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_LINE_OUT_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DT_LINE_OUT_H`"]
pub type DT_LINE_OUT_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT_LINE_OUT_H`"]
pub struct DT_LINE_OUT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_LINE_OUT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DT_LINE_COMPL_OUT`"]
pub type DT_LINE_COMPL_OUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DT_LINE_COMPL_OUT`"]
pub struct DT_LINE_COMPL_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_LINE_COMPL_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - In PWM_DT mode, this field is used to determine the low byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock. Note: This field determines the low byte of the 16-bit dead time before activating 'line_out' when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by this DT_LINE_OUT_L field is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    pub fn dt_line_out_l(&self) -> DT_LINE_OUT_L_R {
        DT_LINE_OUT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In PWM_DT mode, this field is used to determine the high byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    pub fn dt_line_out_h(&self) -> DT_LINE_OUT_H_R {
        DT_LINE_OUT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - In PWM_DT mode, this field is used to determine the dead time before activating the complementary PWM line output signal 'line_compl_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    pub fn dt_line_compl_out(&self) -> DT_LINE_COMPL_OUT_R {
        DT_LINE_COMPL_OUT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - In PWM_DT mode, this field is used to determine the low byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock. Note: This field determines the low byte of the 16-bit dead time before activating 'line_out' when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by this DT_LINE_OUT_L field is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    pub fn dt_line_out_l(&mut self) -> DT_LINE_OUT_L_W {
        DT_LINE_OUT_L_W { w: self }
    }
    #[doc = "Bits 8:15 - In PWM_DT mode, this field is used to determine the high byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    pub fn dt_line_out_h(&mut self) -> DT_LINE_OUT_H_W {
        DT_LINE_OUT_H_W { w: self }
    }
    #[doc = "Bits 16:31 - In PWM_DT mode, this field is used to determine the dead time before activating the complementary PWM line output signal 'line_compl_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    pub fn dt_line_compl_out(&mut self) -> DT_LINE_COMPL_OUT_W {
        DT_LINE_COMPL_OUT_W { w: self }
    }
}
