#[doc = "Reader of register GP_IN_R"]
pub type R = crate::R<u32, super::GP_IN_R>;
#[doc = "Reader of field `GP_IN`"]
pub type GP_IN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - It reflects the value of gp_in ports. NOT USED - ALWAYS READS 0"]
    #[inline(always)]
    pub fn gp_in(&self) -> GP_IN_R {
        GP_IN_R::new((self.bits & 0x01) != 0)
    }
}
