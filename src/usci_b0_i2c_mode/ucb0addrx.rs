#[doc = "Reader of register UCB0ADDRX"]
pub type R = crate::R<u16, super::UCB0ADDRX>;
#[doc = "Writer for register UCB0ADDRX"]
pub type W = crate::W<u16, super::UCB0ADDRX>;
#[doc = "Register UCB0ADDRX `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0ADDRX {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCADDRX0`"]
pub type UCADDRX0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX0`"]
pub struct UCADDRX0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX0_W<'a> {
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
#[doc = "Reader of field `UCADDRX1`"]
pub type UCADDRX1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX1`"]
pub struct UCADDRX1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX1_W<'a> {
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
#[doc = "Reader of field `UCADDRX2`"]
pub type UCADDRX2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX2`"]
pub struct UCADDRX2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX2_W<'a> {
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
#[doc = "Reader of field `UCADDRX3`"]
pub type UCADDRX3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX3`"]
pub struct UCADDRX3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX3_W<'a> {
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
#[doc = "Reader of field `UCADDRX4`"]
pub type UCADDRX4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX4`"]
pub struct UCADDRX4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX4_W<'a> {
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
#[doc = "Reader of field `UCADDRX5`"]
pub type UCADDRX5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX5`"]
pub struct UCADDRX5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX5_W<'a> {
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
#[doc = "Reader of field `UCADDRX6`"]
pub type UCADDRX6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX6`"]
pub struct UCADDRX6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX6_W<'a> {
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
#[doc = "Reader of field `UCADDRX7`"]
pub type UCADDRX7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX7`"]
pub struct UCADDRX7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX7_W<'a> {
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
#[doc = "Reader of field `UCADDRX8`"]
pub type UCADDRX8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX8`"]
pub struct UCADDRX8_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX8_W<'a> {
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
#[doc = "Reader of field `UCADDRX9`"]
pub type UCADDRX9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCADDRX9`"]
pub struct UCADDRX9_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX9_W<'a> {
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
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    pub fn ucaddrx0(&self) -> UCADDRX0_R {
        UCADDRX0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    pub fn ucaddrx1(&self) -> UCADDRX1_R {
        UCADDRX1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    pub fn ucaddrx2(&self) -> UCADDRX2_R {
        UCADDRX2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    pub fn ucaddrx3(&self) -> UCADDRX3_R {
        UCADDRX3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    pub fn ucaddrx4(&self) -> UCADDRX4_R {
        UCADDRX4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    pub fn ucaddrx5(&self) -> UCADDRX5_R {
        UCADDRX5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    pub fn ucaddrx6(&self) -> UCADDRX6_R {
        UCADDRX6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    pub fn ucaddrx7(&self) -> UCADDRX7_R {
        UCADDRX7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    pub fn ucaddrx8(&self) -> UCADDRX8_R {
        UCADDRX8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    pub fn ucaddrx9(&self) -> UCADDRX9_R {
        UCADDRX9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    pub fn ucaddrx0(&mut self) -> UCADDRX0_W {
        UCADDRX0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    pub fn ucaddrx1(&mut self) -> UCADDRX1_W {
        UCADDRX1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    pub fn ucaddrx2(&mut self) -> UCADDRX2_W {
        UCADDRX2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    pub fn ucaddrx3(&mut self) -> UCADDRX3_W {
        UCADDRX3_W { w: self }
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    pub fn ucaddrx4(&mut self) -> UCADDRX4_W {
        UCADDRX4_W { w: self }
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    pub fn ucaddrx5(&mut self) -> UCADDRX5_W {
        UCADDRX5_W { w: self }
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    pub fn ucaddrx6(&mut self) -> UCADDRX6_W {
        UCADDRX6_W { w: self }
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    pub fn ucaddrx7(&mut self) -> UCADDRX7_W {
        UCADDRX7_W { w: self }
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    pub fn ucaddrx8(&mut self) -> UCADDRX8_W {
        UCADDRX8_W { w: self }
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    pub fn ucaddrx9(&mut self) -> UCADDRX9_W {
        UCADDRX9_W { w: self }
    }
}
