#[doc = "Reader of register ANA_CTL1"]
pub type R = crate::R<u32, super::ANA_CTL1>;
#[doc = "Writer for register ANA_CTL1"]
pub type W = crate::W<u32, super::ANA_CTL1>;
#[doc = "Register ANA_CTL1 `reset()`'s with value 0x0d32_fafa"]
impl crate::ResetValue for super::ANA_CTL1 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d32_fafa
    }
}
#[doc = "Reader of field `NDAC_MAX`"]
pub type NDAC_MAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDAC_MAX`"]
pub struct NDAC_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> NDAC_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `NDAC_STEP`"]
pub type NDAC_STEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDAC_STEP`"]
pub struct NDAC_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> NDAC_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PDAC_MAX`"]
pub type PDAC_MAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDAC_MAX`"]
pub struct PDAC_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PDAC_STEP`"]
pub type PDAC_STEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDAC_STEP`"]
pub struct PDAC_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `NPDAC_STEP_TIME`"]
pub type NPDAC_STEP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NPDAC_STEP_TIME`"]
pub struct NPDAC_STEP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> NPDAC_STEP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `NPDAC_ZERO_TIME`"]
pub type NPDAC_ZERO_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NPDAC_ZERO_TIME`"]
pub struct NPDAC_ZERO_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> NPDAC_ZERO_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Ndac Max Value.Trimming of negative pump output Voltage."]
    #[inline(always)]
    pub fn ndac_max(&self) -> NDAC_MAX_R {
        NDAC_MAX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Ndac step increment"]
    #[inline(always)]
    pub fn ndac_step(&self) -> NDAC_STEP_R {
        NDAC_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pdac Max Value.Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac_max(&self) -> PDAC_MAX_R {
        PDAC_MAX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pdac step increment"]
    #[inline(always)]
    pub fn pdac_step(&self) -> PDAC_STEP_R {
        PDAC_STEP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
    #[inline(always)]
    pub fn npdac_step_time(&self) -> NPDAC_STEP_TIME_R {
        NPDAC_STEP_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
    #[inline(always)]
    pub fn npdac_zero_time(&self) -> NPDAC_ZERO_TIME_R {
        NPDAC_ZERO_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ndac Max Value.Trimming of negative pump output Voltage."]
    #[inline(always)]
    pub fn ndac_max(&mut self) -> NDAC_MAX_W {
        NDAC_MAX_W { w: self }
    }
    #[doc = "Bits 4:7 - Ndac step increment"]
    #[inline(always)]
    pub fn ndac_step(&mut self) -> NDAC_STEP_W {
        NDAC_STEP_W { w: self }
    }
    #[doc = "Bits 8:11 - Pdac Max Value.Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac_max(&mut self) -> PDAC_MAX_W {
        PDAC_MAX_W { w: self }
    }
    #[doc = "Bits 12:15 - Pdac step increment"]
    #[inline(always)]
    pub fn pdac_step(&mut self) -> PDAC_STEP_W {
        PDAC_STEP_W { w: self }
    }
    #[doc = "Bits 16:23 - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
    #[inline(always)]
    pub fn npdac_step_time(&mut self) -> NPDAC_STEP_TIME_W {
        NPDAC_STEP_TIME_W { w: self }
    }
    #[doc = "Bits 24:31 - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
    #[inline(always)]
    pub fn npdac_zero_time(&mut self) -> NPDAC_ZERO_TIME_W {
        NPDAC_ZERO_TIME_W { w: self }
    }
}
