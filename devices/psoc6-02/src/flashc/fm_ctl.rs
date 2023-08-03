#[doc = "Flash macro control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_ctl](fm_ctl) module"]
pub type FM_CTL = crate::Reg<u32, _FM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_CTL;
#[doc = "`read()` method returns [fm_ctl::R](fm_ctl::R) reader structure"]
impl crate::Readable for FM_CTL {}
#[doc = "`write(|w| ..)` method takes [fm_ctl::W](fm_ctl::W) writer structure"]
impl crate::Writable for FM_CTL {}
#[doc = "Flash macro control"]
pub mod fm_ctl;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Flash macro address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_addr](fm_addr) module"]
pub type FM_ADDR = crate::Reg<u32, _FM_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_ADDR;
#[doc = "`read()` method returns [fm_addr::R](fm_addr::R) reader structure"]
impl crate::Readable for FM_ADDR {}
#[doc = "`write(|w| ..)` method takes [fm_addr::W](fm_addr::W) writer structure"]
impl crate::Writable for FM_ADDR {}
#[doc = "Flash macro address"]
pub mod fm_addr;
#[doc = "Bookmark register - keeps the current FW HV seq\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bookmark](bookmark) module"]
pub type BOOKMARK = crate::Reg<u32, _BOOKMARK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOKMARK;
#[doc = "`read()` method returns [bookmark::R](bookmark::R) reader structure"]
impl crate::Readable for BOOKMARK {}
#[doc = "`write(|w| ..)` method takes [bookmark::W](bookmark::W) writer structure"]
impl crate::Writable for BOOKMARK {}
#[doc = "Bookmark register - keeps the current FW HV seq"]
pub mod bookmark;
#[doc = "Regular flash geometry\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geometry](geometry) module"]
pub type GEOMETRY = crate::Reg<u32, _GEOMETRY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEOMETRY;
#[doc = "`read()` method returns [geometry::R](geometry::R) reader structure"]
impl crate::Readable for GEOMETRY {}
#[doc = "Regular flash geometry"]
pub mod geometry;
#[doc = "Supervisory flash geometry\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geometry_supervisory](geometry_supervisory) module"]
pub type GEOMETRY_SUPERVISORY = crate::Reg<u32, _GEOMETRY_SUPERVISORY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEOMETRY_SUPERVISORY;
#[doc = "`read()` method returns [geometry_supervisory::R](geometry_supervisory::R) reader structure"]
impl crate::Readable for GEOMETRY_SUPERVISORY {}
#[doc = "Supervisory flash geometry"]
pub mod geometry_supervisory;
#[doc = "Analog control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctl0](ana_ctl0) module"]
pub type ANA_CTL0 = crate::Reg<u32, _ANA_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_CTL0;
#[doc = "`read()` method returns [ana_ctl0::R](ana_ctl0::R) reader structure"]
impl crate::Readable for ANA_CTL0 {}
#[doc = "`write(|w| ..)` method takes [ana_ctl0::W](ana_ctl0::W) writer structure"]
impl crate::Writable for ANA_CTL0 {}
#[doc = "Analog control 0"]
pub mod ana_ctl0;
#[doc = "Analog control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctl1](ana_ctl1) module"]
pub type ANA_CTL1 = crate::Reg<u32, _ANA_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_CTL1;
#[doc = "`read()` method returns [ana_ctl1::R](ana_ctl1::R) reader structure"]
impl crate::Readable for ANA_CTL1 {}
#[doc = "`write(|w| ..)` method takes [ana_ctl1::W](ana_ctl1::W) writer structure"]
impl crate::Writable for ANA_CTL1 {}
#[doc = "Analog control 1"]
pub mod ana_ctl1;
#[doc = "Wait State control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wait_ctl](wait_ctl) module"]
pub type WAIT_CTL = crate::Reg<u32, _WAIT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAIT_CTL;
#[doc = "`read()` method returns [wait_ctl::R](wait_ctl::R) reader structure"]
impl crate::Readable for WAIT_CTL {}
#[doc = "`write(|w| ..)` method takes [wait_ctl::W](wait_ctl::W) writer structure"]
impl crate::Writable for WAIT_CTL {}
#[doc = "Wait State control"]
pub mod wait_ctl;
#[doc = "Timer prescaler (clk_t to timer clock frequency divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_clk_ctl](timer_clk_ctl) module"]
pub type TIMER_CLK_CTL = crate::Reg<u32, _TIMER_CLK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_CLK_CTL;
#[doc = "`read()` method returns [timer_clk_ctl::R](timer_clk_ctl::R) reader structure"]
impl crate::Readable for TIMER_CLK_CTL {}
#[doc = "`write(|w| ..)` method takes [timer_clk_ctl::W](timer_clk_ctl::W) writer structure"]
impl crate::Writable for TIMER_CLK_CTL {}
#[doc = "Timer prescaler (clk_t to timer clock frequency divider)"]
pub mod timer_clk_ctl;
#[doc = "Timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_ctl](timer_ctl) module"]
pub type TIMER_CTL = crate::Reg<u32, _TIMER_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_CTL;
#[doc = "`read()` method returns [timer_ctl::R](timer_ctl::R) reader structure"]
impl crate::Readable for TIMER_CTL {}
#[doc = "`write(|w| ..)` method takes [timer_ctl::W](timer_ctl::W) writer structure"]
impl crate::Writable for TIMER_CTL {}
#[doc = "Timer control"]
pub mod timer_ctl;
#[doc = "MPCON clock\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aclk_ctl](aclk_ctl) module"]
pub type ACLK_CTL = crate::Reg<u32, _ACLK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACLK_CTL;
#[doc = "`write(|w| ..)` method takes [aclk_ctl::W](aclk_ctl::W) writer structure"]
impl crate::Writable for ACLK_CTL {}
#[doc = "MPCON clock"]
pub mod aclk_ctl;
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
#[doc = "Cal control BG LO trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl0](cal_ctl0) module"]
pub type CAL_CTL0 = crate::Reg<u32, _CAL_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL0;
#[doc = "`read()` method returns [cal_ctl0::R](cal_ctl0::R) reader structure"]
impl crate::Readable for CAL_CTL0 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl0::W](cal_ctl0::W) writer structure"]
impl crate::Writable for CAL_CTL0 {}
#[doc = "Cal control BG LO trim bits"]
pub mod cal_ctl0;
#[doc = "Cal control BG HI trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl1](cal_ctl1) module"]
pub type CAL_CTL1 = crate::Reg<u32, _CAL_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL1;
#[doc = "`read()` method returns [cal_ctl1::R](cal_ctl1::R) reader structure"]
impl crate::Readable for CAL_CTL1 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl1::W](cal_ctl1::W) writer structure"]
impl crate::Writable for CAL_CTL1 {}
#[doc = "Cal control BG HI trim bits"]
pub mod cal_ctl1;
#[doc = "Cal control BG LO&HI trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl2](cal_ctl2) module"]
pub type CAL_CTL2 = crate::Reg<u32, _CAL_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL2;
#[doc = "`read()` method returns [cal_ctl2::R](cal_ctl2::R) reader structure"]
impl crate::Readable for CAL_CTL2 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl2::W](cal_ctl2::W) writer structure"]
impl crate::Writable for CAL_CTL2 {}
#[doc = "Cal control BG LO&HI trim bits"]
pub mod cal_ctl2;
#[doc = "Cal control osc trim bits, idac, sdac, itim\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl3](cal_ctl3) module"]
pub type CAL_CTL3 = crate::Reg<u32, _CAL_CTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL3;
#[doc = "`read()` method returns [cal_ctl3::R](cal_ctl3::R) reader structure"]
impl crate::Readable for CAL_CTL3 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl3::W](cal_ctl3::W) writer structure"]
impl crate::Writable for CAL_CTL3 {}
#[doc = "Cal control osc trim bits, idac, sdac, itim"]
pub mod cal_ctl3;
#[doc = "Cal Control Vlim, SA, fdiv, reg_act\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl4](cal_ctl4) module"]
pub type CAL_CTL4 = crate::Reg<u32, _CAL_CTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL4;
#[doc = "`read()` method returns [cal_ctl4::R](cal_ctl4::R) reader structure"]
impl crate::Readable for CAL_CTL4 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl4::W](cal_ctl4::W) writer structure"]
impl crate::Writable for CAL_CTL4 {}
#[doc = "Cal Control Vlim, SA, fdiv, reg_act"]
pub mod cal_ctl4;
#[doc = "Cal control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl5](cal_ctl5) module"]
pub type CAL_CTL5 = crate::Reg<u32, _CAL_CTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL5;
#[doc = "`read()` method returns [cal_ctl5::R](cal_ctl5::R) reader structure"]
impl crate::Readable for CAL_CTL5 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl5::W](cal_ctl5::W) writer structure"]
impl crate::Writable for CAL_CTL5 {}
#[doc = "Cal control"]
pub mod cal_ctl5;
#[doc = "SA trim LP/ULP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl6](cal_ctl6) module"]
pub type CAL_CTL6 = crate::Reg<u32, _CAL_CTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL6;
#[doc = "`read()` method returns [cal_ctl6::R](cal_ctl6::R) reader structure"]
impl crate::Readable for CAL_CTL6 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl6::W](cal_ctl6::W) writer structure"]
impl crate::Writable for CAL_CTL6 {}
#[doc = "SA trim LP/ULP"]
pub mod cal_ctl6;
#[doc = "Cal control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl7](cal_ctl7) module"]
pub type CAL_CTL7 = crate::Reg<u32, _CAL_CTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_CTL7;
#[doc = "`read()` method returns [cal_ctl7::R](cal_ctl7::R) reader structure"]
impl crate::Readable for CAL_CTL7 {}
#[doc = "`write(|w| ..)` method takes [cal_ctl7::W](cal_ctl7::W) writer structure"]
impl crate::Writable for CAL_CTL7 {}
#[doc = "Cal control"]
pub mod cal_ctl7;
#[doc = "Redundancy Control normal sectors 0,1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl01](red_ctl01) module"]
pub type RED_CTL01 = crate::Reg<u32, _RED_CTL01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL01;
#[doc = "`read()` method returns [red_ctl01::R](red_ctl01::R) reader structure"]
impl crate::Readable for RED_CTL01 {}
#[doc = "`write(|w| ..)` method takes [red_ctl01::W](red_ctl01::W) writer structure"]
impl crate::Writable for RED_CTL01 {}
#[doc = "Redundancy Control normal sectors 0,1"]
pub mod red_ctl01;
#[doc = "Redundancy Control normal sectors 2,3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl23](red_ctl23) module"]
pub type RED_CTL23 = crate::Reg<u32, _RED_CTL23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL23;
#[doc = "`read()` method returns [red_ctl23::R](red_ctl23::R) reader structure"]
impl crate::Readable for RED_CTL23 {}
#[doc = "`write(|w| ..)` method takes [red_ctl23::W](red_ctl23::W) writer structure"]
impl crate::Writable for RED_CTL23 {}
#[doc = "Redundancy Control normal sectors 2,3"]
pub mod red_ctl23;
#[doc = "Redundancy Control normal sectors 4,5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl45](red_ctl45) module"]
pub type RED_CTL45 = crate::Reg<u32, _RED_CTL45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL45;
#[doc = "`read()` method returns [red_ctl45::R](red_ctl45::R) reader structure"]
impl crate::Readable for RED_CTL45 {}
#[doc = "`write(|w| ..)` method takes [red_ctl45::W](red_ctl45::W) writer structure"]
impl crate::Writable for RED_CTL45 {}
#[doc = "Redundancy Control normal sectors 4,5"]
pub mod red_ctl45;
#[doc = "Redundancy Control normal sectors 6,7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl67](red_ctl67) module"]
pub type RED_CTL67 = crate::Reg<u32, _RED_CTL67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL67;
#[doc = "`read()` method returns [red_ctl67::R](red_ctl67::R) reader structure"]
impl crate::Readable for RED_CTL67 {}
#[doc = "`write(|w| ..)` method takes [red_ctl67::W](red_ctl67::W) writer structure"]
impl crate::Writable for RED_CTL67 {}
#[doc = "Redundancy Control normal sectors 6,7"]
pub mod red_ctl67;
#[doc = "Redundancy Control special sectors 0,1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl_sm01](red_ctl_sm01) module"]
pub type RED_CTL_SM01 = crate::Reg<u32, _RED_CTL_SM01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RED_CTL_SM01;
#[doc = "`read()` method returns [red_ctl_sm01::R](red_ctl_sm01::R) reader structure"]
impl crate::Readable for RED_CTL_SM01 {}
#[doc = "`write(|w| ..)` method takes [red_ctl_sm01::W](red_ctl_sm01::W) writer structure"]
impl crate::Writable for RED_CTL_SM01 {}
#[doc = "Redundancy Control special sectors 0,1"]
pub mod red_ctl_sm01;
#[doc = "R-grant delay for program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgrant_delay_prg](rgrant_delay_prg) module"]
pub type RGRANT_DELAY_PRG = crate::Reg<u32, _RGRANT_DELAY_PRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGRANT_DELAY_PRG;
#[doc = "`read()` method returns [rgrant_delay_prg::R](rgrant_delay_prg::R) reader structure"]
impl crate::Readable for RGRANT_DELAY_PRG {}
#[doc = "`write(|w| ..)` method takes [rgrant_delay_prg::W](rgrant_delay_prg::W) writer structure"]
impl crate::Writable for RGRANT_DELAY_PRG {}
#[doc = "R-grant delay for program"]
pub mod rgrant_delay_prg;
#[doc = "HV Pulse Delay for seq 1&2 pre\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pw_seq12](pw_seq12) module"]
pub type PW_SEQ12 = crate::Reg<u32, _PW_SEQ12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PW_SEQ12;
#[doc = "`read()` method returns [pw_seq12::R](pw_seq12::R) reader structure"]
impl crate::Readable for PW_SEQ12 {}
#[doc = "`write(|w| ..)` method takes [pw_seq12::W](pw_seq12::W) writer structure"]
impl crate::Writable for PW_SEQ12 {}
#[doc = "HV Pulse Delay for seq 1&2 pre"]
pub mod pw_seq12;
#[doc = "HV Pulse Delay for seq2 post & seq3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pw_seq23](pw_seq23) module"]
pub type PW_SEQ23 = crate::Reg<u32, _PW_SEQ23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PW_SEQ23;
#[doc = "`read()` method returns [pw_seq23::R](pw_seq23::R) reader structure"]
impl crate::Readable for PW_SEQ23 {}
#[doc = "`write(|w| ..)` method takes [pw_seq23::W](pw_seq23::W) writer structure"]
impl crate::Writable for PW_SEQ23 {}
#[doc = "HV Pulse Delay for seq2 post & seq3"]
pub mod pw_seq23;
#[doc = "R-grant delay scale for erase\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgrant_scale_ers](rgrant_scale_ers) module"]
pub type RGRANT_SCALE_ERS = crate::Reg<u32, _RGRANT_SCALE_ERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGRANT_SCALE_ERS;
#[doc = "`read()` method returns [rgrant_scale_ers::R](rgrant_scale_ers::R) reader structure"]
impl crate::Readable for RGRANT_SCALE_ERS {}
#[doc = "`write(|w| ..)` method takes [rgrant_scale_ers::W](rgrant_scale_ers::W) writer structure"]
impl crate::Writable for RGRANT_SCALE_ERS {}
#[doc = "R-grant delay scale for erase"]
pub mod rgrant_scale_ers;
#[doc = "R-grant delay for erase\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgrant_delay_ers](rgrant_delay_ers) module"]
pub type RGRANT_DELAY_ERS = crate::Reg<u32, _RGRANT_DELAY_ERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGRANT_DELAY_ERS;
#[doc = "`read()` method returns [rgrant_delay_ers::R](rgrant_delay_ers::R) reader structure"]
impl crate::Readable for RGRANT_DELAY_ERS {}
#[doc = "`write(|w| ..)` method takes [rgrant_delay_ers::W](rgrant_delay_ers::W) writer structure"]
impl crate::Writable for RGRANT_DELAY_ERS {}
#[doc = "R-grant delay for erase"]
pub mod rgrant_delay_ers;
#[doc = "Flash macro write page latches all\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_pl_wrdata_all](fm_pl_wrdata_all) module"]
pub type FM_PL_WRDATA_ALL = crate::Reg<u32, _FM_PL_WRDATA_ALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_PL_WRDATA_ALL;
#[doc = "`read()` method returns [fm_pl_wrdata_all::R](fm_pl_wrdata_all::R) reader structure"]
impl crate::Readable for FM_PL_WRDATA_ALL {}
#[doc = "`write(|w| ..)` method takes [fm_pl_wrdata_all::W](fm_pl_wrdata_all::W) writer structure"]
impl crate::Writable for FM_PL_WRDATA_ALL {}
#[doc = "Flash macro write page latches all"]
pub mod fm_pl_wrdata_all;
#[doc = "Flash macro Page Latches data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_pl_data](fm_pl_data) module"]
pub type FM_PL_DATA = crate::Reg<u32, _FM_PL_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_PL_DATA;
#[doc = "`read()` method returns [fm_pl_data::R](fm_pl_data::R) reader structure"]
impl crate::Readable for FM_PL_DATA {}
#[doc = "`write(|w| ..)` method takes [fm_pl_data::W](fm_pl_data::W) writer structure"]
impl crate::Writable for FM_PL_DATA {}
#[doc = "Flash macro Page Latches data"]
pub mod fm_pl_data;
#[doc = "Flash macro memory sense amplifier and column decoder data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_mem_data](fm_mem_data) module"]
pub type FM_MEM_DATA = crate::Reg<u32, _FM_MEM_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM_MEM_DATA;
#[doc = "`read()` method returns [fm_mem_data::R](fm_mem_data::R) reader structure"]
impl crate::Readable for FM_MEM_DATA {}
#[doc = "Flash macro memory sense amplifier and column decoder data"]
pub mod fm_mem_data;
