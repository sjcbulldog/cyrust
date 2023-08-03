#[doc = "Reader of register TTTMK"]
pub type R = crate::R<u32, super::TTTMK>;
#[doc = "Writer for register TTTMK"]
pub type W = crate::W<u32, super::TTTMK>;
#[doc = "Register TTTMK `reset()`'s with value 0"]
impl crate::ResetValue for super::TTTMK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TM_`"]
pub type TM__R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TM_`"]
pub struct TM__W<'a> {
    w: &'a mut W,
}
impl<'a> TM__W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `TICC`"]
pub type TICC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TICC`"]
pub struct TICC_W<'a> {
    w: &'a mut W,
}
impl<'a> TICC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `LCKM`"]
pub type LCKM_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Time Mark 0x0000-FFFF Time Mark"]
    #[inline(always)]
    pub fn tm_(&self) -> TM__R {
        TM__R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code Cycle count for which the time mark is valid. 0b000000x valid for all cycles 0b000001c valid every second cycle at cycle count mod2 = c 0b00001cc valid every fourth cycle at cycle count mod4 = cc 0b0001ccc valid every eighth cycle at cycle count mod8 = ccc 0b001cccc valid every sixteenth cycle at cycle count mod16 = cccc 0b01ccccc valid every thirty-second cycle at cycle count mod32 = ccccc 0b1cccccc valid every sixty-fourth cycle at cycle count mod64 = cccccc"]
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked Always set by a write access to registers TTOCN. Set by write access to register TTTMK when TTOCN.TMC != '00'. Reset when the registers have been synchronized into the CAN clock domain. 0= Write access to TTTMK enabled 1= Write access to TTTMK locked"]
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Mark 0x0000-FFFF Time Mark"]
    #[inline(always)]
    pub fn tm_(&mut self) -> TM__W {
        TM__W { w: self }
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code Cycle count for which the time mark is valid. 0b000000x valid for all cycles 0b000001c valid every second cycle at cycle count mod2 = c 0b00001cc valid every fourth cycle at cycle count mod4 = cc 0b0001ccc valid every eighth cycle at cycle count mod8 = ccc 0b001cccc valid every sixteenth cycle at cycle count mod16 = cccc 0b01ccccc valid every thirty-second cycle at cycle count mod32 = ccccc 0b1cccccc valid every sixty-fourth cycle at cycle count mod64 = cccccc"]
    #[inline(always)]
    pub fn ticc(&mut self) -> TICC_W {
        TICC_W { w: self }
    }
}
