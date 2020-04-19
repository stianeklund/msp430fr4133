#[doc = "Reader of register UCB0IE_I2C"]
pub type R = crate::R<u16, super::UCB0IE_I2C>;
#[doc = "Writer for register UCB0IE_I2C"]
pub type W = crate::W<u16, super::UCB0IE_I2C>;
#[doc = "Register UCB0IE_I2C `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0IE_I2C {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCRXIE0`"]
pub type UCRXIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCRXIE0`"]
pub struct UCRXIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE0_W<'a> {
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
#[doc = "Reader of field `UCTXIE0`"]
pub type UCTXIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXIE0`"]
pub struct UCTXIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE0_W<'a> {
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
#[doc = "Reader of field `UCSTTIE`"]
pub type UCSTTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSTTIE`"]
pub struct UCSTTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIE_W<'a> {
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
#[doc = "Reader of field `UCSTPIE`"]
pub type UCSTPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSTPIE`"]
pub struct UCSTPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTPIE_W<'a> {
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
#[doc = "Reader of field `UCALIE`"]
pub type UCALIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCALIE`"]
pub struct UCALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCALIE_W<'a> {
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
#[doc = "Reader of field `UCNACKIE`"]
pub type UCNACKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCNACKIE`"]
pub struct UCNACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCNACKIE_W<'a> {
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
#[doc = "Reader of field `UCBCNTIE`"]
pub type UCBCNTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBCNTIE`"]
pub struct UCBCNTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNTIE_W<'a> {
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
#[doc = "Reader of field `UCCLTOIE`"]
pub type UCCLTOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCCLTOIE`"]
pub struct UCCLTOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCLTOIE_W<'a> {
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
#[doc = "Reader of field `UCRXIE1`"]
pub type UCRXIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCRXIE1`"]
pub struct UCRXIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE1_W<'a> {
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
#[doc = "Reader of field `UCTXIE1`"]
pub type UCTXIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXIE1`"]
pub struct UCTXIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE1_W<'a> {
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
#[doc = "Reader of field `UCRXIE2`"]
pub type UCRXIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCRXIE2`"]
pub struct UCRXIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE2_W<'a> {
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
#[doc = "Reader of field `UCTXIE2`"]
pub type UCTXIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXIE2`"]
pub struct UCTXIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE2_W<'a> {
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
#[doc = "Reader of field `UCRXIE3`"]
pub type UCRXIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCRXIE3`"]
pub struct UCRXIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE3_W<'a> {
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
#[doc = "Reader of field `UCTXIE3`"]
pub type UCTXIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXIE3`"]
pub struct UCTXIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE3_W<'a> {
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
#[doc = "Reader of field `UCBIT9IE`"]
pub type UCBIT9IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBIT9IE`"]
pub struct UCBIT9IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBIT9IE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&self) -> UCRXIE0_R {
        UCRXIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    pub fn uctxie0(&self) -> UCTXIE0_R {
        UCTXIE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&self) -> UCBCNTIE_R {
        UCBCNTIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&self) -> UCCLTOIE_R {
        UCCLTOIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&self) -> UCRXIE1_R {
        UCRXIE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    pub fn uctxie1(&self) -> UCTXIE1_R {
        UCTXIE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&self) -> UCRXIE2_R {
        UCRXIE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    pub fn uctxie2(&self) -> UCTXIE2_R {
        UCTXIE2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&self) -> UCRXIE3_R {
        UCRXIE3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    pub fn uctxie3(&self) -> UCTXIE3_R {
        UCTXIE3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucbit9ie(&self) -> UCBIT9IE_R {
        UCBIT9IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&mut self) -> UCRXIE0_W {
        UCRXIE0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    pub fn uctxie0(&mut self) -> UCTXIE0_W {
        UCTXIE0_W { w: self }
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W {
        UCSTTIE_W { w: self }
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UCSTPIE_W {
        UCSTPIE_W { w: self }
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UCALIE_W {
        UCALIE_W { w: self }
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UCNACKIE_W {
        UCNACKIE_W { w: self }
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&mut self) -> UCBCNTIE_W {
        UCBCNTIE_W { w: self }
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&mut self) -> UCCLTOIE_W {
        UCCLTOIE_W { w: self }
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&mut self) -> UCRXIE1_W {
        UCRXIE1_W { w: self }
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    pub fn uctxie1(&mut self) -> UCTXIE1_W {
        UCTXIE1_W { w: self }
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&mut self) -> UCRXIE2_W {
        UCRXIE2_W { w: self }
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    pub fn uctxie2(&mut self) -> UCTXIE2_W {
        UCTXIE2_W { w: self }
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&mut self) -> UCRXIE3_W {
        UCRXIE3_W { w: self }
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    pub fn uctxie3(&mut self) -> UCTXIE3_W {
        UCTXIE3_W { w: self }
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucbit9ie(&mut self) -> UCBIT9IE_W {
        UCBIT9IE_W { w: self }
    }
}
