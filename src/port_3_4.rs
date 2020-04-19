#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    pub p3in: P3IN,
    #[doc = "0x01 - Port 4 Input"]
    pub p4in: P4IN,
    #[doc = "0x02 - Port 3 Output"]
    pub p3out: P3OUT,
    #[doc = "0x03 - Port 4 Output"]
    pub p4out: P4OUT,
    #[doc = "0x04 - Port 3 Direction"]
    pub p3dir: P3DIR,
    #[doc = "0x05 - Port 4 Direction"]
    pub p4dir: P4DIR,
    #[doc = "0x06 - Port 3 Resistor Enable"]
    pub p3ren: P3REN,
    #[doc = "0x07 - Port 4 Resistor Enable"]
    pub p4ren: P4REN,
    _reserved8: [u8; 2usize],
    #[doc = "0x0a - Port 3 Selection 0"]
    pub p3sel0: P3SEL0,
    #[doc = "0x0b - Port 4 Selection 0"]
    pub p4sel0: P4SEL0,
}
#[doc = "Port 3 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3in](p3in) module"]
pub type P3IN = crate::Reg<u8, _P3IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IN;
#[doc = "`read()` method returns [p3in::R](p3in::R) reader structure"]
impl crate::Readable for P3IN {}
#[doc = "`write(|w| ..)` method takes [p3in::W](p3in::W) writer structure"]
impl crate::Writable for P3IN {}
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "Port 4 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4in](p4in) module"]
pub type P4IN = crate::Reg<u8, _P4IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IN;
#[doc = "`read()` method returns [p4in::R](p4in::R) reader structure"]
impl crate::Readable for P4IN {}
#[doc = "`write(|w| ..)` method takes [p4in::W](p4in::W) writer structure"]
impl crate::Writable for P4IN {}
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "Port 3 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3out](p3out) module"]
pub type P3OUT = crate::Reg<u8, _P3OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3OUT;
#[doc = "`read()` method returns [p3out::R](p3out::R) reader structure"]
impl crate::Readable for P3OUT {}
#[doc = "`write(|w| ..)` method takes [p3out::W](p3out::W) writer structure"]
impl crate::Writable for P3OUT {}
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "Port 4 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4out](p4out) module"]
pub type P4OUT = crate::Reg<u8, _P4OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4OUT;
#[doc = "`read()` method returns [p4out::R](p4out::R) reader structure"]
impl crate::Readable for P4OUT {}
#[doc = "`write(|w| ..)` method takes [p4out::W](p4out::W) writer structure"]
impl crate::Writable for P4OUT {}
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "Port 3 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3dir](p3dir) module"]
pub type P3DIR = crate::Reg<u8, _P3DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3DIR;
#[doc = "`read()` method returns [p3dir::R](p3dir::R) reader structure"]
impl crate::Readable for P3DIR {}
#[doc = "`write(|w| ..)` method takes [p3dir::W](p3dir::W) writer structure"]
impl crate::Writable for P3DIR {}
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "Port 4 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4dir](p4dir) module"]
pub type P4DIR = crate::Reg<u8, _P4DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4DIR;
#[doc = "`read()` method returns [p4dir::R](p4dir::R) reader structure"]
impl crate::Readable for P4DIR {}
#[doc = "`write(|w| ..)` method takes [p4dir::W](p4dir::W) writer structure"]
impl crate::Writable for P4DIR {}
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "Port 3 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ren](p3ren) module"]
pub type P3REN = crate::Reg<u8, _P3REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3REN;
#[doc = "`read()` method returns [p3ren::R](p3ren::R) reader structure"]
impl crate::Readable for P3REN {}
#[doc = "`write(|w| ..)` method takes [p3ren::W](p3ren::W) writer structure"]
impl crate::Writable for P3REN {}
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "Port 4 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4ren](p4ren) module"]
pub type P4REN = crate::Reg<u8, _P4REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4REN;
#[doc = "`read()` method returns [p4ren::R](p4ren::R) reader structure"]
impl crate::Readable for P4REN {}
#[doc = "`write(|w| ..)` method takes [p4ren::W](p4ren::W) writer structure"]
impl crate::Writable for P4REN {}
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
#[doc = "Port 3 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3sel0](p3sel0) module"]
pub type P3SEL0 = crate::Reg<u8, _P3SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3SEL0;
#[doc = "`read()` method returns [p3sel0::R](p3sel0::R) reader structure"]
impl crate::Readable for P3SEL0 {}
#[doc = "`write(|w| ..)` method takes [p3sel0::W](p3sel0::W) writer structure"]
impl crate::Writable for P3SEL0 {}
#[doc = "Port 3 Selection 0"]
pub mod p3sel0;
#[doc = "Port 4 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4sel0](p4sel0) module"]
pub type P4SEL0 = crate::Reg<u8, _P4SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4SEL0;
#[doc = "`read()` method returns [p4sel0::R](p4sel0::R) reader structure"]
impl crate::Readable for P4SEL0 {}
#[doc = "`write(|w| ..)` method takes [p4sel0::W](p4sel0::W) writer structure"]
impl crate::Writable for P4SEL0 {}
#[doc = "Port 4 Selection 0"]
pub mod p4sel0;
