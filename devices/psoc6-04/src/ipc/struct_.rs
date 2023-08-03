#[doc = "IPC acquire\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acquire](acquire) module"]
pub type ACQUIRE = crate::Reg<u32, _ACQUIRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACQUIRE;
#[doc = "`read()` method returns [acquire::R](acquire::R) reader structure"]
impl crate::Readable for ACQUIRE {}
#[doc = "IPC acquire"]
pub mod acquire;
#[doc = "IPC release\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [release](release) module"]
pub type RELEASE = crate::Reg<u32, _RELEASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RELEASE;
#[doc = "`write(|w| ..)` method takes [release::W](release::W) writer structure"]
impl crate::Writable for RELEASE {}
#[doc = "IPC release"]
pub mod release;
#[doc = "IPC notification\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [notify](notify) module"]
pub type NOTIFY = crate::Reg<u32, _NOTIFY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NOTIFY;
#[doc = "`write(|w| ..)` method takes [notify::W](notify::W) writer structure"]
impl crate::Writable for NOTIFY {}
#[doc = "IPC notification"]
pub mod notify;
#[doc = "IPC data 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0](data0) module"]
pub type DATA0 = crate::Reg<u32, _DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0;
#[doc = "`read()` method returns [data0::R](data0::R) reader structure"]
impl crate::Readable for DATA0 {}
#[doc = "`write(|w| ..)` method takes [data0::W](data0::W) writer structure"]
impl crate::Writable for DATA0 {}
#[doc = "IPC data 0"]
pub mod data0;
#[doc = "IPC data 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data1](data1) module"]
pub type DATA1 = crate::Reg<u32, _DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA1;
#[doc = "`read()` method returns [data1::R](data1::R) reader structure"]
impl crate::Readable for DATA1 {}
#[doc = "`write(|w| ..)` method takes [data1::W](data1::W) writer structure"]
impl crate::Writable for DATA1 {}
#[doc = "IPC data 1"]
pub mod data1;
#[doc = "IPC lock status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock_status](lock_status) module"]
pub type LOCK_STATUS = crate::Reg<u32, _LOCK_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK_STATUS;
#[doc = "`read()` method returns [lock_status::R](lock_status::R) reader structure"]
impl crate::Readable for LOCK_STATUS {}
#[doc = "IPC lock status"]
pub mod lock_status;
