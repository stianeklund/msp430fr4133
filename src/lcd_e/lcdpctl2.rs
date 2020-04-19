#[doc = "Reader of register LCDPCTL2"]
pub type R = crate::R<u16, super::LCDPCTL2>;
#[doc = "Writer for register LCDPCTL2"]
pub type W = crate::W<u16, super::LCDPCTL2>;
#[doc = "Register LCDPCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDPCTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDS32`"]
pub type LCDS32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS32`"]
pub struct LCDS32_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS32_W<'a> {
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
#[doc = "Reader of field `LCDS33`"]
pub type LCDS33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS33`"]
pub struct LCDS33_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS33_W<'a> {
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
#[doc = "Reader of field `LCDS34`"]
pub type LCDS34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS34`"]
pub struct LCDS34_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS34_W<'a> {
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
#[doc = "Reader of field `LCDS35`"]
pub type LCDS35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS35`"]
pub struct LCDS35_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS35_W<'a> {
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
#[doc = "Reader of field `LCDS36`"]
pub type LCDS36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS36`"]
pub struct LCDS36_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS36_W<'a> {
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
#[doc = "Reader of field `LCDS37`"]
pub type LCDS37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS37`"]
pub struct LCDS37_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS37_W<'a> {
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
#[doc = "Reader of field `LCDS38`"]
pub type LCDS38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS38`"]
pub struct LCDS38_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS38_W<'a> {
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
#[doc = "Reader of field `LCDS39`"]
pub type LCDS39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS39`"]
pub struct LCDS39_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS39_W<'a> {
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
#[doc = "Reader of field `LCDS40`"]
pub type LCDS40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS40`"]
pub struct LCDS40_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS40_W<'a> {
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
#[doc = "Reader of field `LCDS41`"]
pub type LCDS41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS41`"]
pub struct LCDS41_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS41_W<'a> {
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
#[doc = "Reader of field `LCDS42`"]
pub type LCDS42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS42`"]
pub struct LCDS42_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS42_W<'a> {
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
#[doc = "Reader of field `LCDS43`"]
pub type LCDS43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS43`"]
pub struct LCDS43_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS43_W<'a> {
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
#[doc = "Reader of field `LCDS44`"]
pub type LCDS44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS44`"]
pub struct LCDS44_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS44_W<'a> {
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
#[doc = "Reader of field `LCDS45`"]
pub type LCDS45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS45`"]
pub struct LCDS45_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS45_W<'a> {
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
#[doc = "Reader of field `LCDS46`"]
pub type LCDS46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS46`"]
pub struct LCDS46_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS46_W<'a> {
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
#[doc = "Reader of field `LCDS47`"]
pub type LCDS47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS47`"]
pub struct LCDS47_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS47_W<'a> {
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
    #[doc = "Bit 0 - LCD Segment 32 enable."]
    #[inline(always)]
    pub fn lcds32(&self) -> LCDS32_R {
        LCDS32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 33 enable."]
    #[inline(always)]
    pub fn lcds33(&self) -> LCDS33_R {
        LCDS33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 34 enable."]
    #[inline(always)]
    pub fn lcds34(&self) -> LCDS34_R {
        LCDS34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 35 enable."]
    #[inline(always)]
    pub fn lcds35(&self) -> LCDS35_R {
        LCDS35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 36 enable."]
    #[inline(always)]
    pub fn lcds36(&self) -> LCDS36_R {
        LCDS36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 37 enable."]
    #[inline(always)]
    pub fn lcds37(&self) -> LCDS37_R {
        LCDS37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 38 enable."]
    #[inline(always)]
    pub fn lcds38(&self) -> LCDS38_R {
        LCDS38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 39 enable."]
    #[inline(always)]
    pub fn lcds39(&self) -> LCDS39_R {
        LCDS39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 40 enable."]
    #[inline(always)]
    pub fn lcds40(&self) -> LCDS40_R {
        LCDS40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 41 enable."]
    #[inline(always)]
    pub fn lcds41(&self) -> LCDS41_R {
        LCDS41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 42 enable."]
    #[inline(always)]
    pub fn lcds42(&self) -> LCDS42_R {
        LCDS42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 43 enable."]
    #[inline(always)]
    pub fn lcds43(&self) -> LCDS43_R {
        LCDS43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 44 enable."]
    #[inline(always)]
    pub fn lcds44(&self) -> LCDS44_R {
        LCDS44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 45 enable."]
    #[inline(always)]
    pub fn lcds45(&self) -> LCDS45_R {
        LCDS45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 46 enable."]
    #[inline(always)]
    pub fn lcds46(&self) -> LCDS46_R {
        LCDS46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 47 enable."]
    #[inline(always)]
    pub fn lcds47(&self) -> LCDS47_R {
        LCDS47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 32 enable."]
    #[inline(always)]
    pub fn lcds32(&mut self) -> LCDS32_W {
        LCDS32_W { w: self }
    }
    #[doc = "Bit 1 - LCD Segment 33 enable."]
    #[inline(always)]
    pub fn lcds33(&mut self) -> LCDS33_W {
        LCDS33_W { w: self }
    }
    #[doc = "Bit 2 - LCD Segment 34 enable."]
    #[inline(always)]
    pub fn lcds34(&mut self) -> LCDS34_W {
        LCDS34_W { w: self }
    }
    #[doc = "Bit 3 - LCD Segment 35 enable."]
    #[inline(always)]
    pub fn lcds35(&mut self) -> LCDS35_W {
        LCDS35_W { w: self }
    }
    #[doc = "Bit 4 - LCD Segment 36 enable."]
    #[inline(always)]
    pub fn lcds36(&mut self) -> LCDS36_W {
        LCDS36_W { w: self }
    }
    #[doc = "Bit 5 - LCD Segment 37 enable."]
    #[inline(always)]
    pub fn lcds37(&mut self) -> LCDS37_W {
        LCDS37_W { w: self }
    }
    #[doc = "Bit 6 - LCD Segment 38 enable."]
    #[inline(always)]
    pub fn lcds38(&mut self) -> LCDS38_W {
        LCDS38_W { w: self }
    }
    #[doc = "Bit 7 - LCD Segment 39 enable."]
    #[inline(always)]
    pub fn lcds39(&mut self) -> LCDS39_W {
        LCDS39_W { w: self }
    }
    #[doc = "Bit 8 - LCD Segment 40 enable."]
    #[inline(always)]
    pub fn lcds40(&mut self) -> LCDS40_W {
        LCDS40_W { w: self }
    }
    #[doc = "Bit 9 - LCD Segment 41 enable."]
    #[inline(always)]
    pub fn lcds41(&mut self) -> LCDS41_W {
        LCDS41_W { w: self }
    }
    #[doc = "Bit 10 - LCD Segment 42 enable."]
    #[inline(always)]
    pub fn lcds42(&mut self) -> LCDS42_W {
        LCDS42_W { w: self }
    }
    #[doc = "Bit 11 - LCD Segment 43 enable."]
    #[inline(always)]
    pub fn lcds43(&mut self) -> LCDS43_W {
        LCDS43_W { w: self }
    }
    #[doc = "Bit 12 - LCD Segment 44 enable."]
    #[inline(always)]
    pub fn lcds44(&mut self) -> LCDS44_W {
        LCDS44_W { w: self }
    }
    #[doc = "Bit 13 - LCD Segment 45 enable."]
    #[inline(always)]
    pub fn lcds45(&mut self) -> LCDS45_W {
        LCDS45_W { w: self }
    }
    #[doc = "Bit 14 - LCD Segment 46 enable."]
    #[inline(always)]
    pub fn lcds46(&mut self) -> LCDS46_W {
        LCDS46_W { w: self }
    }
    #[doc = "Bit 15 - LCD Segment 47 enable."]
    #[inline(always)]
    pub fn lcds47(&mut self) -> LCDS47_W {
        LCDS47_W { w: self }
    }
}
