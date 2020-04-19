#[doc = "Reader of register LCDVCTL"]
pub type R = crate::R<u16, super::LCDVCTL>;
#[doc = "Writer for register LCDVCTL"]
pub type W = crate::W<u16, super::LCDVCTL>;
#[doc = "Register LCDVCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDVCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDREFMODE`"]
pub type LCDREFMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDREFMODE`"]
pub struct LCDREFMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDREFMODE_W<'a> {
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
#[doc = "Reader of field `LCDSELVDD`"]
pub type LCDSELVDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDSELVDD`"]
pub struct LCDSELVDD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDSELVDD_W<'a> {
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
#[doc = "Reader of field `LCDREFEN`"]
pub type LCDREFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDREFEN`"]
pub struct LCDREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDREFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LCDCPEN`"]
pub type LCDCPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPEN`"]
pub struct LCDCPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPEN_W<'a> {
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
#[doc = "VLCD select: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VLCD_A {
    #[doc = "0: VLCD = 2.60V"]
    VLCD_0 = 0,
    #[doc = "1: VLCD = 2.66V"]
    VLCD_1 = 1,
    #[doc = "2: VLCD = 2.72V"]
    VLCD_2 = 2,
    #[doc = "3: VLCD = 2.78V"]
    VLCD_3 = 3,
    #[doc = "4: VLCD = 2.84V"]
    VLCD_4 = 4,
    #[doc = "5: VLCD = 2.90V"]
    VLCD_5 = 5,
    #[doc = "6: VLCD = 2.96V"]
    VLCD_6 = 6,
    #[doc = "7: VLCD = 3.02V"]
    VLCD_7 = 7,
    #[doc = "8: VLCD = 3.08V"]
    VLCD_8 = 8,
    #[doc = "9: VLCD = 3.14V"]
    VLCD_9 = 9,
    #[doc = "10: VLCD = 3.20V"]
    VLCD_10 = 10,
    #[doc = "11: VLCD = 3.26V"]
    VLCD_11 = 11,
    #[doc = "12: VLCD = 3.32V"]
    VLCD_12 = 12,
    #[doc = "13: VLCD = 3.38V"]
    VLCD_13 = 13,
    #[doc = "14: VLCD = 3.44V"]
    VLCD_14 = 14,
    #[doc = "15: VLCD = 3.50V"]
    VLCD_15 = 15,
}
impl From<VLCD_A> for u8 {
    #[inline(always)]
    fn from(variant: VLCD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VLCD`"]
pub type VLCD_R = crate::R<u8, VLCD_A>;
impl VLCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLCD_A {
        match self.bits {
            0 => VLCD_A::VLCD_0,
            1 => VLCD_A::VLCD_1,
            2 => VLCD_A::VLCD_2,
            3 => VLCD_A::VLCD_3,
            4 => VLCD_A::VLCD_4,
            5 => VLCD_A::VLCD_5,
            6 => VLCD_A::VLCD_6,
            7 => VLCD_A::VLCD_7,
            8 => VLCD_A::VLCD_8,
            9 => VLCD_A::VLCD_9,
            10 => VLCD_A::VLCD_10,
            11 => VLCD_A::VLCD_11,
            12 => VLCD_A::VLCD_12,
            13 => VLCD_A::VLCD_13,
            14 => VLCD_A::VLCD_14,
            15 => VLCD_A::VLCD_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VLCD_0`"]
    #[inline(always)]
    pub fn is_vlcd_0(&self) -> bool {
        *self == VLCD_A::VLCD_0
    }
    #[doc = "Checks if the value of the field is `VLCD_1`"]
    #[inline(always)]
    pub fn is_vlcd_1(&self) -> bool {
        *self == VLCD_A::VLCD_1
    }
    #[doc = "Checks if the value of the field is `VLCD_2`"]
    #[inline(always)]
    pub fn is_vlcd_2(&self) -> bool {
        *self == VLCD_A::VLCD_2
    }
    #[doc = "Checks if the value of the field is `VLCD_3`"]
    #[inline(always)]
    pub fn is_vlcd_3(&self) -> bool {
        *self == VLCD_A::VLCD_3
    }
    #[doc = "Checks if the value of the field is `VLCD_4`"]
    #[inline(always)]
    pub fn is_vlcd_4(&self) -> bool {
        *self == VLCD_A::VLCD_4
    }
    #[doc = "Checks if the value of the field is `VLCD_5`"]
    #[inline(always)]
    pub fn is_vlcd_5(&self) -> bool {
        *self == VLCD_A::VLCD_5
    }
    #[doc = "Checks if the value of the field is `VLCD_6`"]
    #[inline(always)]
    pub fn is_vlcd_6(&self) -> bool {
        *self == VLCD_A::VLCD_6
    }
    #[doc = "Checks if the value of the field is `VLCD_7`"]
    #[inline(always)]
    pub fn is_vlcd_7(&self) -> bool {
        *self == VLCD_A::VLCD_7
    }
    #[doc = "Checks if the value of the field is `VLCD_8`"]
    #[inline(always)]
    pub fn is_vlcd_8(&self) -> bool {
        *self == VLCD_A::VLCD_8
    }
    #[doc = "Checks if the value of the field is `VLCD_9`"]
    #[inline(always)]
    pub fn is_vlcd_9(&self) -> bool {
        *self == VLCD_A::VLCD_9
    }
    #[doc = "Checks if the value of the field is `VLCD_10`"]
    #[inline(always)]
    pub fn is_vlcd_10(&self) -> bool {
        *self == VLCD_A::VLCD_10
    }
    #[doc = "Checks if the value of the field is `VLCD_11`"]
    #[inline(always)]
    pub fn is_vlcd_11(&self) -> bool {
        *self == VLCD_A::VLCD_11
    }
    #[doc = "Checks if the value of the field is `VLCD_12`"]
    #[inline(always)]
    pub fn is_vlcd_12(&self) -> bool {
        *self == VLCD_A::VLCD_12
    }
    #[doc = "Checks if the value of the field is `VLCD_13`"]
    #[inline(always)]
    pub fn is_vlcd_13(&self) -> bool {
        *self == VLCD_A::VLCD_13
    }
    #[doc = "Checks if the value of the field is `VLCD_14`"]
    #[inline(always)]
    pub fn is_vlcd_14(&self) -> bool {
        *self == VLCD_A::VLCD_14
    }
    #[doc = "Checks if the value of the field is `VLCD_15`"]
    #[inline(always)]
    pub fn is_vlcd_15(&self) -> bool {
        *self == VLCD_A::VLCD_15
    }
}
#[doc = "Write proxy for field `VLCD`"]
pub struct VLCD_W<'a> {
    w: &'a mut W,
}
impl<'a> VLCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLCD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "VLCD = 2.60V"]
    #[inline(always)]
    pub fn vlcd_0(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_0)
    }
    #[doc = "VLCD = 2.66V"]
    #[inline(always)]
    pub fn vlcd_1(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_1)
    }
    #[doc = "VLCD = 2.72V"]
    #[inline(always)]
    pub fn vlcd_2(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_2)
    }
    #[doc = "VLCD = 2.78V"]
    #[inline(always)]
    pub fn vlcd_3(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_3)
    }
    #[doc = "VLCD = 2.84V"]
    #[inline(always)]
    pub fn vlcd_4(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_4)
    }
    #[doc = "VLCD = 2.90V"]
    #[inline(always)]
    pub fn vlcd_5(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_5)
    }
    #[doc = "VLCD = 2.96V"]
    #[inline(always)]
    pub fn vlcd_6(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_6)
    }
    #[doc = "VLCD = 3.02V"]
    #[inline(always)]
    pub fn vlcd_7(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_7)
    }
    #[doc = "VLCD = 3.08V"]
    #[inline(always)]
    pub fn vlcd_8(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_8)
    }
    #[doc = "VLCD = 3.14V"]
    #[inline(always)]
    pub fn vlcd_9(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_9)
    }
    #[doc = "VLCD = 3.20V"]
    #[inline(always)]
    pub fn vlcd_10(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_10)
    }
    #[doc = "VLCD = 3.26V"]
    #[inline(always)]
    pub fn vlcd_11(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_11)
    }
    #[doc = "VLCD = 3.32V"]
    #[inline(always)]
    pub fn vlcd_12(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_12)
    }
    #[doc = "VLCD = 3.38V"]
    #[inline(always)]
    pub fn vlcd_13(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_13)
    }
    #[doc = "VLCD = 3.44V"]
    #[inline(always)]
    pub fn vlcd_14(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_14)
    }
    #[doc = "VLCD = 3.50V"]
    #[inline(always)]
    pub fn vlcd_15(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `LCDCPFSEL0`"]
pub type LCDCPFSEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPFSEL0`"]
pub struct LCDCPFSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPFSEL0_W<'a> {
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
#[doc = "Reader of field `LCDCPFSEL1`"]
pub type LCDCPFSEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPFSEL1`"]
pub struct LCDCPFSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPFSEL1_W<'a> {
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
#[doc = "Reader of field `LCDCPFSEL2`"]
pub type LCDCPFSEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPFSEL2`"]
pub struct LCDCPFSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPFSEL2_W<'a> {
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
#[doc = "Reader of field `LCDCPFSEL3`"]
pub type LCDCPFSEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPFSEL3`"]
pub struct LCDCPFSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPFSEL3_W<'a> {
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
    #[doc = "Bit 0 - Selects wether R13 voltage is switched or in static mode"]
    #[inline(always)]
    pub fn lcdrefmode(&self) -> LCDREFMODE_R {
        LCDREFMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - selects if R33 is supplied either from Vcc internally or from charge pump"]
    #[inline(always)]
    pub fn lcdselvdd(&self) -> LCDSELVDD_R {
        LCDSELVDD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Internal reference voltage enable on R13"]
    #[inline(always)]
    pub fn lcdrefen(&self) -> LCDREFEN_R {
        LCDREFEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Charge pump enable"]
    #[inline(always)]
    pub fn lcdcpen(&self) -> LCDCPEN_R {
        LCDCPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - VLCD select: 0"]
    #[inline(always)]
    pub fn vlcd(&self) -> VLCD_R {
        VLCD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Charge pump frequency selection Bit: 0"]
    #[inline(always)]
    pub fn lcdcpfsel0(&self) -> LCDCPFSEL0_R {
        LCDCPFSEL0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Charge pump frequency selection Bit: 1"]
    #[inline(always)]
    pub fn lcdcpfsel1(&self) -> LCDCPFSEL1_R {
        LCDCPFSEL1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Charge pump frequency selection Bit: 2"]
    #[inline(always)]
    pub fn lcdcpfsel2(&self) -> LCDCPFSEL2_R {
        LCDCPFSEL2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Charge pump frequency selection Bit: 3"]
    #[inline(always)]
    pub fn lcdcpfsel3(&self) -> LCDCPFSEL3_R {
        LCDCPFSEL3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects wether R13 voltage is switched or in static mode"]
    #[inline(always)]
    pub fn lcdrefmode(&mut self) -> LCDREFMODE_W {
        LCDREFMODE_W { w: self }
    }
    #[doc = "Bit 5 - selects if R33 is supplied either from Vcc internally or from charge pump"]
    #[inline(always)]
    pub fn lcdselvdd(&mut self) -> LCDSELVDD_W {
        LCDSELVDD_W { w: self }
    }
    #[doc = "Bit 6 - Internal reference voltage enable on R13"]
    #[inline(always)]
    pub fn lcdrefen(&mut self) -> LCDREFEN_W {
        LCDREFEN_W { w: self }
    }
    #[doc = "Bit 7 - Charge pump enable"]
    #[inline(always)]
    pub fn lcdcpen(&mut self) -> LCDCPEN_W {
        LCDCPEN_W { w: self }
    }
    #[doc = "Bits 8:11 - VLCD select: 0"]
    #[inline(always)]
    pub fn vlcd(&mut self) -> VLCD_W {
        VLCD_W { w: self }
    }
    #[doc = "Bit 12 - Charge pump frequency selection Bit: 0"]
    #[inline(always)]
    pub fn lcdcpfsel0(&mut self) -> LCDCPFSEL0_W {
        LCDCPFSEL0_W { w: self }
    }
    #[doc = "Bit 13 - Charge pump frequency selection Bit: 1"]
    #[inline(always)]
    pub fn lcdcpfsel1(&mut self) -> LCDCPFSEL1_W {
        LCDCPFSEL1_W { w: self }
    }
    #[doc = "Bit 14 - Charge pump frequency selection Bit: 2"]
    #[inline(always)]
    pub fn lcdcpfsel2(&mut self) -> LCDCPFSEL2_W {
        LCDCPFSEL2_W { w: self }
    }
    #[doc = "Bit 15 - Charge pump frequency selection Bit: 3"]
    #[inline(always)]
    pub fn lcdcpfsel3(&mut self) -> LCDCPFSEL3_W {
        LCDCPFSEL3_W { w: self }
    }
}
