#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 1 Input"]
    pub p1in: P1IN,
    #[doc = "0x01 - Port 2 Input"]
    pub p2in: P2IN,
    #[doc = "0x02 - Port 1 Output"]
    pub p1out: P1OUT,
    #[doc = "0x03 - Port 2 Output"]
    pub p2out: P2OUT,
    #[doc = "0x04 - Port 1 Direction"]
    pub p1dir: P1DIR,
    #[doc = "0x05 - Port 2 Direction"]
    pub p2dir: P2DIR,
    #[doc = "0x06 - Port 1 Resistor Enable"]
    pub p1ren: P1REN,
    #[doc = "0x07 - Port 2 Resistor Enable"]
    pub p2ren: P2REN,
    _reserved8: [u8; 2usize],
    #[doc = "0x0a - Port 1 Selection 0"]
    pub p1sel0: P1SEL0,
    #[doc = "0x0b - Port 2 Selection 0"]
    pub p2sel0: P2SEL0,
    _reserved10: [u8; 2usize],
    #[doc = "0x0e - Port 1 Interrupt Vector Word"]
    pub p1iv: P1IV,
    _reserved11: [u8; 8usize],
    #[doc = "0x18 - Port 1 Interrupt Edge Select"]
    pub p1ies: P1IES,
    #[doc = "0x19 - Port 2 Interrupt Edge Select"]
    pub p2ies: P2IES,
    #[doc = "0x1a - Port 1 Interrupt Enable"]
    pub p1ie: P1IE,
    #[doc = "0x1b - Port 2 Interrupt Enable"]
    pub p2ie: P2IE,
    #[doc = "0x1c - Port 1 Interrupt Flag"]
    pub p1ifg: P1IFG,
    #[doc = "0x1d - Port 2 Interrupt Flag"]
    pub p2ifg: P2IFG,
    #[doc = "0x1e - Port 2 Interrupt Vector Word"]
    pub p2iv: P2IV,
}
#[doc = "Port 1 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1in](p1in) module"]
pub type P1IN = crate::Reg<u8, _P1IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IN;
#[doc = "`read()` method returns [p1in::R](p1in::R) reader structure"]
impl crate::Readable for P1IN {}
#[doc = "`write(|w| ..)` method takes [p1in::W](p1in::W) writer structure"]
impl crate::Writable for P1IN {}
#[doc = "Port 1 Input"]
pub mod p1in;
#[doc = "Port 2 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2in](p2in) module"]
pub type P2IN = crate::Reg<u8, _P2IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IN;
#[doc = "`read()` method returns [p2in::R](p2in::R) reader structure"]
impl crate::Readable for P2IN {}
#[doc = "`write(|w| ..)` method takes [p2in::W](p2in::W) writer structure"]
impl crate::Writable for P2IN {}
#[doc = "Port 2 Input"]
pub mod p2in;
#[doc = "Port 1 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1out](p1out) module"]
pub type P1OUT = crate::Reg<u8, _P1OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1OUT;
#[doc = "`read()` method returns [p1out::R](p1out::R) reader structure"]
impl crate::Readable for P1OUT {}
#[doc = "`write(|w| ..)` method takes [p1out::W](p1out::W) writer structure"]
impl crate::Writable for P1OUT {}
#[doc = "Port 1 Output"]
pub mod p1out;
#[doc = "Port 2 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2out](p2out) module"]
pub type P2OUT = crate::Reg<u8, _P2OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2OUT;
#[doc = "`read()` method returns [p2out::R](p2out::R) reader structure"]
impl crate::Readable for P2OUT {}
#[doc = "`write(|w| ..)` method takes [p2out::W](p2out::W) writer structure"]
impl crate::Writable for P2OUT {}
#[doc = "Port 2 Output"]
pub mod p2out;
#[doc = "Port 1 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1dir](p1dir) module"]
pub type P1DIR = crate::Reg<u8, _P1DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1DIR;
#[doc = "`read()` method returns [p1dir::R](p1dir::R) reader structure"]
impl crate::Readable for P1DIR {}
#[doc = "`write(|w| ..)` method takes [p1dir::W](p1dir::W) writer structure"]
impl crate::Writable for P1DIR {}
#[doc = "Port 1 Direction"]
pub mod p1dir;
#[doc = "Port 2 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2dir](p2dir) module"]
pub type P2DIR = crate::Reg<u8, _P2DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2DIR;
#[doc = "`read()` method returns [p2dir::R](p2dir::R) reader structure"]
impl crate::Readable for P2DIR {}
#[doc = "`write(|w| ..)` method takes [p2dir::W](p2dir::W) writer structure"]
impl crate::Writable for P2DIR {}
#[doc = "Port 2 Direction"]
pub mod p2dir;
#[doc = "Port 1 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ren](p1ren) module"]
pub type P1REN = crate::Reg<u8, _P1REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1REN;
#[doc = "`read()` method returns [p1ren::R](p1ren::R) reader structure"]
impl crate::Readable for P1REN {}
#[doc = "`write(|w| ..)` method takes [p1ren::W](p1ren::W) writer structure"]
impl crate::Writable for P1REN {}
#[doc = "Port 1 Resistor Enable"]
pub mod p1ren;
#[doc = "Port 2 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ren](p2ren) module"]
pub type P2REN = crate::Reg<u8, _P2REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2REN;
#[doc = "`read()` method returns [p2ren::R](p2ren::R) reader structure"]
impl crate::Readable for P2REN {}
#[doc = "`write(|w| ..)` method takes [p2ren::W](p2ren::W) writer structure"]
impl crate::Writable for P2REN {}
#[doc = "Port 2 Resistor Enable"]
pub mod p2ren;
#[doc = "Port 1 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1sel0](p1sel0) module"]
pub type P1SEL0 = crate::Reg<u8, _P1SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1SEL0;
#[doc = "`read()` method returns [p1sel0::R](p1sel0::R) reader structure"]
impl crate::Readable for P1SEL0 {}
#[doc = "`write(|w| ..)` method takes [p1sel0::W](p1sel0::W) writer structure"]
impl crate::Writable for P1SEL0 {}
#[doc = "Port 1 Selection 0"]
pub mod p1sel0;
#[doc = "Port 2 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2sel0](p2sel0) module"]
pub type P2SEL0 = crate::Reg<u8, _P2SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2SEL0;
#[doc = "`read()` method returns [p2sel0::R](p2sel0::R) reader structure"]
impl crate::Readable for P2SEL0 {}
#[doc = "`write(|w| ..)` method takes [p2sel0::W](p2sel0::W) writer structure"]
impl crate::Writable for P2SEL0 {}
#[doc = "Port 2 Selection 0"]
pub mod p2sel0;
#[doc = "Port 1 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ies](p1ies) module"]
pub type P1IES = crate::Reg<u8, _P1IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IES;
#[doc = "`read()` method returns [p1ies::R](p1ies::R) reader structure"]
impl crate::Readable for P1IES {}
#[doc = "`write(|w| ..)` method takes [p1ies::W](p1ies::W) writer structure"]
impl crate::Writable for P1IES {}
#[doc = "Port 1 Interrupt Edge Select"]
pub mod p1ies;
#[doc = "Port 2 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ies](p2ies) module"]
pub type P2IES = crate::Reg<u8, _P2IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IES;
#[doc = "`read()` method returns [p2ies::R](p2ies::R) reader structure"]
impl crate::Readable for P2IES {}
#[doc = "`write(|w| ..)` method takes [p2ies::W](p2ies::W) writer structure"]
impl crate::Writable for P2IES {}
#[doc = "Port 2 Interrupt Edge Select"]
pub mod p2ies;
#[doc = "Port 1 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ie](p1ie) module"]
pub type P1IE = crate::Reg<u8, _P1IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IE;
#[doc = "`read()` method returns [p1ie::R](p1ie::R) reader structure"]
impl crate::Readable for P1IE {}
#[doc = "`write(|w| ..)` method takes [p1ie::W](p1ie::W) writer structure"]
impl crate::Writable for P1IE {}
#[doc = "Port 1 Interrupt Enable"]
pub mod p1ie;
#[doc = "Port 2 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ie](p2ie) module"]
pub type P2IE = crate::Reg<u8, _P2IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IE;
#[doc = "`read()` method returns [p2ie::R](p2ie::R) reader structure"]
impl crate::Readable for P2IE {}
#[doc = "`write(|w| ..)` method takes [p2ie::W](p2ie::W) writer structure"]
impl crate::Writable for P2IE {}
#[doc = "Port 2 Interrupt Enable"]
pub mod p2ie;
#[doc = "Port 1 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ifg](p1ifg) module"]
pub type P1IFG = crate::Reg<u8, _P1IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IFG;
#[doc = "`read()` method returns [p1ifg::R](p1ifg::R) reader structure"]
impl crate::Readable for P1IFG {}
#[doc = "`write(|w| ..)` method takes [p1ifg::W](p1ifg::W) writer structure"]
impl crate::Writable for P1IFG {}
#[doc = "Port 1 Interrupt Flag"]
pub mod p1ifg;
#[doc = "Port 2 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ifg](p2ifg) module"]
pub type P2IFG = crate::Reg<u8, _P2IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IFG;
#[doc = "`read()` method returns [p2ifg::R](p2ifg::R) reader structure"]
impl crate::Readable for P2IFG {}
#[doc = "`write(|w| ..)` method takes [p2ifg::W](p2ifg::W) writer structure"]
impl crate::Writable for P2IFG {}
#[doc = "Port 2 Interrupt Flag"]
pub mod p2ifg;
#[doc = "Port 1 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1iv](p1iv) module"]
pub type P1IV = crate::Reg<u16, _P1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IV;
#[doc = "`read()` method returns [p1iv::R](p1iv::R) reader structure"]
impl crate::Readable for P1IV {}
#[doc = "`write(|w| ..)` method takes [p1iv::W](p1iv::W) writer structure"]
impl crate::Writable for P1IV {}
#[doc = "Port 1 Interrupt Vector Word"]
pub mod p1iv;
#[doc = "Port 2 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2iv](p2iv) module"]
pub type P2IV = crate::Reg<u16, _P2IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IV;
#[doc = "`read()` method returns [p2iv::R](p2iv::R) reader structure"]
impl crate::Readable for P2IV {}
#[doc = "`write(|w| ..)` method takes [p2iv::W](p2iv::W) writer structure"]
impl crate::Writable for P2IV {}
#[doc = "Port 2 Interrupt Vector Word"]
pub mod p2iv;
