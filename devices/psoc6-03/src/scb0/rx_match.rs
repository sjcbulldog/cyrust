#[doc = "Reader of register RX_MATCH"]
pub type R = crate::R<u32, super::RX_MATCH>;
#[doc = "Writer for register RX_MATCH"]
pub type W = crate::W<u32, super::RX_MATCH>;
#[doc = "Register RX_MATCH `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_MATCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR & MASK) == ('slave address' & MASK))."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bits 16:23 - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR & MASK) == ('slave address' & MASK))."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
}
