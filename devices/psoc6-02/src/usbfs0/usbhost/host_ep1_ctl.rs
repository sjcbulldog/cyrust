#[doc = "Reader of register HOST_EP1_CTL"]
pub type R = crate::R<u32, super::HOST_EP1_CTL>;
#[doc = "Writer for register HOST_EP1_CTL"]
pub type W = crate::W<u32, super::HOST_EP1_CTL>;
#[doc = "Register HOST_EP1_CTL `reset()`'s with value 0x8100"]
impl crate::ResetValue for super::HOST_EP1_CTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8100
    }
}
#[doc = "Reader of field `PKS1`"]
pub type PKS1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKS1`"]
pub struct PKS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PKS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `NULLE`"]
pub type NULLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NULLE`"]
pub struct NULLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NULLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DMAE`"]
pub type DMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAE`"]
pub struct DMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `BFINI`"]
pub type BFINI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFINI`"]
pub struct BFINI_W<'a> {
    w: &'a mut W,
}
impl<'a> BFINI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x100. - If automatic buffer transfer mode (DMAE='1') is used, Endpoint 0,1, or 2 cannot be used,"]
    #[inline(always)]
    pub fn pks1(&self) -> PKS1_R {
        PKS1_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 10 - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    pub fn nulle(&self) -> NULLE_R {
        NULLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the packet transfer mode. '1' : Sets the packet transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS1 bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 1 Status Register (HOST_EP1_STATUS) is '1'."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP1 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP1DRQ and EP1SPK bits."]
    #[inline(always)]
    pub fn bfini(&self) -> BFINI_R {
        BFINI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x100. - If automatic buffer transfer mode (DMAE='1') is used, Endpoint 0,1, or 2 cannot be used,"]
    #[inline(always)]
    pub fn pks1(&mut self) -> PKS1_W {
        PKS1_W { w: self }
    }
    #[doc = "Bit 10 - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    pub fn nulle(&mut self) -> NULLE_W {
        NULLE_W { w: self }
    }
    #[doc = "Bit 11 - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the packet transfer mode. '1' : Sets the packet transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS1 bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W {
        DMAE_W { w: self }
    }
    #[doc = "Bit 12 - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 1 Status Register (HOST_EP1_STATUS) is '1'."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 15 - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP1 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP1DRQ and EP1SPK bits."]
    #[inline(always)]
    pub fn bfini(&mut self) -> BFINI_W {
        BFINI_W { w: self }
    }
}
