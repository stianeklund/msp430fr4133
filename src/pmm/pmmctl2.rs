#[doc = "Reader of register PMMCTL2"]
pub type R = crate::R<u16, super::PMMCTL2>;
#[doc = "Writer for register PMMCTL2"]
pub type W = crate::W<u16, super::PMMCTL2>;
#[doc = "Register PMMCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMCTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTREFEN`"]
pub type INTREFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTREFEN`"]
pub struct INTREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTREFEN_W<'a> {
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
#[doc = "Reader of field `EXTREFEN`"]
pub type EXTREFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTREFEN`"]
pub struct EXTREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTREFEN_W<'a> {
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
#[doc = "Reader of field `TSENSOREN`"]
pub type TSENSOREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENSOREN`"]
pub struct TSENSOREN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENSOREN_W<'a> {
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
#[doc = "Reader of field `REFGENACT`"]
pub type REFGENACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFGENACT`"]
pub struct REFGENACT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGENACT_W<'a> {
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
#[doc = "Reader of field `REFBGACT`"]
pub type REFBGACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFBGACT`"]
pub struct REFBGACT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBGACT_W<'a> {
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
#[doc = "Reader of field `BGMODE`"]
pub type BGMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BGMODE`"]
pub struct BGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `REFGENRDY`"]
pub type REFGENRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFGENRDY`"]
pub struct REFGENRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGENRDY_W<'a> {
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
#[doc = "Reader of field `REFBGRDY`"]
pub type REFBGRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFBGRDY`"]
pub struct REFBGRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBGRDY_W<'a> {
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
#[doc = "Reader of field `REFGEN`"]
pub type REFGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFGEN`"]
pub struct REFGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGEN_W<'a> {
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
#[doc = "Reader of field `REFBGEN`"]
pub type REFBGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFBGEN`"]
pub struct REFBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBGEN_W<'a> {
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
#[doc = "Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFVSEL_A {
    #[doc = "0: 00b = 1.5V"]
    REFVSEL_0 = 0,
    #[doc = "1: 01b = 2.0V"]
    REFVSEL_1 = 1,
    #[doc = "2: 10b = 2.5V"]
    REFVSEL_2 = 2,
    #[doc = "3: 11b = Reserved"]
    REFVSEL_3 = 3,
}
impl From<REFVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFVSEL`"]
pub type REFVSEL_R = crate::R<u8, REFVSEL_A>;
impl REFVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFVSEL_A {
        match self.bits {
            0 => REFVSEL_A::REFVSEL_0,
            1 => REFVSEL_A::REFVSEL_1,
            2 => REFVSEL_A::REFVSEL_2,
            3 => REFVSEL_A::REFVSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFVSEL_0`"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_0
    }
    #[doc = "Checks if the value of the field is `REFVSEL_1`"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_1
    }
    #[doc = "Checks if the value of the field is `REFVSEL_2`"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_2
    }
    #[doc = "Checks if the value of the field is `REFVSEL_3`"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_3
    }
}
#[doc = "Write proxy for field `REFVSEL`"]
pub struct REFVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFVSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "00b = 1.5V"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_0)
    }
    #[doc = "01b = 2.0V"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_1)
    }
    #[doc = "10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel_2(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_2)
    }
    #[doc = "11b = Reserved"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    pub fn intrefen(&self) -> INTREFEN_R {
        INTREFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    pub fn extrefen(&self) -> EXTREFEN_R {
        EXTREFEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsensoren(&self) -> TSENSOREN_R {
        TSENSOREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    pub fn refgenrdy(&self) -> REFGENRDY_R {
        REFGENRDY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    pub fn refbgrdy(&self) -> REFBGRDY_R {
        REFBGRDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refgen(&self) -> REFGEN_R {
        REFGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refbgen(&self) -> REFBGEN_R {
        REFBGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel(&self) -> REFVSEL_R {
        REFVSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    pub fn intrefen(&mut self) -> INTREFEN_W {
        INTREFEN_W { w: self }
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    pub fn extrefen(&mut self) -> EXTREFEN_W {
        EXTREFEN_W { w: self }
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsensoren(&mut self) -> TSENSOREN_W {
        TSENSOREN_W { w: self }
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&mut self) -> REFGENACT_W {
        REFGENACT_W { w: self }
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&mut self) -> REFBGACT_W {
        REFBGACT_W { w: self }
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BGMODE_W {
        BGMODE_W { w: self }
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    pub fn refgenrdy(&mut self) -> REFGENRDY_W {
        REFGENRDY_W { w: self }
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    pub fn refbgrdy(&mut self) -> REFBGRDY_W {
        REFBGRDY_W { w: self }
    }
    #[doc = "Bit 6 - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refgen(&mut self) -> REFGEN_W {
        REFGEN_W { w: self }
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refbgen(&mut self) -> REFBGEN_W {
        REFBGEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel(&mut self) -> REFVSEL_W {
        REFVSEL_W { w: self }
    }
}
