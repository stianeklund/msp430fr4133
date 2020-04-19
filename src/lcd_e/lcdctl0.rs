#[doc = "Reader of register LCDCTL0"]
pub type R = crate::R<u16, super::LCDCTL0>;
#[doc = "Writer for register LCDCTL0"]
pub type W = crate::W<u16, super::LCDCTL0>;
#[doc = "Register LCDCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDON`"]
pub type LCDON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDON`"]
pub struct LCDON_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDON_W<'a> {
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
#[doc = "Reader of field `LCDLP`"]
pub type LCDLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDLP`"]
pub struct LCDLP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDLP_W<'a> {
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
#[doc = "Reader of field `LCDSON`"]
pub type LCDSON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDSON`"]
pub struct LCDSON_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDSON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `LCDMX0`"]
pub type LCDMX0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDMX0`"]
pub struct LCDMX0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDMX0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `LCDMX1`"]
pub type LCDMX1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDMX1`"]
pub struct LCDMX1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDMX1_W<'a> {
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
#[doc = "Reader of field `LCDMX2`"]
pub type LCDMX2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDMX2`"]
pub struct LCDMX2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDMX2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "LCD_E Clock Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDSSEL_A {
    #[doc = "0: LCD_E Clock Select: 0"]
    LCDSSEL_0 = 0,
    #[doc = "1: LCD_E Clock Select: 1"]
    LCDSSEL_1 = 1,
    #[doc = "2: LCD_E Clock Select: 2"]
    LCDSSEL_2 = 2,
    #[doc = "3: LCD_E Clock Select: 3"]
    LCDSSEL_3 = 3,
}
impl From<LCDSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCDSSEL`"]
pub type LCDSSEL_R = crate::R<u8, LCDSSEL_A>;
impl LCDSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDSSEL_A {
        match self.bits {
            0 => LCDSSEL_A::LCDSSEL_0,
            1 => LCDSSEL_A::LCDSSEL_1,
            2 => LCDSSEL_A::LCDSSEL_2,
            3 => LCDSSEL_A::LCDSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDSSEL_0`"]
    #[inline(always)]
    pub fn is_lcdssel_0(&self) -> bool {
        *self == LCDSSEL_A::LCDSSEL_0
    }
    #[doc = "Checks if the value of the field is `LCDSSEL_1`"]
    #[inline(always)]
    pub fn is_lcdssel_1(&self) -> bool {
        *self == LCDSSEL_A::LCDSSEL_1
    }
    #[doc = "Checks if the value of the field is `LCDSSEL_2`"]
    #[inline(always)]
    pub fn is_lcdssel_2(&self) -> bool {
        *self == LCDSSEL_A::LCDSSEL_2
    }
    #[doc = "Checks if the value of the field is `LCDSSEL_3`"]
    #[inline(always)]
    pub fn is_lcdssel_3(&self) -> bool {
        *self == LCDSSEL_A::LCDSSEL_3
    }
}
#[doc = "Write proxy for field `LCDSSEL`"]
pub struct LCDSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LCD_E Clock Select: 0"]
    #[inline(always)]
    pub fn lcdssel_0(self) -> &'a mut W {
        self.variant(LCDSSEL_A::LCDSSEL_0)
    }
    #[doc = "LCD_E Clock Select: 1"]
    #[inline(always)]
    pub fn lcdssel_1(self) -> &'a mut W {
        self.variant(LCDSSEL_A::LCDSSEL_1)
    }
    #[doc = "LCD_E Clock Select: 2"]
    #[inline(always)]
    pub fn lcdssel_2(self) -> &'a mut W {
        self.variant(LCDSSEL_A::LCDSSEL_2)
    }
    #[doc = "LCD_E Clock Select: 3"]
    #[inline(always)]
    pub fn lcdssel_3(self) -> &'a mut W {
        self.variant(LCDSSEL_A::LCDSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "LCD_E LCD frequency divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDDIV_A {
    #[doc = "0: LCD_E LCD frequency divider: /1"]
    LCDDIV_0 = 0,
    #[doc = "1: LCD_E LCD frequency divider: /2"]
    LCDDIV_1 = 1,
    #[doc = "2: LCD_E LCD frequency divider: /3"]
    LCDDIV_2 = 2,
    #[doc = "3: LCD_E LCD frequency divider: /4"]
    LCDDIV_3 = 3,
    #[doc = "4: LCD_E LCD frequency divider: /5"]
    LCDDIV_4 = 4,
    #[doc = "5: LCD_E LCD frequency divider: /6"]
    LCDDIV_5 = 5,
    #[doc = "6: LCD_E LCD frequency divider: /7"]
    LCDDIV_6 = 6,
    #[doc = "7: LCD_E LCD frequency divider: /8"]
    LCDDIV_7 = 7,
    #[doc = "8: LCD_E LCD frequency divider: /9"]
    LCDDIV_8 = 8,
    #[doc = "9: LCD_E LCD frequency divider: /10"]
    LCDDIV_9 = 9,
    #[doc = "10: LCD_E LCD frequency divider: /11"]
    LCDDIV_10 = 10,
    #[doc = "11: LCD_E LCD frequency divider: /12"]
    LCDDIV_11 = 11,
    #[doc = "12: LCD_E LCD frequency divider: /13"]
    LCDDIV_12 = 12,
    #[doc = "13: LCD_E LCD frequency divider: /14"]
    LCDDIV_13 = 13,
    #[doc = "14: LCD_E LCD frequency divider: /15"]
    LCDDIV_14 = 14,
    #[doc = "15: LCD_E LCD frequency divider: /16"]
    LCDDIV_15 = 15,
    #[doc = "16: LCD_E LCD frequency divider: /17"]
    LCDDIV_16 = 16,
    #[doc = "17: LCD_E LCD frequency divider: /18"]
    LCDDIV_17 = 17,
    #[doc = "18: LCD_E LCD frequency divider: /19"]
    LCDDIV_18 = 18,
    #[doc = "19: LCD_E LCD frequency divider: /20"]
    LCDDIV_19 = 19,
    #[doc = "20: LCD_E LCD frequency divider: /21"]
    LCDDIV_20 = 20,
    #[doc = "21: LCD_E LCD frequency divider: /22"]
    LCDDIV_21 = 21,
    #[doc = "22: LCD_E LCD frequency divider: /23"]
    LCDDIV_22 = 22,
    #[doc = "23: LCD_E LCD frequency divider: /24"]
    LCDDIV_23 = 23,
    #[doc = "24: LCD_E LCD frequency divider: /25"]
    LCDDIV_24 = 24,
    #[doc = "25: LCD_E LCD frequency divider: /26"]
    LCDDIV_25 = 25,
    #[doc = "26: LCD_E LCD frequency divider: /27"]
    LCDDIV_26 = 26,
    #[doc = "27: LCD_E LCD frequency divider: /28"]
    LCDDIV_27 = 27,
    #[doc = "28: LCD_E LCD frequency divider: /29"]
    LCDDIV_28 = 28,
    #[doc = "29: LCD_E LCD frequency divider: /30"]
    LCDDIV_29 = 29,
    #[doc = "30: LCD_E LCD frequency divider: /31"]
    LCDDIV_30 = 30,
    #[doc = "31: LCD_E LCD frequency divider: /32"]
    LCDDIV_31 = 31,
}
impl From<LCDDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCDDIV`"]
pub type LCDDIV_R = crate::R<u8, LCDDIV_A>;
impl LCDDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDDIV_A {
        match self.bits {
            0 => LCDDIV_A::LCDDIV_0,
            1 => LCDDIV_A::LCDDIV_1,
            2 => LCDDIV_A::LCDDIV_2,
            3 => LCDDIV_A::LCDDIV_3,
            4 => LCDDIV_A::LCDDIV_4,
            5 => LCDDIV_A::LCDDIV_5,
            6 => LCDDIV_A::LCDDIV_6,
            7 => LCDDIV_A::LCDDIV_7,
            8 => LCDDIV_A::LCDDIV_8,
            9 => LCDDIV_A::LCDDIV_9,
            10 => LCDDIV_A::LCDDIV_10,
            11 => LCDDIV_A::LCDDIV_11,
            12 => LCDDIV_A::LCDDIV_12,
            13 => LCDDIV_A::LCDDIV_13,
            14 => LCDDIV_A::LCDDIV_14,
            15 => LCDDIV_A::LCDDIV_15,
            16 => LCDDIV_A::LCDDIV_16,
            17 => LCDDIV_A::LCDDIV_17,
            18 => LCDDIV_A::LCDDIV_18,
            19 => LCDDIV_A::LCDDIV_19,
            20 => LCDDIV_A::LCDDIV_20,
            21 => LCDDIV_A::LCDDIV_21,
            22 => LCDDIV_A::LCDDIV_22,
            23 => LCDDIV_A::LCDDIV_23,
            24 => LCDDIV_A::LCDDIV_24,
            25 => LCDDIV_A::LCDDIV_25,
            26 => LCDDIV_A::LCDDIV_26,
            27 => LCDDIV_A::LCDDIV_27,
            28 => LCDDIV_A::LCDDIV_28,
            29 => LCDDIV_A::LCDDIV_29,
            30 => LCDDIV_A::LCDDIV_30,
            31 => LCDDIV_A::LCDDIV_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDDIV_0`"]
    #[inline(always)]
    pub fn is_lcddiv_0(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_0
    }
    #[doc = "Checks if the value of the field is `LCDDIV_1`"]
    #[inline(always)]
    pub fn is_lcddiv_1(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_1
    }
    #[doc = "Checks if the value of the field is `LCDDIV_2`"]
    #[inline(always)]
    pub fn is_lcddiv_2(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_2
    }
    #[doc = "Checks if the value of the field is `LCDDIV_3`"]
    #[inline(always)]
    pub fn is_lcddiv_3(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_3
    }
    #[doc = "Checks if the value of the field is `LCDDIV_4`"]
    #[inline(always)]
    pub fn is_lcddiv_4(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_4
    }
    #[doc = "Checks if the value of the field is `LCDDIV_5`"]
    #[inline(always)]
    pub fn is_lcddiv_5(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_5
    }
    #[doc = "Checks if the value of the field is `LCDDIV_6`"]
    #[inline(always)]
    pub fn is_lcddiv_6(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_6
    }
    #[doc = "Checks if the value of the field is `LCDDIV_7`"]
    #[inline(always)]
    pub fn is_lcddiv_7(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_7
    }
    #[doc = "Checks if the value of the field is `LCDDIV_8`"]
    #[inline(always)]
    pub fn is_lcddiv_8(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_8
    }
    #[doc = "Checks if the value of the field is `LCDDIV_9`"]
    #[inline(always)]
    pub fn is_lcddiv_9(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_9
    }
    #[doc = "Checks if the value of the field is `LCDDIV_10`"]
    #[inline(always)]
    pub fn is_lcddiv_10(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_10
    }
    #[doc = "Checks if the value of the field is `LCDDIV_11`"]
    #[inline(always)]
    pub fn is_lcddiv_11(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_11
    }
    #[doc = "Checks if the value of the field is `LCDDIV_12`"]
    #[inline(always)]
    pub fn is_lcddiv_12(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_12
    }
    #[doc = "Checks if the value of the field is `LCDDIV_13`"]
    #[inline(always)]
    pub fn is_lcddiv_13(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_13
    }
    #[doc = "Checks if the value of the field is `LCDDIV_14`"]
    #[inline(always)]
    pub fn is_lcddiv_14(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_14
    }
    #[doc = "Checks if the value of the field is `LCDDIV_15`"]
    #[inline(always)]
    pub fn is_lcddiv_15(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_15
    }
    #[doc = "Checks if the value of the field is `LCDDIV_16`"]
    #[inline(always)]
    pub fn is_lcddiv_16(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_16
    }
    #[doc = "Checks if the value of the field is `LCDDIV_17`"]
    #[inline(always)]
    pub fn is_lcddiv_17(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_17
    }
    #[doc = "Checks if the value of the field is `LCDDIV_18`"]
    #[inline(always)]
    pub fn is_lcddiv_18(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_18
    }
    #[doc = "Checks if the value of the field is `LCDDIV_19`"]
    #[inline(always)]
    pub fn is_lcddiv_19(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_19
    }
    #[doc = "Checks if the value of the field is `LCDDIV_20`"]
    #[inline(always)]
    pub fn is_lcddiv_20(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_20
    }
    #[doc = "Checks if the value of the field is `LCDDIV_21`"]
    #[inline(always)]
    pub fn is_lcddiv_21(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_21
    }
    #[doc = "Checks if the value of the field is `LCDDIV_22`"]
    #[inline(always)]
    pub fn is_lcddiv_22(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_22
    }
    #[doc = "Checks if the value of the field is `LCDDIV_23`"]
    #[inline(always)]
    pub fn is_lcddiv_23(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_23
    }
    #[doc = "Checks if the value of the field is `LCDDIV_24`"]
    #[inline(always)]
    pub fn is_lcddiv_24(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_24
    }
    #[doc = "Checks if the value of the field is `LCDDIV_25`"]
    #[inline(always)]
    pub fn is_lcddiv_25(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_25
    }
    #[doc = "Checks if the value of the field is `LCDDIV_26`"]
    #[inline(always)]
    pub fn is_lcddiv_26(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_26
    }
    #[doc = "Checks if the value of the field is `LCDDIV_27`"]
    #[inline(always)]
    pub fn is_lcddiv_27(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_27
    }
    #[doc = "Checks if the value of the field is `LCDDIV_28`"]
    #[inline(always)]
    pub fn is_lcddiv_28(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_28
    }
    #[doc = "Checks if the value of the field is `LCDDIV_29`"]
    #[inline(always)]
    pub fn is_lcddiv_29(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_29
    }
    #[doc = "Checks if the value of the field is `LCDDIV_30`"]
    #[inline(always)]
    pub fn is_lcddiv_30(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_30
    }
    #[doc = "Checks if the value of the field is `LCDDIV_31`"]
    #[inline(always)]
    pub fn is_lcddiv_31(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_31
    }
}
#[doc = "Write proxy for field `LCDDIV`"]
pub struct LCDDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LCD_E LCD frequency divider: /1"]
    #[inline(always)]
    pub fn lcddiv_0(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_0)
    }
    #[doc = "LCD_E LCD frequency divider: /2"]
    #[inline(always)]
    pub fn lcddiv_1(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_1)
    }
    #[doc = "LCD_E LCD frequency divider: /3"]
    #[inline(always)]
    pub fn lcddiv_2(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_2)
    }
    #[doc = "LCD_E LCD frequency divider: /4"]
    #[inline(always)]
    pub fn lcddiv_3(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_3)
    }
    #[doc = "LCD_E LCD frequency divider: /5"]
    #[inline(always)]
    pub fn lcddiv_4(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_4)
    }
    #[doc = "LCD_E LCD frequency divider: /6"]
    #[inline(always)]
    pub fn lcddiv_5(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_5)
    }
    #[doc = "LCD_E LCD frequency divider: /7"]
    #[inline(always)]
    pub fn lcddiv_6(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_6)
    }
    #[doc = "LCD_E LCD frequency divider: /8"]
    #[inline(always)]
    pub fn lcddiv_7(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_7)
    }
    #[doc = "LCD_E LCD frequency divider: /9"]
    #[inline(always)]
    pub fn lcddiv_8(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_8)
    }
    #[doc = "LCD_E LCD frequency divider: /10"]
    #[inline(always)]
    pub fn lcddiv_9(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_9)
    }
    #[doc = "LCD_E LCD frequency divider: /11"]
    #[inline(always)]
    pub fn lcddiv_10(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_10)
    }
    #[doc = "LCD_E LCD frequency divider: /12"]
    #[inline(always)]
    pub fn lcddiv_11(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_11)
    }
    #[doc = "LCD_E LCD frequency divider: /13"]
    #[inline(always)]
    pub fn lcddiv_12(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_12)
    }
    #[doc = "LCD_E LCD frequency divider: /14"]
    #[inline(always)]
    pub fn lcddiv_13(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_13)
    }
    #[doc = "LCD_E LCD frequency divider: /15"]
    #[inline(always)]
    pub fn lcddiv_14(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_14)
    }
    #[doc = "LCD_E LCD frequency divider: /16"]
    #[inline(always)]
    pub fn lcddiv_15(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_15)
    }
    #[doc = "LCD_E LCD frequency divider: /17"]
    #[inline(always)]
    pub fn lcddiv_16(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_16)
    }
    #[doc = "LCD_E LCD frequency divider: /18"]
    #[inline(always)]
    pub fn lcddiv_17(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_17)
    }
    #[doc = "LCD_E LCD frequency divider: /19"]
    #[inline(always)]
    pub fn lcddiv_18(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_18)
    }
    #[doc = "LCD_E LCD frequency divider: /20"]
    #[inline(always)]
    pub fn lcddiv_19(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_19)
    }
    #[doc = "LCD_E LCD frequency divider: /21"]
    #[inline(always)]
    pub fn lcddiv_20(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_20)
    }
    #[doc = "LCD_E LCD frequency divider: /22"]
    #[inline(always)]
    pub fn lcddiv_21(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_21)
    }
    #[doc = "LCD_E LCD frequency divider: /23"]
    #[inline(always)]
    pub fn lcddiv_22(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_22)
    }
    #[doc = "LCD_E LCD frequency divider: /24"]
    #[inline(always)]
    pub fn lcddiv_23(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_23)
    }
    #[doc = "LCD_E LCD frequency divider: /25"]
    #[inline(always)]
    pub fn lcddiv_24(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_24)
    }
    #[doc = "LCD_E LCD frequency divider: /26"]
    #[inline(always)]
    pub fn lcddiv_25(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_25)
    }
    #[doc = "LCD_E LCD frequency divider: /27"]
    #[inline(always)]
    pub fn lcddiv_26(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_26)
    }
    #[doc = "LCD_E LCD frequency divider: /28"]
    #[inline(always)]
    pub fn lcddiv_27(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_27)
    }
    #[doc = "LCD_E LCD frequency divider: /29"]
    #[inline(always)]
    pub fn lcddiv_28(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_28)
    }
    #[doc = "LCD_E LCD frequency divider: /30"]
    #[inline(always)]
    pub fn lcddiv_29(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_29)
    }
    #[doc = "LCD_E LCD frequency divider: /31"]
    #[inline(always)]
    pub fn lcddiv_30(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_30)
    }
    #[doc = "LCD_E LCD frequency divider: /32"]
    #[inline(always)]
    pub fn lcddiv_31(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u16) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LCD_E LCD On"]
    #[inline(always)]
    pub fn lcdon(&self) -> LCDON_R {
        LCDON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD_E Low Power Waveform"]
    #[inline(always)]
    pub fn lcdlp(&self) -> LCDLP_R {
        LCDLP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD_E LCD Segments On"]
    #[inline(always)]
    pub fn lcdson(&self) -> LCDSON_R {
        LCDSON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD_E Mux Rate Bit: 0"]
    #[inline(always)]
    pub fn lcdmx0(&self) -> LCDMX0_R {
        LCDMX0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCD_E Mux Rate Bit: 1"]
    #[inline(always)]
    pub fn lcdmx1(&self) -> LCDMX1_R {
        LCDMX1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD_E Mux Rate Bit: 2"]
    #[inline(always)]
    pub fn lcdmx2(&self) -> LCDMX2_R {
        LCDMX2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - LCD_E Clock Select Bit: 0"]
    #[inline(always)]
    pub fn lcdssel(&self) -> LCDSSEL_R {
        LCDSSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 11:15 - LCD_E LCD frequency divider Bit: 0"]
    #[inline(always)]
    pub fn lcddiv(&self) -> LCDDIV_R {
        LCDDIV_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_E LCD On"]
    #[inline(always)]
    pub fn lcdon(&mut self) -> LCDON_W {
        LCDON_W { w: self }
    }
    #[doc = "Bit 1 - LCD_E Low Power Waveform"]
    #[inline(always)]
    pub fn lcdlp(&mut self) -> LCDLP_W {
        LCDLP_W { w: self }
    }
    #[doc = "Bit 2 - LCD_E LCD Segments On"]
    #[inline(always)]
    pub fn lcdson(&mut self) -> LCDSON_W {
        LCDSON_W { w: self }
    }
    #[doc = "Bit 3 - LCD_E Mux Rate Bit: 0"]
    #[inline(always)]
    pub fn lcdmx0(&mut self) -> LCDMX0_W {
        LCDMX0_W { w: self }
    }
    #[doc = "Bit 4 - LCD_E Mux Rate Bit: 1"]
    #[inline(always)]
    pub fn lcdmx1(&mut self) -> LCDMX1_W {
        LCDMX1_W { w: self }
    }
    #[doc = "Bit 5 - LCD_E Mux Rate Bit: 2"]
    #[inline(always)]
    pub fn lcdmx2(&mut self) -> LCDMX2_W {
        LCDMX2_W { w: self }
    }
    #[doc = "Bits 6:7 - LCD_E Clock Select Bit: 0"]
    #[inline(always)]
    pub fn lcdssel(&mut self) -> LCDSSEL_W {
        LCDSSEL_W { w: self }
    }
    #[doc = "Bits 11:15 - LCD_E LCD frequency divider Bit: 0"]
    #[inline(always)]
    pub fn lcddiv(&mut self) -> LCDDIV_W {
        LCDDIV_W { w: self }
    }
}
