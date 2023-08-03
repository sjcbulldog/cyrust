#[doc = "Reader of register IDX"]
pub type R = crate::R<u32, super::IDX>;
#[doc = "Reader of field `X`"]
pub type X_R = crate::R<u16, u16>;
#[doc = "Reader of field `Y`"]
pub type Y_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies the Y loop index, with Y_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor.."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
