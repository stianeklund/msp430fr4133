#[doc = "Reader of register UCB0ADDMASK"]
pub type R = crate::R<u16, super::UCB0ADDMASK>;
#[doc = "Writer for register UCB0ADDMASK"]
pub type W = crate::W<u16, super::UCB0ADDMASK>;
#[doc = "Register UCB0ADDMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0ADDMASK {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCADDMASK0`"]
pub type UCADDMASK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK0`"]
pub struct UCADDMASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK0_W<'a> {
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
#[doc = "Reader of field `UCADDMASK1`"]
pub type UCADDMASK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK1`"]
pub struct UCADDMASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK1_W<'a> {
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
#[doc = "Reader of field `UCADDMASK2`"]
pub type UCADDMASK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK2`"]
pub struct UCADDMASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK2_W<'a> {
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
#[doc = "Reader of field `UCADDMASK3`"]
pub type UCADDMASK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK3`"]
pub struct UCADDMASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK3_W<'a> {
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
#[doc = "Reader of field `UCADDMASK4`"]
pub type UCADDMASK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK4`"]
pub struct UCADDMASK4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK4_W<'a> {
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
#[doc = "Reader of field `UCADDMASK5`"]
pub type UCADDMASK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK5`"]
pub struct UCADDMASK5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK5_W<'a> {
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
#[doc = "Reader of field `UCADDMASK6`"]
pub type UCADDMASK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK6`"]
pub struct UCADDMASK6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK6_W<'a> {
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
#[doc = "Reader of field `UCADDMASK7`"]
pub type UCADDMASK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK7`"]
pub struct UCADDMASK7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK7_W<'a> {
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
#[doc = "Reader of field `UCADDMASK8`"]
pub type UCADDMASK8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK8`"]
pub struct UCADDMASK8_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK8_W<'a> {
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
#[doc = "Reader of field `UCADDMASK9`"]
pub type UCADDMASK9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDMASK9`"]
pub struct UCADDMASK9_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK9_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline(always)]
    pub fn ucaddmask0(&self) -> UCADDMASK0_R {
        UCADDMASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline(always)]
    pub fn ucaddmask1(&self) -> UCADDMASK1_R {
        UCADDMASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline(always)]
    pub fn ucaddmask2(&self) -> UCADDMASK2_R {
        UCADDMASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline(always)]
    pub fn ucaddmask3(&self) -> UCADDMASK3_R {
        UCADDMASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline(always)]
    pub fn ucaddmask4(&self) -> UCADDMASK4_R {
        UCADDMASK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline(always)]
    pub fn ucaddmask5(&self) -> UCADDMASK5_R {
        UCADDMASK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline(always)]
    pub fn ucaddmask6(&self) -> UCADDMASK6_R {
        UCADDMASK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline(always)]
    pub fn ucaddmask7(&self) -> UCADDMASK7_R {
        UCADDMASK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline(always)]
    pub fn ucaddmask8(&self) -> UCADDMASK8_R {
        UCADDMASK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline(always)]
    pub fn ucaddmask9(&self) -> UCADDMASK9_R {
        UCADDMASK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline(always)]
    pub fn ucaddmask0(&mut self) -> UCADDMASK0_W {
        UCADDMASK0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline(always)]
    pub fn ucaddmask1(&mut self) -> UCADDMASK1_W {
        UCADDMASK1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline(always)]
    pub fn ucaddmask2(&mut self) -> UCADDMASK2_W {
        UCADDMASK2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline(always)]
    pub fn ucaddmask3(&mut self) -> UCADDMASK3_W {
        UCADDMASK3_W { w: self }
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline(always)]
    pub fn ucaddmask4(&mut self) -> UCADDMASK4_W {
        UCADDMASK4_W { w: self }
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline(always)]
    pub fn ucaddmask5(&mut self) -> UCADDMASK5_W {
        UCADDMASK5_W { w: self }
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline(always)]
    pub fn ucaddmask6(&mut self) -> UCADDMASK6_W {
        UCADDMASK6_W { w: self }
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline(always)]
    pub fn ucaddmask7(&mut self) -> UCADDMASK7_W {
        UCADDMASK7_W { w: self }
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline(always)]
    pub fn ucaddmask8(&mut self) -> UCADDMASK8_W {
        UCADDMASK8_W { w: self }
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline(always)]
    pub fn ucaddmask9(&mut self) -> UCADDMASK9_W {
        UCADDMASK9_W { w: self }
    }
}
