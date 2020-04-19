#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 7 Input"]
    pub p7in: P7IN,
    #[doc = "0x01 - Port 8 Input"]
    pub p8in: P8IN,
    #[doc = "0x02 - Port 7 Output"]
    pub p7out: P7OUT,
    #[doc = "0x03 - Port 8 Output"]
    pub p8out: P8OUT,
    #[doc = "0x04 - Port 7 Direction"]
    pub p7dir: P7DIR,
    #[doc = "0x05 - Port 8 Direction"]
    pub p8dir: P8DIR,
    #[doc = "0x06 - Port 7 Resistor Enable"]
    pub p7ren: P7REN,
    #[doc = "0x07 - Port 8 Resistor Enable"]
    pub p8ren: P8REN,
    _reserved8: [u8; 2usize],
    #[doc = "0x0a - Port 7 Selection 0"]
    pub p7sel0: P7SEL0,
    #[doc = "0x0b - Port 8 Selection 0"]
    pub p8sel0: P8SEL0,
}
#[doc = "Port 7 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7in](p7in) module"]
pub type P7IN = crate::Reg<u8, _P7IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7IN;
#[doc = "`read()` method returns [p7in::R](p7in::R) reader structure"]
impl crate::Readable for P7IN {}
#[doc = "`write(|w| ..)` method takes [p7in::W](p7in::W) writer structure"]
impl crate::Writable for P7IN {}
#[doc = "Port 7 Input"]
pub mod p7in;
#[doc = "Port 8 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8in](p8in) module"]
pub type P8IN = crate::Reg<u8, _P8IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8IN;
#[doc = "`read()` method returns [p8in::R](p8in::R) reader structure"]
impl crate::Readable for P8IN {}
#[doc = "`write(|w| ..)` method takes [p8in::W](p8in::W) writer structure"]
impl crate::Writable for P8IN {}
#[doc = "Port 8 Input"]
pub mod p8in;
#[doc = "Port 7 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7out](p7out) module"]
pub type P7OUT = crate::Reg<u8, _P7OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7OUT;
#[doc = "`read()` method returns [p7out::R](p7out::R) reader structure"]
impl crate::Readable for P7OUT {}
#[doc = "`write(|w| ..)` method takes [p7out::W](p7out::W) writer structure"]
impl crate::Writable for P7OUT {}
#[doc = "Port 7 Output"]
pub mod p7out;
#[doc = "Port 8 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8out](p8out) module"]
pub type P8OUT = crate::Reg<u8, _P8OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8OUT;
#[doc = "`read()` method returns [p8out::R](p8out::R) reader structure"]
impl crate::Readable for P8OUT {}
#[doc = "`write(|w| ..)` method takes [p8out::W](p8out::W) writer structure"]
impl crate::Writable for P8OUT {}
#[doc = "Port 8 Output"]
pub mod p8out;
#[doc = "Port 7 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7dir](p7dir) module"]
pub type P7DIR = crate::Reg<u8, _P7DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7DIR;
#[doc = "`read()` method returns [p7dir::R](p7dir::R) reader structure"]
impl crate::Readable for P7DIR {}
#[doc = "`write(|w| ..)` method takes [p7dir::W](p7dir::W) writer structure"]
impl crate::Writable for P7DIR {}
#[doc = "Port 7 Direction"]
pub mod p7dir;
#[doc = "Port 8 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8dir](p8dir) module"]
pub type P8DIR = crate::Reg<u8, _P8DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8DIR;
#[doc = "`read()` method returns [p8dir::R](p8dir::R) reader structure"]
impl crate::Readable for P8DIR {}
#[doc = "`write(|w| ..)` method takes [p8dir::W](p8dir::W) writer structure"]
impl crate::Writable for P8DIR {}
#[doc = "Port 8 Direction"]
pub mod p8dir;
#[doc = "Port 7 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7ren](p7ren) module"]
pub type P7REN = crate::Reg<u8, _P7REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7REN;
#[doc = "`read()` method returns [p7ren::R](p7ren::R) reader structure"]
impl crate::Readable for P7REN {}
#[doc = "`write(|w| ..)` method takes [p7ren::W](p7ren::W) writer structure"]
impl crate::Writable for P7REN {}
#[doc = "Port 7 Resistor Enable"]
pub mod p7ren;
#[doc = "Port 8 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8ren](p8ren) module"]
pub type P8REN = crate::Reg<u8, _P8REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8REN;
#[doc = "`read()` method returns [p8ren::R](p8ren::R) reader structure"]
impl crate::Readable for P8REN {}
#[doc = "`write(|w| ..)` method takes [p8ren::W](p8ren::W) writer structure"]
impl crate::Writable for P8REN {}
#[doc = "Port 8 Resistor Enable"]
pub mod p8ren;
#[doc = "Port 7 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7sel0](p7sel0) module"]
pub type P7SEL0 = crate::Reg<u8, _P7SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7SEL0;
#[doc = "`read()` method returns [p7sel0::R](p7sel0::R) reader structure"]
impl crate::Readable for P7SEL0 {}
#[doc = "`write(|w| ..)` method takes [p7sel0::W](p7sel0::W) writer structure"]
impl crate::Writable for P7SEL0 {}
#[doc = "Port 7 Selection 0"]
pub mod p7sel0;
#[doc = "Port 8 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8sel0](p8sel0) module"]
pub type P8SEL0 = crate::Reg<u8, _P8SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8SEL0;
#[doc = "`read()` method returns [p8sel0::R](p8sel0::R) reader structure"]
impl crate::Readable for P8SEL0 {}
#[doc = "`write(|w| ..)` method takes [p8sel0::W](p8sel0::W) writer structure"]
impl crate::Writable for P8SEL0 {}
#[doc = "Port 8 Selection 0"]
pub mod p8sel0;
