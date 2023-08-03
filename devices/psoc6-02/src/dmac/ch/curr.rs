#[doc = "Reader of register CURR"]
pub type R = crate::R<u32, super::CURR>;
#[doc = "Writer for register CURR"]
pub type W = crate::W<u32, super::CURR>;
#[doc = "Register CURR `reset()`'s with value 0"]
impl crate::ResetValue for super::CURR {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `PTR`"]
pub type PTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PTR`"]
pub struct PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    pub fn ptr(&mut self) -> PTR_W {
        PTR_W { w: self }
    }
}
