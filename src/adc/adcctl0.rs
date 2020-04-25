#[doc = "Reader of register ADCCTL0"]
pub type R = crate::R<u16, super::ADCCTL0>;
#[doc = "Writer for register ADCCTL0"]
pub type W = crate::W<u16, super::ADCCTL0>;
#[doc = "Register ADCCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCSC`"]
pub type ADCSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCSC`"]
pub struct ADCSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ADCENC`"]
pub type ADCENC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCENC`"]
pub struct ADCENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCENC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADCON`"]
pub type ADCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCON`"]
pub struct ADCON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADCMSC`"]
pub type ADCMSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCMSC`"]
pub struct ADCMSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "ADC Sample Hold Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSHT_A {
    #[doc = "0: ADC Sample Hold Select 0"]
    ADCSHT_0 = 0,
    #[doc = "1: ADC Sample Hold Select 1"]
    ADCSHT_1 = 1,
    #[doc = "2: ADC Sample Hold Select 2"]
    ADCSHT_2 = 2,
    #[doc = "3: ADC Sample Hold Select 3"]
    ADCSHT_3 = 3,
    #[doc = "4: ADC Sample Hold Select 4"]
    ADCSHT_4 = 4,
    #[doc = "5: ADC Sample Hold Select 5"]
    ADCSHT_5 = 5,
    #[doc = "6: ADC Sample Hold Select 6"]
    ADCSHT_6 = 6,
    #[doc = "7: ADC Sample Hold Select 7"]
    ADCSHT_7 = 7,
    #[doc = "8: ADC Sample Hold Select 8"]
    ADCSHT_8 = 8,
    #[doc = "9: ADC Sample Hold Select 9"]
    ADCSHT_9 = 9,
    #[doc = "10: ADC Sample Hold Select 10"]
    ADCSHT_10 = 10,
    #[doc = "11: ADC Sample Hold Select 11"]
    ADCSHT_11 = 11,
    #[doc = "12: ADC Sample Hold Select 12"]
    ADCSHT_12 = 12,
    #[doc = "13: ADC Sample Hold Select 13"]
    ADCSHT_13 = 13,
    #[doc = "14: ADC Sample Hold Select 14"]
    ADCSHT_14 = 14,
    #[doc = "15: ADC Sample Hold Select 15"]
    ADCSHT_15 = 15,
}
impl From<ADCSHT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSHT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCSHT`"]
pub type ADCSHT_R = crate::R<u8, ADCSHT_A>;
impl ADCSHT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSHT_A {
        match self.bits {
            0 => ADCSHT_A::ADCSHT_0,
            1 => ADCSHT_A::ADCSHT_1,
            2 => ADCSHT_A::ADCSHT_2,
            3 => ADCSHT_A::ADCSHT_3,
            4 => ADCSHT_A::ADCSHT_4,
            5 => ADCSHT_A::ADCSHT_5,
            6 => ADCSHT_A::ADCSHT_6,
            7 => ADCSHT_A::ADCSHT_7,
            8 => ADCSHT_A::ADCSHT_8,
            9 => ADCSHT_A::ADCSHT_9,
            10 => ADCSHT_A::ADCSHT_10,
            11 => ADCSHT_A::ADCSHT_11,
            12 => ADCSHT_A::ADCSHT_12,
            13 => ADCSHT_A::ADCSHT_13,
            14 => ADCSHT_A::ADCSHT_14,
            15 => ADCSHT_A::ADCSHT_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSHT_0`"]
    #[inline(always)]
    pub fn is_adcsht_0(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_0
    }
    #[doc = "Checks if the value of the field is `ADCSHT_1`"]
    #[inline(always)]
    pub fn is_adcsht_1(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_1
    }
    #[doc = "Checks if the value of the field is `ADCSHT_2`"]
    #[inline(always)]
    pub fn is_adcsht_2(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_2
    }
    #[doc = "Checks if the value of the field is `ADCSHT_3`"]
    #[inline(always)]
    pub fn is_adcsht_3(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_3
    }
    #[doc = "Checks if the value of the field is `ADCSHT_4`"]
    #[inline(always)]
    pub fn is_adcsht_4(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_4
    }
    #[doc = "Checks if the value of the field is `ADCSHT_5`"]
    #[inline(always)]
    pub fn is_adcsht_5(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_5
    }
    #[doc = "Checks if the value of the field is `ADCSHT_6`"]
    #[inline(always)]
    pub fn is_adcsht_6(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_6
    }
    #[doc = "Checks if the value of the field is `ADCSHT_7`"]
    #[inline(always)]
    pub fn is_adcsht_7(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_7
    }
    #[doc = "Checks if the value of the field is `ADCSHT_8`"]
    #[inline(always)]
    pub fn is_adcsht_8(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_8
    }
    #[doc = "Checks if the value of the field is `ADCSHT_9`"]
    #[inline(always)]
    pub fn is_adcsht_9(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_9
    }
    #[doc = "Checks if the value of the field is `ADCSHT_10`"]
    #[inline(always)]
    pub fn is_adcsht_10(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_10
    }
    #[doc = "Checks if the value of the field is `ADCSHT_11`"]
    #[inline(always)]
    pub fn is_adcsht_11(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_11
    }
    #[doc = "Checks if the value of the field is `ADCSHT_12`"]
    #[inline(always)]
    pub fn is_adcsht_12(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_12
    }
    #[doc = "Checks if the value of the field is `ADCSHT_13`"]
    #[inline(always)]
    pub fn is_adcsht_13(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_13
    }
    #[doc = "Checks if the value of the field is `ADCSHT_14`"]
    #[inline(always)]
    pub fn is_adcsht_14(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_14
    }
    #[doc = "Checks if the value of the field is `ADCSHT_15`"]
    #[inline(always)]
    pub fn is_adcsht_15(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_15
    }
}
#[doc = "Write proxy for field `ADCSHT`"]
pub struct ADCSHT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSHT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSHT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC Sample Hold Select 0"]
    #[inline(always)]
    pub fn adcsht_0(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_0)
    }
    #[doc = "ADC Sample Hold Select 1"]
    #[inline(always)]
    pub fn adcsht_1(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_1)
    }
    #[doc = "ADC Sample Hold Select 2"]
    #[inline(always)]
    pub fn adcsht_2(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_2)
    }
    #[doc = "ADC Sample Hold Select 3"]
    #[inline(always)]
    pub fn adcsht_3(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_3)
    }
    #[doc = "ADC Sample Hold Select 4"]
    #[inline(always)]
    pub fn adcsht_4(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_4)
    }
    #[doc = "ADC Sample Hold Select 5"]
    #[inline(always)]
    pub fn adcsht_5(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_5)
    }
    #[doc = "ADC Sample Hold Select 6"]
    #[inline(always)]
    pub fn adcsht_6(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_6)
    }
    #[doc = "ADC Sample Hold Select 7"]
    #[inline(always)]
    pub fn adcsht_7(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_7)
    }
    #[doc = "ADC Sample Hold Select 8"]
    #[inline(always)]
    pub fn adcsht_8(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_8)
    }
    #[doc = "ADC Sample Hold Select 9"]
    #[inline(always)]
    pub fn adcsht_9(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_9)
    }
    #[doc = "ADC Sample Hold Select 10"]
    #[inline(always)]
    pub fn adcsht_10(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_10)
    }
    #[doc = "ADC Sample Hold Select 11"]
    #[inline(always)]
    pub fn adcsht_11(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_11)
    }
    #[doc = "ADC Sample Hold Select 12"]
    #[inline(always)]
    pub fn adcsht_12(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_12)
    }
    #[doc = "ADC Sample Hold Select 13"]
    #[inline(always)]
    pub fn adcsht_13(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_13)
    }
    #[doc = "ADC Sample Hold Select 14"]
    #[inline(always)]
    pub fn adcsht_14(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_14)
    }
    #[doc = "ADC Sample Hold Select 15"]
    #[inline(always)]
    pub fn adcsht_15(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC Start Conversion"]
    #[inline(always)]
    pub fn adcsc(&self) -> ADCSC_R {
        ADCSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC Enable Conversion"]
    #[inline(always)]
    pub fn adcenc(&self) -> ADCENC_R {
        ADCENC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC On/enable"]
    #[inline(always)]
    pub fn adcon(&self) -> ADCON_R {
        ADCON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC Multiple SampleConversion"]
    #[inline(always)]
    pub fn adcmsc(&self) -> ADCMSC_R {
        ADCMSC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - ADC Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adcsht(&self) -> ADCSHT_R {
        ADCSHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Start Conversion"]
    #[inline(always)]
    pub fn adcsc(&mut self) -> ADCSC_W {
        ADCSC_W { w: self }
    }
    #[doc = "Bit 1 - ADC Enable Conversion"]
    #[inline(always)]
    pub fn adcenc(&mut self) -> ADCENC_W {
        ADCENC_W { w: self }
    }
    #[doc = "Bit 4 - ADC On/enable"]
    #[inline(always)]
    pub fn adcon(&mut self) -> ADCON_W {
        ADCON_W { w: self }
    }
    #[doc = "Bit 7 - ADC Multiple SampleConversion"]
    #[inline(always)]
    pub fn adcmsc(&mut self) -> ADCMSC_W {
        ADCMSC_W { w: self }
    }
    #[doc = "Bits 8:11 - ADC Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adcsht(&mut self) -> ADCSHT_W {
        ADCSHT_W { w: self }
    }
}
