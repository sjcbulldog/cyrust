#[doc = "Reader of register ATT1"]
pub type R = crate::R<u32, super::ATT1>;
#[doc = "Writer for register ATT1"]
pub type W = crate::W<u32, super::ATT1>;
#[doc = "Register ATT1 `reset()`'s with value 0x0700_0109"]
impl crate::ResetValue for super::ATT1 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700_0109
    }
}
#[doc = "Reader of field `UR`"]
pub type UR_R = crate::R<bool, bool>;
#[doc = "Reader of field `UW`"]
pub type UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UW`"]
pub struct UW_W<'a> {
    w: &'a mut W,
}
impl<'a> UW_W<'a> {
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
#[doc = "Reader of field `UX`"]
pub type UX_R = crate::R<bool, bool>;
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PW`"]
pub type PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PW`"]
pub struct PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PX`"]
pub type PX_R = crate::R<bool, bool>;
#[doc = "Reader of field `NS`"]
pub type NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NS`"]
pub struct NS_W<'a> {
    w: &'a mut W,
}
impl<'a> NS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PC_MASK_0`"]
pub type PC_MASK_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC_MASK_15_TO_1`"]
pub type PC_MASK_15_TO_1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PC_MASK_15_TO_1`"]
pub struct PC_MASK_15_TO_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_MASK_15_TO_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 9)) | (((value as u32) & 0x7fff) << 9);
        self.w
    }
}
#[doc = "Reader of field `REGION_SIZE`"]
pub type REGION_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PC_MATCH`"]
pub type PC_MATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC_MATCH`"]
pub struct PC_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_MATCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed). Note that this register is constant '1'; i.e. user read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn uw(&self) -> UW_R {
        UW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed). Note that this register is constant '0'; i.e. user execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn ux(&self) -> UX_R {
        UX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed). Note that this register is constant '1'; i.e. privileged read accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed). Note that this register is constant '0'; i.e. privileged execute accesses are NEVER allowed."]
    #[inline(always)]
    pub fn px(&self) -> PX_R {
        PX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This field specifies protection context identifier based access control for protection context '0'."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:23 - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:28 - This field specifies the region size: '7': 256 B region (8 32 B subregions) Note: this field is read-only."]
    #[inline(always)]
    pub fn region_size(&self) -> REGION_SIZE_R {
        REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub fn pc_match(&self) -> PC_MATCH_R {
        PC_MATCH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn uw(&mut self) -> UW_W {
        UW_W { w: self }
    }
    #[doc = "Bit 4 - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn pw(&mut self) -> PW_W {
        PW_W { w: self }
    }
    #[doc = "Bit 6 - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W {
        NS_W { w: self }
    }
    #[doc = "Bits 9:23 - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W {
        PC_MASK_15_TO_1_W { w: self }
    }
    #[doc = "Bit 30 - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub fn pc_match(&mut self) -> PC_MATCH_W {
        PC_MATCH_W { w: self }
    }
    #[doc = "Bit 31 - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
