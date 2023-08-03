#[doc = "Counter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Counter control register"]
pub mod ctrl;
#[doc = "Counter status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Counter status register"]
pub mod status;
#[doc = "Counter count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter](counter) module"]
pub type COUNTER = crate::Reg<u32, _COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTER;
#[doc = "`read()` method returns [counter::R](counter::R) reader structure"]
impl crate::Readable for COUNTER {}
#[doc = "`write(|w| ..)` method takes [counter::W](counter::W) writer structure"]
impl crate::Writable for COUNTER {}
#[doc = "Counter count register"]
pub mod counter;
#[doc = "Counter compare/capture 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0](cc0) module"]
pub type CC0 = crate::Reg<u32, _CC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0;
#[doc = "`read()` method returns [cc0::R](cc0::R) reader structure"]
impl crate::Readable for CC0 {}
#[doc = "`write(|w| ..)` method takes [cc0::W](cc0::W) writer structure"]
impl crate::Writable for CC0 {}
#[doc = "Counter compare/capture 0 register"]
pub mod cc0;
#[doc = "Counter buffered compare/capture 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0_buff](cc0_buff) module"]
pub type CC0_BUFF = crate::Reg<u32, _CC0_BUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0_BUFF;
#[doc = "`read()` method returns [cc0_buff::R](cc0_buff::R) reader structure"]
impl crate::Readable for CC0_BUFF {}
#[doc = "`write(|w| ..)` method takes [cc0_buff::W](cc0_buff::W) writer structure"]
impl crate::Writable for CC0_BUFF {}
#[doc = "Counter buffered compare/capture 0 register"]
pub mod cc0_buff;
#[doc = "Counter compare/capture 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc1](cc1) module"]
pub type CC1 = crate::Reg<u32, _CC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1;
#[doc = "`read()` method returns [cc1::R](cc1::R) reader structure"]
impl crate::Readable for CC1 {}
#[doc = "`write(|w| ..)` method takes [cc1::W](cc1::W) writer structure"]
impl crate::Writable for CC1 {}
#[doc = "Counter compare/capture 1 register"]
pub mod cc1;
#[doc = "Counter buffered compare/capture 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc1_buff](cc1_buff) module"]
pub type CC1_BUFF = crate::Reg<u32, _CC1_BUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1_BUFF;
#[doc = "`read()` method returns [cc1_buff::R](cc1_buff::R) reader structure"]
impl crate::Readable for CC1_BUFF {}
#[doc = "`write(|w| ..)` method takes [cc1_buff::W](cc1_buff::W) writer structure"]
impl crate::Writable for CC1_BUFF {}
#[doc = "Counter buffered compare/capture 1 register"]
pub mod cc1_buff;
#[doc = "Counter period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period](period) module"]
pub type PERIOD = crate::Reg<u32, _PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIOD;
#[doc = "`read()` method returns [period::R](period::R) reader structure"]
impl crate::Readable for PERIOD {}
#[doc = "`write(|w| ..)` method takes [period::W](period::W) writer structure"]
impl crate::Writable for PERIOD {}
#[doc = "Counter period register"]
pub mod period;
#[doc = "Counter buffered period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period_buff](period_buff) module"]
pub type PERIOD_BUFF = crate::Reg<u32, _PERIOD_BUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIOD_BUFF;
#[doc = "`read()` method returns [period_buff::R](period_buff::R) reader structure"]
impl crate::Readable for PERIOD_BUFF {}
#[doc = "`write(|w| ..)` method takes [period_buff::W](period_buff::W) writer structure"]
impl crate::Writable for PERIOD_BUFF {}
#[doc = "Counter buffered period register"]
pub mod period_buff;
#[doc = "Counter line selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [line_sel](line_sel) module"]
pub type LINE_SEL = crate::Reg<u32, _LINE_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINE_SEL;
#[doc = "`read()` method returns [line_sel::R](line_sel::R) reader structure"]
impl crate::Readable for LINE_SEL {}
#[doc = "`write(|w| ..)` method takes [line_sel::W](line_sel::W) writer structure"]
impl crate::Writable for LINE_SEL {}
#[doc = "Counter line selection register"]
pub mod line_sel;
#[doc = "Counter buffered line selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [line_sel_buff](line_sel_buff) module"]
pub type LINE_SEL_BUFF = crate::Reg<u32, _LINE_SEL_BUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINE_SEL_BUFF;
#[doc = "`read()` method returns [line_sel_buff::R](line_sel_buff::R) reader structure"]
impl crate::Readable for LINE_SEL_BUFF {}
#[doc = "`write(|w| ..)` method takes [line_sel_buff::W](line_sel_buff::W) writer structure"]
impl crate::Writable for LINE_SEL_BUFF {}
#[doc = "Counter buffered line selection register"]
pub mod line_sel_buff;
#[doc = "Counter PWM dead time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt](dt) module"]
pub type DT = crate::Reg<u32, _DT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT;
#[doc = "`read()` method returns [dt::R](dt::R) reader structure"]
impl crate::Readable for DT {}
#[doc = "`write(|w| ..)` method takes [dt::W](dt::W) writer structure"]
impl crate::Writable for DT {}
#[doc = "Counter PWM dead time register"]
pub mod dt;
#[doc = "Counter trigger command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_cmd](tr_cmd) module"]
pub type TR_CMD = crate::Reg<u32, _TR_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_CMD;
#[doc = "`read()` method returns [tr_cmd::R](tr_cmd::R) reader structure"]
impl crate::Readable for TR_CMD {}
#[doc = "`write(|w| ..)` method takes [tr_cmd::W](tr_cmd::W) writer structure"]
impl crate::Writable for TR_CMD {}
#[doc = "Counter trigger command register"]
pub mod tr_cmd;
#[doc = "Counter input trigger selection register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_in_sel0](tr_in_sel0) module"]
pub type TR_IN_SEL0 = crate::Reg<u32, _TR_IN_SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_IN_SEL0;
#[doc = "`read()` method returns [tr_in_sel0::R](tr_in_sel0::R) reader structure"]
impl crate::Readable for TR_IN_SEL0 {}
#[doc = "`write(|w| ..)` method takes [tr_in_sel0::W](tr_in_sel0::W) writer structure"]
impl crate::Writable for TR_IN_SEL0 {}
#[doc = "Counter input trigger selection register 0"]
pub mod tr_in_sel0;
#[doc = "Counter input trigger selection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_in_sel1](tr_in_sel1) module"]
pub type TR_IN_SEL1 = crate::Reg<u32, _TR_IN_SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_IN_SEL1;
#[doc = "`read()` method returns [tr_in_sel1::R](tr_in_sel1::R) reader structure"]
impl crate::Readable for TR_IN_SEL1 {}
#[doc = "`write(|w| ..)` method takes [tr_in_sel1::W](tr_in_sel1::W) writer structure"]
impl crate::Writable for TR_IN_SEL1 {}
#[doc = "Counter input trigger selection register 1"]
pub mod tr_in_sel1;
#[doc = "Counter input trigger edge selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_in_edge_sel](tr_in_edge_sel) module"]
pub type TR_IN_EDGE_SEL = crate::Reg<u32, _TR_IN_EDGE_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_IN_EDGE_SEL;
#[doc = "`read()` method returns [tr_in_edge_sel::R](tr_in_edge_sel::R) reader structure"]
impl crate::Readable for TR_IN_EDGE_SEL {}
#[doc = "`write(|w| ..)` method takes [tr_in_edge_sel::W](tr_in_edge_sel::W) writer structure"]
impl crate::Writable for TR_IN_EDGE_SEL {}
#[doc = "Counter input trigger edge selection register"]
pub mod tr_in_edge_sel;
#[doc = "Counter trigger PWM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_pwm_ctrl](tr_pwm_ctrl) module"]
pub type TR_PWM_CTRL = crate::Reg<u32, _TR_PWM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_PWM_CTRL;
#[doc = "`read()` method returns [tr_pwm_ctrl::R](tr_pwm_ctrl::R) reader structure"]
impl crate::Readable for TR_PWM_CTRL {}
#[doc = "`write(|w| ..)` method takes [tr_pwm_ctrl::W](tr_pwm_ctrl::W) writer structure"]
impl crate::Writable for TR_PWM_CTRL {}
#[doc = "Counter trigger PWM control register"]
pub mod tr_pwm_ctrl;
#[doc = "Counter output trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_out_sel](tr_out_sel) module"]
pub type TR_OUT_SEL = crate::Reg<u32, _TR_OUT_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_OUT_SEL;
#[doc = "`read()` method returns [tr_out_sel::R](tr_out_sel::R) reader structure"]
impl crate::Readable for TR_OUT_SEL {}
#[doc = "`write(|w| ..)` method takes [tr_out_sel::W](tr_out_sel::W) writer structure"]
impl crate::Writable for TR_OUT_SEL {}
#[doc = "Counter output trigger selection register"]
pub mod tr_out_sel;
#[doc = "Interrupt request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "Interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Interrupt set request register"]
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
#[doc = "Interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
