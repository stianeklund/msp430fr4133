#[doc = "Reader of register LCDPCTL0"]
pub type R = crate::R<u16, super::LCDPCTL0>;
#[doc = "Writer for register LCDPCTL0"]
pub type W = crate::W<u16, super::LCDPCTL0>;
#[doc = "Register LCDPCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDPCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDS0`"]
pub type LCDS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS0`"]
pub struct LCDS0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS0_W<'a> {
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
#[doc = "Reader of field `LCDS1`"]
pub type LCDS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS1`"]
pub struct LCDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS1_W<'a> {
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
#[doc = "Reader of field `LCDS2`"]
pub type LCDS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS2`"]
pub struct LCDS2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS2_W<'a> {
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
#[doc = "Reader of field `LCDS3`"]
pub type LCDS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS3`"]
pub struct LCDS3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS3_W<'a> {
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
#[doc = "Reader of field `LCDS4`"]
pub type LCDS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS4`"]
pub struct LCDS4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS4_W<'a> {
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
#[doc = "Reader of field `LCDS5`"]
pub type LCDS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS5`"]
pub struct LCDS5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS5_W<'a> {
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
#[doc = "Reader of field `LCDS6`"]
pub type LCDS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS6`"]
pub struct LCDS6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS6_W<'a> {
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
#[doc = "Reader of field `LCDS7`"]
pub type LCDS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS7`"]
pub struct LCDS7_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS7_W<'a> {
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
#[doc = "Reader of field `LCDS8`"]
pub type LCDS8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS8`"]
pub struct LCDS8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS8_W<'a> {
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
#[doc = "Reader of field `LCDS9`"]
pub type LCDS9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS9`"]
pub struct LCDS9_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS9_W<'a> {
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
#[doc = "Reader of field `LCDS10`"]
pub type LCDS10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS10`"]
pub struct LCDS10_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS10_W<'a> {
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
#[doc = "Reader of field `LCDS11`"]
pub type LCDS11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS11`"]
pub struct LCDS11_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS11_W<'a> {
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
#[doc = "Reader of field `LCDS12`"]
pub type LCDS12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS12`"]
pub struct LCDS12_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS12_W<'a> {
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
#[doc = "Reader of field `LCDS13`"]
pub type LCDS13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS13`"]
pub struct LCDS13_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS13_W<'a> {
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
#[doc = "Reader of field `LCDS14`"]
pub type LCDS14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS14`"]
pub struct LCDS14_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS14_W<'a> {
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
#[doc = "Reader of field `LCDS15`"]
pub type LCDS15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDS15`"]
pub struct LCDS15_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS15_W<'a> {
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
    #[doc = "Bit 0 - LCD Segment 0 enable."]
    #[inline(always)]
    pub fn lcds0(&self) -> LCDS0_R {
        LCDS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 1 enable."]
    #[inline(always)]
    pub fn lcds1(&self) -> LCDS1_R {
        LCDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 2 enable."]
    #[inline(always)]
    pub fn lcds2(&self) -> LCDS2_R {
        LCDS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 3 enable."]
    #[inline(always)]
    pub fn lcds3(&self) -> LCDS3_R {
        LCDS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 4 enable."]
    #[inline(always)]
    pub fn lcds4(&self) -> LCDS4_R {
        LCDS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 5 enable."]
    #[inline(always)]
    pub fn lcds5(&self) -> LCDS5_R {
        LCDS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 6 enable."]
    #[inline(always)]
    pub fn lcds6(&self) -> LCDS6_R {
        LCDS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 7 enable."]
    #[inline(always)]
    pub fn lcds7(&self) -> LCDS7_R {
        LCDS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 8 enable."]
    #[inline(always)]
    pub fn lcds8(&self) -> LCDS8_R {
        LCDS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 9 enable."]
    #[inline(always)]
    pub fn lcds9(&self) -> LCDS9_R {
        LCDS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 10 enable."]
    #[inline(always)]
    pub fn lcds10(&self) -> LCDS10_R {
        LCDS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 11 enable."]
    #[inline(always)]
    pub fn lcds11(&self) -> LCDS11_R {
        LCDS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 12 enable."]
    #[inline(always)]
    pub fn lcds12(&self) -> LCDS12_R {
        LCDS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 13 enable."]
    #[inline(always)]
    pub fn lcds13(&self) -> LCDS13_R {
        LCDS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 14 enable."]
    #[inline(always)]
    pub fn lcds14(&self) -> LCDS14_R {
        LCDS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 15 enable."]
    #[inline(always)]
    pub fn lcds15(&self) -> LCDS15_R {
        LCDS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 0 enable."]
    #[inline(always)]
    pub fn lcds0(&mut self) -> LCDS0_W {
        LCDS0_W { w: self }
    }
    #[doc = "Bit 1 - LCD Segment 1 enable."]
    #[inline(always)]
    pub fn lcds1(&mut self) -> LCDS1_W {
        LCDS1_W { w: self }
    }
    #[doc = "Bit 2 - LCD Segment 2 enable."]
    #[inline(always)]
    pub fn lcds2(&mut self) -> LCDS2_W {
        LCDS2_W { w: self }
    }
    #[doc = "Bit 3 - LCD Segment 3 enable."]
    #[inline(always)]
    pub fn lcds3(&mut self) -> LCDS3_W {
        LCDS3_W { w: self }
    }
    #[doc = "Bit 4 - LCD Segment 4 enable."]
    #[inline(always)]
    pub fn lcds4(&mut self) -> LCDS4_W {
        LCDS4_W { w: self }
    }
    #[doc = "Bit 5 - LCD Segment 5 enable."]
    #[inline(always)]
    pub fn lcds5(&mut self) -> LCDS5_W {
        LCDS5_W { w: self }
    }
    #[doc = "Bit 6 - LCD Segment 6 enable."]
    #[inline(always)]
    pub fn lcds6(&mut self) -> LCDS6_W {
        LCDS6_W { w: self }
    }
    #[doc = "Bit 7 - LCD Segment 7 enable."]
    #[inline(always)]
    pub fn lcds7(&mut self) -> LCDS7_W {
        LCDS7_W { w: self }
    }
    #[doc = "Bit 8 - LCD Segment 8 enable."]
    #[inline(always)]
    pub fn lcds8(&mut self) -> LCDS8_W {
        LCDS8_W { w: self }
    }
    #[doc = "Bit 9 - LCD Segment 9 enable."]
    #[inline(always)]
    pub fn lcds9(&mut self) -> LCDS9_W {
        LCDS9_W { w: self }
    }
    #[doc = "Bit 10 - LCD Segment 10 enable."]
    #[inline(always)]
    pub fn lcds10(&mut self) -> LCDS10_W {
        LCDS10_W { w: self }
    }
    #[doc = "Bit 11 - LCD Segment 11 enable."]
    #[inline(always)]
    pub fn lcds11(&mut self) -> LCDS11_W {
        LCDS11_W { w: self }
    }
    #[doc = "Bit 12 - LCD Segment 12 enable."]
    #[inline(always)]
    pub fn lcds12(&mut self) -> LCDS12_W {
        LCDS12_W { w: self }
    }
    #[doc = "Bit 13 - LCD Segment 13 enable."]
    #[inline(always)]
    pub fn lcds13(&mut self) -> LCDS13_W {
        LCDS13_W { w: self }
    }
    #[doc = "Bit 14 - LCD Segment 14 enable."]
    #[inline(always)]
    pub fn lcds14(&mut self) -> LCDS14_W {
        LCDS14_W { w: self }
    }
    #[doc = "Bit 15 - LCD Segment 15 enable."]
    #[inline(always)]
    pub fn lcds15(&mut self) -> LCDS15_W {
        LCDS15_W { w: self }
    }
}
