#[doc = "Reader of register UCB0BCNT_I2C"]
pub type R = crate::R<u8, super::UCB0BCNT_I2C>;
#[doc = "Writer for register UCB0BCNT_I2C"]
pub type W = crate::W<u8, super::UCB0BCNT_I2C>;
#[doc = "Register UCB0BCNT_I2C `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0BCNT_I2C {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCBCNT0`"]
pub type UCBCNT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBCNT0`"]
pub struct UCBCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT0_W<'a> {
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
#[doc = "Reader of field `UCBCNT1`"]
pub type UCBCNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBCNT1`"]
pub struct UCBCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT1_W<'a> {
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
#[doc = "Reader of field `UCBCNT2`"]
pub type UCBCNT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBCNT2`"]
pub struct UCBCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT2_W<'a> {
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
#[doc = "Reader of field `UCBCNT3`"]
pub type UCBCNT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBCNT3`"]
pub struct UCBCNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT3_W<'a> {
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
#[doc = "Reader of field `UCBCNT4`"]
pub type UCBCNT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBCNT4`"]
pub struct UCBCNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT4_W<'a> {
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
#[doc = "Reader of field `UCBCNT5`"]
pub type UCBCNT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBCNT5`"]
pub struct UCBCNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT5_W<'a> {
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
#[doc = "Reader of field `UCBCNT6`"]
pub type UCBCNT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBCNT6`"]
pub struct UCBCNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT6_W<'a> {
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
#[doc = "Reader of field `UCBCNT7`"]
pub type UCBCNT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBCNT7`"]
pub struct UCBCNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT7_W<'a> {
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
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline(always)]
    pub fn ucbcnt0(&self) -> UCBCNT0_R {
        UCBCNT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline(always)]
    pub fn ucbcnt1(&self) -> UCBCNT1_R {
        UCBCNT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline(always)]
    pub fn ucbcnt2(&self) -> UCBCNT2_R {
        UCBCNT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline(always)]
    pub fn ucbcnt3(&self) -> UCBCNT3_R {
        UCBCNT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline(always)]
    pub fn ucbcnt4(&self) -> UCBCNT4_R {
        UCBCNT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline(always)]
    pub fn ucbcnt5(&self) -> UCBCNT5_R {
        UCBCNT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline(always)]
    pub fn ucbcnt6(&self) -> UCBCNT6_R {
        UCBCNT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline(always)]
    pub fn ucbcnt7(&self) -> UCBCNT7_R {
        UCBCNT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline(always)]
    pub fn ucbcnt0(&mut self) -> UCBCNT0_W {
        UCBCNT0_W { w: self }
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline(always)]
    pub fn ucbcnt1(&mut self) -> UCBCNT1_W {
        UCBCNT1_W { w: self }
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline(always)]
    pub fn ucbcnt2(&mut self) -> UCBCNT2_W {
        UCBCNT2_W { w: self }
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline(always)]
    pub fn ucbcnt3(&mut self) -> UCBCNT3_W {
        UCBCNT3_W { w: self }
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline(always)]
    pub fn ucbcnt4(&mut self) -> UCBCNT4_W {
        UCBCNT4_W { w: self }
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline(always)]
    pub fn ucbcnt5(&mut self) -> UCBCNT5_W {
        UCBCNT5_W { w: self }
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline(always)]
    pub fn ucbcnt6(&mut self) -> UCBCNT6_W {
        UCBCNT6_W { w: self }
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline(always)]
    pub fn ucbcnt7(&mut self) -> UCBCNT7_W {
        UCBCNT7_W { w: self }
    }
}
