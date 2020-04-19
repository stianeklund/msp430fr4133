#[doc = "Reader of register LCDCSSEL0"]
pub type R = crate::R<u16, super::LCDCSSEL0>;
#[doc = "Writer for register LCDCSSEL0"]
pub type W = crate::W<u16, super::LCDCSSEL0>;
#[doc = "Register LCDCSSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDCSSEL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDCSS0`"]
pub type LCDCSS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS0`"]
pub struct LCDCSS0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS0_W<'a> {
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
#[doc = "Reader of field `LCDCSS1`"]
pub type LCDCSS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS1`"]
pub struct LCDCSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS1_W<'a> {
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
#[doc = "Reader of field `LCDCSS2`"]
pub type LCDCSS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS2`"]
pub struct LCDCSS2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS2_W<'a> {
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
#[doc = "Reader of field `LCDCSS3`"]
pub type LCDCSS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS3`"]
pub struct LCDCSS3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS3_W<'a> {
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
#[doc = "Reader of field `LCDCSS4`"]
pub type LCDCSS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS4`"]
pub struct LCDCSS4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS4_W<'a> {
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
#[doc = "Reader of field `LCDCSS5`"]
pub type LCDCSS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS5`"]
pub struct LCDCSS5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS5_W<'a> {
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
#[doc = "Reader of field `LCDCSS6`"]
pub type LCDCSS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS6`"]
pub struct LCDCSS6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS6_W<'a> {
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
#[doc = "Reader of field `LCDCSS7`"]
pub type LCDCSS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS7`"]
pub struct LCDCSS7_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS7_W<'a> {
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
#[doc = "Reader of field `LCDCSS8`"]
pub type LCDCSS8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS8`"]
pub struct LCDCSS8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS8_W<'a> {
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
#[doc = "Reader of field `LCDCSS9`"]
pub type LCDCSS9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS9`"]
pub struct LCDCSS9_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS9_W<'a> {
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
#[doc = "Reader of field `LCDCSS10`"]
pub type LCDCSS10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS10`"]
pub struct LCDCSS10_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS10_W<'a> {
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
#[doc = "Reader of field `LCDCSS11`"]
pub type LCDCSS11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS11`"]
pub struct LCDCSS11_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS11_W<'a> {
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
#[doc = "Reader of field `LCDCSS12`"]
pub type LCDCSS12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS12`"]
pub struct LCDCSS12_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS12_W<'a> {
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
#[doc = "Reader of field `LCDCSS13`"]
pub type LCDCSS13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS13`"]
pub struct LCDCSS13_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS13_W<'a> {
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
#[doc = "Reader of field `LCDCSS14`"]
pub type LCDCSS14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS14`"]
pub struct LCDCSS14_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS14_W<'a> {
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
#[doc = "Reader of field `LCDCSS15`"]
pub type LCDCSS15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS15`"]
pub struct LCDCSS15_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS15_W<'a> {
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
    #[doc = "Bit 0 - Selects pin L0 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss0(&self) -> LCDCSS0_R {
        LCDCSS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects pin L1 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss1(&self) -> LCDCSS1_R {
        LCDCSS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects pin L2 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss2(&self) -> LCDCSS2_R {
        LCDCSS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects pin L3 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss3(&self) -> LCDCSS3_R {
        LCDCSS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects pin L4 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss4(&self) -> LCDCSS4_R {
        LCDCSS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects pin L5 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss5(&self) -> LCDCSS5_R {
        LCDCSS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects pin L6 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss6(&self) -> LCDCSS6_R {
        LCDCSS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects pin L7 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss7(&self) -> LCDCSS7_R {
        LCDCSS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects pin L8 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss8(&self) -> LCDCSS8_R {
        LCDCSS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects pin L9 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss9(&self) -> LCDCSS9_R {
        LCDCSS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects pin L10 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss10(&self) -> LCDCSS10_R {
        LCDCSS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects pin L11 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss11(&self) -> LCDCSS11_R {
        LCDCSS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects pin L12 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss12(&self) -> LCDCSS12_R {
        LCDCSS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects pin L13 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss13(&self) -> LCDCSS13_R {
        LCDCSS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects pin L14 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss14(&self) -> LCDCSS14_R {
        LCDCSS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects pin L15 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss15(&self) -> LCDCSS15_R {
        LCDCSS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L0 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss0(&mut self) -> LCDCSS0_W {
        LCDCSS0_W { w: self }
    }
    #[doc = "Bit 1 - Selects pin L1 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss1(&mut self) -> LCDCSS1_W {
        LCDCSS1_W { w: self }
    }
    #[doc = "Bit 2 - Selects pin L2 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss2(&mut self) -> LCDCSS2_W {
        LCDCSS2_W { w: self }
    }
    #[doc = "Bit 3 - Selects pin L3 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss3(&mut self) -> LCDCSS3_W {
        LCDCSS3_W { w: self }
    }
    #[doc = "Bit 4 - Selects pin L4 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss4(&mut self) -> LCDCSS4_W {
        LCDCSS4_W { w: self }
    }
    #[doc = "Bit 5 - Selects pin L5 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss5(&mut self) -> LCDCSS5_W {
        LCDCSS5_W { w: self }
    }
    #[doc = "Bit 6 - Selects pin L6 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss6(&mut self) -> LCDCSS6_W {
        LCDCSS6_W { w: self }
    }
    #[doc = "Bit 7 - Selects pin L7 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss7(&mut self) -> LCDCSS7_W {
        LCDCSS7_W { w: self }
    }
    #[doc = "Bit 8 - Selects pin L8 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss8(&mut self) -> LCDCSS8_W {
        LCDCSS8_W { w: self }
    }
    #[doc = "Bit 9 - Selects pin L9 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss9(&mut self) -> LCDCSS9_W {
        LCDCSS9_W { w: self }
    }
    #[doc = "Bit 10 - Selects pin L10 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss10(&mut self) -> LCDCSS10_W {
        LCDCSS10_W { w: self }
    }
    #[doc = "Bit 11 - Selects pin L11 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss11(&mut self) -> LCDCSS11_W {
        LCDCSS11_W { w: self }
    }
    #[doc = "Bit 12 - Selects pin L12 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss12(&mut self) -> LCDCSS12_W {
        LCDCSS12_W { w: self }
    }
    #[doc = "Bit 13 - Selects pin L13 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss13(&mut self) -> LCDCSS13_W {
        LCDCSS13_W { w: self }
    }
    #[doc = "Bit 14 - Selects pin L14 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss14(&mut self) -> LCDCSS14_W {
        LCDCSS14_W { w: self }
    }
    #[doc = "Bit 15 - Selects pin L15 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss15(&mut self) -> LCDCSS15_W {
        LCDCSS15_W { w: self }
    }
}
