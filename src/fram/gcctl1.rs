#[doc = "Reader of register GCCTL1"]
pub type R = crate::R<u16, super::GCCTL1>;
#[doc = "Writer for register GCCTL1"]
pub type W = crate::W<u16, super::GCCTL1>;
#[doc = "Register GCCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GCCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CBDIFG`"]
pub type CBDIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBDIFG`"]
pub struct CBDIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CBDIFG_W<'a> {
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
#[doc = "Reader of field `UBDIFG`"]
pub type UBDIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UBDIFG`"]
pub struct UBDIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UBDIFG_W<'a> {
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
#[doc = "Reader of field `ACCTEIFG`"]
pub type ACCTEIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCTEIFG`"]
pub struct ACCTEIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCTEIFG_W<'a> {
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
impl R {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    pub fn cbdifg(&self) -> CBDIFG_R {
        CBDIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    pub fn ubdifg(&self) -> UBDIFG_R {
        UBDIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&self) -> ACCTEIFG_R {
        ACCTEIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    pub fn cbdifg(&mut self) -> CBDIFG_W {
        CBDIFG_W { w: self }
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    pub fn ubdifg(&mut self) -> UBDIFG_W {
        UBDIFG_W { w: self }
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&mut self) -> ACCTEIFG_W {
        ACCTEIFG_W { w: self }
    }
}
