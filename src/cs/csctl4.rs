#[doc = "Reader of register CSCTL4"]
pub type R = crate::R<u16, super::CSCTL4>;
#[doc = "Writer for register CSCTL4"]
pub type W = crate::W<u16, super::CSCTL4>;
#[doc = "Register CSCTL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCLK and SMCLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELMS_A {
    #[doc = "0: MCLK and SMCLK Source Select 0"]
    SELMS_0 = 0,
    #[doc = "1: MCLK and SMCLK Source Select 1"]
    SELMS_1 = 1,
    #[doc = "2: MCLK and SMCLK Source Select 2"]
    SELMS_2 = 2,
    #[doc = "3: MCLK and SMCLK Source Select 3"]
    SELMS_3 = 3,
    #[doc = "4: MCLK and SMCLK Source Select 4"]
    SELMS_4 = 4,
    #[doc = "5: MCLK and SMCLK Source Select 5"]
    SELMS_5 = 5,
    #[doc = "6: MCLK and SMCLK Source Select 6"]
    SELMS_6 = 6,
    #[doc = "7: MCLK and SMCLK Source Select 7"]
    SELMS_7 = 7,
}
impl From<SELMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELMS`"]
pub type SELMS_R = crate::R<u8, SELMS_A>;
impl SELMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELMS_A {
        match self.bits {
            0 => SELMS_A::SELMS_0,
            1 => SELMS_A::SELMS_1,
            2 => SELMS_A::SELMS_2,
            3 => SELMS_A::SELMS_3,
            4 => SELMS_A::SELMS_4,
            5 => SELMS_A::SELMS_5,
            6 => SELMS_A::SELMS_6,
            7 => SELMS_A::SELMS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELMS_0`"]
    #[inline(always)]
    pub fn is_selms_0(&self) -> bool {
        *self == SELMS_A::SELMS_0
    }
    #[doc = "Checks if the value of the field is `SELMS_1`"]
    #[inline(always)]
    pub fn is_selms_1(&self) -> bool {
        *self == SELMS_A::SELMS_1
    }
    #[doc = "Checks if the value of the field is `SELMS_2`"]
    #[inline(always)]
    pub fn is_selms_2(&self) -> bool {
        *self == SELMS_A::SELMS_2
    }
    #[doc = "Checks if the value of the field is `SELMS_3`"]
    #[inline(always)]
    pub fn is_selms_3(&self) -> bool {
        *self == SELMS_A::SELMS_3
    }
    #[doc = "Checks if the value of the field is `SELMS_4`"]
    #[inline(always)]
    pub fn is_selms_4(&self) -> bool {
        *self == SELMS_A::SELMS_4
    }
    #[doc = "Checks if the value of the field is `SELMS_5`"]
    #[inline(always)]
    pub fn is_selms_5(&self) -> bool {
        *self == SELMS_A::SELMS_5
    }
    #[doc = "Checks if the value of the field is `SELMS_6`"]
    #[inline(always)]
    pub fn is_selms_6(&self) -> bool {
        *self == SELMS_A::SELMS_6
    }
    #[doc = "Checks if the value of the field is `SELMS_7`"]
    #[inline(always)]
    pub fn is_selms_7(&self) -> bool {
        *self == SELMS_A::SELMS_7
    }
}
#[doc = "Write proxy for field `SELMS`"]
pub struct SELMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELMS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MCLK and SMCLK Source Select 0"]
    #[inline(always)]
    pub fn selms_0(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_0)
    }
    #[doc = "MCLK and SMCLK Source Select 1"]
    #[inline(always)]
    pub fn selms_1(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_1)
    }
    #[doc = "MCLK and SMCLK Source Select 2"]
    #[inline(always)]
    pub fn selms_2(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_2)
    }
    #[doc = "MCLK and SMCLK Source Select 3"]
    #[inline(always)]
    pub fn selms_3(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_3)
    }
    #[doc = "MCLK and SMCLK Source Select 4"]
    #[inline(always)]
    pub fn selms_4(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_4)
    }
    #[doc = "MCLK and SMCLK Source Select 5"]
    #[inline(always)]
    pub fn selms_5(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_5)
    }
    #[doc = "MCLK and SMCLK Source Select 6"]
    #[inline(always)]
    pub fn selms_6(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_6)
    }
    #[doc = "MCLK and SMCLK Source Select 7"]
    #[inline(always)]
    pub fn selms_7(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SELA`"]
pub type SELA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELA`"]
pub struct SELA_W<'a> {
    w: &'a mut W,
}
impl<'a> SELA_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selms(&self) -> SELMS_R {
        SELMS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&self) -> SELA_R {
        SELA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selms(&mut self) -> SELMS_W {
        SELMS_W { w: self }
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&mut self) -> SELA_W {
        SELA_W { w: self }
    }
}
