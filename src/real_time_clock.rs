#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC control Register"]
    pub rtcctl: RTCCTL,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - RTC interrupt vector"]
    pub rtciv: RTCIV,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - RTC moduloRegister"]
    pub rtcmod: RTCMOD,
    _reserved3: [u8; 2usize],
    #[doc = "0x0c - RTC counter Register"]
    pub rtccnt: RTCCNT,
}
#[doc = "RTC control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl](rtcctl) module"]
pub type RTCCTL = crate::Reg<u16, _RTCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCTL;
#[doc = "`read()` method returns [rtcctl::R](rtcctl::R) reader structure"]
impl crate::Readable for RTCCTL {}
#[doc = "`write(|w| ..)` method takes [rtcctl::W](rtcctl::W) writer structure"]
impl crate::Writable for RTCCTL {}
#[doc = "RTC control Register"]
pub mod rtcctl;
#[doc = "RTC interrupt vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtciv](rtciv) module"]
pub type RTCIV = crate::Reg<u16, _RTCIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCIV;
#[doc = "`read()` method returns [rtciv::R](rtciv::R) reader structure"]
impl crate::Readable for RTCIV {}
#[doc = "`write(|w| ..)` method takes [rtciv::W](rtciv::W) writer structure"]
impl crate::Writable for RTCIV {}
#[doc = "RTC interrupt vector"]
pub mod rtciv;
#[doc = "RTC moduloRegister\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcmod](rtcmod) module"]
pub type RTCMOD = crate::Reg<u16, _RTCMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCMOD;
#[doc = "`read()` method returns [rtcmod::R](rtcmod::R) reader structure"]
impl crate::Readable for RTCMOD {}
#[doc = "`write(|w| ..)` method takes [rtcmod::W](rtcmod::W) writer structure"]
impl crate::Writable for RTCMOD {}
#[doc = "RTC moduloRegister"]
pub mod rtcmod;
#[doc = "RTC counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccnt](rtccnt) module"]
pub type RTCCNT = crate::Reg<u16, _RTCCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCNT;
#[doc = "`read()` method returns [rtccnt::R](rtccnt::R) reader structure"]
impl crate::Readable for RTCCNT {}
#[doc = "`write(|w| ..)` method takes [rtccnt::W](rtccnt::W) writer structure"]
impl crate::Writable for RTCCNT {}
#[doc = "RTC counter Register"]
pub mod rtccnt;
