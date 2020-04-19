#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 5 Input"]
    pub p5in: P5IN,
    #[doc = "0x01 - Port 6 Input"]
    pub p6in: P6IN,
    #[doc = "0x02 - Port 5 Output"]
    pub p5out: P5OUT,
    #[doc = "0x03 - Port 6 Output"]
    pub p6out: P6OUT,
    #[doc = "0x04 - Port 5 Direction"]
    pub p5dir: P5DIR,
    #[doc = "0x05 - Port 6 Direction"]
    pub p6dir: P6DIR,
    #[doc = "0x06 - Port 5 Resistor Enable"]
    pub p5ren: P5REN,
    #[doc = "0x07 - Port 6 Resistor Enable"]
    pub p6ren: P6REN,
    _reserved8: [u8; 2usize],
    #[doc = "0x0a - Port 5 Selection 0"]
    pub p5sel0: P5SEL0,
    #[doc = "0x0b - Port 6 Selection 0"]
    pub p6sel0: P6SEL0,
}
#[doc = "Port 5 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5in](p5in) module"]
pub type P5IN = crate::Reg<u8, _P5IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5IN;
#[doc = "`read()` method returns [p5in::R](p5in::R) reader structure"]
impl crate::Readable for P5IN {}
#[doc = "`write(|w| ..)` method takes [p5in::W](p5in::W) writer structure"]
impl crate::Writable for P5IN {}
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "Port 6 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6in](p6in) module"]
pub type P6IN = crate::Reg<u8, _P6IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6IN;
#[doc = "`read()` method returns [p6in::R](p6in::R) reader structure"]
impl crate::Readable for P6IN {}
#[doc = "`write(|w| ..)` method takes [p6in::W](p6in::W) writer structure"]
impl crate::Writable for P6IN {}
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "Port 5 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5out](p5out) module"]
pub type P5OUT = crate::Reg<u8, _P5OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5OUT;
#[doc = "`read()` method returns [p5out::R](p5out::R) reader structure"]
impl crate::Readable for P5OUT {}
#[doc = "`write(|w| ..)` method takes [p5out::W](p5out::W) writer structure"]
impl crate::Writable for P5OUT {}
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "Port 6 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6out](p6out) module"]
pub type P6OUT = crate::Reg<u8, _P6OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6OUT;
#[doc = "`read()` method returns [p6out::R](p6out::R) reader structure"]
impl crate::Readable for P6OUT {}
#[doc = "`write(|w| ..)` method takes [p6out::W](p6out::W) writer structure"]
impl crate::Writable for P6OUT {}
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "Port 5 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5dir](p5dir) module"]
pub type P5DIR = crate::Reg<u8, _P5DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5DIR;
#[doc = "`read()` method returns [p5dir::R](p5dir::R) reader structure"]
impl crate::Readable for P5DIR {}
#[doc = "`write(|w| ..)` method takes [p5dir::W](p5dir::W) writer structure"]
impl crate::Writable for P5DIR {}
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "Port 6 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6dir](p6dir) module"]
pub type P6DIR = crate::Reg<u8, _P6DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6DIR;
#[doc = "`read()` method returns [p6dir::R](p6dir::R) reader structure"]
impl crate::Readable for P6DIR {}
#[doc = "`write(|w| ..)` method takes [p6dir::W](p6dir::W) writer structure"]
impl crate::Writable for P6DIR {}
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "Port 5 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5ren](p5ren) module"]
pub type P5REN = crate::Reg<u8, _P5REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5REN;
#[doc = "`read()` method returns [p5ren::R](p5ren::R) reader structure"]
impl crate::Readable for P5REN {}
#[doc = "`write(|w| ..)` method takes [p5ren::W](p5ren::W) writer structure"]
impl crate::Writable for P5REN {}
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "Port 6 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6ren](p6ren) module"]
pub type P6REN = crate::Reg<u8, _P6REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6REN;
#[doc = "`read()` method returns [p6ren::R](p6ren::R) reader structure"]
impl crate::Readable for P6REN {}
#[doc = "`write(|w| ..)` method takes [p6ren::W](p6ren::W) writer structure"]
impl crate::Writable for P6REN {}
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "Port 5 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5sel0](p5sel0) module"]
pub type P5SEL0 = crate::Reg<u8, _P5SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5SEL0;
#[doc = "`read()` method returns [p5sel0::R](p5sel0::R) reader structure"]
impl crate::Readable for P5SEL0 {}
#[doc = "`write(|w| ..)` method takes [p5sel0::W](p5sel0::W) writer structure"]
impl crate::Writable for P5SEL0 {}
#[doc = "Port 5 Selection 0"]
pub mod p5sel0;
#[doc = "Port 6 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6sel0](p6sel0) module"]
pub type P6SEL0 = crate::Reg<u8, _P6SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6SEL0;
#[doc = "`read()` method returns [p6sel0::R](p6sel0::R) reader structure"]
impl crate::Readable for P6SEL0 {}
#[doc = "`write(|w| ..)` method takes [p6sel0::W](p6sel0::W) writer structure"]
impl crate::Writable for P6SEL0 {}
#[doc = "Port 6 Selection 0"]
pub mod p6sel0;
