#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    pub ucb0ctl1: UCB0CTL1,
    #[doc = "0x01 - USCI B0 Control Register 0"]
    pub ucb0ctl0: UCB0CTL0,
    #[doc = "0x02 - USCI B0 Control Word Register 1"]
    pub ucb0ctlw1: UCB0CTLW1,
    _reserved3: [u8; 2usize],
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    pub ucb0br0: UCB0BR0,
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    pub ucb0br1: UCB0BR1,
    #[doc = "0x08 - USCI B0 Status Register"]
    pub ucb0stat_i2c: UCB0STAT_I2C,
    #[doc = "0x09 - USCI B0 Byte Counter Register"]
    pub ucb0bcnt_i2c: UCB0BCNT_I2C,
    #[doc = "0x0a - USCI B0 Byte Counter Threshold Register"]
    pub ucb0tbcnt: UCB0TBCNT,
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    pub ucb0rxbuf: UCB0RXBUF,
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    pub ucb0txbuf: UCB0TXBUF,
    _reserved10: [u8; 4usize],
    #[doc = "0x14 - USCI B0 I2C Own Address 0"]
    pub ucb0i2coa0: UCB0I2COA0,
    #[doc = "0x16 - USCI B0 I2C Own Address 1"]
    pub ucb0i2coa1: UCB0I2COA1,
    #[doc = "0x18 - USCI B0 I2C Own Address 2"]
    pub ucb0i2coa2: UCB0I2COA2,
    #[doc = "0x1a - USCI B0 I2C Own Address 3"]
    pub ucb0i2coa3: UCB0I2COA3,
    #[doc = "0x1c - USCI B0 Received Address Register"]
    pub ucb0addrx: UCB0ADDRX,
    #[doc = "0x1e - USCI B0 Address Mask Register"]
    pub ucb0addmask: UCB0ADDMASK,
    #[doc = "0x20 - USCI B0 I2C Slave Address"]
    pub ucb0i2csa: UCB0I2CSA,
    _reserved17: [u8; 8usize],
    _reserved_17_ucb0: [u8; 2usize],
    _reserved_18_ucb0: [u8; 2usize],
    #[doc = "0x2e - USCI B0 Interrupt Vector Register"]
    pub ucb0iv: UCB0IV,
}
impl RegisterBlock {
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie_i2c(&self) -> &UCB0IE_I2C {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB0IE_I2C) }
    }
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie_i2c_mut(&self) -> &mut UCB0IE_I2C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB0IE_I2C) }
    }
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie(&self) -> &UCB0IE {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB0IE) }
    }
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie_mut(&self) -> &mut UCB0IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB0IE) }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb0ifg_i2c(&self) -> &UCB0IFG_I2C {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB0IFG_I2C) }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb0ifg_i2c_mut(&self) -> &mut UCB0IFG_I2C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB0IFG_I2C) }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb0ifg(&self) -> &UCB0IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB0IFG) }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb0ifg_mut(&self) -> &mut UCB0IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB0IFG) }
    }
}
#[doc = "USCI B0 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctl1](ucb0ctl1) module"]
pub type UCB0CTL1 = crate::Reg<u8, _UCB0CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTL1;
#[doc = "`read()` method returns [ucb0ctl1::R](ucb0ctl1::R) reader structure"]
impl crate::Readable for UCB0CTL1 {}
#[doc = "`write(|w| ..)` method takes [ucb0ctl1::W](ucb0ctl1::W) writer structure"]
impl crate::Writable for UCB0CTL1 {}
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "USCI B0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctl0](ucb0ctl0) module"]
pub type UCB0CTL0 = crate::Reg<u8, _UCB0CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTL0;
#[doc = "`read()` method returns [ucb0ctl0::R](ucb0ctl0::R) reader structure"]
impl crate::Readable for UCB0CTL0 {}
#[doc = "`write(|w| ..)` method takes [ucb0ctl0::W](ucb0ctl0::W) writer structure"]
impl crate::Writable for UCB0CTL0 {}
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "USCI B0 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0br0](ucb0br0) module"]
pub type UCB0BR0 = crate::Reg<u8, _UCB0BR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0BR0;
#[doc = "`read()` method returns [ucb0br0::R](ucb0br0::R) reader structure"]
impl crate::Readable for UCB0BR0 {}
#[doc = "`write(|w| ..)` method takes [ucb0br0::W](ucb0br0::W) writer structure"]
impl crate::Writable for UCB0BR0 {}
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "USCI B0 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0br1](ucb0br1) module"]
pub type UCB0BR1 = crate::Reg<u8, _UCB0BR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0BR1;
#[doc = "`read()` method returns [ucb0br1::R](ucb0br1::R) reader structure"]
impl crate::Readable for UCB0BR1 {}
#[doc = "`write(|w| ..)` method takes [ucb0br1::W](ucb0br1::W) writer structure"]
impl crate::Writable for UCB0BR1 {}
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "USCI B0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0stat_i2c](ucb0stat_i2c) module"]
pub type UCB0STAT_I2C = crate::Reg<u8, _UCB0STAT_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0STAT_I2C;
#[doc = "`read()` method returns [ucb0stat_i2c::R](ucb0stat_i2c::R) reader structure"]
impl crate::Readable for UCB0STAT_I2C {}
#[doc = "`write(|w| ..)` method takes [ucb0stat_i2c::W](ucb0stat_i2c::W) writer structure"]
impl crate::Writable for UCB0STAT_I2C {}
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat_i2c;
#[doc = "USCI B0 Byte Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0bcnt_i2c](ucb0bcnt_i2c) module"]
pub type UCB0BCNT_I2C = crate::Reg<u8, _UCB0BCNT_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0BCNT_I2C;
#[doc = "`read()` method returns [ucb0bcnt_i2c::R](ucb0bcnt_i2c::R) reader structure"]
impl crate::Readable for UCB0BCNT_I2C {}
#[doc = "`write(|w| ..)` method takes [ucb0bcnt_i2c::W](ucb0bcnt_i2c::W) writer structure"]
impl crate::Writable for UCB0BCNT_I2C {}
#[doc = "USCI B0 Byte Counter Register"]
pub mod ucb0bcnt_i2c;
#[doc = "USCI B0 Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctlw1](ucb0ctlw1) module"]
pub type UCB0CTLW1 = crate::Reg<u16, _UCB0CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTLW1;
#[doc = "`read()` method returns [ucb0ctlw1::R](ucb0ctlw1::R) reader structure"]
impl crate::Readable for UCB0CTLW1 {}
#[doc = "`write(|w| ..)` method takes [ucb0ctlw1::W](ucb0ctlw1::W) writer structure"]
impl crate::Writable for UCB0CTLW1 {}
#[doc = "USCI B0 Control Word Register 1"]
pub mod ucb0ctlw1;
#[doc = "USCI B0 Byte Counter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0tbcnt](ucb0tbcnt) module"]
pub type UCB0TBCNT = crate::Reg<u16, _UCB0TBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0TBCNT;
#[doc = "`read()` method returns [ucb0tbcnt::R](ucb0tbcnt::R) reader structure"]
impl crate::Readable for UCB0TBCNT {}
#[doc = "`write(|w| ..)` method takes [ucb0tbcnt::W](ucb0tbcnt::W) writer structure"]
impl crate::Writable for UCB0TBCNT {}
#[doc = "USCI B0 Byte Counter Threshold Register"]
pub mod ucb0tbcnt;
#[doc = "USCI B0 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0rxbuf](ucb0rxbuf) module"]
pub type UCB0RXBUF = crate::Reg<u16, _UCB0RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0RXBUF;
#[doc = "`read()` method returns [ucb0rxbuf::R](ucb0rxbuf::R) reader structure"]
impl crate::Readable for UCB0RXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb0rxbuf::W](ucb0rxbuf::W) writer structure"]
impl crate::Writable for UCB0RXBUF {}
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "USCI B0 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0txbuf](ucb0txbuf) module"]
pub type UCB0TXBUF = crate::Reg<u16, _UCB0TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0TXBUF;
#[doc = "`read()` method returns [ucb0txbuf::R](ucb0txbuf::R) reader structure"]
impl crate::Readable for UCB0TXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb0txbuf::W](ucb0txbuf::W) writer structure"]
impl crate::Writable for UCB0TXBUF {}
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "USCI B0 I2C Own Address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2coa0](ucb0i2coa0) module"]
pub type UCB0I2COA0 = crate::Reg<u16, _UCB0I2COA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA0;
#[doc = "`read()` method returns [ucb0i2coa0::R](ucb0i2coa0::R) reader structure"]
impl crate::Readable for UCB0I2COA0 {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa0::W](ucb0i2coa0::W) writer structure"]
impl crate::Writable for UCB0I2COA0 {}
#[doc = "USCI B0 I2C Own Address 0"]
pub mod ucb0i2coa0;
#[doc = "USCI B0 I2C Own Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2coa1](ucb0i2coa1) module"]
pub type UCB0I2COA1 = crate::Reg<u16, _UCB0I2COA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA1;
#[doc = "`read()` method returns [ucb0i2coa1::R](ucb0i2coa1::R) reader structure"]
impl crate::Readable for UCB0I2COA1 {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa1::W](ucb0i2coa1::W) writer structure"]
impl crate::Writable for UCB0I2COA1 {}
#[doc = "USCI B0 I2C Own Address 1"]
pub mod ucb0i2coa1;
#[doc = "USCI B0 I2C Own Address 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2coa2](ucb0i2coa2) module"]
pub type UCB0I2COA2 = crate::Reg<u16, _UCB0I2COA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA2;
#[doc = "`read()` method returns [ucb0i2coa2::R](ucb0i2coa2::R) reader structure"]
impl crate::Readable for UCB0I2COA2 {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa2::W](ucb0i2coa2::W) writer structure"]
impl crate::Writable for UCB0I2COA2 {}
#[doc = "USCI B0 I2C Own Address 2"]
pub mod ucb0i2coa2;
#[doc = "USCI B0 I2C Own Address 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2coa3](ucb0i2coa3) module"]
pub type UCB0I2COA3 = crate::Reg<u16, _UCB0I2COA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA3;
#[doc = "`read()` method returns [ucb0i2coa3::R](ucb0i2coa3::R) reader structure"]
impl crate::Readable for UCB0I2COA3 {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa3::W](ucb0i2coa3::W) writer structure"]
impl crate::Writable for UCB0I2COA3 {}
#[doc = "USCI B0 I2C Own Address 3"]
pub mod ucb0i2coa3;
#[doc = "USCI B0 Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0addrx](ucb0addrx) module"]
pub type UCB0ADDRX = crate::Reg<u16, _UCB0ADDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0ADDRX;
#[doc = "`read()` method returns [ucb0addrx::R](ucb0addrx::R) reader structure"]
impl crate::Readable for UCB0ADDRX {}
#[doc = "`write(|w| ..)` method takes [ucb0addrx::W](ucb0addrx::W) writer structure"]
impl crate::Writable for UCB0ADDRX {}
#[doc = "USCI B0 Received Address Register"]
pub mod ucb0addrx;
#[doc = "USCI B0 Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0addmask](ucb0addmask) module"]
pub type UCB0ADDMASK = crate::Reg<u16, _UCB0ADDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0ADDMASK;
#[doc = "`read()` method returns [ucb0addmask::R](ucb0addmask::R) reader structure"]
impl crate::Readable for UCB0ADDMASK {}
#[doc = "`write(|w| ..)` method takes [ucb0addmask::W](ucb0addmask::W) writer structure"]
impl crate::Writable for UCB0ADDMASK {}
#[doc = "USCI B0 Address Mask Register"]
pub mod ucb0addmask;
#[doc = "USCI B0 I2C Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2csa](ucb0i2csa) module"]
pub type UCB0I2CSA = crate::Reg<u16, _UCB0I2CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2CSA;
#[doc = "`read()` method returns [ucb0i2csa::R](ucb0i2csa::R) reader structure"]
impl crate::Readable for UCB0I2CSA {}
#[doc = "`write(|w| ..)` method takes [ucb0i2csa::W](ucb0i2csa::W) writer structure"]
impl crate::Writable for UCB0I2CSA {}
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
#[doc = "USCI B0 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ie](ucb0ie) module"]
pub type UCB0IE = crate::Reg<u16, _UCB0IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IE;
#[doc = "`read()` method returns [ucb0ie::R](ucb0ie::R) reader structure"]
impl crate::Readable for UCB0IE {}
#[doc = "`write(|w| ..)` method takes [ucb0ie::W](ucb0ie::W) writer structure"]
impl crate::Writable for UCB0IE {}
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "USCI B0 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ie_i2c](ucb0ie_i2c) module"]
pub type UCB0IE_I2C = crate::Reg<u16, _UCB0IE_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IE_I2C;
#[doc = "`read()` method returns [ucb0ie_i2c::R](ucb0ie_i2c::R) reader structure"]
impl crate::Readable for UCB0IE_I2C {}
#[doc = "`write(|w| ..)` method takes [ucb0ie_i2c::W](ucb0ie_i2c::W) writer structure"]
impl crate::Writable for UCB0IE_I2C {}
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie_i2c;
#[doc = "USCI B0 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ifg](ucb0ifg) module"]
pub type UCB0IFG = crate::Reg<u16, _UCB0IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IFG;
#[doc = "`read()` method returns [ucb0ifg::R](ucb0ifg::R) reader structure"]
impl crate::Readable for UCB0IFG {}
#[doc = "`write(|w| ..)` method takes [ucb0ifg::W](ucb0ifg::W) writer structure"]
impl crate::Writable for UCB0IFG {}
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg;
#[doc = "USCI B0 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ifg_i2c](ucb0ifg_i2c) module"]
pub type UCB0IFG_I2C = crate::Reg<u16, _UCB0IFG_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IFG_I2C;
#[doc = "`read()` method returns [ucb0ifg_i2c::R](ucb0ifg_i2c::R) reader structure"]
impl crate::Readable for UCB0IFG_I2C {}
#[doc = "`write(|w| ..)` method takes [ucb0ifg_i2c::W](ucb0ifg_i2c::W) writer structure"]
impl crate::Writable for UCB0IFG_I2C {}
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg_i2c;
#[doc = "USCI B0 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0iv](ucb0iv) module"]
pub type UCB0IV = crate::Reg<u16, _UCB0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IV;
#[doc = "`read()` method returns [ucb0iv::R](ucb0iv::R) reader structure"]
impl crate::Readable for UCB0IV {}
#[doc = "`write(|w| ..)` method takes [ucb0iv::W](ucb0iv::W) writer structure"]
impl crate::Writable for UCB0IV {}
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv;
