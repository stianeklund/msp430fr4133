#[doc = "Reader of register PM5CTL0"]
pub type R = crate::R<u16, super::PM5CTL0>;
#[doc = "Writer for register PM5CTL0"]
pub type W = crate::W<u16, super::PM5CTL0>;
#[doc = "Register PM5CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PM5CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCKLPM5`"]
pub type LOCKLPM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKLPM5`"]
pub struct LOCKLPM5_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKLPM5_W<'a> {
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
#[doc = "Reader of field `LPM5SW`"]
pub type LPM5SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM5SW`"]
pub struct LPM5SW_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM5SW_W<'a> {
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
#[doc = "Reader of field `LPM5SM`"]
pub type LPM5SM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM5SM`"]
pub struct LPM5SM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM5SM_W<'a> {
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
    #[doc = "Bit 0 - Lock I/O pin configuration upon entry/exit to/from LPM5"]
    #[inline(always)]
    pub fn locklpm5(&self) -> LOCKLPM5_R {
        LOCKLPM5_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - LPMx.5 switch dis/connected"]
    #[inline(always)]
    pub fn lpm5sw(&self) -> LPM5SW_R {
        LPM5SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Manual mode for LPM3.5 switch"]
    #[inline(always)]
    pub fn lpm5sm(&self) -> LPM5SM_R {
        LPM5SM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock I/O pin configuration upon entry/exit to/from LPM5"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> LOCKLPM5_W {
        LOCKLPM5_W { w: self }
    }
    #[doc = "Bit 4 - LPMx.5 switch dis/connected"]
    #[inline(always)]
    pub fn lpm5sw(&mut self) -> LPM5SW_W {
        LPM5SW_W { w: self }
    }
    #[doc = "Bit 5 - Manual mode for LPM3.5 switch"]
    #[inline(always)]
    pub fn lpm5sm(&mut self) -> LPM5SM_W {
        LPM5SM_W { w: self }
    }
}
