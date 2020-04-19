#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMM Control 0"]
    pub pmmctl0: PMMCTL0,
    #[doc = "0x02 - PMM Control 1"]
    pub pmmctl1: PMMCTL1,
    #[doc = "0x04 - PMM Control 2"]
    pub pmmctl2: PMMCTL2,
    _reserved3: [u8; 4usize],
    #[doc = "0x0a - PMM Interrupt Flag"]
    pub pmmifg: PMMIFG,
    _reserved4: [u8; 2usize],
    #[doc = "0x0e - PMM Interrupt Enable"]
    pub pmmie: PMMIE,
    #[doc = "0x10 - PMM Power Mode 5 Control Register 0"]
    pub pm5ctl0: PM5CTL0,
}
#[doc = "PMM Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl0](pmmctl0) module"]
pub type PMMCTL0 = crate::Reg<u16, _PMMCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMCTL0;
#[doc = "`read()` method returns [pmmctl0::R](pmmctl0::R) reader structure"]
impl crate::Readable for PMMCTL0 {}
#[doc = "`write(|w| ..)` method takes [pmmctl0::W](pmmctl0::W) writer structure"]
impl crate::Writable for PMMCTL0 {}
#[doc = "PMM Control 0"]
pub mod pmmctl0;
#[doc = "PMM Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl1](pmmctl1) module"]
pub type PMMCTL1 = crate::Reg<u16, _PMMCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMCTL1;
#[doc = "`read()` method returns [pmmctl1::R](pmmctl1::R) reader structure"]
impl crate::Readable for PMMCTL1 {}
#[doc = "`write(|w| ..)` method takes [pmmctl1::W](pmmctl1::W) writer structure"]
impl crate::Writable for PMMCTL1 {}
#[doc = "PMM Control 1"]
pub mod pmmctl1;
#[doc = "PMM Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl2](pmmctl2) module"]
pub type PMMCTL2 = crate::Reg<u16, _PMMCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMCTL2;
#[doc = "`read()` method returns [pmmctl2::R](pmmctl2::R) reader structure"]
impl crate::Readable for PMMCTL2 {}
#[doc = "`write(|w| ..)` method takes [pmmctl2::W](pmmctl2::W) writer structure"]
impl crate::Writable for PMMCTL2 {}
#[doc = "PMM Control 2"]
pub mod pmmctl2;
#[doc = "PMM Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmifg](pmmifg) module"]
pub type PMMIFG = crate::Reg<u16, _PMMIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMIFG;
#[doc = "`read()` method returns [pmmifg::R](pmmifg::R) reader structure"]
impl crate::Readable for PMMIFG {}
#[doc = "`write(|w| ..)` method takes [pmmifg::W](pmmifg::W) writer structure"]
impl crate::Writable for PMMIFG {}
#[doc = "PMM Interrupt Flag"]
pub mod pmmifg;
#[doc = "PMM Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmie](pmmie) module"]
pub type PMMIE = crate::Reg<u16, _PMMIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMIE;
#[doc = "`read()` method returns [pmmie::R](pmmie::R) reader structure"]
impl crate::Readable for PMMIE {}
#[doc = "`write(|w| ..)` method takes [pmmie::W](pmmie::W) writer structure"]
impl crate::Writable for PMMIE {}
#[doc = "PMM Interrupt Enable"]
pub mod pmmie;
#[doc = "PMM Power Mode 5 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pm5ctl0](pm5ctl0) module"]
pub type PM5CTL0 = crate::Reg<u16, _PM5CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PM5CTL0;
#[doc = "`read()` method returns [pm5ctl0::R](pm5ctl0::R) reader structure"]
impl crate::Readable for PM5CTL0 {}
#[doc = "`write(|w| ..)` method takes [pm5ctl0::W](pm5ctl0::W) writer structure"]
impl crate::Writable for PM5CTL0 {}
#[doc = "PMM Power Mode 5 Control Register 0"]
pub mod pm5ctl0;
