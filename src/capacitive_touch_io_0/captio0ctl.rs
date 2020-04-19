#[doc = "Reader of register CAPTIO0CTL"]
pub type R = crate::R<u16, super::CAPTIO0CTL>;
#[doc = "Writer for register CAPTIO0CTL"]
pub type W = crate::W<u16, super::CAPTIO0CTL>;
#[doc = "Register CAPTIO0CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CAPTIO0CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPTIOPISEL0`"]
pub type CAPTIOPISEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTIOPISEL0`"]
pub struct CAPTIOPISEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPISEL0_W<'a> {
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
#[doc = "Reader of field `CAPTIOPISEL1`"]
pub type CAPTIOPISEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTIOPISEL1`"]
pub struct CAPTIOPISEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPISEL1_W<'a> {
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
#[doc = "Reader of field `CAPTIOPISEL2`"]
pub type CAPTIOPISEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTIOPISEL2`"]
pub struct CAPTIOPISEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPISEL2_W<'a> {
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
#[doc = "Reader of field `CAPTIOPOSEL0`"]
pub type CAPTIOPOSEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTIOPOSEL0`"]
pub struct CAPTIOPOSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPOSEL0_W<'a> {
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
#[doc = "Reader of field `CAPTIOPOSEL1`"]
pub type CAPTIOPOSEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTIOPOSEL1`"]
pub struct CAPTIOPOSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPOSEL1_W<'a> {
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
#[doc = "Reader of field `CAPTIOPOSEL2`"]
pub type CAPTIOPOSEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTIOPOSEL2`"]
pub struct CAPTIOPOSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPOSEL2_W<'a> {
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
#[doc = "Reader of field `CAPTIOPOSEL3`"]
pub type CAPTIOPOSEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTIOPOSEL3`"]
pub struct CAPTIOPOSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPOSEL3_W<'a> {
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
#[doc = "Reader of field `CAPTIOEN`"]
pub type CAPTIOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTIOEN`"]
pub struct CAPTIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOEN_W<'a> {
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
#[doc = "Reader of field `CAPTIO`"]
pub type CAPTIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTIO`"]
pub struct CAPTIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIO_W<'a> {
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
impl R {
    #[doc = "Bit 1 - CapTouchIO Pin Select Bit: 0"]
    #[inline(always)]
    pub fn captiopisel0(&self) -> CAPTIOPISEL0_R {
        CAPTIOPISEL0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CapTouchIO Pin Select Bit: 1"]
    #[inline(always)]
    pub fn captiopisel1(&self) -> CAPTIOPISEL1_R {
        CAPTIOPISEL1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CapTouchIO Pin Select Bit: 2"]
    #[inline(always)]
    pub fn captiopisel2(&self) -> CAPTIOPISEL2_R {
        CAPTIOPISEL2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CapTouchIO Port Select Bit: 0"]
    #[inline(always)]
    pub fn captioposel0(&self) -> CAPTIOPOSEL0_R {
        CAPTIOPOSEL0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CapTouchIO Port Select Bit: 1"]
    #[inline(always)]
    pub fn captioposel1(&self) -> CAPTIOPOSEL1_R {
        CAPTIOPOSEL1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CapTouchIO Port Select Bit: 2"]
    #[inline(always)]
    pub fn captioposel2(&self) -> CAPTIOPOSEL2_R {
        CAPTIOPOSEL2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CapTouchIO Port Select Bit: 3"]
    #[inline(always)]
    pub fn captioposel3(&self) -> CAPTIOPOSEL3_R {
        CAPTIOPOSEL3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CapTouchIO Enable"]
    #[inline(always)]
    pub fn captioen(&self) -> CAPTIOEN_R {
        CAPTIOEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CapTouchIO state"]
    #[inline(always)]
    pub fn captio(&self) -> CAPTIO_R {
        CAPTIO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CapTouchIO Pin Select Bit: 0"]
    #[inline(always)]
    pub fn captiopisel0(&mut self) -> CAPTIOPISEL0_W {
        CAPTIOPISEL0_W { w: self }
    }
    #[doc = "Bit 2 - CapTouchIO Pin Select Bit: 1"]
    #[inline(always)]
    pub fn captiopisel1(&mut self) -> CAPTIOPISEL1_W {
        CAPTIOPISEL1_W { w: self }
    }
    #[doc = "Bit 3 - CapTouchIO Pin Select Bit: 2"]
    #[inline(always)]
    pub fn captiopisel2(&mut self) -> CAPTIOPISEL2_W {
        CAPTIOPISEL2_W { w: self }
    }
    #[doc = "Bit 4 - CapTouchIO Port Select Bit: 0"]
    #[inline(always)]
    pub fn captioposel0(&mut self) -> CAPTIOPOSEL0_W {
        CAPTIOPOSEL0_W { w: self }
    }
    #[doc = "Bit 5 - CapTouchIO Port Select Bit: 1"]
    #[inline(always)]
    pub fn captioposel1(&mut self) -> CAPTIOPOSEL1_W {
        CAPTIOPOSEL1_W { w: self }
    }
    #[doc = "Bit 6 - CapTouchIO Port Select Bit: 2"]
    #[inline(always)]
    pub fn captioposel2(&mut self) -> CAPTIOPOSEL2_W {
        CAPTIOPOSEL2_W { w: self }
    }
    #[doc = "Bit 7 - CapTouchIO Port Select Bit: 3"]
    #[inline(always)]
    pub fn captioposel3(&mut self) -> CAPTIOPOSEL3_W {
        CAPTIOPOSEL3_W { w: self }
    }
    #[doc = "Bit 8 - CapTouchIO Enable"]
    #[inline(always)]
    pub fn captioen(&mut self) -> CAPTIOEN_W {
        CAPTIOEN_W { w: self }
    }
    #[doc = "Bit 9 - CapTouchIO state"]
    #[inline(always)]
    pub fn captio(&mut self) -> CAPTIO_W {
        CAPTIO_W { w: self }
    }
}
