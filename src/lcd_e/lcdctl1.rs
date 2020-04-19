#[doc = "Reader of register LCDCTL1"]
pub type R = crate::R<u16, super::LCDCTL1>;
#[doc = "Writer for register LCDCTL1"]
pub type W = crate::W<u16, super::LCDCTL1>;
#[doc = "Register LCDCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDFRMIFG`"]
pub type LCDFRMIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDFRMIFG`"]
pub struct LCDFRMIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDFRMIFG_W<'a> {
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
#[doc = "Reader of field `LCDBLKOFFIFG`"]
pub type LCDBLKOFFIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDBLKOFFIFG`"]
pub struct LCDBLKOFFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKOFFIFG_W<'a> {
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
#[doc = "Reader of field `LCDBLKONIFG`"]
pub type LCDBLKONIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDBLKONIFG`"]
pub struct LCDBLKONIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKONIFG_W<'a> {
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
#[doc = "Reader of field `LCDFRMIE`"]
pub type LCDFRMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDFRMIE`"]
pub struct LCDFRMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDFRMIE_W<'a> {
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
#[doc = "Reader of field `LCDBLKOFFIE`"]
pub type LCDBLKOFFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDBLKOFFIE`"]
pub struct LCDBLKOFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKOFFIE_W<'a> {
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
#[doc = "Reader of field `LCDBLKONIE`"]
pub type LCDBLKONIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDBLKONIE`"]
pub struct LCDBLKONIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKONIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LCD_E LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&self) -> LCDFRMIFG_R {
        LCDFRMIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD_E LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&self) -> LCDBLKOFFIFG_R {
        LCDBLKOFFIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD_E LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&self) -> LCDBLKONIFG_R {
        LCDBLKONIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD_E LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&self) -> LCDFRMIE_R {
        LCDFRMIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD_E LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&self) -> LCDBLKOFFIE_R {
        LCDBLKOFFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCD_E LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&self) -> LCDBLKONIE_R {
        LCDBLKONIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_E LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&mut self) -> LCDFRMIFG_W {
        LCDFRMIFG_W { w: self }
    }
    #[doc = "Bit 1 - LCD_E LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&mut self) -> LCDBLKOFFIFG_W {
        LCDBLKOFFIFG_W { w: self }
    }
    #[doc = "Bit 2 - LCD_E LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&mut self) -> LCDBLKONIFG_W {
        LCDBLKONIFG_W { w: self }
    }
    #[doc = "Bit 8 - LCD_E LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&mut self) -> LCDFRMIE_W {
        LCDFRMIE_W { w: self }
    }
    #[doc = "Bit 9 - LCD_E LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&mut self) -> LCDBLKOFFIE_W {
        LCDBLKOFFIE_W { w: self }
    }
    #[doc = "Bit 10 - LCD_E LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&mut self) -> LCDBLKONIE_W {
        LCDBLKONIE_W { w: self }
    }
}
