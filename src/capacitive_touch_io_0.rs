#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Capacitive_Touch_IO 0 control register"]
    pub captio0ctl: CAPTIO0CTL,
}
#[doc = "Capacitive_Touch_IO 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [captio0ctl](captio0ctl) module"]
pub type CAPTIO0CTL = crate::Reg<u16, _CAPTIO0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTIO0CTL;
#[doc = "`read()` method returns [captio0ctl::R](captio0ctl::R) reader structure"]
impl crate::Readable for CAPTIO0CTL {}
#[doc = "`write(|w| ..)` method takes [captio0ctl::W](captio0ctl::W) writer structure"]
impl crate::Writable for CAPTIO0CTL {}
#[doc = "Capacitive_Touch_IO 0 control register"]
pub mod captio0ctl;
