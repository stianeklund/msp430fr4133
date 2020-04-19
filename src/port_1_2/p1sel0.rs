#[doc = "Reader of register P1SEL0"]
pub type R = crate::R<u8, super::P1SEL0>;
#[doc = "Writer for register P1SEL0"]
pub type W = crate::W<u8, super::P1SEL0>;
#[doc = "Register P1SEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::P1SEL0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1SEL0_0`"]
pub type P1SEL0_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL0_0`"]
pub struct P1SEL0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `P1SEL0_1`"]
pub type P1SEL0_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL0_1`"]
pub struct P1SEL0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `P1SEL0_2`"]
pub type P1SEL0_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL0_2`"]
pub struct P1SEL0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `P1SEL0_3`"]
pub type P1SEL0_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL0_3`"]
pub struct P1SEL0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `P1SEL0_4`"]
pub type P1SEL0_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL0_4`"]
pub struct P1SEL0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `P1SEL0_5`"]
pub type P1SEL0_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL0_5`"]
pub struct P1SEL0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `P1SEL0_6`"]
pub type P1SEL0_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL0_6`"]
pub struct P1SEL0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `P1SEL0_7`"]
pub type P1SEL0_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SEL0_7`"]
pub struct P1SEL0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SEL0_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - P1SEL0_0"]
    #[inline(always)]
    pub fn p1sel0_0(&self) -> P1SEL0_0_R {
        P1SEL0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1SEL0_1"]
    #[inline(always)]
    pub fn p1sel0_1(&self) -> P1SEL0_1_R {
        P1SEL0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1SEL0_2"]
    #[inline(always)]
    pub fn p1sel0_2(&self) -> P1SEL0_2_R {
        P1SEL0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1SEL0_3"]
    #[inline(always)]
    pub fn p1sel0_3(&self) -> P1SEL0_3_R {
        P1SEL0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1SEL0_4"]
    #[inline(always)]
    pub fn p1sel0_4(&self) -> P1SEL0_4_R {
        P1SEL0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1SEL0_5"]
    #[inline(always)]
    pub fn p1sel0_5(&self) -> P1SEL0_5_R {
        P1SEL0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1SEL0_6"]
    #[inline(always)]
    pub fn p1sel0_6(&self) -> P1SEL0_6_R {
        P1SEL0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1SEL0_7"]
    #[inline(always)]
    pub fn p1sel0_7(&self) -> P1SEL0_7_R {
        P1SEL0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1SEL0_0"]
    #[inline(always)]
    pub fn p1sel0_0(&mut self) -> P1SEL0_0_W {
        P1SEL0_0_W { w: self }
    }
    #[doc = "Bit 1 - P1SEL0_1"]
    #[inline(always)]
    pub fn p1sel0_1(&mut self) -> P1SEL0_1_W {
        P1SEL0_1_W { w: self }
    }
    #[doc = "Bit 2 - P1SEL0_2"]
    #[inline(always)]
    pub fn p1sel0_2(&mut self) -> P1SEL0_2_W {
        P1SEL0_2_W { w: self }
    }
    #[doc = "Bit 3 - P1SEL0_3"]
    #[inline(always)]
    pub fn p1sel0_3(&mut self) -> P1SEL0_3_W {
        P1SEL0_3_W { w: self }
    }
    #[doc = "Bit 4 - P1SEL0_4"]
    #[inline(always)]
    pub fn p1sel0_4(&mut self) -> P1SEL0_4_W {
        P1SEL0_4_W { w: self }
    }
    #[doc = "Bit 5 - P1SEL0_5"]
    #[inline(always)]
    pub fn p1sel0_5(&mut self) -> P1SEL0_5_W {
        P1SEL0_5_W { w: self }
    }
    #[doc = "Bit 6 - P1SEL0_6"]
    #[inline(always)]
    pub fn p1sel0_6(&mut self) -> P1SEL0_6_W {
        P1SEL0_6_W { w: self }
    }
    #[doc = "Bit 7 - P1SEL0_7"]
    #[inline(always)]
    pub fn p1sel0_7(&mut self) -> P1SEL0_7_W {
        P1SEL0_7_W { w: self }
    }
}
