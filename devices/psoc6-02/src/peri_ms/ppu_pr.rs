#[doc = "Slave region, base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_addr](sl_addr) module"]
pub type SL_ADDR = crate::Reg<u32, _SL_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL_ADDR;
#[doc = "`read()` method returns [sl_addr::R](sl_addr::R) reader structure"]
impl crate::Readable for SL_ADDR {}
#[doc = "`write(|w| ..)` method takes [sl_addr::W](sl_addr::W) writer structure"]
impl crate::Writable for SL_ADDR {}
#[doc = "Slave region, base address"]
pub mod sl_addr;
#[doc = "Slave region, size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_size](sl_size) module"]
pub type SL_SIZE = crate::Reg<u32, _SL_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL_SIZE;
#[doc = "`read()` method returns [sl_size::R](sl_size::R) reader structure"]
impl crate::Readable for SL_SIZE {}
#[doc = "`write(|w| ..)` method takes [sl_size::W](sl_size::W) writer structure"]
impl crate::Writable for SL_SIZE {}
#[doc = "Slave region, size"]
pub mod sl_size;
#[doc = "Slave attributes 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_att0](sl_att0) module"]
pub type SL_ATT0 = crate::Reg<u32, _SL_ATT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL_ATT0;
#[doc = "`read()` method returns [sl_att0::R](sl_att0::R) reader structure"]
impl crate::Readable for SL_ATT0 {}
#[doc = "`write(|w| ..)` method takes [sl_att0::W](sl_att0::W) writer structure"]
impl crate::Writable for SL_ATT0 {}
#[doc = "Slave attributes 0"]
pub mod sl_att0;
#[doc = "Slave attributes 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_att1](sl_att1) module"]
pub type SL_ATT1 = crate::Reg<u32, _SL_ATT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL_ATT1;
#[doc = "`read()` method returns [sl_att1::R](sl_att1::R) reader structure"]
impl crate::Readable for SL_ATT1 {}
#[doc = "`write(|w| ..)` method takes [sl_att1::W](sl_att1::W) writer structure"]
impl crate::Writable for SL_ATT1 {}
#[doc = "Slave attributes 1"]
pub mod sl_att1;
#[doc = "Slave attributes 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_att2](sl_att2) module"]
pub type SL_ATT2 = crate::Reg<u32, _SL_ATT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL_ATT2;
#[doc = "`read()` method returns [sl_att2::R](sl_att2::R) reader structure"]
impl crate::Readable for SL_ATT2 {}
#[doc = "`write(|w| ..)` method takes [sl_att2::W](sl_att2::W) writer structure"]
impl crate::Writable for SL_ATT2 {}
#[doc = "Slave attributes 2"]
pub mod sl_att2;
#[doc = "Slave attributes 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_att3](sl_att3) module"]
pub type SL_ATT3 = crate::Reg<u32, _SL_ATT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL_ATT3;
#[doc = "`read()` method returns [sl_att3::R](sl_att3::R) reader structure"]
impl crate::Readable for SL_ATT3 {}
#[doc = "`write(|w| ..)` method takes [sl_att3::W](sl_att3::W) writer structure"]
impl crate::Writable for SL_ATT3 {}
#[doc = "Slave attributes 3"]
pub mod sl_att3;
#[doc = "Master region, base address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_addr](ms_addr) module"]
pub type MS_ADDR = crate::Reg<u32, _MS_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS_ADDR;
#[doc = "`read()` method returns [ms_addr::R](ms_addr::R) reader structure"]
impl crate::Readable for MS_ADDR {}
#[doc = "Master region, base address"]
pub mod ms_addr;
#[doc = "Master region, size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_size](ms_size) module"]
pub type MS_SIZE = crate::Reg<u32, _MS_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS_SIZE;
#[doc = "`read()` method returns [ms_size::R](ms_size::R) reader structure"]
impl crate::Readable for MS_SIZE {}
#[doc = "Master region, size"]
pub mod ms_size;
#[doc = "Master attributes 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_att0](ms_att0) module"]
pub type MS_ATT0 = crate::Reg<u32, _MS_ATT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS_ATT0;
#[doc = "`read()` method returns [ms_att0::R](ms_att0::R) reader structure"]
impl crate::Readable for MS_ATT0 {}
#[doc = "`write(|w| ..)` method takes [ms_att0::W](ms_att0::W) writer structure"]
impl crate::Writable for MS_ATT0 {}
#[doc = "Master attributes 0"]
pub mod ms_att0;
#[doc = "Master attributes 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_att1](ms_att1) module"]
pub type MS_ATT1 = crate::Reg<u32, _MS_ATT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS_ATT1;
#[doc = "`read()` method returns [ms_att1::R](ms_att1::R) reader structure"]
impl crate::Readable for MS_ATT1 {}
#[doc = "`write(|w| ..)` method takes [ms_att1::W](ms_att1::W) writer structure"]
impl crate::Writable for MS_ATT1 {}
#[doc = "Master attributes 1"]
pub mod ms_att1;
#[doc = "Master attributes 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_att2](ms_att2) module"]
pub type MS_ATT2 = crate::Reg<u32, _MS_ATT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS_ATT2;
#[doc = "`read()` method returns [ms_att2::R](ms_att2::R) reader structure"]
impl crate::Readable for MS_ATT2 {}
#[doc = "`write(|w| ..)` method takes [ms_att2::W](ms_att2::W) writer structure"]
impl crate::Writable for MS_ATT2 {}
#[doc = "Master attributes 2"]
pub mod ms_att2;
#[doc = "Master attributes 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_att3](ms_att3) module"]
pub type MS_ATT3 = crate::Reg<u32, _MS_ATT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS_ATT3;
#[doc = "`read()` method returns [ms_att3::R](ms_att3::R) reader structure"]
impl crate::Readable for MS_ATT3 {}
#[doc = "`write(|w| ..)` method takes [ms_att3::W](ms_att3::W) writer structure"]
impl crate::Writable for MS_ATT3 {}
#[doc = "Master attributes 3"]
pub mod ms_att3;
