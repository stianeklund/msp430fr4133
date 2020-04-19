#[doc = "Reader of register LCDBLKCTL"]
pub type R = crate::R<u16, super::LCDBLKCTL>;
#[doc = "Writer for register LCDBLKCTL"]
pub type W = crate::W<u16, super::LCDBLKCTL>;
#[doc = "Register LCDBLKCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDBLKCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LCD_E Blinking mode Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDBLKMOD_A {
    #[doc = "0: LCD_E Blinking mode: Off"]
    LCDBLKMOD_0 = 0,
    #[doc = "1: LCD_E Blinking mode: Individual"]
    LCDBLKMOD_1 = 1,
    #[doc = "2: LCD_E Blinking mode: All"]
    LCDBLKMOD_2 = 2,
    #[doc = "3: LCD_E Blinking mode: Switching"]
    LCDBLKMOD_3 = 3,
}
impl From<LCDBLKMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDBLKMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCDBLKMOD`"]
pub type LCDBLKMOD_R = crate::R<u8, LCDBLKMOD_A>;
impl LCDBLKMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKMOD_A {
        match self.bits {
            0 => LCDBLKMOD_A::LCDBLKMOD_0,
            1 => LCDBLKMOD_A::LCDBLKMOD_1,
            2 => LCDBLKMOD_A::LCDBLKMOD_2,
            3 => LCDBLKMOD_A::LCDBLKMOD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKMOD_0`"]
    #[inline(always)]
    pub fn is_lcdblkmod_0(&self) -> bool {
        *self == LCDBLKMOD_A::LCDBLKMOD_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKMOD_1`"]
    #[inline(always)]
    pub fn is_lcdblkmod_1(&self) -> bool {
        *self == LCDBLKMOD_A::LCDBLKMOD_1
    }
    #[doc = "Checks if the value of the field is `LCDBLKMOD_2`"]
    #[inline(always)]
    pub fn is_lcdblkmod_2(&self) -> bool {
        *self == LCDBLKMOD_A::LCDBLKMOD_2
    }
    #[doc = "Checks if the value of the field is `LCDBLKMOD_3`"]
    #[inline(always)]
    pub fn is_lcdblkmod_3(&self) -> bool {
        *self == LCDBLKMOD_A::LCDBLKMOD_3
    }
}
#[doc = "Write proxy for field `LCDBLKMOD`"]
pub struct LCDBLKMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDBLKMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LCD_E Blinking mode: Off"]
    #[inline(always)]
    pub fn lcdblkmod_0(self) -> &'a mut W {
        self.variant(LCDBLKMOD_A::LCDBLKMOD_0)
    }
    #[doc = "LCD_E Blinking mode: Individual"]
    #[inline(always)]
    pub fn lcdblkmod_1(self) -> &'a mut W {
        self.variant(LCDBLKMOD_A::LCDBLKMOD_1)
    }
    #[doc = "LCD_E Blinking mode: All"]
    #[inline(always)]
    pub fn lcdblkmod_2(self) -> &'a mut W {
        self.variant(LCDBLKMOD_A::LCDBLKMOD_2)
    }
    #[doc = "LCD_E Blinking mode: Switching"]
    #[inline(always)]
    pub fn lcdblkmod_3(self) -> &'a mut W {
        self.variant(LCDBLKMOD_A::LCDBLKMOD_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "LCD_E Clock pre-scaler for blinking frequency Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDBLKPRE_A {
    #[doc = "0: LCD_E Clock pre-scaler for blinking frequency: 0"]
    LCDBLKPRE_0 = 0,
    #[doc = "1: LCD_E Clock pre-scaler for blinking frequency: 1"]
    LCDBLKPRE_1 = 1,
    #[doc = "2: LCD_E Clock pre-scaler for blinking frequency: 2"]
    LCDBLKPRE_2 = 2,
    #[doc = "3: LCD_E Clock pre-scaler for blinking frequency: 3"]
    LCDBLKPRE_3 = 3,
    #[doc = "4: LCD_E Clock pre-scaler for blinking frequency: 4"]
    LCDBLKPRE_4 = 4,
    #[doc = "5: LCD_E Clock pre-scaler for blinking frequency: 5"]
    LCDBLKPRE_5 = 5,
    #[doc = "6: LCD_E Clock pre-scaler for blinking frequency: 6"]
    LCDBLKPRE_6 = 6,
    #[doc = "7: LCD_E Clock pre-scaler for blinking frequency: 7"]
    LCDBLKPRE_7 = 7,
}
impl From<LCDBLKPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDBLKPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCDBLKPRE`"]
pub type LCDBLKPRE_R = crate::R<u8, LCDBLKPRE_A>;
impl LCDBLKPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKPRE_A {
        match self.bits {
            0 => LCDBLKPRE_A::LCDBLKPRE_0,
            1 => LCDBLKPRE_A::LCDBLKPRE_1,
            2 => LCDBLKPRE_A::LCDBLKPRE_2,
            3 => LCDBLKPRE_A::LCDBLKPRE_3,
            4 => LCDBLKPRE_A::LCDBLKPRE_4,
            5 => LCDBLKPRE_A::LCDBLKPRE_5,
            6 => LCDBLKPRE_A::LCDBLKPRE_6,
            7 => LCDBLKPRE_A::LCDBLKPRE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_0`"]
    #[inline(always)]
    pub fn is_lcdblkpre_0(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_1`"]
    #[inline(always)]
    pub fn is_lcdblkpre_1(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_1
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_2`"]
    #[inline(always)]
    pub fn is_lcdblkpre_2(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_2
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_3`"]
    #[inline(always)]
    pub fn is_lcdblkpre_3(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_3
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_4`"]
    #[inline(always)]
    pub fn is_lcdblkpre_4(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_4
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_5`"]
    #[inline(always)]
    pub fn is_lcdblkpre_5(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_5
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_6`"]
    #[inline(always)]
    pub fn is_lcdblkpre_6(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_6
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_7`"]
    #[inline(always)]
    pub fn is_lcdblkpre_7(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_7
    }
}
#[doc = "Write proxy for field `LCDBLKPRE`"]
pub struct LCDBLKPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDBLKPRE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 0"]
    #[inline(always)]
    pub fn lcdblkpre_0(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_0)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 1"]
    #[inline(always)]
    pub fn lcdblkpre_1(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_1)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 2"]
    #[inline(always)]
    pub fn lcdblkpre_2(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_2)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 3"]
    #[inline(always)]
    pub fn lcdblkpre_3(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_3)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 4"]
    #[inline(always)]
    pub fn lcdblkpre_4(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_4)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 5"]
    #[inline(always)]
    pub fn lcdblkpre_5(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_5)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 6"]
    #[inline(always)]
    pub fn lcdblkpre_6(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_6)
    }
    #[doc = "LCD_E Clock pre-scaler for blinking frequency: 7"]
    #[inline(always)]
    pub fn lcdblkpre_7(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u16) & 0x07) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - LCD_E Blinking mode Bit: 0"]
    #[inline(always)]
    pub fn lcdblkmod(&self) -> LCDBLKMOD_R {
        LCDBLKMOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - LCD_E Clock pre-scaler for blinking frequency Bit: 0"]
    #[inline(always)]
    pub fn lcdblkpre(&self) -> LCDBLKPRE_R {
        LCDBLKPRE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LCD_E Blinking mode Bit: 0"]
    #[inline(always)]
    pub fn lcdblkmod(&mut self) -> LCDBLKMOD_W {
        LCDBLKMOD_W { w: self }
    }
    #[doc = "Bits 2:4 - LCD_E Clock pre-scaler for blinking frequency Bit: 0"]
    #[inline(always)]
    pub fn lcdblkpre(&mut self) -> LCDBLKPRE_W {
        LCDBLKPRE_W { w: self }
    }
}
