#[doc = "Channel control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Channel control"]
pub mod ctl;
#[doc = "Channel current indices\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idx](idx) module"]
pub type IDX = crate::Reg<u32, _IDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDX;
#[doc = "`read()` method returns [idx::R](idx::R) reader structure"]
impl crate::Readable for IDX {}
#[doc = "Channel current indices"]
pub mod idx;
#[doc = "Channel current source address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src](src) module"]
pub type SRC = crate::Reg<u32, _SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRC;
#[doc = "`read()` method returns [src::R](src::R) reader structure"]
impl crate::Readable for SRC {}
#[doc = "Channel current source address"]
pub mod src;
#[doc = "Channel current destination address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst](dst) module"]
pub type DST = crate::Reg<u32, _DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DST;
#[doc = "`read()` method returns [dst::R](dst::R) reader structure"]
impl crate::Readable for DST {}
#[doc = "Channel current destination address"]
pub mod dst;
#[doc = "Channel current descriptor pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curr](curr) module"]
pub type CURR = crate::Reg<u32, _CURR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURR;
#[doc = "`read()` method returns [curr::R](curr::R) reader structure"]
impl crate::Readable for CURR {}
#[doc = "`write(|w| ..)` method takes [curr::W](curr::W) writer structure"]
impl crate::Writable for CURR {}
#[doc = "Channel current descriptor pointer"]
pub mod curr;
#[doc = "Channle software trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_cmd](tr_cmd) module"]
pub type TR_CMD = crate::Reg<u32, _TR_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_CMD;
#[doc = "`read()` method returns [tr_cmd::R](tr_cmd::R) reader structure"]
impl crate::Readable for TR_CMD {}
#[doc = "`write(|w| ..)` method takes [tr_cmd::W](tr_cmd::W) writer structure"]
impl crate::Writable for TR_CMD {}
#[doc = "Channle software trigger"]
pub mod tr_cmd;
#[doc = "Channel descriptor status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_status](descr_status) module"]
pub type DESCR_STATUS = crate::Reg<u32, _DESCR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCR_STATUS;
#[doc = "`read()` method returns [descr_status::R](descr_status::R) reader structure"]
impl crate::Readable for DESCR_STATUS {}
#[doc = "Channel descriptor status"]
pub mod descr_status;
#[doc = "Channel descriptor control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_ctl](descr_ctl) module"]
pub type DESCR_CTL = crate::Reg<u32, _DESCR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCR_CTL;
#[doc = "`read()` method returns [descr_ctl::R](descr_ctl::R) reader structure"]
impl crate::Readable for DESCR_CTL {}
#[doc = "Channel descriptor control"]
pub mod descr_ctl;
#[doc = "Channel descriptor source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_src](descr_src) module"]
pub type DESCR_SRC = crate::Reg<u32, _DESCR_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCR_SRC;
#[doc = "`read()` method returns [descr_src::R](descr_src::R) reader structure"]
impl crate::Readable for DESCR_SRC {}
#[doc = "Channel descriptor source"]
pub mod descr_src;
#[doc = "Channel descriptor destination\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_dst](descr_dst) module"]
pub type DESCR_DST = crate::Reg<u32, _DESCR_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCR_DST;
#[doc = "`read()` method returns [descr_dst::R](descr_dst::R) reader structure"]
impl crate::Readable for DESCR_DST {}
#[doc = "Channel descriptor destination"]
pub mod descr_dst;
#[doc = "Channel descriptor X size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_x_size](descr_x_size) module"]
pub type DESCR_X_SIZE = crate::Reg<u32, _DESCR_X_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCR_X_SIZE;
#[doc = "`read()` method returns [descr_x_size::R](descr_x_size::R) reader structure"]
impl crate::Readable for DESCR_X_SIZE {}
#[doc = "Channel descriptor X size"]
pub mod descr_x_size;
#[doc = "Channel descriptor X increment\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_x_incr](descr_x_incr) module"]
pub type DESCR_X_INCR = crate::Reg<u32, _DESCR_X_INCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCR_X_INCR;
#[doc = "`read()` method returns [descr_x_incr::R](descr_x_incr::R) reader structure"]
impl crate::Readable for DESCR_X_INCR {}
#[doc = "Channel descriptor X increment"]
pub mod descr_x_incr;
#[doc = "Channel descriptor Y size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_y_size](descr_y_size) module"]
pub type DESCR_Y_SIZE = crate::Reg<u32, _DESCR_Y_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCR_Y_SIZE;
#[doc = "`read()` method returns [descr_y_size::R](descr_y_size::R) reader structure"]
impl crate::Readable for DESCR_Y_SIZE {}
#[doc = "Channel descriptor Y size"]
pub mod descr_y_size;
#[doc = "Channel descriptor Y increment\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_y_incr](descr_y_incr) module"]
pub type DESCR_Y_INCR = crate::Reg<u32, _DESCR_Y_INCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCR_Y_INCR;
#[doc = "`read()` method returns [descr_y_incr::R](descr_y_incr::R) reader structure"]
impl crate::Readable for DESCR_Y_INCR {}
#[doc = "Channel descriptor Y increment"]
pub mod descr_y_incr;
#[doc = "Channel descriptor next pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_next](descr_next) module"]
pub type DESCR_NEXT = crate::Reg<u32, _DESCR_NEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESCR_NEXT;
#[doc = "`read()` method returns [descr_next::R](descr_next::R) reader structure"]
impl crate::Readable for DESCR_NEXT {}
#[doc = "Channel descriptor next pointer"]
pub mod descr_next;
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Interrupt"]
pub mod intr;
#[doc = "Interrupt set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "Interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Interrupt masked"]
pub mod intr_masked;
