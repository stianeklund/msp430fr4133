#[doc = "Reader of register CSCTL7"]
pub type R = crate::R<u16, super::CSCTL7>;
#[doc = "Writer for register CSCTL7"]
pub type W = crate::W<u16, super::CSCTL7>;
#[doc = "Register CSCTL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL7 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCOFFG`"]
pub type DCOFFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCOFFG`"]
pub struct DCOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFFG_W<'a> {
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
#[doc = "Reader of field `XT1OFFG`"]
pub type XT1OFFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XT1OFFG`"]
pub struct XT1OFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1OFFG_W<'a> {
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
#[doc = "Reader of field `FLLULIFG`"]
pub type FLLULIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLLULIFG`"]
pub struct FLLULIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLULIFG_W<'a> {
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
#[doc = "Reader of field `ENSTFCNT1`"]
pub type ENSTFCNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENSTFCNT1`"]
pub struct ENSTFCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTFCNT1_W<'a> {
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
#[doc = "FLL unlock condition Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLLUNLOCK_A {
    #[doc = "0: FLL unlock condition: 0"]
    FLLUNLOCK_0 = 0,
    #[doc = "1: FLL unlock condition: 1"]
    FLLUNLOCK_1 = 1,
    #[doc = "2: FLL unlock condition: 2"]
    FLLUNLOCK_2 = 2,
    #[doc = "3: FLL unlock condition: 3"]
    FLLUNLOCK_3 = 3,
}
impl From<FLLUNLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLUNLOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLLUNLOCK`"]
pub type FLLUNLOCK_R = crate::R<u8, FLLUNLOCK_A>;
impl FLLUNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLUNLOCK_A {
        match self.bits {
            0 => FLLUNLOCK_A::FLLUNLOCK_0,
            1 => FLLUNLOCK_A::FLLUNLOCK_1,
            2 => FLLUNLOCK_A::FLLUNLOCK_2,
            3 => FLLUNLOCK_A::FLLUNLOCK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_0`"]
    #[inline(always)]
    pub fn is_fllunlock_0(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_0
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_1`"]
    #[inline(always)]
    pub fn is_fllunlock_1(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_1
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_2`"]
    #[inline(always)]
    pub fn is_fllunlock_2(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_2
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_3`"]
    #[inline(always)]
    pub fn is_fllunlock_3(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_3
    }
}
#[doc = "Write proxy for field `FLLUNLOCK`"]
pub struct FLLUNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLUNLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLUNLOCK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FLL unlock condition: 0"]
    #[inline(always)]
    pub fn fllunlock_0(self) -> &'a mut W {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_0)
    }
    #[doc = "FLL unlock condition: 1"]
    #[inline(always)]
    pub fn fllunlock_1(self) -> &'a mut W {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_1)
    }
    #[doc = "FLL unlock condition: 2"]
    #[inline(always)]
    pub fn fllunlock_2(self) -> &'a mut W {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_2)
    }
    #[doc = "FLL unlock condition: 3"]
    #[inline(always)]
    pub fn fllunlock_3(self) -> &'a mut W {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Unlock history Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLLUNLOCKHIS_A {
    #[doc = "0: Unlock history: 0"]
    FLLUNLOCKHIS_0 = 0,
    #[doc = "1: Unlock history: 1"]
    FLLUNLOCKHIS_1 = 1,
    #[doc = "2: Unlock history: 2"]
    FLLUNLOCKHIS_2 = 2,
    #[doc = "3: Unlock history: 3"]
    FLLUNLOCKHIS_3 = 3,
}
impl From<FLLUNLOCKHIS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLUNLOCKHIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLLUNLOCKHIS`"]
pub type FLLUNLOCKHIS_R = crate::R<u8, FLLUNLOCKHIS_A>;
impl FLLUNLOCKHIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLUNLOCKHIS_A {
        match self.bits {
            0 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_0,
            1 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_1,
            2 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_2,
            3 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_0`"]
    #[inline(always)]
    pub fn is_fllunlockhis_0(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_0
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_1`"]
    #[inline(always)]
    pub fn is_fllunlockhis_1(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_1
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_2`"]
    #[inline(always)]
    pub fn is_fllunlockhis_2(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_2
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_3`"]
    #[inline(always)]
    pub fn is_fllunlockhis_3(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_3
    }
}
#[doc = "Write proxy for field `FLLUNLOCKHIS`"]
pub struct FLLUNLOCKHIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLUNLOCKHIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLUNLOCKHIS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Unlock history: 0"]
    #[inline(always)]
    pub fn fllunlockhis_0(self) -> &'a mut W {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_0)
    }
    #[doc = "Unlock history: 1"]
    #[inline(always)]
    pub fn fllunlockhis_1(self) -> &'a mut W {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_1)
    }
    #[doc = "Unlock history: 2"]
    #[inline(always)]
    pub fn fllunlockhis_2(self) -> &'a mut W {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_2)
    }
    #[doc = "Unlock history: 3"]
    #[inline(always)]
    pub fn fllunlockhis_3(self) -> &'a mut W {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `FLLULPUC`"]
pub type FLLULPUC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLLULPUC`"]
pub struct FLLULPUC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLULPUC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FLLWARNEN`"]
pub type FLLWARNEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLLWARNEN`"]
pub struct FLLWARNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLWARNEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DCO fault flag"]
    #[inline(always)]
    pub fn dcoffg(&self) -> DCOFFG_R {
        DCOFFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1offg(&self) -> XT1OFFG_R {
        XT1OFFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag"]
    #[inline(always)]
    pub fn fllulifg(&self) -> FLLULIFG_R {
        FLLULIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> ENSTFCNT1_R {
        ENSTFCNT1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - FLL unlock condition Bit: 0"]
    #[inline(always)]
    pub fn fllunlock(&self) -> FLLUNLOCK_R {
        FLLUNLOCK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Unlock history Bit: 0"]
    #[inline(always)]
    pub fn fllunlockhis(&self) -> FLLUNLOCKHIS_R {
        FLLUNLOCKHIS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable"]
    #[inline(always)]
    pub fn fllulpuc(&self) -> FLLULPUC_R {
        FLLULPUC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Warning enable"]
    #[inline(always)]
    pub fn fllwarnen(&self) -> FLLWARNEN_R {
        FLLWARNEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO fault flag"]
    #[inline(always)]
    pub fn dcoffg(&mut self) -> DCOFFG_W {
        DCOFFG_W { w: self }
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1offg(&mut self) -> XT1OFFG_W {
        XT1OFFG_W { w: self }
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag"]
    #[inline(always)]
    pub fn fllulifg(&mut self) -> FLLULIFG_W {
        FLLULIFG_W { w: self }
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    pub fn enstfcnt1(&mut self) -> ENSTFCNT1_W {
        ENSTFCNT1_W { w: self }
    }
    #[doc = "Bits 8:9 - FLL unlock condition Bit: 0"]
    #[inline(always)]
    pub fn fllunlock(&mut self) -> FLLUNLOCK_W {
        FLLUNLOCK_W { w: self }
    }
    #[doc = "Bits 10:11 - Unlock history Bit: 0"]
    #[inline(always)]
    pub fn fllunlockhis(&mut self) -> FLLUNLOCKHIS_W {
        FLLUNLOCKHIS_W { w: self }
    }
    #[doc = "Bit 12 - FLL unlock PUC enable"]
    #[inline(always)]
    pub fn fllulpuc(&mut self) -> FLLULPUC_W {
        FLLULPUC_W { w: self }
    }
    #[doc = "Bit 13 - Warning enable"]
    #[inline(always)]
    pub fn fllwarnen(&mut self) -> FLLWARNEN_W {
        FLLWARNEN_W { w: self }
    }
}
