#[doc = "Trigger control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctl](tr_ctl) module"]
pub type TR_CTL = crate::Reg<u32, _TR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_CTL;
#[doc = "`read()` method returns [tr_ctl::R](tr_ctl::R) reader structure"]
impl crate::Readable for TR_CTL {}
#[doc = "`write(|w| ..)` method takes [tr_ctl::W](tr_ctl::W) writer structure"]
impl crate::Writable for TR_CTL {}
#[doc = "Trigger control register"]
pub mod tr_ctl;
