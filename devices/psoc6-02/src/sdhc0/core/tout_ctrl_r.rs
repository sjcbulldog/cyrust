#[doc = "Reader of register TOUT_CTRL_R"]
pub type R = crate::R<u8, super::TOUT_CTRL_R>;
#[doc = "Writer for register TOUT_CTRL_R"]
pub type W = crate::W<u8, super::TOUT_CTRL_R>;
#[doc = "Register TOUT_CTRL_R `reset()`'s with value 0"]
impl crate::ResetValue for super::TOUT_CTRL_R {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `TOUT_CNT`"]
pub type TOUT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOUT_CNT`"]
pub struct TOUT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn tout_cnt(&self) -> TOUT_CNT_R {
        TOUT_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn tout_cnt(&mut self) -> TOUT_CNT_W {
        TOUT_CNT_W { w: self }
    }
}
