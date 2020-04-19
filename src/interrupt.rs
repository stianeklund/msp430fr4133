#[cfg(feature = "rt")]
global_asm!(
    "
                DH_TRAMPOLINE:
                    br #DEFAULT_HANDLER
                "
);
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak LCD_E\nLCD_E = DH_TRAMPOLINE\n.weak PORT2\nPORT2 = DH_TRAMPOLINE\n.weak PORT1\nPORT1 = DH_TRAMPOLINE\n.weak ADC\nADC = DH_TRAMPOLINE\n.weak USCI_B0\nUSCI_B0 = DH_TRAMPOLINE\n.weak USCI_A0\nUSCI_A0 = DH_TRAMPOLINE\n.weak WDT\nWDT = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak TIMER1_A1\nTIMER1_A1 = DH_TRAMPOLINE\n.weak TIMER1_A0\nTIMER1_A0 = DH_TRAMPOLINE\n.weak TIMER0_A1\nTIMER0_A1 = DH_TRAMPOLINE\n.weak TIMER0_A0\nTIMER0_A0 = DH_TRAMPOLINE\n.weak UNMI\nUNMI = DH_TRAMPOLINE\n.weak SYSNMI\nSYSNMI = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn LCD_E();
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn UNMI();
    fn SYSNMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Vector; 59] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LCD_E },
    Vector { _handler: PORT2 },
    Vector { _handler: PORT1 },
    Vector { _handler: ADC },
    Vector { _handler: USCI_B0 },
    Vector { _handler: USCI_A0 },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector {
        _handler: TIMER1_A1,
    },
    Vector {
        _handler: TIMER1_A0,
    },
    Vector {
        _handler: TIMER0_A1,
    },
    Vector {
        _handler: TIMER0_A0,
    },
    Vector { _handler: UNMI },
    Vector { _handler: SYSNMI },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "45 - 0xFFE2 LCD E"]
    LCD_E = 45,
    #[doc = "46 - 0xFFE4 Port 2"]
    PORT2 = 46,
    #[doc = "47 - 0xFFE6 Port 1"]
    PORT1 = 47,
    #[doc = "48 - 0xFFE8 ADC"]
    ADC = 48,
    #[doc = "49 - 0xFFEA USCI B0 Receive/Transmit"]
    USCI_B0 = 49,
    #[doc = "50 - 0xFFEC USCI A0 Receive/Transmit"]
    USCI_A0 = 50,
    #[doc = "51 - 0xFFEE Watchdog Timer"]
    WDT = 51,
    #[doc = "52 - 0xFFF0 RTC"]
    RTC = 52,
    #[doc = "53 - 0xFFF2 Timer1_A3 CC1-2, TA"]
    TIMER1_A1 = 53,
    #[doc = "54 - 0xFFF4 Timer1_A3 CC0"]
    TIMER1_A0 = 54,
    #[doc = "55 - 0xFFF6 Timer0_A3 CC1-2, TA"]
    TIMER0_A1 = 55,
    #[doc = "56 - 0xFFE8 Timer0_A3 CC0"]
    TIMER0_A0 = 56,
    #[doc = "57 - 0xFFFA User Non-maskable"]
    UNMI = 57,
    #[doc = "58 - 0xFFFC System Non-maskable"]
    SYSNMI = 58,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            45 => Ok(Interrupt::LCD_E),
            46 => Ok(Interrupt::PORT2),
            47 => Ok(Interrupt::PORT1),
            48 => Ok(Interrupt::ADC),
            49 => Ok(Interrupt::USCI_B0),
            50 => Ok(Interrupt::USCI_A0),
            51 => Ok(Interrupt::WDT),
            52 => Ok(Interrupt::RTC),
            53 => Ok(Interrupt::TIMER1_A1),
            54 => Ok(Interrupt::TIMER1_A0),
            55 => Ok(Interrupt::TIMER0_A1),
            56 => Ok(Interrupt::TIMER0_A0),
            57 => Ok(Interrupt::UNMI),
            58 => Ok(Interrupt::SYSNMI),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
