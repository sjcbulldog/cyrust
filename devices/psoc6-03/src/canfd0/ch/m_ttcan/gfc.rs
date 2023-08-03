#[doc = "Reader of register GFC"]
pub type R = crate::R<u32, super::GFC>;
#[doc = "Writer for register GFC"]
pub type W = crate::W<u32, super::GFC>;
#[doc = "Register GFC `reset()`'s with value 0"]
impl crate::ResetValue for super::GFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RRFE`"]
pub type RRFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RRFE`"]
pub struct RRFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RRFS`"]
pub type RRFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RRFS`"]
pub struct RRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ANFE`"]
pub type ANFE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ANFE`"]
pub struct ANFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ANFS`"]
pub type ANFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ANFS`"]
pub struct ANFS_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs"]
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W {
        RRFE_W { w: self }
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W {
        RRFS_W { w: self }
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W {
        ANFE_W { w: self }
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W {
        ANFS_W { w: self }
    }
}
