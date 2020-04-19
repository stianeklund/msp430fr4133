#[doc = "Reader of register CSCTL1"]
pub type R = crate::R<u16, super::CSCTL1>;
#[doc = "Writer for register CSCTL1"]
pub type W = crate::W<u16, super::CSCTL1>;
#[doc = "Register CSCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DISMOD`"]
pub type DISMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISMOD`"]
pub struct DISMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISMOD_W<'a> {
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
#[doc = "DCO frequency range select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCORSEL_A {
    #[doc = "0: DCO frequency range select: 0"]
    DCORSEL_0 = 0,
    #[doc = "1: DCO frequency range select: 1"]
    DCORSEL_1 = 1,
    #[doc = "2: DCO frequency range select: 2"]
    DCORSEL_2 = 2,
    #[doc = "3: DCO frequency range select: 3"]
    DCORSEL_3 = 3,
    #[doc = "4: DCO frequency range select: 4"]
    DCORSEL_4 = 4,
    #[doc = "5: DCO frequency range select: 5"]
    DCORSEL_5 = 5,
    #[doc = "6: DCO frequency range select: 6"]
    DCORSEL_6 = 6,
    #[doc = "7: DCO frequency range select: 7"]
    DCORSEL_7 = 7,
}
impl From<DCORSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCORSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCORSEL`"]
pub type DCORSEL_R = crate::R<u8, DCORSEL_A>;
impl DCORSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCORSEL_A {
        match self.bits {
            0 => DCORSEL_A::DCORSEL_0,
            1 => DCORSEL_A::DCORSEL_1,
            2 => DCORSEL_A::DCORSEL_2,
            3 => DCORSEL_A::DCORSEL_3,
            4 => DCORSEL_A::DCORSEL_4,
            5 => DCORSEL_A::DCORSEL_5,
            6 => DCORSEL_A::DCORSEL_6,
            7 => DCORSEL_A::DCORSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCORSEL_0`"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_0
    }
    #[doc = "Checks if the value of the field is `DCORSEL_1`"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_1
    }
    #[doc = "Checks if the value of the field is `DCORSEL_2`"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_2
    }
    #[doc = "Checks if the value of the field is `DCORSEL_3`"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_3
    }
    #[doc = "Checks if the value of the field is `DCORSEL_4`"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_4
    }
    #[doc = "Checks if the value of the field is `DCORSEL_5`"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_5
    }
    #[doc = "Checks if the value of the field is `DCORSEL_6`"]
    #[inline(always)]
    pub fn is_dcorsel_6(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_6
    }
    #[doc = "Checks if the value of the field is `DCORSEL_7`"]
    #[inline(always)]
    pub fn is_dcorsel_7(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_7
    }
}
#[doc = "Write proxy for field `DCORSEL`"]
pub struct DCORSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCORSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCORSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DCO frequency range select: 0"]
    #[inline(always)]
    pub fn dcorsel_0(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_0)
    }
    #[doc = "DCO frequency range select: 1"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_1)
    }
    #[doc = "DCO frequency range select: 2"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_2)
    }
    #[doc = "DCO frequency range select: 3"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_3)
    }
    #[doc = "DCO frequency range select: 4"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_4)
    }
    #[doc = "DCO frequency range select: 5"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_5)
    }
    #[doc = "DCO frequency range select: 6"]
    #[inline(always)]
    pub fn dcorsel_6(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_6)
    }
    #[doc = "DCO frequency range select: 7"]
    #[inline(always)]
    pub fn dcorsel_7(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u16) & 0x07) << 1);
        self.w
    }
}
#[doc = "DCO frequency trim. Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCOFTRIM_A {
    #[doc = "0: DCO frequency trim: 0"]
    DCOFTRIM_0 = 0,
    #[doc = "1: DCO frequency trim: 1"]
    DCOFTRIM_1 = 1,
    #[doc = "2: DCO frequency trim: 2"]
    DCOFTRIM_2 = 2,
    #[doc = "3: DCO frequency trim: 3"]
    DCOFTRIM_3 = 3,
    #[doc = "4: DCO frequency trim: 4"]
    DCOFTRIM_4 = 4,
    #[doc = "5: DCO frequency trim: 5"]
    DCOFTRIM_5 = 5,
    #[doc = "6: DCO frequency trim: 6"]
    DCOFTRIM_6 = 6,
    #[doc = "7: DCO frequency trim: 7"]
    DCOFTRIM_7 = 7,
}
impl From<DCOFTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: DCOFTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCOFTRIM`"]
pub type DCOFTRIM_R = crate::R<u8, DCOFTRIM_A>;
impl DCOFTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOFTRIM_A {
        match self.bits {
            0 => DCOFTRIM_A::DCOFTRIM_0,
            1 => DCOFTRIM_A::DCOFTRIM_1,
            2 => DCOFTRIM_A::DCOFTRIM_2,
            3 => DCOFTRIM_A::DCOFTRIM_3,
            4 => DCOFTRIM_A::DCOFTRIM_4,
            5 => DCOFTRIM_A::DCOFTRIM_5,
            6 => DCOFTRIM_A::DCOFTRIM_6,
            7 => DCOFTRIM_A::DCOFTRIM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_0`"]
    #[inline(always)]
    pub fn is_dcoftrim_0(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_0
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_1`"]
    #[inline(always)]
    pub fn is_dcoftrim_1(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_1
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_2`"]
    #[inline(always)]
    pub fn is_dcoftrim_2(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_2
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_3`"]
    #[inline(always)]
    pub fn is_dcoftrim_3(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_3
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_4`"]
    #[inline(always)]
    pub fn is_dcoftrim_4(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_4
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_5`"]
    #[inline(always)]
    pub fn is_dcoftrim_5(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_5
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_6`"]
    #[inline(always)]
    pub fn is_dcoftrim_6(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_6
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_7`"]
    #[inline(always)]
    pub fn is_dcoftrim_7(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_7
    }
}
#[doc = "Write proxy for field `DCOFTRIM`"]
pub struct DCOFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCOFTRIM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DCO frequency trim: 0"]
    #[inline(always)]
    pub fn dcoftrim_0(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_0)
    }
    #[doc = "DCO frequency trim: 1"]
    #[inline(always)]
    pub fn dcoftrim_1(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_1)
    }
    #[doc = "DCO frequency trim: 2"]
    #[inline(always)]
    pub fn dcoftrim_2(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_2)
    }
    #[doc = "DCO frequency trim: 3"]
    #[inline(always)]
    pub fn dcoftrim_3(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_3)
    }
    #[doc = "DCO frequency trim: 4"]
    #[inline(always)]
    pub fn dcoftrim_4(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_4)
    }
    #[doc = "DCO frequency trim: 5"]
    #[inline(always)]
    pub fn dcoftrim_5(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_5)
    }
    #[doc = "DCO frequency trim: 6"]
    #[inline(always)]
    pub fn dcoftrim_6(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_6)
    }
    #[doc = "DCO frequency trim: 7"]
    #[inline(always)]
    pub fn dcoftrim_7(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DCOFTRIMEN`"]
pub type DCOFTRIMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCOFTRIMEN`"]
pub struct DCOFTRIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFTRIMEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline(always)]
    pub fn dismod(&self) -> DISMOD_R {
        DISMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - DCO frequency range select Bit: 0"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DCORSEL_R {
        DCORSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. Bit: 0"]
    #[inline(always)]
    pub fn dcoftrim(&self) -> DCOFTRIM_R {
        DCOFTRIM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - DCO frequency trim enable"]
    #[inline(always)]
    pub fn dcoftrimen(&self) -> DCOFTRIMEN_R {
        DCOFTRIMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline(always)]
    pub fn dismod(&mut self) -> DISMOD_W {
        DISMOD_W { w: self }
    }
    #[doc = "Bits 1:3 - DCO frequency range select Bit: 0"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DCORSEL_W {
        DCORSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - DCO frequency trim. Bit: 0"]
    #[inline(always)]
    pub fn dcoftrim(&mut self) -> DCOFTRIM_W {
        DCOFTRIM_W { w: self }
    }
    #[doc = "Bit 7 - DCO frequency trim enable"]
    #[inline(always)]
    pub fn dcoftrimen(&mut self) -> DCOFTRIMEN_W {
        DCOFTRIMEN_W { w: self }
    }
}
