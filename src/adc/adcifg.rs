#[doc = "Reader of register ADCIFG"]
pub type R = crate::R<u16, super::ADCIFG>;
#[doc = "Writer for register ADCIFG"]
pub type W = crate::W<u16, super::ADCIFG>;
#[doc = "Register ADCIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCIFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCIFG0`"]
pub type ADCIFG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCIFG0`"]
pub struct ADCIFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIFG0_W<'a> {
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
#[doc = "Reader of field `ADCINIFG`"]
pub type ADCINIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCINIFG`"]
pub struct ADCINIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCINIFG_W<'a> {
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
#[doc = "Reader of field `ADCLOIFG`"]
pub type ADCLOIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCLOIFG`"]
pub struct ADCLOIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCLOIFG_W<'a> {
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
#[doc = "Reader of field `ADCHIIFG`"]
pub type ADCHIIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCHIIFG`"]
pub struct ADCHIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCHIIFG_W<'a> {
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
#[doc = "Reader of field `ADCOVIFG`"]
pub type ADCOVIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCOVIFG`"]
pub struct ADCOVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCOVIFG_W<'a> {
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
#[doc = "Reader of field `ADCTOVIFG`"]
pub type ADCTOVIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCTOVIFG`"]
pub struct ADCTOVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTOVIFG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ADC Interrupt Flag"]
    #[inline(always)]
    pub fn adcifg0(&self) -> ADCIFG0_R {
        ADCIFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinifg(&self) -> ADCINIFG_R {
        ADCINIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloifg(&self) -> ADCLOIFG_R {
        ADCLOIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiifg(&self) -> ADCHIIFG_R {
        ADCHIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adcovifg(&self) -> ADCOVIFG_R {
        ADCOVIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adctovifg(&self) -> ADCTOVIFG_R {
        ADCTOVIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Interrupt Flag"]
    #[inline(always)]
    pub fn adcifg0(&mut self) -> ADCIFG0_W {
        ADCIFG0_W { w: self }
    }
    #[doc = "Bit 1 - ADC Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinifg(&mut self) -> ADCINIFG_W {
        ADCINIFG_W { w: self }
    }
    #[doc = "Bit 2 - ADC Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloifg(&mut self) -> ADCLOIFG_W {
        ADCLOIFG_W { w: self }
    }
    #[doc = "Bit 3 - ADC Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiifg(&mut self) -> ADCHIIFG_W {
        ADCHIIFG_W { w: self }
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adcovifg(&mut self) -> ADCOVIFG_W {
        ADCOVIFG_W { w: self }
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adctovifg(&mut self) -> ADCTOVIFG_W {
        ADCTOVIFG_W { w: self }
    }
}
