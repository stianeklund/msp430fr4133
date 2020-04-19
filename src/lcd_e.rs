#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD_E Control Register 0"]
    pub lcdctl0: LCDCTL0,
    #[doc = "0x02 - LCD_E Control Register 1"]
    pub lcdctl1: LCDCTL1,
    #[doc = "0x04 - LCD_E blinking control register"]
    pub lcdblkctl: LCDBLKCTL,
    #[doc = "0x06 - LCD_E memory control register"]
    pub lcdmemctl: LCDMEMCTL,
    #[doc = "0x08 - LCD_E Voltage Control Register"]
    pub lcdvctl: LCDVCTL,
    #[doc = "0x0a - LCD_E Port Control Register 0"]
    pub lcdpctl0: LCDPCTL0,
    #[doc = "0x0c - LCD_E Port Control Register 1"]
    pub lcdpctl1: LCDPCTL1,
    #[doc = "0x0e - LCD_E Port Control Register 2"]
    pub lcdpctl2: LCDPCTL2,
    _reserved8: [u8; 4usize],
    #[doc = "0x14 - LCD_E COM/SEG select register 0"]
    pub lcdcssel0: LCDCSSEL0,
    #[doc = "0x16 - LCD_E COM/SEG select register 1"]
    pub lcdcssel1: LCDCSSEL1,
    #[doc = "0x18 - LCD_E COM/SEG select register 2"]
    pub lcdcssel2: LCDCSSEL2,
    _reserved11: [u8; 4usize],
    #[doc = "0x1e - LCD_E Interrupt Vector Register"]
    pub lcdiv: LCDIV,
    #[doc = "0x20 - LCD Memory 0/1"]
    pub lcdm0w: LCDM0W,
    #[doc = "0x22 - LCD Memory 2/3"]
    pub lcdm2w: LCDM2W,
    #[doc = "0x24 - LCD Memory 4/5"]
    pub lcdm4w: LCDM4W,
    #[doc = "0x26 - LCD Memory 6/7"]
    pub lcdm6w: LCDM6W,
    #[doc = "0x28 - LCD Memory 8/9"]
    pub lcdm8w: LCDM8W,
    #[doc = "0x2a - LCD Memory 10/11"]
    pub lcdm10w: LCDM10W,
    #[doc = "0x2c - LCD Memory 12/13"]
    pub lcdm12w: LCDM12W,
    #[doc = "0x2e - LCD Memory 14/15"]
    pub lcdm14w: LCDM14W,
    #[doc = "0x30 - LCD Memory 16/17"]
    pub lcdm16w: LCDM16W,
    #[doc = "0x32 - LCD Memory 18/19"]
    pub lcdm18w: LCDM18W,
    #[doc = "0x34 - LCD Memory 20/21"]
    pub lcdm20w: LCDM20W,
    #[doc = "0x36 - LCD Memory 22/23"]
    pub lcdm22w: LCDM22W,
    #[doc = "0x38 - LCD Memory 24/25"]
    pub lcdm24w: LCDM24W,
    #[doc = "0x3a - LCD Memory 26/27"]
    pub lcdm26w: LCDM26W,
    #[doc = "0x3c - LCD Memory 28/29"]
    pub lcdm28w: LCDM28W,
    #[doc = "0x3e - LCD Memory 30/31"]
    pub lcdm30w: LCDM30W,
    _reserved_28_lcdbm0w: [u8; 2usize],
    _reserved_29_lcdbm2w: [u8; 2usize],
    _reserved_30_lcdbm4w: [u8; 2usize],
    _reserved_31_lcdbm6w: [u8; 2usize],
    #[doc = "0x48 - LCD Blinking Memory 8/9"]
    pub lcdbm8w: LCDBM8W,
    #[doc = "0x4a - LCD Blinking Memory 10/11"]
    pub lcdbm10w: LCDBM10W,
    #[doc = "0x4c - LCD Blinking Memory 12/13"]
    pub lcdbm12w: LCDBM12W,
    #[doc = "0x4e - LCD Blinking Memory 14/15"]
    pub lcdbm14w: LCDBM14W,
    #[doc = "0x50 - LCD Blinking Memory 16/17"]
    pub lcdbm16w: LCDBM16W,
    #[doc = "0x52 - LCD Blinking Memory 18/19"]
    pub lcdbm18w: LCDBM18W,
}
impl RegisterBlock {
    #[doc = "0x40 - LCD Memory 32/33"]
    #[inline(always)]
    pub fn lcdm32w(&self) -> &LCDM32W {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const LCDM32W) }
    }
    #[doc = "0x40 - LCD Memory 32/33"]
    #[inline(always)]
    pub fn lcdm32w_mut(&self) -> &mut LCDM32W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut LCDM32W) }
    }
    #[doc = "0x40 - LCD Blinking Memory 0/1"]
    #[inline(always)]
    pub fn lcdbm0w(&self) -> &LCDBM0W {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const LCDBM0W) }
    }
    #[doc = "0x40 - LCD Blinking Memory 0/1"]
    #[inline(always)]
    pub fn lcdbm0w_mut(&self) -> &mut LCDBM0W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut LCDBM0W) }
    }
    #[doc = "0x42 - LCD Memory 34/35"]
    #[inline(always)]
    pub fn lcdm34w(&self) -> &LCDM34W {
        unsafe { &*(((self as *const Self) as *const u8).add(66usize) as *const LCDM34W) }
    }
    #[doc = "0x42 - LCD Memory 34/35"]
    #[inline(always)]
    pub fn lcdm34w_mut(&self) -> &mut LCDM34W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(66usize) as *mut LCDM34W) }
    }
    #[doc = "0x42 - LCD Blinking Memory 2/3"]
    #[inline(always)]
    pub fn lcdbm2w(&self) -> &LCDBM2W {
        unsafe { &*(((self as *const Self) as *const u8).add(66usize) as *const LCDBM2W) }
    }
    #[doc = "0x42 - LCD Blinking Memory 2/3"]
    #[inline(always)]
    pub fn lcdbm2w_mut(&self) -> &mut LCDBM2W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(66usize) as *mut LCDBM2W) }
    }
    #[doc = "0x44 - LCD Memory 36/37"]
    #[inline(always)]
    pub fn lcdm36w(&self) -> &LCDM36W {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const LCDM36W) }
    }
    #[doc = "0x44 - LCD Memory 36/37"]
    #[inline(always)]
    pub fn lcdm36w_mut(&self) -> &mut LCDM36W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut LCDM36W) }
    }
    #[doc = "0x44 - LCD Blinking Memory 4/5"]
    #[inline(always)]
    pub fn lcdbm4w(&self) -> &LCDBM4W {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const LCDBM4W) }
    }
    #[doc = "0x44 - LCD Blinking Memory 4/5"]
    #[inline(always)]
    pub fn lcdbm4w_mut(&self) -> &mut LCDBM4W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut LCDBM4W) }
    }
    #[doc = "0x46 - LCD Memory 38/39"]
    #[inline(always)]
    pub fn lcdm38w(&self) -> &LCDM38W {
        unsafe { &*(((self as *const Self) as *const u8).add(70usize) as *const LCDM38W) }
    }
    #[doc = "0x46 - LCD Memory 38/39"]
    #[inline(always)]
    pub fn lcdm38w_mut(&self) -> &mut LCDM38W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(70usize) as *mut LCDM38W) }
    }
    #[doc = "0x46 - LCD Blinking Memory 6/7"]
    #[inline(always)]
    pub fn lcdbm6w(&self) -> &LCDBM6W {
        unsafe { &*(((self as *const Self) as *const u8).add(70usize) as *const LCDBM6W) }
    }
    #[doc = "0x46 - LCD Blinking Memory 6/7"]
    #[inline(always)]
    pub fn lcdbm6w_mut(&self) -> &mut LCDBM6W {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(70usize) as *mut LCDBM6W) }
    }
}
#[doc = "LCD_E Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdctl0](lcdctl0) module"]
pub type LCDCTL0 = crate::Reg<u16, _LCDCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCTL0;
#[doc = "`read()` method returns [lcdctl0::R](lcdctl0::R) reader structure"]
impl crate::Readable for LCDCTL0 {}
#[doc = "`write(|w| ..)` method takes [lcdctl0::W](lcdctl0::W) writer structure"]
impl crate::Writable for LCDCTL0 {}
#[doc = "LCD_E Control Register 0"]
pub mod lcdctl0;
#[doc = "LCD_E Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdctl1](lcdctl1) module"]
pub type LCDCTL1 = crate::Reg<u16, _LCDCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCTL1;
#[doc = "`read()` method returns [lcdctl1::R](lcdctl1::R) reader structure"]
impl crate::Readable for LCDCTL1 {}
#[doc = "`write(|w| ..)` method takes [lcdctl1::W](lcdctl1::W) writer structure"]
impl crate::Writable for LCDCTL1 {}
#[doc = "LCD_E Control Register 1"]
pub mod lcdctl1;
#[doc = "LCD_E blinking control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdblkctl](lcdblkctl) module"]
pub type LCDBLKCTL = crate::Reg<u16, _LCDBLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBLKCTL;
#[doc = "`read()` method returns [lcdblkctl::R](lcdblkctl::R) reader structure"]
impl crate::Readable for LCDBLKCTL {}
#[doc = "`write(|w| ..)` method takes [lcdblkctl::W](lcdblkctl::W) writer structure"]
impl crate::Writable for LCDBLKCTL {}
#[doc = "LCD_E blinking control register"]
pub mod lcdblkctl;
#[doc = "LCD_E memory control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdmemctl](lcdmemctl) module"]
pub type LCDMEMCTL = crate::Reg<u16, _LCDMEMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDMEMCTL;
#[doc = "`read()` method returns [lcdmemctl::R](lcdmemctl::R) reader structure"]
impl crate::Readable for LCDMEMCTL {}
#[doc = "`write(|w| ..)` method takes [lcdmemctl::W](lcdmemctl::W) writer structure"]
impl crate::Writable for LCDMEMCTL {}
#[doc = "LCD_E memory control register"]
pub mod lcdmemctl;
#[doc = "LCD_E Voltage Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdvctl](lcdvctl) module"]
pub type LCDVCTL = crate::Reg<u16, _LCDVCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDVCTL;
#[doc = "`read()` method returns [lcdvctl::R](lcdvctl::R) reader structure"]
impl crate::Readable for LCDVCTL {}
#[doc = "`write(|w| ..)` method takes [lcdvctl::W](lcdvctl::W) writer structure"]
impl crate::Writable for LCDVCTL {}
#[doc = "LCD_E Voltage Control Register"]
pub mod lcdvctl;
#[doc = "LCD_E Port Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdpctl0](lcdpctl0) module"]
pub type LCDPCTL0 = crate::Reg<u16, _LCDPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDPCTL0;
#[doc = "`read()` method returns [lcdpctl0::R](lcdpctl0::R) reader structure"]
impl crate::Readable for LCDPCTL0 {}
#[doc = "`write(|w| ..)` method takes [lcdpctl0::W](lcdpctl0::W) writer structure"]
impl crate::Writable for LCDPCTL0 {}
#[doc = "LCD_E Port Control Register 0"]
pub mod lcdpctl0;
#[doc = "LCD_E Port Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdpctl1](lcdpctl1) module"]
pub type LCDPCTL1 = crate::Reg<u16, _LCDPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDPCTL1;
#[doc = "`read()` method returns [lcdpctl1::R](lcdpctl1::R) reader structure"]
impl crate::Readable for LCDPCTL1 {}
#[doc = "`write(|w| ..)` method takes [lcdpctl1::W](lcdpctl1::W) writer structure"]
impl crate::Writable for LCDPCTL1 {}
#[doc = "LCD_E Port Control Register 1"]
pub mod lcdpctl1;
#[doc = "LCD_E Port Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdpctl2](lcdpctl2) module"]
pub type LCDPCTL2 = crate::Reg<u16, _LCDPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDPCTL2;
#[doc = "`read()` method returns [lcdpctl2::R](lcdpctl2::R) reader structure"]
impl crate::Readable for LCDPCTL2 {}
#[doc = "`write(|w| ..)` method takes [lcdpctl2::W](lcdpctl2::W) writer structure"]
impl crate::Writable for LCDPCTL2 {}
#[doc = "LCD_E Port Control Register 2"]
pub mod lcdpctl2;
#[doc = "LCD_E COM/SEG select register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcssel0](lcdcssel0) module"]
pub type LCDCSSEL0 = crate::Reg<u16, _LCDCSSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCSSEL0;
#[doc = "`read()` method returns [lcdcssel0::R](lcdcssel0::R) reader structure"]
impl crate::Readable for LCDCSSEL0 {}
#[doc = "`write(|w| ..)` method takes [lcdcssel0::W](lcdcssel0::W) writer structure"]
impl crate::Writable for LCDCSSEL0 {}
#[doc = "LCD_E COM/SEG select register 0"]
pub mod lcdcssel0;
#[doc = "LCD_E COM/SEG select register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcssel1](lcdcssel1) module"]
pub type LCDCSSEL1 = crate::Reg<u16, _LCDCSSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCSSEL1;
#[doc = "`read()` method returns [lcdcssel1::R](lcdcssel1::R) reader structure"]
impl crate::Readable for LCDCSSEL1 {}
#[doc = "`write(|w| ..)` method takes [lcdcssel1::W](lcdcssel1::W) writer structure"]
impl crate::Writable for LCDCSSEL1 {}
#[doc = "LCD_E COM/SEG select register 1"]
pub mod lcdcssel1;
#[doc = "LCD_E COM/SEG select register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcssel2](lcdcssel2) module"]
pub type LCDCSSEL2 = crate::Reg<u16, _LCDCSSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCSSEL2;
#[doc = "`read()` method returns [lcdcssel2::R](lcdcssel2::R) reader structure"]
impl crate::Readable for LCDCSSEL2 {}
#[doc = "`write(|w| ..)` method takes [lcdcssel2::W](lcdcssel2::W) writer structure"]
impl crate::Writable for LCDCSSEL2 {}
#[doc = "LCD_E COM/SEG select register 2"]
pub mod lcdcssel2;
#[doc = "LCD_E Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdiv](lcdiv) module"]
pub type LCDIV = crate::Reg<u16, _LCDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDIV;
#[doc = "`read()` method returns [lcdiv::R](lcdiv::R) reader structure"]
impl crate::Readable for LCDIV {}
#[doc = "`write(|w| ..)` method takes [lcdiv::W](lcdiv::W) writer structure"]
impl crate::Writable for LCDIV {}
#[doc = "LCD_E Interrupt Vector Register"]
pub mod lcdiv;
#[doc = "LCD Memory 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm0w](lcdm0w) module"]
pub type LCDM0W = crate::Reg<u16, _LCDM0W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM0W;
#[doc = "`read()` method returns [lcdm0w::R](lcdm0w::R) reader structure"]
impl crate::Readable for LCDM0W {}
#[doc = "`write(|w| ..)` method takes [lcdm0w::W](lcdm0w::W) writer structure"]
impl crate::Writable for LCDM0W {}
#[doc = "LCD Memory 0/1"]
pub mod lcdm0w;
#[doc = "LCD Memory 2/3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm2w](lcdm2w) module"]
pub type LCDM2W = crate::Reg<u16, _LCDM2W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM2W;
#[doc = "`read()` method returns [lcdm2w::R](lcdm2w::R) reader structure"]
impl crate::Readable for LCDM2W {}
#[doc = "`write(|w| ..)` method takes [lcdm2w::W](lcdm2w::W) writer structure"]
impl crate::Writable for LCDM2W {}
#[doc = "LCD Memory 2/3"]
pub mod lcdm2w;
#[doc = "LCD Memory 4/5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm4w](lcdm4w) module"]
pub type LCDM4W = crate::Reg<u16, _LCDM4W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM4W;
#[doc = "`read()` method returns [lcdm4w::R](lcdm4w::R) reader structure"]
impl crate::Readable for LCDM4W {}
#[doc = "`write(|w| ..)` method takes [lcdm4w::W](lcdm4w::W) writer structure"]
impl crate::Writable for LCDM4W {}
#[doc = "LCD Memory 4/5"]
pub mod lcdm4w;
#[doc = "LCD Memory 6/7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm6w](lcdm6w) module"]
pub type LCDM6W = crate::Reg<u16, _LCDM6W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM6W;
#[doc = "`read()` method returns [lcdm6w::R](lcdm6w::R) reader structure"]
impl crate::Readable for LCDM6W {}
#[doc = "`write(|w| ..)` method takes [lcdm6w::W](lcdm6w::W) writer structure"]
impl crate::Writable for LCDM6W {}
#[doc = "LCD Memory 6/7"]
pub mod lcdm6w;
#[doc = "LCD Memory 8/9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm8w](lcdm8w) module"]
pub type LCDM8W = crate::Reg<u16, _LCDM8W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM8W;
#[doc = "`read()` method returns [lcdm8w::R](lcdm8w::R) reader structure"]
impl crate::Readable for LCDM8W {}
#[doc = "`write(|w| ..)` method takes [lcdm8w::W](lcdm8w::W) writer structure"]
impl crate::Writable for LCDM8W {}
#[doc = "LCD Memory 8/9"]
pub mod lcdm8w;
#[doc = "LCD Memory 10/11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm10w](lcdm10w) module"]
pub type LCDM10W = crate::Reg<u16, _LCDM10W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM10W;
#[doc = "`read()` method returns [lcdm10w::R](lcdm10w::R) reader structure"]
impl crate::Readable for LCDM10W {}
#[doc = "`write(|w| ..)` method takes [lcdm10w::W](lcdm10w::W) writer structure"]
impl crate::Writable for LCDM10W {}
#[doc = "LCD Memory 10/11"]
pub mod lcdm10w;
#[doc = "LCD Memory 12/13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm12w](lcdm12w) module"]
pub type LCDM12W = crate::Reg<u16, _LCDM12W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM12W;
#[doc = "`read()` method returns [lcdm12w::R](lcdm12w::R) reader structure"]
impl crate::Readable for LCDM12W {}
#[doc = "`write(|w| ..)` method takes [lcdm12w::W](lcdm12w::W) writer structure"]
impl crate::Writable for LCDM12W {}
#[doc = "LCD Memory 12/13"]
pub mod lcdm12w;
#[doc = "LCD Memory 14/15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm14w](lcdm14w) module"]
pub type LCDM14W = crate::Reg<u16, _LCDM14W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM14W;
#[doc = "`read()` method returns [lcdm14w::R](lcdm14w::R) reader structure"]
impl crate::Readable for LCDM14W {}
#[doc = "`write(|w| ..)` method takes [lcdm14w::W](lcdm14w::W) writer structure"]
impl crate::Writable for LCDM14W {}
#[doc = "LCD Memory 14/15"]
pub mod lcdm14w;
#[doc = "LCD Memory 16/17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm16w](lcdm16w) module"]
pub type LCDM16W = crate::Reg<u16, _LCDM16W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM16W;
#[doc = "`read()` method returns [lcdm16w::R](lcdm16w::R) reader structure"]
impl crate::Readable for LCDM16W {}
#[doc = "`write(|w| ..)` method takes [lcdm16w::W](lcdm16w::W) writer structure"]
impl crate::Writable for LCDM16W {}
#[doc = "LCD Memory 16/17"]
pub mod lcdm16w;
#[doc = "LCD Memory 18/19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm18w](lcdm18w) module"]
pub type LCDM18W = crate::Reg<u16, _LCDM18W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM18W;
#[doc = "`read()` method returns [lcdm18w::R](lcdm18w::R) reader structure"]
impl crate::Readable for LCDM18W {}
#[doc = "`write(|w| ..)` method takes [lcdm18w::W](lcdm18w::W) writer structure"]
impl crate::Writable for LCDM18W {}
#[doc = "LCD Memory 18/19"]
pub mod lcdm18w;
#[doc = "LCD Memory 20/21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm20w](lcdm20w) module"]
pub type LCDM20W = crate::Reg<u16, _LCDM20W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM20W;
#[doc = "`read()` method returns [lcdm20w::R](lcdm20w::R) reader structure"]
impl crate::Readable for LCDM20W {}
#[doc = "`write(|w| ..)` method takes [lcdm20w::W](lcdm20w::W) writer structure"]
impl crate::Writable for LCDM20W {}
#[doc = "LCD Memory 20/21"]
pub mod lcdm20w;
#[doc = "LCD Memory 22/23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm22w](lcdm22w) module"]
pub type LCDM22W = crate::Reg<u16, _LCDM22W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM22W;
#[doc = "`read()` method returns [lcdm22w::R](lcdm22w::R) reader structure"]
impl crate::Readable for LCDM22W {}
#[doc = "`write(|w| ..)` method takes [lcdm22w::W](lcdm22w::W) writer structure"]
impl crate::Writable for LCDM22W {}
#[doc = "LCD Memory 22/23"]
pub mod lcdm22w;
#[doc = "LCD Memory 24/25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm24w](lcdm24w) module"]
pub type LCDM24W = crate::Reg<u16, _LCDM24W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM24W;
#[doc = "`read()` method returns [lcdm24w::R](lcdm24w::R) reader structure"]
impl crate::Readable for LCDM24W {}
#[doc = "`write(|w| ..)` method takes [lcdm24w::W](lcdm24w::W) writer structure"]
impl crate::Writable for LCDM24W {}
#[doc = "LCD Memory 24/25"]
pub mod lcdm24w;
#[doc = "LCD Memory 26/27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm26w](lcdm26w) module"]
pub type LCDM26W = crate::Reg<u16, _LCDM26W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM26W;
#[doc = "`read()` method returns [lcdm26w::R](lcdm26w::R) reader structure"]
impl crate::Readable for LCDM26W {}
#[doc = "`write(|w| ..)` method takes [lcdm26w::W](lcdm26w::W) writer structure"]
impl crate::Writable for LCDM26W {}
#[doc = "LCD Memory 26/27"]
pub mod lcdm26w;
#[doc = "LCD Memory 28/29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm28w](lcdm28w) module"]
pub type LCDM28W = crate::Reg<u16, _LCDM28W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM28W;
#[doc = "`read()` method returns [lcdm28w::R](lcdm28w::R) reader structure"]
impl crate::Readable for LCDM28W {}
#[doc = "`write(|w| ..)` method takes [lcdm28w::W](lcdm28w::W) writer structure"]
impl crate::Writable for LCDM28W {}
#[doc = "LCD Memory 28/29"]
pub mod lcdm28w;
#[doc = "LCD Memory 30/31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm30w](lcdm30w) module"]
pub type LCDM30W = crate::Reg<u16, _LCDM30W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM30W;
#[doc = "`read()` method returns [lcdm30w::R](lcdm30w::R) reader structure"]
impl crate::Readable for LCDM30W {}
#[doc = "`write(|w| ..)` method takes [lcdm30w::W](lcdm30w::W) writer structure"]
impl crate::Writable for LCDM30W {}
#[doc = "LCD Memory 30/31"]
pub mod lcdm30w;
#[doc = "LCD Blinking Memory 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm0w](lcdbm0w) module"]
pub type LCDBM0W = crate::Reg<u16, _LCDBM0W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM0W;
#[doc = "`read()` method returns [lcdbm0w::R](lcdbm0w::R) reader structure"]
impl crate::Readable for LCDBM0W {}
#[doc = "`write(|w| ..)` method takes [lcdbm0w::W](lcdbm0w::W) writer structure"]
impl crate::Writable for LCDBM0W {}
#[doc = "LCD Blinking Memory 0/1"]
pub mod lcdbm0w;
#[doc = "LCD Memory 32/33\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm32w](lcdm32w) module"]
pub type LCDM32W = crate::Reg<u16, _LCDM32W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM32W;
#[doc = "`read()` method returns [lcdm32w::R](lcdm32w::R) reader structure"]
impl crate::Readable for LCDM32W {}
#[doc = "`write(|w| ..)` method takes [lcdm32w::W](lcdm32w::W) writer structure"]
impl crate::Writable for LCDM32W {}
#[doc = "LCD Memory 32/33"]
pub mod lcdm32w;
#[doc = "LCD Blinking Memory 2/3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm2w](lcdbm2w) module"]
pub type LCDBM2W = crate::Reg<u16, _LCDBM2W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM2W;
#[doc = "`read()` method returns [lcdbm2w::R](lcdbm2w::R) reader structure"]
impl crate::Readable for LCDBM2W {}
#[doc = "`write(|w| ..)` method takes [lcdbm2w::W](lcdbm2w::W) writer structure"]
impl crate::Writable for LCDBM2W {}
#[doc = "LCD Blinking Memory 2/3"]
pub mod lcdbm2w;
#[doc = "LCD Memory 34/35\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm34w](lcdm34w) module"]
pub type LCDM34W = crate::Reg<u16, _LCDM34W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM34W;
#[doc = "`read()` method returns [lcdm34w::R](lcdm34w::R) reader structure"]
impl crate::Readable for LCDM34W {}
#[doc = "`write(|w| ..)` method takes [lcdm34w::W](lcdm34w::W) writer structure"]
impl crate::Writable for LCDM34W {}
#[doc = "LCD Memory 34/35"]
pub mod lcdm34w;
#[doc = "LCD Blinking Memory 4/5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm4w](lcdbm4w) module"]
pub type LCDBM4W = crate::Reg<u16, _LCDBM4W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM4W;
#[doc = "`read()` method returns [lcdbm4w::R](lcdbm4w::R) reader structure"]
impl crate::Readable for LCDBM4W {}
#[doc = "`write(|w| ..)` method takes [lcdbm4w::W](lcdbm4w::W) writer structure"]
impl crate::Writable for LCDBM4W {}
#[doc = "LCD Blinking Memory 4/5"]
pub mod lcdbm4w;
#[doc = "LCD Memory 36/37\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm36w](lcdm36w) module"]
pub type LCDM36W = crate::Reg<u16, _LCDM36W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM36W;
#[doc = "`read()` method returns [lcdm36w::R](lcdm36w::R) reader structure"]
impl crate::Readable for LCDM36W {}
#[doc = "`write(|w| ..)` method takes [lcdm36w::W](lcdm36w::W) writer structure"]
impl crate::Writable for LCDM36W {}
#[doc = "LCD Memory 36/37"]
pub mod lcdm36w;
#[doc = "LCD Blinking Memory 6/7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm6w](lcdbm6w) module"]
pub type LCDBM6W = crate::Reg<u16, _LCDBM6W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM6W;
#[doc = "`read()` method returns [lcdbm6w::R](lcdbm6w::R) reader structure"]
impl crate::Readable for LCDBM6W {}
#[doc = "`write(|w| ..)` method takes [lcdbm6w::W](lcdbm6w::W) writer structure"]
impl crate::Writable for LCDBM6W {}
#[doc = "LCD Blinking Memory 6/7"]
pub mod lcdbm6w;
#[doc = "LCD Memory 38/39\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm38w](lcdm38w) module"]
pub type LCDM38W = crate::Reg<u16, _LCDM38W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM38W;
#[doc = "`read()` method returns [lcdm38w::R](lcdm38w::R) reader structure"]
impl crate::Readable for LCDM38W {}
#[doc = "`write(|w| ..)` method takes [lcdm38w::W](lcdm38w::W) writer structure"]
impl crate::Writable for LCDM38W {}
#[doc = "LCD Memory 38/39"]
pub mod lcdm38w;
#[doc = "LCD Blinking Memory 8/9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm8w](lcdbm8w) module"]
pub type LCDBM8W = crate::Reg<u16, _LCDBM8W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM8W;
#[doc = "`read()` method returns [lcdbm8w::R](lcdbm8w::R) reader structure"]
impl crate::Readable for LCDBM8W {}
#[doc = "`write(|w| ..)` method takes [lcdbm8w::W](lcdbm8w::W) writer structure"]
impl crate::Writable for LCDBM8W {}
#[doc = "LCD Blinking Memory 8/9"]
pub mod lcdbm8w;
#[doc = "LCD Blinking Memory 10/11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm10w](lcdbm10w) module"]
pub type LCDBM10W = crate::Reg<u16, _LCDBM10W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM10W;
#[doc = "`read()` method returns [lcdbm10w::R](lcdbm10w::R) reader structure"]
impl crate::Readable for LCDBM10W {}
#[doc = "`write(|w| ..)` method takes [lcdbm10w::W](lcdbm10w::W) writer structure"]
impl crate::Writable for LCDBM10W {}
#[doc = "LCD Blinking Memory 10/11"]
pub mod lcdbm10w;
#[doc = "LCD Blinking Memory 12/13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm12w](lcdbm12w) module"]
pub type LCDBM12W = crate::Reg<u16, _LCDBM12W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM12W;
#[doc = "`read()` method returns [lcdbm12w::R](lcdbm12w::R) reader structure"]
impl crate::Readable for LCDBM12W {}
#[doc = "`write(|w| ..)` method takes [lcdbm12w::W](lcdbm12w::W) writer structure"]
impl crate::Writable for LCDBM12W {}
#[doc = "LCD Blinking Memory 12/13"]
pub mod lcdbm12w;
#[doc = "LCD Blinking Memory 14/15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm14w](lcdbm14w) module"]
pub type LCDBM14W = crate::Reg<u16, _LCDBM14W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM14W;
#[doc = "`read()` method returns [lcdbm14w::R](lcdbm14w::R) reader structure"]
impl crate::Readable for LCDBM14W {}
#[doc = "`write(|w| ..)` method takes [lcdbm14w::W](lcdbm14w::W) writer structure"]
impl crate::Writable for LCDBM14W {}
#[doc = "LCD Blinking Memory 14/15"]
pub mod lcdbm14w;
#[doc = "LCD Blinking Memory 16/17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm16w](lcdbm16w) module"]
pub type LCDBM16W = crate::Reg<u16, _LCDBM16W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM16W;
#[doc = "`read()` method returns [lcdbm16w::R](lcdbm16w::R) reader structure"]
impl crate::Readable for LCDBM16W {}
#[doc = "`write(|w| ..)` method takes [lcdbm16w::W](lcdbm16w::W) writer structure"]
impl crate::Writable for LCDBM16W {}
#[doc = "LCD Blinking Memory 16/17"]
pub mod lcdbm16w;
#[doc = "LCD Blinking Memory 18/19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm18w](lcdbm18w) module"]
pub type LCDBM18W = crate::Reg<u16, _LCDBM18W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM18W;
#[doc = "`read()` method returns [lcdbm18w::R](lcdbm18w::R) reader structure"]
impl crate::Readable for LCDBM18W {}
#[doc = "`write(|w| ..)` method takes [lcdbm18w::W](lcdbm18w::W) writer structure"]
impl crate::Writable for LCDBM18W {}
#[doc = "LCD Blinking Memory 18/19"]
pub mod lcdbm18w;
