#[doc = "Reader of register CE_LENGTH"]
pub type R = crate::R<u32, super::CE_LENGTH>;
#[doc = "Writer for register CE_LENGTH"]
pub type W = crate::W<u32, super::CE_LENGTH>;
#[doc = "Register CE_LENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::CE_LENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONNECTION_EVENT_LENGTH`"]
pub type CONNECTION_EVENT_LENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONNECTION_EVENT_LENGTH`"]
pub struct CONNECTION_EVENT_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECTION_EVENT_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn connection_event_length(&self) -> CONNECTION_EVENT_LENGTH_R {
        CONNECTION_EVENT_LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn connection_event_length(&mut self) -> CONNECTION_EVENT_LENGTH_W {
        CONNECTION_EVENT_LENGTH_W { w: self }
    }
}
