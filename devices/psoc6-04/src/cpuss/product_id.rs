#[doc = "Reader of register PRODUCT_ID"]
pub type R = crate::R<u32, super::PRODUCT_ID>;
#[doc = "Reader of field `FAMILY_ID`"]
pub type FAMILY_ID_R = crate::R<u16, u16>;
#[doc = "Reader of field `MAJOR_REV`"]
pub type MAJOR_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `MINOR_REV`"]
pub type MINOR_REV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Family ID a.k.a. Partnumber a.k.a. Silicon ID"]
    #[inline(always)]
    pub fn family_id(&self) -> FAMILY_ID_R {
        FAMILY_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Major Revision, starts with 1, increments with all layer tape-out (implemented with metal ECO-able tie-off)"]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Minor Revision, starts with 1, increments with metal layer only tape-out (implemented with metal ECO-able tie-off)"]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
