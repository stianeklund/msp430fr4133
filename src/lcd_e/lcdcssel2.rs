#[doc = "Reader of register LCDCSSEL2"]
pub type R = crate::R<u16, super::LCDCSSEL2>;
#[doc = "Writer for register LCDCSSEL2"]
pub type W = crate::W<u16, super::LCDCSSEL2>;
#[doc = "Register LCDCSSEL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDCSSEL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDCSS32`"]
pub type LCDCSS32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS32`"]
pub struct LCDCSS32_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS32_W<'a> {
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
#[doc = "Reader of field `LCDCSS33`"]
pub type LCDCSS33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS33`"]
pub struct LCDCSS33_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS33_W<'a> {
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
#[doc = "Reader of field `LCDCSS34`"]
pub type LCDCSS34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS34`"]
pub struct LCDCSS34_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS34_W<'a> {
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
#[doc = "Reader of field `LCDCSS35`"]
pub type LCDCSS35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS35`"]
pub struct LCDCSS35_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS35_W<'a> {
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
#[doc = "Reader of field `LCDCSS36`"]
pub type LCDCSS36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS36`"]
pub struct LCDCSS36_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS36_W<'a> {
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
#[doc = "Reader of field `LCDCSS37`"]
pub type LCDCSS37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS37`"]
pub struct LCDCSS37_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS37_W<'a> {
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
#[doc = "Reader of field `LCDCSS38`"]
pub type LCDCSS38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS38`"]
pub struct LCDCSS38_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS38_W<'a> {
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
#[doc = "Reader of field `LCDCSS39`"]
pub type LCDCSS39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS39`"]
pub struct LCDCSS39_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS39_W<'a> {
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
#[doc = "Reader of field `LCDCSS40`"]
pub type LCDCSS40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS40`"]
pub struct LCDCSS40_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS40_W<'a> {
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
#[doc = "Reader of field `LCDCSS41`"]
pub type LCDCSS41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS41`"]
pub struct LCDCSS41_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS41_W<'a> {
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
#[doc = "Reader of field `LCDCSS42`"]
pub type LCDCSS42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS42`"]
pub struct LCDCSS42_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS42_W<'a> {
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
#[doc = "Reader of field `LCDCSS43`"]
pub type LCDCSS43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS43`"]
pub struct LCDCSS43_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS43_W<'a> {
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
#[doc = "Reader of field `LCDCSS44`"]
pub type LCDCSS44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS44`"]
pub struct LCDCSS44_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS44_W<'a> {
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
#[doc = "Reader of field `LCDCSS45`"]
pub type LCDCSS45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS45`"]
pub struct LCDCSS45_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS45_W<'a> {
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
#[doc = "Reader of field `LCDCSS46`"]
pub type LCDCSS46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS46`"]
pub struct LCDCSS46_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS46_W<'a> {
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
#[doc = "Reader of field `LCDCSS47`"]
pub type LCDCSS47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCSS47`"]
pub struct LCDCSS47_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCSS47_W<'a> {
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
    #[doc = "Bit 0 - Selects pin L32 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss32(&self) -> LCDCSS32_R {
        LCDCSS32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects pin L33 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss33(&self) -> LCDCSS33_R {
        LCDCSS33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects pin L34 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss34(&self) -> LCDCSS34_R {
        LCDCSS34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects pin L35 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss35(&self) -> LCDCSS35_R {
        LCDCSS35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects pin L36 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss36(&self) -> LCDCSS36_R {
        LCDCSS36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects pin L37 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss37(&self) -> LCDCSS37_R {
        LCDCSS37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects pin L38 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss38(&self) -> LCDCSS38_R {
        LCDCSS38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects pin L39 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss39(&self) -> LCDCSS39_R {
        LCDCSS39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects pin L40 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss40(&self) -> LCDCSS40_R {
        LCDCSS40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects pin L41 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss41(&self) -> LCDCSS41_R {
        LCDCSS41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects pin L42 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss42(&self) -> LCDCSS42_R {
        LCDCSS42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects pin L43 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss43(&self) -> LCDCSS43_R {
        LCDCSS43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects pin L44 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss44(&self) -> LCDCSS44_R {
        LCDCSS44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects pin L45 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss45(&self) -> LCDCSS45_R {
        LCDCSS45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects pin L46 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss46(&self) -> LCDCSS46_R {
        LCDCSS46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects pin L47 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss47(&self) -> LCDCSS47_R {
        LCDCSS47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin L32 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss32(&mut self) -> LCDCSS32_W {
        LCDCSS32_W { w: self }
    }
    #[doc = "Bit 1 - Selects pin L33 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss33(&mut self) -> LCDCSS33_W {
        LCDCSS33_W { w: self }
    }
    #[doc = "Bit 2 - Selects pin L34 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss34(&mut self) -> LCDCSS34_W {
        LCDCSS34_W { w: self }
    }
    #[doc = "Bit 3 - Selects pin L35 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss35(&mut self) -> LCDCSS35_W {
        LCDCSS35_W { w: self }
    }
    #[doc = "Bit 4 - Selects pin L36 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss36(&mut self) -> LCDCSS36_W {
        LCDCSS36_W { w: self }
    }
    #[doc = "Bit 5 - Selects pin L37 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss37(&mut self) -> LCDCSS37_W {
        LCDCSS37_W { w: self }
    }
    #[doc = "Bit 6 - Selects pin L38 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss38(&mut self) -> LCDCSS38_W {
        LCDCSS38_W { w: self }
    }
    #[doc = "Bit 7 - Selects pin L39 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss39(&mut self) -> LCDCSS39_W {
        LCDCSS39_W { w: self }
    }
    #[doc = "Bit 8 - Selects pin L40 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss40(&mut self) -> LCDCSS40_W {
        LCDCSS40_W { w: self }
    }
    #[doc = "Bit 9 - Selects pin L41 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss41(&mut self) -> LCDCSS41_W {
        LCDCSS41_W { w: self }
    }
    #[doc = "Bit 10 - Selects pin L42 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss42(&mut self) -> LCDCSS42_W {
        LCDCSS42_W { w: self }
    }
    #[doc = "Bit 11 - Selects pin L43 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss43(&mut self) -> LCDCSS43_W {
        LCDCSS43_W { w: self }
    }
    #[doc = "Bit 12 - Selects pin L44 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss44(&mut self) -> LCDCSS44_W {
        LCDCSS44_W { w: self }
    }
    #[doc = "Bit 13 - Selects pin L45 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss45(&mut self) -> LCDCSS45_W {
        LCDCSS45_W { w: self }
    }
    #[doc = "Bit 14 - Selects pin L46 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss46(&mut self) -> LCDCSS46_W {
        LCDCSS46_W { w: self }
    }
    #[doc = "Bit 15 - Selects pin L47 as either common or segment line"]
    #[inline(always)]
    pub fn lcdcss47(&mut self) -> LCDCSS47_W {
        LCDCSS47_W { w: self }
    }
}
