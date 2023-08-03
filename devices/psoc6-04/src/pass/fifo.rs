#[doc = "FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "FIFO control register"]
pub mod ctrl;
#[doc = "FIFO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "FIFO configuration register"]
pub mod config;
#[doc = "FIFO clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clear](clear) module"]
pub type CLEAR = crate::Reg<u32, _CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEAR;
#[doc = "`read()` method returns [clear::R](clear::R) reader structure"]
impl crate::Readable for CLEAR {}
#[doc = "`write(|w| ..)` method takes [clear::W](clear::W) writer structure"]
impl crate::Writable for CLEAR {}
#[doc = "FIFO clear register"]
pub mod clear;
#[doc = "FIFO level register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [level](level) module"]
pub type LEVEL = crate::Reg<u32, _LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEVEL;
#[doc = "`read()` method returns [level::R](level::R) reader structure"]
impl crate::Readable for LEVEL {}
#[doc = "`write(|w| ..)` method takes [level::W](level::W) writer structure"]
impl crate::Writable for LEVEL {}
#[doc = "FIFO level register"]
pub mod level;
#[doc = "FIFO used register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [used](used) module"]
pub type USED = crate::Reg<u32, _USED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USED;
#[doc = "`read()` method returns [used::R](used::R) reader structure"]
impl crate::Readable for USED {}
#[doc = "FIFO used register"]
pub mod used;
#[doc = "FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "FIFO status register"]
pub mod status;
#[doc = "FIFO read data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_data](rd_data) module"]
pub type RD_DATA = crate::Reg<u32, _RD_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_DATA;
#[doc = "`read()` method returns [rd_data::R](rd_data::R) reader structure"]
impl crate::Readable for RD_DATA {}
#[doc = "FIFO read data register"]
pub mod rd_data;
#[doc = "Interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Interrupt register"]
pub mod intr;
#[doc = "Interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Interrupt set register"]
pub mod intr_set;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "Interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Interrupt masked register"]
pub mod intr_masked;
