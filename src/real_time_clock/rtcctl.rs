#[doc = "Reader of register RTCCTL"]
pub type R = crate::R<u16, super::RTCCTL>;
#[doc = "Writer for register RTCCTL"]
pub type W = crate::W<u16, super::RTCCTL>;
#[doc = "Register RTCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCIF`"]
pub type RTCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCIF`"]
pub struct RTCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCIF_W<'a> {
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
#[doc = "Reader of field `RTCIE`"]
pub type RTCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCIE`"]
pub struct RTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCIE_W<'a> {
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
#[doc = "Reader of field `RTCSR`"]
pub type RTCSR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCSR`"]
pub struct RTCSR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSR_W<'a> {
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
#[doc = "Low-Power-Counter Clock Pre-divider Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCPS_A {
    #[doc = "0: Low-Power-Counter Clock Pre-divider Select: 0"]
    RTCPS_0 = 0,
    #[doc = "1: Low-Power-Counter Clock Pre-divider Select: 1"]
    RTCPS_1 = 1,
    #[doc = "2: Low-Power-Counter Clock Pre-divider Select: 2"]
    RTCPS_2 = 2,
    #[doc = "3: Low-Power-Counter Clock Pre-divider Select: 3"]
    RTCPS_3 = 3,
    #[doc = "4: Low-Power-Counter Clock Pre-divider Select: 4"]
    RTCPS_4 = 4,
    #[doc = "5: Low-Power-Counter Clock Pre-divider Select: 5"]
    RTCPS_5 = 5,
    #[doc = "6: Low-Power-Counter Clock Pre-divider Select: 6"]
    RTCPS_6 = 6,
    #[doc = "7: Low-Power-Counter Clock Pre-divider Select: 7"]
    RTCPS_7 = 7,
}
impl From<RTCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCPS`"]
pub type RTCPS_R = crate::R<u8, RTCPS_A>;
impl RTCPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPS_A {
        match self.bits {
            0 => RTCPS_A::RTCPS_0,
            1 => RTCPS_A::RTCPS_1,
            2 => RTCPS_A::RTCPS_2,
            3 => RTCPS_A::RTCPS_3,
            4 => RTCPS_A::RTCPS_4,
            5 => RTCPS_A::RTCPS_5,
            6 => RTCPS_A::RTCPS_6,
            7 => RTCPS_A::RTCPS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCPS_0`"]
    #[inline(always)]
    pub fn is_rtcps_0(&self) -> bool {
        *self == RTCPS_A::RTCPS_0
    }
    #[doc = "Checks if the value of the field is `RTCPS_1`"]
    #[inline(always)]
    pub fn is_rtcps_1(&self) -> bool {
        *self == RTCPS_A::RTCPS_1
    }
    #[doc = "Checks if the value of the field is `RTCPS_2`"]
    #[inline(always)]
    pub fn is_rtcps_2(&self) -> bool {
        *self == RTCPS_A::RTCPS_2
    }
    #[doc = "Checks if the value of the field is `RTCPS_3`"]
    #[inline(always)]
    pub fn is_rtcps_3(&self) -> bool {
        *self == RTCPS_A::RTCPS_3
    }
    #[doc = "Checks if the value of the field is `RTCPS_4`"]
    #[inline(always)]
    pub fn is_rtcps_4(&self) -> bool {
        *self == RTCPS_A::RTCPS_4
    }
    #[doc = "Checks if the value of the field is `RTCPS_5`"]
    #[inline(always)]
    pub fn is_rtcps_5(&self) -> bool {
        *self == RTCPS_A::RTCPS_5
    }
    #[doc = "Checks if the value of the field is `RTCPS_6`"]
    #[inline(always)]
    pub fn is_rtcps_6(&self) -> bool {
        *self == RTCPS_A::RTCPS_6
    }
    #[doc = "Checks if the value of the field is `RTCPS_7`"]
    #[inline(always)]
    pub fn is_rtcps_7(&self) -> bool {
        *self == RTCPS_A::RTCPS_7
    }
}
#[doc = "Write proxy for field `RTCPS`"]
pub struct RTCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 0"]
    #[inline(always)]
    pub fn rtcps_0(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_0)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 1"]
    #[inline(always)]
    pub fn rtcps_1(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_1)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 2"]
    #[inline(always)]
    pub fn rtcps_2(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_2)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 3"]
    #[inline(always)]
    pub fn rtcps_3(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_3)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 4"]
    #[inline(always)]
    pub fn rtcps_4(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_4)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 5"]
    #[inline(always)]
    pub fn rtcps_5(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_5)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 6"]
    #[inline(always)]
    pub fn rtcps_6(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_6)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 7"]
    #[inline(always)]
    pub fn rtcps_7(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Low-Power-Counter Clock Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSS_A {
    #[doc = "0: Low-Power-Counter Clock Source Select: 0"]
    RTCSS_0 = 0,
    #[doc = "1: Low-Power-Counter Clock Source Select: 1"]
    RTCSS_1 = 1,
    #[doc = "2: Low-Power-Counter Clock Source Select: 2"]
    RTCSS_2 = 2,
    #[doc = "3: Low-Power-Counter Clock Source Select: 3"]
    RTCSS_3 = 3,
}
impl From<RTCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCSS`"]
pub type RTCSS_R = crate::R<u8, RTCSS_A>;
impl RTCSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSS_A {
        match self.bits {
            0 => RTCSS_A::RTCSS_0,
            1 => RTCSS_A::RTCSS_1,
            2 => RTCSS_A::RTCSS_2,
            3 => RTCSS_A::RTCSS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCSS_0`"]
    #[inline(always)]
    pub fn is_rtcss_0(&self) -> bool {
        *self == RTCSS_A::RTCSS_0
    }
    #[doc = "Checks if the value of the field is `RTCSS_1`"]
    #[inline(always)]
    pub fn is_rtcss_1(&self) -> bool {
        *self == RTCSS_A::RTCSS_1
    }
    #[doc = "Checks if the value of the field is `RTCSS_2`"]
    #[inline(always)]
    pub fn is_rtcss_2(&self) -> bool {
        *self == RTCSS_A::RTCSS_2
    }
    #[doc = "Checks if the value of the field is `RTCSS_3`"]
    #[inline(always)]
    pub fn is_rtcss_3(&self) -> bool {
        *self == RTCSS_A::RTCSS_3
    }
}
#[doc = "Write proxy for field `RTCSS`"]
pub struct RTCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low-Power-Counter Clock Source Select: 0"]
    #[inline(always)]
    pub fn rtcss_0(self) -> &'a mut W {
        self.variant(RTCSS_A::RTCSS_0)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 1"]
    #[inline(always)]
    pub fn rtcss_1(self) -> &'a mut W {
        self.variant(RTCSS_A::RTCSS_1)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 2"]
    #[inline(always)]
    pub fn rtcss_2(self) -> &'a mut W {
        self.variant(RTCSS_A::RTCSS_2)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 3"]
    #[inline(always)]
    pub fn rtcss_3(self) -> &'a mut W {
        self.variant(RTCSS_A::RTCSS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    pub fn rtcif(&self) -> RTCIF_R {
        RTCIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    pub fn rtcsr(&self) -> RTCSR_R {
        RTCSR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    pub fn rtcps(&self) -> RTCPS_R {
        RTCPS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn rtcss(&self) -> RTCSS_R {
        RTCSS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    pub fn rtcif(&mut self) -> RTCIF_W {
        RTCIF_W { w: self }
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    pub fn rtcie(&mut self) -> RTCIE_W {
        RTCIE_W { w: self }
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    pub fn rtcsr(&mut self) -> RTCSR_W {
        RTCSR_W { w: self }
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RTCPS_W {
        RTCPS_W { w: self }
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn rtcss(&mut self) -> RTCSS_W {
        RTCSS_W { w: self }
    }
}
