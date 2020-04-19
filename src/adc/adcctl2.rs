#[doc = "Reader of register ADCCTL2"]
pub type R = crate::R<u16, super::ADCCTL2>;
#[doc = "Writer for register ADCCTL2"]
pub type W = crate::W<u16, super::ADCCTL2>;
#[doc = "Register ADCCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCSR`"]
pub type ADCSR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCSR`"]
pub struct ADCSR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSR_W<'a> {
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
#[doc = "Reader of field `ADCDF`"]
pub type ADCDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCDF`"]
pub struct ADCDF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCDF_W<'a> {
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
#[doc = "ADC Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCRES_A {
    #[doc = "0: 8 bit"]
    ADCRES_0 = 0,
    #[doc = "1: 10 bit"]
    ADCRES_1 = 1,
    #[doc = "2: Reserved"]
    ADCRES_2 = 2,
    #[doc = "3: Reserved"]
    ADCRES_3 = 3,
}
impl From<ADCRES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCRES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCRES`"]
pub type ADCRES_R = crate::R<u8, ADCRES_A>;
impl ADCRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCRES_A {
        match self.bits {
            0 => ADCRES_A::ADCRES_0,
            1 => ADCRES_A::ADCRES_1,
            2 => ADCRES_A::ADCRES_2,
            3 => ADCRES_A::ADCRES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCRES_0`"]
    #[inline(always)]
    pub fn is_adcres_0(&self) -> bool {
        *self == ADCRES_A::ADCRES_0
    }
    #[doc = "Checks if the value of the field is `ADCRES_1`"]
    #[inline(always)]
    pub fn is_adcres_1(&self) -> bool {
        *self == ADCRES_A::ADCRES_1
    }
    #[doc = "Checks if the value of the field is `ADCRES_2`"]
    #[inline(always)]
    pub fn is_adcres_2(&self) -> bool {
        *self == ADCRES_A::ADCRES_2
    }
    #[doc = "Checks if the value of the field is `ADCRES_3`"]
    #[inline(always)]
    pub fn is_adcres_3(&self) -> bool {
        *self == ADCRES_A::ADCRES_3
    }
}
#[doc = "Write proxy for field `ADCRES`"]
pub struct ADCRES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCRES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn adcres_0(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_0)
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn adcres_1(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcres_2(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcres_3(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "ADC predivider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCPDIV_A {
    #[doc = "0: ADC predivider /1"]
    ADCPDIV_0 = 0,
    #[doc = "1: ADC predivider /2"]
    ADCPDIV_1 = 1,
    #[doc = "2: ADC predivider /64"]
    ADCPDIV_2 = 2,
    #[doc = "3: ADC predivider reserved"]
    ADCPDIV_3 = 3,
}
impl From<ADCPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCPDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCPDIV`"]
pub type ADCPDIV_R = crate::R<u8, ADCPDIV_A>;
impl ADCPDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPDIV_A {
        match self.bits {
            0 => ADCPDIV_A::ADCPDIV_0,
            1 => ADCPDIV_A::ADCPDIV_1,
            2 => ADCPDIV_A::ADCPDIV_2,
            3 => ADCPDIV_A::ADCPDIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_0`"]
    #[inline(always)]
    pub fn is_adcpdiv_0(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_0
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_1`"]
    #[inline(always)]
    pub fn is_adcpdiv_1(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_1
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_2`"]
    #[inline(always)]
    pub fn is_adcpdiv_2(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_2
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_3`"]
    #[inline(always)]
    pub fn is_adcpdiv_3(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_3
    }
}
#[doc = "Write proxy for field `ADCPDIV`"]
pub struct ADCPDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC predivider /1"]
    #[inline(always)]
    pub fn adcpdiv_0(self) -> &'a mut W {
        self.variant(ADCPDIV_A::ADCPDIV_0)
    }
    #[doc = "ADC predivider /2"]
    #[inline(always)]
    pub fn adcpdiv_1(self) -> &'a mut W {
        self.variant(ADCPDIV_A::ADCPDIV_1)
    }
    #[doc = "ADC predivider /64"]
    #[inline(always)]
    pub fn adcpdiv_2(self) -> &'a mut W {
        self.variant(ADCPDIV_A::ADCPDIV_2)
    }
    #[doc = "ADC predivider reserved"]
    #[inline(always)]
    pub fn adcpdiv_3(self) -> &'a mut W {
        self.variant(ADCPDIV_A::ADCPDIV_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline(always)]
    pub fn adcsr(&self) -> ADCSR_R {
        ADCSR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline(always)]
    pub fn adcdf(&self) -> ADCDF_R {
        ADCDF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline(always)]
    pub fn adcres(&self) -> ADCRES_R {
        ADCRES_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline(always)]
    pub fn adcpdiv(&self) -> ADCPDIV_R {
        ADCPDIV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline(always)]
    pub fn adcsr(&mut self) -> ADCSR_W {
        ADCSR_W { w: self }
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline(always)]
    pub fn adcdf(&mut self) -> ADCDF_W {
        ADCDF_W { w: self }
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline(always)]
    pub fn adcres(&mut self) -> ADCRES_W {
        ADCRES_W { w: self }
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline(always)]
    pub fn adcpdiv(&mut self) -> ADCPDIV_W {
        ADCPDIV_W { w: self }
    }
}
