#[doc = "Reader of register TSCC"]
pub type R = crate::R<u32, super::TSCC>;
#[doc = "Writer for register TSCC"]
pub type W = crate::W<u32, super::TSCC>;
#[doc = "Register TSCC `reset()`'s with value 0"]
impl crate::ResetValue for super::TSCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSS`"]
pub type TSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSS`"]
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TCP`"]
pub type TCP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCP`"]
pub struct TCP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timestamp Select, should always be set to external timestamp counter 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as '00'"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler (still used for TOCC) 0x0-0xF Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1...16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp Select, should always be set to external timestamp counter 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as '00'"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler (still used for TOCC) 0x0-0xF Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1...16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W {
        TCP_W { w: self }
    }
}
