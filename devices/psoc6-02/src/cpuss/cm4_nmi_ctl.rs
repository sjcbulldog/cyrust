#[doc = "Reader of register CM4_NMI_CTL[%s]"]
pub type R = crate::R<u32, super::CM4_NMI_CTL>;
#[doc = "Writer for register CM4_NMI_CTL[%s]"]
pub type W = crate::W<u32, super::CM4_NMI_CTL>;
#[doc = "Register CM4_NMI_CTL[%s]
`reset()`'s with value 0x03ff"]
impl crate::ResetValue for super::CM4_NMI_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
#[doc = "Reader of field `SYSTEM_INT_IDX`"]
pub type SYSTEM_INT_IDX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSTEM_INT_IDX`"]
pub struct SYSTEM_INT_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_INT_IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SYSTEM_INT_IDX_R {
        SYSTEM_INT_IDX_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn system_int_idx(&mut self) -> SYSTEM_INT_IDX_W {
        SYSTEM_INT_IDX_W { w: self }
    }
}
