#[doc = "Reader of register UCA0MCTLW"]
pub type R = crate::R<u16, super::UCA0MCTLW>;
#[doc = "Writer for register UCA0MCTLW"]
pub type W = crate::W<u16, super::UCA0MCTLW>;
#[doc = "Register UCA0MCTLW `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0MCTLW {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCOS16`"]
pub type UCOS16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOS16`"]
pub struct UCOS16_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOS16_W<'a> {
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
#[doc = "USCI First Stage Modulation Select 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCBRF_A {
    #[doc = "0: USCI First Stage Modulation: 0"]
    UCBRF_0 = 0,
    #[doc = "1: USCI First Stage Modulation: 1"]
    UCBRF_1 = 1,
    #[doc = "2: USCI First Stage Modulation: 2"]
    UCBRF_2 = 2,
    #[doc = "3: USCI First Stage Modulation: 3"]
    UCBRF_3 = 3,
    #[doc = "4: USCI First Stage Modulation: 4"]
    UCBRF_4 = 4,
    #[doc = "5: USCI First Stage Modulation: 5"]
    UCBRF_5 = 5,
    #[doc = "6: USCI First Stage Modulation: 6"]
    UCBRF_6 = 6,
    #[doc = "7: USCI First Stage Modulation: 7"]
    UCBRF_7 = 7,
    #[doc = "8: USCI First Stage Modulation: 8"]
    UCBRF_8 = 8,
    #[doc = "9: USCI First Stage Modulation: 9"]
    UCBRF_9 = 9,
    #[doc = "10: USCI First Stage Modulation: A"]
    UCBRF_10 = 10,
    #[doc = "11: USCI First Stage Modulation: B"]
    UCBRF_11 = 11,
    #[doc = "12: USCI First Stage Modulation: C"]
    UCBRF_12 = 12,
    #[doc = "13: USCI First Stage Modulation: D"]
    UCBRF_13 = 13,
    #[doc = "14: USCI First Stage Modulation: E"]
    UCBRF_14 = 14,
    #[doc = "15: USCI First Stage Modulation: F"]
    UCBRF_15 = 15,
}
impl From<UCBRF_A> for u8 {
    #[inline(always)]
    fn from(variant: UCBRF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UCBRF`"]
pub type UCBRF_R = crate::R<u8, UCBRF_A>;
impl UCBRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBRF_A {
        match self.bits {
            0 => UCBRF_A::UCBRF_0,
            1 => UCBRF_A::UCBRF_1,
            2 => UCBRF_A::UCBRF_2,
            3 => UCBRF_A::UCBRF_3,
            4 => UCBRF_A::UCBRF_4,
            5 => UCBRF_A::UCBRF_5,
            6 => UCBRF_A::UCBRF_6,
            7 => UCBRF_A::UCBRF_7,
            8 => UCBRF_A::UCBRF_8,
            9 => UCBRF_A::UCBRF_9,
            10 => UCBRF_A::UCBRF_10,
            11 => UCBRF_A::UCBRF_11,
            12 => UCBRF_A::UCBRF_12,
            13 => UCBRF_A::UCBRF_13,
            14 => UCBRF_A::UCBRF_14,
            15 => UCBRF_A::UCBRF_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCBRF_0`"]
    #[inline(always)]
    pub fn is_ucbrf_0(&self) -> bool {
        *self == UCBRF_A::UCBRF_0
    }
    #[doc = "Checks if the value of the field is `UCBRF_1`"]
    #[inline(always)]
    pub fn is_ucbrf_1(&self) -> bool {
        *self == UCBRF_A::UCBRF_1
    }
    #[doc = "Checks if the value of the field is `UCBRF_2`"]
    #[inline(always)]
    pub fn is_ucbrf_2(&self) -> bool {
        *self == UCBRF_A::UCBRF_2
    }
    #[doc = "Checks if the value of the field is `UCBRF_3`"]
    #[inline(always)]
    pub fn is_ucbrf_3(&self) -> bool {
        *self == UCBRF_A::UCBRF_3
    }
    #[doc = "Checks if the value of the field is `UCBRF_4`"]
    #[inline(always)]
    pub fn is_ucbrf_4(&self) -> bool {
        *self == UCBRF_A::UCBRF_4
    }
    #[doc = "Checks if the value of the field is `UCBRF_5`"]
    #[inline(always)]
    pub fn is_ucbrf_5(&self) -> bool {
        *self == UCBRF_A::UCBRF_5
    }
    #[doc = "Checks if the value of the field is `UCBRF_6`"]
    #[inline(always)]
    pub fn is_ucbrf_6(&self) -> bool {
        *self == UCBRF_A::UCBRF_6
    }
    #[doc = "Checks if the value of the field is `UCBRF_7`"]
    #[inline(always)]
    pub fn is_ucbrf_7(&self) -> bool {
        *self == UCBRF_A::UCBRF_7
    }
    #[doc = "Checks if the value of the field is `UCBRF_8`"]
    #[inline(always)]
    pub fn is_ucbrf_8(&self) -> bool {
        *self == UCBRF_A::UCBRF_8
    }
    #[doc = "Checks if the value of the field is `UCBRF_9`"]
    #[inline(always)]
    pub fn is_ucbrf_9(&self) -> bool {
        *self == UCBRF_A::UCBRF_9
    }
    #[doc = "Checks if the value of the field is `UCBRF_10`"]
    #[inline(always)]
    pub fn is_ucbrf_10(&self) -> bool {
        *self == UCBRF_A::UCBRF_10
    }
    #[doc = "Checks if the value of the field is `UCBRF_11`"]
    #[inline(always)]
    pub fn is_ucbrf_11(&self) -> bool {
        *self == UCBRF_A::UCBRF_11
    }
    #[doc = "Checks if the value of the field is `UCBRF_12`"]
    #[inline(always)]
    pub fn is_ucbrf_12(&self) -> bool {
        *self == UCBRF_A::UCBRF_12
    }
    #[doc = "Checks if the value of the field is `UCBRF_13`"]
    #[inline(always)]
    pub fn is_ucbrf_13(&self) -> bool {
        *self == UCBRF_A::UCBRF_13
    }
    #[doc = "Checks if the value of the field is `UCBRF_14`"]
    #[inline(always)]
    pub fn is_ucbrf_14(&self) -> bool {
        *self == UCBRF_A::UCBRF_14
    }
    #[doc = "Checks if the value of the field is `UCBRF_15`"]
    #[inline(always)]
    pub fn is_ucbrf_15(&self) -> bool {
        *self == UCBRF_A::UCBRF_15
    }
}
#[doc = "Write proxy for field `UCBRF`"]
pub struct UCBRF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBRF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "USCI First Stage Modulation: 0"]
    #[inline(always)]
    pub fn ucbrf_0(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_0)
    }
    #[doc = "USCI First Stage Modulation: 1"]
    #[inline(always)]
    pub fn ucbrf_1(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_1)
    }
    #[doc = "USCI First Stage Modulation: 2"]
    #[inline(always)]
    pub fn ucbrf_2(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_2)
    }
    #[doc = "USCI First Stage Modulation: 3"]
    #[inline(always)]
    pub fn ucbrf_3(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_3)
    }
    #[doc = "USCI First Stage Modulation: 4"]
    #[inline(always)]
    pub fn ucbrf_4(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_4)
    }
    #[doc = "USCI First Stage Modulation: 5"]
    #[inline(always)]
    pub fn ucbrf_5(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_5)
    }
    #[doc = "USCI First Stage Modulation: 6"]
    #[inline(always)]
    pub fn ucbrf_6(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_6)
    }
    #[doc = "USCI First Stage Modulation: 7"]
    #[inline(always)]
    pub fn ucbrf_7(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_7)
    }
    #[doc = "USCI First Stage Modulation: 8"]
    #[inline(always)]
    pub fn ucbrf_8(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_8)
    }
    #[doc = "USCI First Stage Modulation: 9"]
    #[inline(always)]
    pub fn ucbrf_9(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_9)
    }
    #[doc = "USCI First Stage Modulation: A"]
    #[inline(always)]
    pub fn ucbrf_10(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_10)
    }
    #[doc = "USCI First Stage Modulation: B"]
    #[inline(always)]
    pub fn ucbrf_11(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_11)
    }
    #[doc = "USCI First Stage Modulation: C"]
    #[inline(always)]
    pub fn ucbrf_12(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_12)
    }
    #[doc = "USCI First Stage Modulation: D"]
    #[inline(always)]
    pub fn ucbrf_13(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_13)
    }
    #[doc = "USCI First Stage Modulation: E"]
    #[inline(always)]
    pub fn ucbrf_14(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_14)
    }
    #[doc = "USCI First Stage Modulation: F"]
    #[inline(always)]
    pub fn ucbrf_15(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `UCBRS0`"]
pub type UCBRS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRS0`"]
pub struct UCBRS0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `UCBRS1`"]
pub type UCBRS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRS1`"]
pub struct UCBRS1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `UCBRS2`"]
pub type UCBRS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRS2`"]
pub struct UCBRS2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `UCBRS3`"]
pub type UCBRS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRS3`"]
pub struct UCBRS3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `UCBRS4`"]
pub type UCBRS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRS4`"]
pub struct UCBRS4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `UCBRS5`"]
pub type UCBRS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRS5`"]
pub struct UCBRS5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `UCBRS6`"]
pub type UCBRS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRS6`"]
pub struct UCBRS6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `UCBRS7`"]
pub type UCBRS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRS7`"]
pub struct UCBRS7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&self) -> UCOS16_R {
        UCOS16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UCBRF_R {
        UCBRF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - USCI Second Stage Modulation Select 0"]
    #[inline(always)]
    pub fn ucbrs0(&self) -> UCBRS0_R {
        UCBRS0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USCI Second Stage Modulation Select 1"]
    #[inline(always)]
    pub fn ucbrs1(&self) -> UCBRS1_R {
        UCBRS1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs2(&self) -> UCBRS2_R {
        UCBRS2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USCI Second Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrs3(&self) -> UCBRS3_R {
        UCBRS3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USCI Second Stage Modulation Select 4"]
    #[inline(always)]
    pub fn ucbrs4(&self) -> UCBRS4_R {
        UCBRS4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USCI Second Stage Modulation Select 5"]
    #[inline(always)]
    pub fn ucbrs5(&self) -> UCBRS5_R {
        UCBRS5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USCI Second Stage Modulation Select 6"]
    #[inline(always)]
    pub fn ucbrs6(&self) -> UCBRS6_R {
        UCBRS6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USCI Second Stage Modulation Select 7"]
    #[inline(always)]
    pub fn ucbrs7(&self) -> UCBRS7_R {
        UCBRS7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> UCOS16_W {
        UCOS16_W { w: self }
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UCBRF_W {
        UCBRF_W { w: self }
    }
    #[doc = "Bit 8 - USCI Second Stage Modulation Select 0"]
    #[inline(always)]
    pub fn ucbrs0(&mut self) -> UCBRS0_W {
        UCBRS0_W { w: self }
    }
    #[doc = "Bit 9 - USCI Second Stage Modulation Select 1"]
    #[inline(always)]
    pub fn ucbrs1(&mut self) -> UCBRS1_W {
        UCBRS1_W { w: self }
    }
    #[doc = "Bit 10 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs2(&mut self) -> UCBRS2_W {
        UCBRS2_W { w: self }
    }
    #[doc = "Bit 11 - USCI Second Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrs3(&mut self) -> UCBRS3_W {
        UCBRS3_W { w: self }
    }
    #[doc = "Bit 12 - USCI Second Stage Modulation Select 4"]
    #[inline(always)]
    pub fn ucbrs4(&mut self) -> UCBRS4_W {
        UCBRS4_W { w: self }
    }
    #[doc = "Bit 13 - USCI Second Stage Modulation Select 5"]
    #[inline(always)]
    pub fn ucbrs5(&mut self) -> UCBRS5_W {
        UCBRS5_W { w: self }
    }
    #[doc = "Bit 14 - USCI Second Stage Modulation Select 6"]
    #[inline(always)]
    pub fn ucbrs6(&mut self) -> UCBRS6_W {
        UCBRS6_W { w: self }
    }
    #[doc = "Bit 15 - USCI Second Stage Modulation Select 7"]
    #[inline(always)]
    pub fn ucbrs7(&mut self) -> UCBRS7_W {
        UCBRS7_W { w: self }
    }
}
