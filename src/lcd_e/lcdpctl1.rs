#[doc = "Reader of register LCDPCTL1"]
pub type R = crate::R<u16, super::LCDPCTL1>;
#[doc = "Writer for register LCDPCTL1"]
pub type W = crate::W<u16, super::LCDPCTL1>;
#[doc = "Register LCDPCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDPCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDS16`"]
pub type LCDS16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS16`"]
pub struct LCDS16_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS16_W<'a> {
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
#[doc = "Reader of field `LCDS17`"]
pub type LCDS17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS17`"]
pub struct LCDS17_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS17_W<'a> {
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
#[doc = "Reader of field `LCDS18`"]
pub type LCDS18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS18`"]
pub struct LCDS18_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS18_W<'a> {
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
#[doc = "Reader of field `LCDS19`"]
pub type LCDS19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS19`"]
pub struct LCDS19_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS19_W<'a> {
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
#[doc = "Reader of field `LCDS20`"]
pub type LCDS20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS20`"]
pub struct LCDS20_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS20_W<'a> {
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
#[doc = "Reader of field `LCDS21`"]
pub type LCDS21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS21`"]
pub struct LCDS21_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS21_W<'a> {
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
#[doc = "Reader of field `LCDS22`"]
pub type LCDS22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS22`"]
pub struct LCDS22_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS22_W<'a> {
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
#[doc = "Reader of field `LCDS23`"]
pub type LCDS23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS23`"]
pub struct LCDS23_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS23_W<'a> {
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
#[doc = "Reader of field `LCDS24`"]
pub type LCDS24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS24`"]
pub struct LCDS24_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS24_W<'a> {
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
#[doc = "Reader of field `LCDS25`"]
pub type LCDS25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS25`"]
pub struct LCDS25_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS25_W<'a> {
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
#[doc = "Reader of field `LCDS26`"]
pub type LCDS26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS26`"]
pub struct LCDS26_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS26_W<'a> {
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
#[doc = "Reader of field `LCDS27`"]
pub type LCDS27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS27`"]
pub struct LCDS27_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS27_W<'a> {
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
#[doc = "Reader of field `LCDS28`"]
pub type LCDS28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS28`"]
pub struct LCDS28_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS28_W<'a> {
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
#[doc = "Reader of field `LCDS29`"]
pub type LCDS29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS29`"]
pub struct LCDS29_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS29_W<'a> {
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
#[doc = "Reader of field `LCDS30`"]
pub type LCDS30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS30`"]
pub struct LCDS30_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS30_W<'a> {
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
#[doc = "Reader of field `LCDS31`"]
pub type LCDS31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS31`"]
pub struct LCDS31_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS31_W<'a> {
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
    #[doc = "Bit 0 - LCD Segment 16 enable."]
    #[inline(always)]
    pub fn lcds16(&self) -> LCDS16_R {
        LCDS16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 17 enable."]
    #[inline(always)]
    pub fn lcds17(&self) -> LCDS17_R {
        LCDS17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 18 enable."]
    #[inline(always)]
    pub fn lcds18(&self) -> LCDS18_R {
        LCDS18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 19 enable."]
    #[inline(always)]
    pub fn lcds19(&self) -> LCDS19_R {
        LCDS19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 20 enable."]
    #[inline(always)]
    pub fn lcds20(&self) -> LCDS20_R {
        LCDS20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 21 enable."]
    #[inline(always)]
    pub fn lcds21(&self) -> LCDS21_R {
        LCDS21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 22 enable."]
    #[inline(always)]
    pub fn lcds22(&self) -> LCDS22_R {
        LCDS22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 23 enable."]
    #[inline(always)]
    pub fn lcds23(&self) -> LCDS23_R {
        LCDS23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 24 enable."]
    #[inline(always)]
    pub fn lcds24(&self) -> LCDS24_R {
        LCDS24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 25 enable."]
    #[inline(always)]
    pub fn lcds25(&self) -> LCDS25_R {
        LCDS25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 26 enable."]
    #[inline(always)]
    pub fn lcds26(&self) -> LCDS26_R {
        LCDS26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 27 enable."]
    #[inline(always)]
    pub fn lcds27(&self) -> LCDS27_R {
        LCDS27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 28 enable."]
    #[inline(always)]
    pub fn lcds28(&self) -> LCDS28_R {
        LCDS28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 29 enable."]
    #[inline(always)]
    pub fn lcds29(&self) -> LCDS29_R {
        LCDS29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 30 enable."]
    #[inline(always)]
    pub fn lcds30(&self) -> LCDS30_R {
        LCDS30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 31 enable."]
    #[inline(always)]
    pub fn lcds31(&self) -> LCDS31_R {
        LCDS31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 16 enable."]
    #[inline(always)]
    pub fn lcds16(&mut self) -> LCDS16_W {
        LCDS16_W { w: self }
    }
    #[doc = "Bit 1 - LCD Segment 17 enable."]
    #[inline(always)]
    pub fn lcds17(&mut self) -> LCDS17_W {
        LCDS17_W { w: self }
    }
    #[doc = "Bit 2 - LCD Segment 18 enable."]
    #[inline(always)]
    pub fn lcds18(&mut self) -> LCDS18_W {
        LCDS18_W { w: self }
    }
    #[doc = "Bit 3 - LCD Segment 19 enable."]
    #[inline(always)]
    pub fn lcds19(&mut self) -> LCDS19_W {
        LCDS19_W { w: self }
    }
    #[doc = "Bit 4 - LCD Segment 20 enable."]
    #[inline(always)]
    pub fn lcds20(&mut self) -> LCDS20_W {
        LCDS20_W { w: self }
    }
    #[doc = "Bit 5 - LCD Segment 21 enable."]
    #[inline(always)]
    pub fn lcds21(&mut self) -> LCDS21_W {
        LCDS21_W { w: self }
    }
    #[doc = "Bit 6 - LCD Segment 22 enable."]
    #[inline(always)]
    pub fn lcds22(&mut self) -> LCDS22_W {
        LCDS22_W { w: self }
    }
    #[doc = "Bit 7 - LCD Segment 23 enable."]
    #[inline(always)]
    pub fn lcds23(&mut self) -> LCDS23_W {
        LCDS23_W { w: self }
    }
    #[doc = "Bit 8 - LCD Segment 24 enable."]
    #[inline(always)]
    pub fn lcds24(&mut self) -> LCDS24_W {
        LCDS24_W { w: self }
    }
    #[doc = "Bit 9 - LCD Segment 25 enable."]
    #[inline(always)]
    pub fn lcds25(&mut self) -> LCDS25_W {
        LCDS25_W { w: self }
    }
    #[doc = "Bit 10 - LCD Segment 26 enable."]
    #[inline(always)]
    pub fn lcds26(&mut self) -> LCDS26_W {
        LCDS26_W { w: self }
    }
    #[doc = "Bit 11 - LCD Segment 27 enable."]
    #[inline(always)]
    pub fn lcds27(&mut self) -> LCDS27_W {
        LCDS27_W { w: self }
    }
    #[doc = "Bit 12 - LCD Segment 28 enable."]
    #[inline(always)]
    pub fn lcds28(&mut self) -> LCDS28_W {
        LCDS28_W { w: self }
    }
    #[doc = "Bit 13 - LCD Segment 29 enable."]
    #[inline(always)]
    pub fn lcds29(&mut self) -> LCDS29_W {
        LCDS29_W { w: self }
    }
    #[doc = "Bit 14 - LCD Segment 30 enable."]
    #[inline(always)]
    pub fn lcds30(&mut self) -> LCDS30_W {
        LCDS30_W { w: self }
    }
    #[doc = "Bit 15 - LCD Segment 31 enable."]
    #[inline(always)]
    pub fn lcds31(&mut self) -> LCDS31_W {
        LCDS31_W { w: self }
    }
}
