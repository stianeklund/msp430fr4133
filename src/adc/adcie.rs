#[doc = "Reader of register ADCIE"]
pub type R = crate::R<u16, super::ADCIE>;
#[doc = "Writer for register ADCIE"]
pub type W = crate::W<u16, super::ADCIE>;
#[doc = "Register ADCIE `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCIE0`"]
pub type ADCIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCIE0`"]
pub struct ADCIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIE0_W<'a> {
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
#[doc = "Reader of field `ADCINIE`"]
pub type ADCINIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCINIE`"]
pub struct ADCINIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCINIE_W<'a> {
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
#[doc = "Reader of field `ADCLOIE`"]
pub type ADCLOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCLOIE`"]
pub struct ADCLOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCLOIE_W<'a> {
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
#[doc = "Reader of field `ADCHIIE`"]
pub type ADCHIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCHIIE`"]
pub struct ADCHIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCHIIE_W<'a> {
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
#[doc = "Reader of field `ADCOVIE`"]
pub type ADCOVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCOVIE`"]
pub struct ADCOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCOVIE_W<'a> {
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
#[doc = "Reader of field `ADCTOVIE`"]
pub type ADCTOVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCTOVIE`"]
pub struct ADCTOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTOVIE_W<'a> {
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
    #[doc = "Bit 0 - ADC Interrupt enable"]
    #[inline(always)]
    pub fn adcie0(&self) -> ADCIE0_R {
        ADCIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinie(&self) -> ADCINIE_R {
        ADCINIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloie(&self) -> ADCLOIE_R {
        ADCLOIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiie(&self) -> ADCHIIE_R {
        ADCHIIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt enable"]
    #[inline(always)]
    pub fn adcovie(&self) -> ADCOVIE_R {
        ADCOVIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    pub fn adctovie(&self) -> ADCTOVIE_R {
        ADCTOVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Interrupt enable"]
    #[inline(always)]
    pub fn adcie0(&mut self) -> ADCIE0_W {
        ADCIE0_W { w: self }
    }
    #[doc = "Bit 1 - ADC Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinie(&mut self) -> ADCINIE_W {
        ADCINIE_W { w: self }
    }
    #[doc = "Bit 2 - ADC Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloie(&mut self) -> ADCLOIE_W {
        ADCLOIE_W { w: self }
    }
    #[doc = "Bit 3 - ADC Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiie(&mut self) -> ADCHIIE_W {
        ADCHIIE_W { w: self }
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt enable"]
    #[inline(always)]
    pub fn adcovie(&mut self) -> ADCOVIE_W {
        ADCOVIE_W { w: self }
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    pub fn adctovie(&mut self) -> ADCTOVIE_W {
        ADCTOVIE_W { w: self }
    }
}
