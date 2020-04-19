#[doc = "Reader of register SYSCFG0"]
pub type R = crate::R<u16, super::SYSCFG0>;
#[doc = "Writer for register SYSCFG0"]
pub type W = crate::W<u16, super::SYSCFG0>;
#[doc = "Register SYSCFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PFWP`"]
pub type PFWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFWP`"]
pub struct PFWP_W<'a> {
    w: &'a mut W,
}
impl<'a> PFWP_W<'a> {
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
#[doc = "Reader of field `DFWP`"]
pub type DFWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFWP`"]
pub struct DFWP_W<'a> {
    w: &'a mut W,
}
impl<'a> DFWP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    pub fn pfwp(&self) -> PFWP_R {
        PFWP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    pub fn dfwp(&self) -> DFWP_R {
        DFWP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    pub fn pfwp(&mut self) -> PFWP_W {
        PFWP_W { w: self }
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    pub fn dfwp(&mut self) -> DFWP_W {
        DFWP_W { w: self }
    }
}
