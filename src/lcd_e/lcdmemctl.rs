#[doc = "Reader of register LCDMEMCTL"]
pub type R = crate::R<u16, super::LCDMEMCTL>;
#[doc = "Writer for register LCDMEMCTL"]
pub type W = crate::W<u16, super::LCDMEMCTL>;
#[doc = "Register LCDMEMCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDMEMCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDDISP`"]
pub type LCDDISP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDDISP`"]
pub struct LCDDISP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDDISP_W<'a> {
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
#[doc = "Reader of field `LCDCLRM`"]
pub type LCDCLRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCLRM`"]
pub struct LCDCLRM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCLRM_W<'a> {
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
#[doc = "Reader of field `LCDCLRBM`"]
pub type LCDCLRBM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCLRBM`"]
pub struct LCDCLRBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCLRBM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LCD_E LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&self) -> LCDDISP_R {
        LCDDISP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD_E Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&self) -> LCDCLRM_R {
        LCDCLRM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD_E Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&self) -> LCDCLRBM_R {
        LCDCLRBM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_E LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&mut self) -> LCDDISP_W {
        LCDDISP_W { w: self }
    }
    #[doc = "Bit 1 - LCD_E Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&mut self) -> LCDCLRM_W {
        LCDCLRM_W { w: self }
    }
    #[doc = "Bit 2 - LCD_E Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&mut self) -> LCDCLRBM_W {
        LCDCLRBM_W { w: self }
    }
}
