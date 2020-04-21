#![feature(abi_msp430_interrupt)]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for MSP430FR4133 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate msp430;
#[cfg(feature = "rt")]
extern crate msp430_rt;
#[cfg(feature = "rt")]
pub use msp430_rt::default_handler;
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Port 1/2"]
pub struct PORT_1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_1_2 {}
impl PORT_1_2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_1_2::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for PORT_1_2 {
    type Target = port_1_2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_1_2::ptr() }
    }
}
#[doc = "Port 1/2"]
pub mod port_1_2;
#[doc = "Port 3/4"]
pub struct PORT_3_4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_3_4 {}
impl PORT_3_4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_3_4::RegisterBlock {
        0x0220 as *const _
    }
}
impl Deref for PORT_3_4 {
    type Target = port_3_4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_3_4::ptr() }
    }
}
#[doc = "Port 3/4"]
pub mod port_3_4;
#[doc = "Port 5/6"]
pub struct PORT_5_6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_5_6 {}
impl PORT_5_6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_5_6::RegisterBlock {
        0x0240 as *const _
    }
}
impl Deref for PORT_5_6 {
    type Target = port_5_6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_5_6::ptr() }
    }
}
#[doc = "Port 5/6"]
pub mod port_5_6;
#[doc = "Port 7/8"]
pub struct PORT_7_8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_7_8 {}
impl PORT_7_8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_7_8::RegisterBlock {
        0x0260 as *const _
    }
}
impl Deref for PORT_7_8 {
    type Target = port_7_8::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_7_8::ptr() }
    }
}
#[doc = "Port 7/8"]
pub mod port_7_8;
#[doc = "USCI_A0 UART Mode"]
pub struct USCI_A0_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_UART_MODE {}
impl USCI_A0_UART_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_uart_mode::RegisterBlock {
        0x0500 as *const _
    }
}
impl Deref for USCI_A0_UART_MODE {
    type Target = usci_a0_uart_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A0_UART_MODE::ptr() }
    }
}
#[doc = "USCI_A0 UART Mode"]
pub mod usci_a0_uart_mode;
#[doc = "USCI_A0 SPI Mode"]
pub struct USCI_A0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_SPI_MODE {}
impl USCI_A0_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_spi_mode::RegisterBlock {
        0x0500 as *const _
    }
}
impl Deref for USCI_A0_SPI_MODE {
    type Target = usci_a0_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_A0 SPI Mode"]
pub mod usci_a0_spi_mode;
#[doc = "USCI_B0 I2C Mode"]
pub struct USCI_B0_I2C_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_I2C_MODE {}
impl USCI_B0_I2C_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_i2c_mode::RegisterBlock {
        0x0540 as *const _
    }
}
impl Deref for USCI_B0_I2C_MODE {
    type Target = usci_b0_i2c_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B0_I2C_MODE::ptr() }
    }
}
#[doc = "USCI_B0 I2C Mode"]
pub mod usci_b0_i2c_mode;
#[doc = "USCI_B0 SPI Mode"]
pub struct USCI_B0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_SPI_MODE {}
impl USCI_B0_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_spi_mode::RegisterBlock {
        0x0540 as *const _
    }
}
impl Deref for USCI_B0_SPI_MODE {
    type Target = usci_b0_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_B0 SPI Mode"]
pub mod usci_b0_spi_mode;
#[doc = "SFR Special Function Registers"]
pub struct SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SFR {}
impl SFR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sfr::RegisterBlock {
        0x0100 as *const _
    }
}
impl Deref for SFR {
    type Target = sfr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SFR::ptr() }
    }
}
#[doc = "SFR Special Function Registers"]
pub mod sfr;
#[doc = "PMM Power Management System"]
pub struct PMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMM {}
impl PMM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmm::RegisterBlock {
        0x0120 as *const _
    }
}
impl Deref for PMM {
    type Target = pmm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMM::ptr() }
    }
}
#[doc = "PMM Power Management System"]
pub mod pmm;
#[doc = "SYS System Module"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys::RegisterBlock {
        0x0140 as *const _
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYS::ptr() }
    }
}
#[doc = "SYS System Module"]
pub mod sys;
#[doc = "CS Clock System"]
pub struct CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CS {}
impl CS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cs::RegisterBlock {
        0x0180 as *const _
    }
}
impl Deref for CS {
    type Target = cs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CS::ptr() }
    }
}
#[doc = "CS Clock System"]
pub mod cs;
#[doc = "FRAM"]
pub struct FRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FRAM {}
impl FRAM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fram::RegisterBlock {
        0x01a0 as *const _
    }
}
impl Deref for FRAM {
    type Target = fram::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FRAM::ptr() }
    }
}
#[doc = "FRAM"]
pub mod fram;
#[doc = "CRC16"]
pub struct CRC16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC16 {}
impl CRC16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc16::RegisterBlock {
        0x01c0 as *const _
    }
}
impl Deref for CRC16 {
    type Target = crc16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC16::ptr() }
    }
}
#[doc = "CRC16"]
pub mod crc16;
#[doc = "Watchdog Timer"]
pub struct WATCHDOG_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG_TIMER {}
impl WATCHDOG_TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog_timer::RegisterBlock {
        0x01cc as *const _
    }
}
impl Deref for WATCHDOG_TIMER {
    type Target = watchdog_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG_TIMER::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog_timer;
#[doc = "Capacitive_Touch_IO 0"]
pub struct CAPACITIVE_TOUCH_IO_0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAPACITIVE_TOUCH_IO_0 {}
impl CAPACITIVE_TOUCH_IO_0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const capacitive_touch_io_0::RegisterBlock {
        0x02ee as *const _
    }
}
impl Deref for CAPACITIVE_TOUCH_IO_0 {
    type Target = capacitive_touch_io_0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAPACITIVE_TOUCH_IO_0::ptr() }
    }
}
#[doc = "Capacitive_Touch_IO 0"]
pub mod capacitive_touch_io_0;
#[doc = "Timer0_A3"]
pub struct TIMER_0_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_0_A3 {}
impl TIMER_0_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_0_a3::RegisterBlock {
        0x0300 as *const _
    }
}
impl Deref for TIMER_0_A3 {
    type Target = timer_0_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_0_A3::ptr() }
    }
}
#[doc = "Timer0_A3"]
pub mod timer_0_a3;
#[doc = "Timer1_A3"]
pub struct TIMER_1_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_1_A3 {}
impl TIMER_1_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_1_a3::RegisterBlock {
        0x0340 as *const _
    }
}
impl Deref for TIMER_1_A3 {
    type Target = timer_1_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_1_A3::ptr() }
    }
}
#[doc = "Timer1_A3"]
pub mod timer_1_a3;
#[doc = "Real-Time Clock"]
pub struct REAL_TIME_CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REAL_TIME_CLOCK {}
impl REAL_TIME_CLOCK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const real_time_clock::RegisterBlock {
        0x03c0 as *const _
    }
}
impl Deref for REAL_TIME_CLOCK {
    type Target = real_time_clock::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*REAL_TIME_CLOCK::ptr() }
    }
}
#[doc = "Real-Time Clock"]
pub mod real_time_clock;
#[doc = "LCD_E"]
pub struct LCD_E {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD_E {}
impl LCD_E {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lcd_e::RegisterBlock {
        0x0600 as *const _
    }
}
impl Deref for LCD_E {
    type Target = lcd_e::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LCD_E::ptr() }
    }
}
#[doc = "LCD_E"]
pub mod lcd_e;
#[doc = "Backup Memory"]
pub struct BACKUP_MEMORY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BACKUP_MEMORY {}
impl BACKUP_MEMORY {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const backup_memory::RegisterBlock {
        0x0660 as *const _
    }
}
impl Deref for BACKUP_MEMORY {
    type Target = backup_memory::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BACKUP_MEMORY::ptr() }
    }
}
#[doc = "Backup Memory"]
pub mod backup_memory;
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x0700 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PORT_1_2"]
    pub PORT_1_2: PORT_1_2,
    #[doc = "PORT_3_4"]
    pub PORT_3_4: PORT_3_4,
    #[doc = "PORT_5_6"]
    pub PORT_5_6: PORT_5_6,
    #[doc = "PORT_7_8"]
    pub PORT_7_8: PORT_7_8,
    #[doc = "USCI_A0_UART_MODE"]
    pub USCI_A0_UART_MODE: USCI_A0_UART_MODE,
    #[doc = "USCI_A0_SPI_MODE"]
    pub USCI_A0_SPI_MODE: USCI_A0_SPI_MODE,
    #[doc = "USCI_B0_I2C_MODE"]
    pub USCI_B0_I2C_MODE: USCI_B0_I2C_MODE,
    #[doc = "USCI_B0_SPI_MODE"]
    pub USCI_B0_SPI_MODE: USCI_B0_SPI_MODE,
    #[doc = "SFR"]
    pub SFR: SFR,
    #[doc = "PMM"]
    pub PMM: PMM,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "CS"]
    pub CS: CS,
    #[doc = "FRAM"]
    pub FRAM: FRAM,
    #[doc = "CRC16"]
    pub CRC16: CRC16,
    #[doc = "WATCHDOG_TIMER"]
    pub WATCHDOG_TIMER: WATCHDOG_TIMER,
    #[doc = "CAPACITIVE_TOUCH_IO_0"]
    pub CAPACITIVE_TOUCH_IO_0: CAPACITIVE_TOUCH_IO_0,
    #[doc = "TIMER_0_A3"]
    pub TIMER_0_A3: TIMER_0_A3,
    #[doc = "TIMER_1_A3"]
    pub TIMER_1_A3: TIMER_1_A3,
    #[doc = "REAL_TIME_CLOCK"]
    pub REAL_TIME_CLOCK: REAL_TIME_CLOCK,
    #[doc = "LCD_E"]
    pub LCD_E: LCD_E,
    #[doc = "BACKUP_MEMORY"]
    pub BACKUP_MEMORY: BACKUP_MEMORY,
    #[doc = "ADC"]
    pub ADC: ADC,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        msp430::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PORT_1_2: PORT_1_2 {
                _marker: PhantomData,
            },
            PORT_3_4: PORT_3_4 {
                _marker: PhantomData,
            },
            PORT_5_6: PORT_5_6 {
                _marker: PhantomData,
            },
            PORT_7_8: PORT_7_8 {
                _marker: PhantomData,
            },
            USCI_A0_UART_MODE: USCI_A0_UART_MODE {
                _marker: PhantomData,
            },
            USCI_A0_SPI_MODE: USCI_A0_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_B0_I2C_MODE: USCI_B0_I2C_MODE {
                _marker: PhantomData,
            },
            USCI_B0_SPI_MODE: USCI_B0_SPI_MODE {
                _marker: PhantomData,
            },
            SFR: SFR {
                _marker: PhantomData,
            },
            PMM: PMM {
                _marker: PhantomData,
            },
            SYS: SYS {
                _marker: PhantomData,
            },
            CS: CS {
                _marker: PhantomData,
            },
            FRAM: FRAM {
                _marker: PhantomData,
            },
            CRC16: CRC16 {
                _marker: PhantomData,
            },
            WATCHDOG_TIMER: WATCHDOG_TIMER {
                _marker: PhantomData,
            },
            CAPACITIVE_TOUCH_IO_0: CAPACITIVE_TOUCH_IO_0 {
                _marker: PhantomData,
            },
            TIMER_0_A3: TIMER_0_A3 {
                _marker: PhantomData,
            },
            TIMER_1_A3: TIMER_1_A3 {
                _marker: PhantomData,
            },
            REAL_TIME_CLOCK: REAL_TIME_CLOCK {
                _marker: PhantomData,
            },
            LCD_E: LCD_E {
                _marker: PhantomData,
            },
            BACKUP_MEMORY: BACKUP_MEMORY {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
        }
    }
}
