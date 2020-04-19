#[doc = "Reader of register ADCMCTL0"]
pub type R = crate::R<u16, super::ADCMCTL0>;
#[doc = "Writer for register ADCMCTL0"]
pub type W = crate::W<u16, super::ADCMCTL0>;
#[doc = "Register ADCMCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCMCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC Input Channel Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCINCH_A {
    #[doc = "0: ADC Input Channel 0"]
    ADCINCH_0 = 0,
    #[doc = "1: ADC Input Channel 1"]
    ADCINCH_1 = 1,
    #[doc = "2: ADC Input Channel 2"]
    ADCINCH_2 = 2,
    #[doc = "3: ADC Input Channel 3"]
    ADCINCH_3 = 3,
    #[doc = "4: ADC Input Channel 4"]
    ADCINCH_4 = 4,
    #[doc = "5: ADC Input Channel 5"]
    ADCINCH_5 = 5,
    #[doc = "6: ADC Input Channel 6"]
    ADCINCH_6 = 6,
    #[doc = "7: ADC Input Channel 7"]
    ADCINCH_7 = 7,
    #[doc = "8: ADC Input Channel 8"]
    ADCINCH_8 = 8,
    #[doc = "9: ADC Input Channel 9"]
    ADCINCH_9 = 9,
    #[doc = "10: ADC Input Channel 10"]
    ADCINCH_10 = 10,
    #[doc = "11: ADC Input Channel 11"]
    ADCINCH_11 = 11,
    #[doc = "12: ADC Input Channel 12"]
    ADCINCH_12 = 12,
    #[doc = "13: ADC Input Channel 13"]
    ADCINCH_13 = 13,
    #[doc = "14: ADC Input Channel 14"]
    ADCINCH_14 = 14,
    #[doc = "15: ADC Input Channel 15"]
    ADCINCH_15 = 15,
}
impl From<ADCINCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCINCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCINCH`"]
pub type ADCINCH_R = crate::R<u8, ADCINCH_A>;
impl ADCINCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCINCH_A {
        match self.bits {
            0 => ADCINCH_A::ADCINCH_0,
            1 => ADCINCH_A::ADCINCH_1,
            2 => ADCINCH_A::ADCINCH_2,
            3 => ADCINCH_A::ADCINCH_3,
            4 => ADCINCH_A::ADCINCH_4,
            5 => ADCINCH_A::ADCINCH_5,
            6 => ADCINCH_A::ADCINCH_6,
            7 => ADCINCH_A::ADCINCH_7,
            8 => ADCINCH_A::ADCINCH_8,
            9 => ADCINCH_A::ADCINCH_9,
            10 => ADCINCH_A::ADCINCH_10,
            11 => ADCINCH_A::ADCINCH_11,
            12 => ADCINCH_A::ADCINCH_12,
            13 => ADCINCH_A::ADCINCH_13,
            14 => ADCINCH_A::ADCINCH_14,
            15 => ADCINCH_A::ADCINCH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCINCH_0`"]
    #[inline(always)]
    pub fn is_adcinch_0(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_0
    }
    #[doc = "Checks if the value of the field is `ADCINCH_1`"]
    #[inline(always)]
    pub fn is_adcinch_1(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_1
    }
    #[doc = "Checks if the value of the field is `ADCINCH_2`"]
    #[inline(always)]
    pub fn is_adcinch_2(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_2
    }
    #[doc = "Checks if the value of the field is `ADCINCH_3`"]
    #[inline(always)]
    pub fn is_adcinch_3(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_3
    }
    #[doc = "Checks if the value of the field is `ADCINCH_4`"]
    #[inline(always)]
    pub fn is_adcinch_4(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_4
    }
    #[doc = "Checks if the value of the field is `ADCINCH_5`"]
    #[inline(always)]
    pub fn is_adcinch_5(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_5
    }
    #[doc = "Checks if the value of the field is `ADCINCH_6`"]
    #[inline(always)]
    pub fn is_adcinch_6(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_6
    }
    #[doc = "Checks if the value of the field is `ADCINCH_7`"]
    #[inline(always)]
    pub fn is_adcinch_7(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_7
    }
    #[doc = "Checks if the value of the field is `ADCINCH_8`"]
    #[inline(always)]
    pub fn is_adcinch_8(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_8
    }
    #[doc = "Checks if the value of the field is `ADCINCH_9`"]
    #[inline(always)]
    pub fn is_adcinch_9(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_9
    }
    #[doc = "Checks if the value of the field is `ADCINCH_10`"]
    #[inline(always)]
    pub fn is_adcinch_10(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_10
    }
    #[doc = "Checks if the value of the field is `ADCINCH_11`"]
    #[inline(always)]
    pub fn is_adcinch_11(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_11
    }
    #[doc = "Checks if the value of the field is `ADCINCH_12`"]
    #[inline(always)]
    pub fn is_adcinch_12(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_12
    }
    #[doc = "Checks if the value of the field is `ADCINCH_13`"]
    #[inline(always)]
    pub fn is_adcinch_13(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_13
    }
    #[doc = "Checks if the value of the field is `ADCINCH_14`"]
    #[inline(always)]
    pub fn is_adcinch_14(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_14
    }
    #[doc = "Checks if the value of the field is `ADCINCH_15`"]
    #[inline(always)]
    pub fn is_adcinch_15(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_15
    }
}
#[doc = "Write proxy for field `ADCINCH`"]
pub struct ADCINCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCINCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCINCH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC Input Channel 0"]
    #[inline(always)]
    pub fn adcinch_0(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_0)
    }
    #[doc = "ADC Input Channel 1"]
    #[inline(always)]
    pub fn adcinch_1(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_1)
    }
    #[doc = "ADC Input Channel 2"]
    #[inline(always)]
    pub fn adcinch_2(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_2)
    }
    #[doc = "ADC Input Channel 3"]
    #[inline(always)]
    pub fn adcinch_3(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_3)
    }
    #[doc = "ADC Input Channel 4"]
    #[inline(always)]
    pub fn adcinch_4(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_4)
    }
    #[doc = "ADC Input Channel 5"]
    #[inline(always)]
    pub fn adcinch_5(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_5)
    }
    #[doc = "ADC Input Channel 6"]
    #[inline(always)]
    pub fn adcinch_6(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_6)
    }
    #[doc = "ADC Input Channel 7"]
    #[inline(always)]
    pub fn adcinch_7(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_7)
    }
    #[doc = "ADC Input Channel 8"]
    #[inline(always)]
    pub fn adcinch_8(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_8)
    }
    #[doc = "ADC Input Channel 9"]
    #[inline(always)]
    pub fn adcinch_9(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_9)
    }
    #[doc = "ADC Input Channel 10"]
    #[inline(always)]
    pub fn adcinch_10(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_10)
    }
    #[doc = "ADC Input Channel 11"]
    #[inline(always)]
    pub fn adcinch_11(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_11)
    }
    #[doc = "ADC Input Channel 12"]
    #[inline(always)]
    pub fn adcinch_12(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_12)
    }
    #[doc = "ADC Input Channel 13"]
    #[inline(always)]
    pub fn adcinch_13(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_13)
    }
    #[doc = "ADC Input Channel 14"]
    #[inline(always)]
    pub fn adcinch_14(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_14)
    }
    #[doc = "ADC Input Channel 15"]
    #[inline(always)]
    pub fn adcinch_15(self) -> &'a mut W {
        self.variant(ADCINCH_A::ADCINCH_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "ADC Select Reference Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSREF_A {
    #[doc = "0: ADC Select Reference 0"]
    ADCSREF_0 = 0,
    #[doc = "1: ADC Select Reference 1"]
    ADCSREF_1 = 1,
    #[doc = "2: ADC Select Reference 2"]
    ADCSREF_2 = 2,
    #[doc = "3: ADC Select Reference 3"]
    ADCSREF_3 = 3,
    #[doc = "4: ADC Select Reference 4"]
    ADCSREF_4 = 4,
    #[doc = "5: ADC Select Reference 5"]
    ADCSREF_5 = 5,
    #[doc = "6: ADC Select Reference 6"]
    ADCSREF_6 = 6,
    #[doc = "7: ADC Select Reference 7"]
    ADCSREF_7 = 7,
}
impl From<ADCSREF_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSREF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCSREF`"]
pub type ADCSREF_R = crate::R<u8, ADCSREF_A>;
impl ADCSREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSREF_A {
        match self.bits {
            0 => ADCSREF_A::ADCSREF_0,
            1 => ADCSREF_A::ADCSREF_1,
            2 => ADCSREF_A::ADCSREF_2,
            3 => ADCSREF_A::ADCSREF_3,
            4 => ADCSREF_A::ADCSREF_4,
            5 => ADCSREF_A::ADCSREF_5,
            6 => ADCSREF_A::ADCSREF_6,
            7 => ADCSREF_A::ADCSREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSREF_0`"]
    #[inline(always)]
    pub fn is_adcsref_0(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_0
    }
    #[doc = "Checks if the value of the field is `ADCSREF_1`"]
    #[inline(always)]
    pub fn is_adcsref_1(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_1
    }
    #[doc = "Checks if the value of the field is `ADCSREF_2`"]
    #[inline(always)]
    pub fn is_adcsref_2(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_2
    }
    #[doc = "Checks if the value of the field is `ADCSREF_3`"]
    #[inline(always)]
    pub fn is_adcsref_3(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_3
    }
    #[doc = "Checks if the value of the field is `ADCSREF_4`"]
    #[inline(always)]
    pub fn is_adcsref_4(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_4
    }
    #[doc = "Checks if the value of the field is `ADCSREF_5`"]
    #[inline(always)]
    pub fn is_adcsref_5(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_5
    }
    #[doc = "Checks if the value of the field is `ADCSREF_6`"]
    #[inline(always)]
    pub fn is_adcsref_6(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_6
    }
    #[doc = "Checks if the value of the field is `ADCSREF_7`"]
    #[inline(always)]
    pub fn is_adcsref_7(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_7
    }
}
#[doc = "Write proxy for field `ADCSREF`"]
pub struct ADCSREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSREF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC Select Reference 0"]
    #[inline(always)]
    pub fn adcsref_0(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_0)
    }
    #[doc = "ADC Select Reference 1"]
    #[inline(always)]
    pub fn adcsref_1(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_1)
    }
    #[doc = "ADC Select Reference 2"]
    #[inline(always)]
    pub fn adcsref_2(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_2)
    }
    #[doc = "ADC Select Reference 3"]
    #[inline(always)]
    pub fn adcsref_3(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_3)
    }
    #[doc = "ADC Select Reference 4"]
    #[inline(always)]
    pub fn adcsref_4(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_4)
    }
    #[doc = "ADC Select Reference 5"]
    #[inline(always)]
    pub fn adcsref_5(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_5)
    }
    #[doc = "ADC Select Reference 6"]
    #[inline(always)]
    pub fn adcsref_6(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_6)
    }
    #[doc = "ADC Select Reference 7"]
    #[inline(always)]
    pub fn adcsref_7(self) -> &'a mut W {
        self.variant(ADCSREF_A::ADCSREF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adcinch(&self) -> ADCINCH_R {
        ADCINCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - ADC Select Reference Bit 0"]
    #[inline(always)]
    pub fn adcsref(&self) -> ADCSREF_R {
        ADCSREF_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adcinch(&mut self) -> ADCINCH_W {
        ADCINCH_W { w: self }
    }
    #[doc = "Bits 4:6 - ADC Select Reference Bit 0"]
    #[inline(always)]
    pub fn adcsref(&mut self) -> ADCSREF_W {
        ADCSREF_W { w: self }
    }
}
