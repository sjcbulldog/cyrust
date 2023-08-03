#[doc = "Master control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_ctl](ms_ctl) module"]
pub type MS_CTL = crate::Reg<u32, _MS_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS_CTL;
#[doc = "`read()` method returns [ms_ctl::R](ms_ctl::R) reader structure"]
impl crate::Readable for MS_CTL {}
#[doc = "`write(|w| ..)` method takes [ms_ctl::W](ms_ctl::W) writer structure"]
impl crate::Writable for MS_CTL {}
#[doc = "Master control"]
pub mod ms_ctl;
#[doc = "Master control read mirror\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_ctl_read_mir](ms_ctl_read_mir) module"]
pub type MS_CTL_READ_MIR = crate::Reg<u32, _MS_CTL_READ_MIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS_CTL_READ_MIR;
#[doc = "`read()` method returns [ms_ctl_read_mir::R](ms_ctl_read_mir::R) reader structure"]
impl crate::Readable for MS_CTL_READ_MIR {}
#[doc = "Master control read mirror"]
pub mod ms_ctl_read_mir;
#[doc = r"Register block"]
#[repr(C)]
pub struct MPU_STRUCT {
    #[doc = "0x00 - MPU region address"]
    pub addr: self::mpu_struct::ADDR,
    #[doc = "0x04 - MPU region attrributes"]
    pub att: self::mpu_struct::ATT,
}
#[doc = r"Register block"]
#[doc = "MPU structure"]
pub mod mpu_struct;
