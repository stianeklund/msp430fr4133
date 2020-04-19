#[doc = "Reader of register SYSCFG2"]
pub type R = crate::R<u16, super::SYSCFG2>;
#[doc = "Writer for register SYSCFG2"]
pub type W = crate::W<u16, super::SYSCFG2>;
#[doc = "Register SYSCFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCPCTL0`"]
pub type ADCPCTL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL0`"]
pub struct ADCPCTL0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL0_W<'a> {
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
#[doc = "Reader of field `ADCPCTL1`"]
pub type ADCPCTL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL1`"]
pub struct ADCPCTL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL1_W<'a> {
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
#[doc = "Reader of field `ADCPCTL2`"]
pub type ADCPCTL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL2`"]
pub struct ADCPCTL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL2_W<'a> {
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
#[doc = "Reader of field `ADCPCTL3`"]
pub type ADCPCTL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL3`"]
pub struct ADCPCTL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL3_W<'a> {
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
#[doc = "Reader of field `ADCPCTL4`"]
pub type ADCPCTL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL4`"]
pub struct ADCPCTL4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL4_W<'a> {
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
#[doc = "Reader of field `ADCPCTL5`"]
pub type ADCPCTL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL5`"]
pub struct ADCPCTL5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL5_W<'a> {
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
#[doc = "Reader of field `ADCPCTL6`"]
pub type ADCPCTL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL6`"]
pub struct ADCPCTL6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL6_W<'a> {
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
#[doc = "Reader of field `ADCPCTL7`"]
pub type ADCPCTL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL7`"]
pub struct ADCPCTL7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL7_W<'a> {
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
#[doc = "Reader of field `ADCPCTL8`"]
pub type ADCPCTL8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL8`"]
pub struct ADCPCTL8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL8_W<'a> {
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
#[doc = "Reader of field `ADCPCTL9`"]
pub type ADCPCTL9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCPCTL9`"]
pub struct ADCPCTL9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL9_W<'a> {
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
#[doc = "Reader of field `LCDPCTL`"]
pub type LCDPCTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDPCTL`"]
pub struct LCDPCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDPCTL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&self) -> ADCPCTL0_R {
        ADCPCTL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&self) -> ADCPCTL1_R {
        ADCPCTL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&self) -> ADCPCTL2_R {
        ADCPCTL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&self) -> ADCPCTL3_R {
        ADCPCTL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&self) -> ADCPCTL4_R {
        ADCPCTL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&self) -> ADCPCTL5_R {
        ADCPCTL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&self) -> ADCPCTL6_R {
        ADCPCTL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&self) -> ADCPCTL7_R {
        ADCPCTL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&self) -> ADCPCTL8_R {
        ADCPCTL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&self) -> ADCPCTL9_R {
        ADCPCTL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCD Power Pin"]
    #[inline(always)]
    pub fn lcdpctl(&self) -> LCDPCTL_R {
        LCDPCTL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&mut self) -> ADCPCTL0_W {
        ADCPCTL0_W { w: self }
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&mut self) -> ADCPCTL1_W {
        ADCPCTL1_W { w: self }
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&mut self) -> ADCPCTL2_W {
        ADCPCTL2_W { w: self }
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&mut self) -> ADCPCTL3_W {
        ADCPCTL3_W { w: self }
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&mut self) -> ADCPCTL4_W {
        ADCPCTL4_W { w: self }
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&mut self) -> ADCPCTL5_W {
        ADCPCTL5_W { w: self }
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&mut self) -> ADCPCTL6_W {
        ADCPCTL6_W { w: self }
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&mut self) -> ADCPCTL7_W {
        ADCPCTL7_W { w: self }
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&mut self) -> ADCPCTL8_W {
        ADCPCTL8_W { w: self }
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&mut self) -> ADCPCTL9_W {
        ADCPCTL9_W { w: self }
    }
    #[doc = "Bit 12 - LCD Power Pin"]
    #[inline(always)]
    pub fn lcdpctl(&mut self) -> LCDPCTL_W {
        LCDPCTL_W { w: self }
    }
}
