#[doc = "Reader of register LCDCSSEL1"]
pub type R = crate::R<u16, super::LCDCSSEL1>;
#[doc = "Writer for register LCDCSSEL1"]
pub type W = crate::W<u16, super::LCDCSSEL1>;
#[doc = "Register LCDCSSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDCSSEL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDCSS16`"]
pub type LCDCSS16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS16`"]
pub struct LCDCSS16_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS16_W<'a> {
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
#[doc = "Reader of field `LCDCSS17`"]
pub type LCDCSS17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS17`"]
pub struct LCDCSS17_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS17_W<'a> {
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
#[doc = "Reader of field `LCDCSS18`"]
pub type LCDCSS18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS18`"]
pub struct LCDCSS18_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS18_W<'a> {
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
#[doc = "Reader of field `LCDCSS19`"]
pub type LCDCSS19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS19`"]
pub struct LCDCSS19_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS19_W<'a> {
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
#[doc = "Reader of field `LCDCSS20`"]
pub type LCDCSS20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS20`"]
pub struct LCDCSS20_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS20_W<'a> {
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
#[doc = "Reader of field `LCDCSS21`"]
pub type LCDCSS21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS21`"]
pub struct LCDCSS21_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS21_W<'a> {
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
#[doc = "Reader of field `LCDCSS22`"]
pub type LCDCSS22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS22`"]
pub struct LCDCSS22_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS22_W<'a> {
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
#[doc = "Reader of field `LCDCSS23`"]
pub type LCDCSS23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS23`"]
pub struct LCDCSS23_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS23_W<'a> {
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
#[doc = "Reader of field `LCDCSS24`"]
pub type LCDCSS24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS24`"]
pub struct LCDCSS24_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS24_W<'a> {
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
#[doc = "Reader of field `LCDCSS25`"]
pub type LCDCSS25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS25`"]
pub struct LCDCSS25_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS25_W<'a> {
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
#[doc = "Reader of field `LCDCSS26`"]
pub type LCDCSS26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS26`"]
pub struct LCDCSS26_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS26_W<'a> {
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
#[doc = "Reader of field `LCDCSS27`"]
pub type LCDCSS27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS27`"]
pub struct LCDCSS27_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS27_W<'a> {
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
#[doc = "Reader of field `LCDCSS28`"]
pub type LCDCSS28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS28`"]
pub struct LCDCSS28_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS28_W<'a> {
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
#[doc = "Reader of field `LCDCSS29`"]
pub type LCDCSS29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS29`"]
pub struct LCDCSS29_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS29_W<'a> {
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
#[doc = "Reader of field `LCDCSS30`"]
pub type LCDCSS30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS30`"]
pub struct LCDCSS30_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS30_W<'a> {
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
#[doc = "Reader of field `LCDCSS31`"]
pub type LCDCSS31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS31`"]
pub struct LCDCSS31_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS31_W<'a> {
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
    #[doc = "Bit 0 - Selects pin L16 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss16(&self) -> LCDCSS16_R {
        LCDCSS16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects pin L17 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss17(&self) -> LCDCSS17_R {
        LCDCSS17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects pin L18 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss18(&self) -> LCDCSS18_R {
        LCDCSS18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects pin L19 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss19(&self) -> LCDCSS19_R {
        LCDCSS19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects pin L20 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss20(&self) -> LCDCSS20_R {
        LCDCSS20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects pin L21 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss21(&self) -> LCDCSS21_R {
        LCDCSS21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects pin L22 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss22(&self) -> LCDCSS22_R {
        LCDCSS22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects pin L23 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss23(&self) -> LCDCSS23_R {
        LCDCSS23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects pin L24 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss24(&self) -> LCDCSS24_R {
        LCDCSS24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects pin L25 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss25(&self) -> LCDCSS25_R {
        LCDCSS25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects pin L26 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss26(&self) -> LCDCSS26_R {
        LCDCSS26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects pin L27 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss27(&self) -> LCDCSS27_R {
        LCDCSS27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects pin L28 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss28(&self) -> LCDCSS28_R {
        LCDCSS28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects pin L29 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss29(&self) -> LCDCSS29_R {
        LCDCSS29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects pin L30 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss30(&self) -> LCDCSS30_R {
        LCDCSS30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects pin L31 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss31(&self) -> LCDCSS31_R {
        LCDCSS31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L16 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss16(&mut self) -> LCDCSS16_W {
        LCDCSS16_W { w: self }
    }
    #[doc = "Bit 1 - Selects pin L17 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss17(&mut self) -> LCDCSS17_W {
        LCDCSS17_W { w: self }
    }
    #[doc = "Bit 2 - Selects pin L18 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss18(&mut self) -> LCDCSS18_W {
        LCDCSS18_W { w: self }
    }
    #[doc = "Bit 3 - Selects pin L19 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss19(&mut self) -> LCDCSS19_W {
        LCDCSS19_W { w: self }
    }
    #[doc = "Bit 4 - Selects pin L20 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss20(&mut self) -> LCDCSS20_W {
        LCDCSS20_W { w: self }
    }
    #[doc = "Bit 5 - Selects pin L21 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss21(&mut self) -> LCDCSS21_W {
        LCDCSS21_W { w: self }
    }
    #[doc = "Bit 6 - Selects pin L22 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss22(&mut self) -> LCDCSS22_W {
        LCDCSS22_W { w: self }
    }
    #[doc = "Bit 7 - Selects pin L23 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss23(&mut self) -> LCDCSS23_W {
        LCDCSS23_W { w: self }
    }
    #[doc = "Bit 8 - Selects pin L24 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss24(&mut self) -> LCDCSS24_W {
        LCDCSS24_W { w: self }
    }
    #[doc = "Bit 9 - Selects pin L25 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss25(&mut self) -> LCDCSS25_W {
        LCDCSS25_W { w: self }
    }
    #[doc = "Bit 10 - Selects pin L26 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss26(&mut self) -> LCDCSS26_W {
        LCDCSS26_W { w: self }
    }
    #[doc = "Bit 11 - Selects pin L27 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss27(&mut self) -> LCDCSS27_W {
        LCDCSS27_W { w: self }
    }
    #[doc = "Bit 12 - Selects pin L28 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss28(&mut self) -> LCDCSS28_W {
        LCDCSS28_W { w: self }
    }
    #[doc = "Bit 13 - Selects pin L29 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss29(&mut self) -> LCDCSS29_W {
        LCDCSS29_W { w: self }
    }
    #[doc = "Bit 14 - Selects pin L30 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss30(&mut self) -> LCDCSS30_W {
        LCDCSS30_W { w: self }
    }
    #[doc = "Bit 15 - Selects pin L31 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss31(&mut self) -> LCDCSS31_W {
        LCDCSS31_W { w: self }
    }
}
