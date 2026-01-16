#[doc = "Register `PMMCTL2` reader"]
pub type R = crate::R<Pmmctl2Spec>;
#[doc = "Register `PMMCTL2` writer"]
pub type W = crate::W<Pmmctl2Spec>;
#[doc = "Field `INTREFEN` reader - Internal Reference Enable"]
pub type IntrefenR = crate::BitReader;
#[doc = "Field `INTREFEN` writer - Internal Reference Enable"]
pub type IntrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTREFEN` reader - External Reference output Enable"]
pub type ExtrefenR = crate::BitReader;
#[doc = "Field `EXTREFEN` writer - External Reference output Enable"]
pub type ExtrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENSOREN` reader - Temperature Sensor Enable"]
pub type TsensorenR = crate::BitReader;
#[doc = "Field `TSENSOREN` writer - Temperature Sensor Enable"]
pub type TsensorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refvsel {
    #[doc = "0: 00b = 1.5V"]
    Refvsel0 = 0,
    #[doc = "1: 01b = 2.0V"]
    Refvsel1 = 1,
    #[doc = "2: 10b = 2.5V"]
    Refvsel2 = 2,
    #[doc = "3: 11b = Reserved"]
    Refvsel3 = 3,
}
impl From<Refvsel> for u8 {
    #[inline(always)]
    fn from(variant: Refvsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refvsel {
    type Ux = u8;
}
impl crate::IsEnum for Refvsel {}
#[doc = "Field `REFVSEL` reader - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
pub type RefvselR = crate::FieldReader<Refvsel>;
impl RefvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refvsel {
        match self.bits {
            0 => Refvsel::Refvsel0,
            1 => Refvsel::Refvsel1,
            2 => Refvsel::Refvsel2,
            3 => Refvsel::Refvsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "00b = 1.5V"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == Refvsel::Refvsel0
    }
    #[doc = "01b = 2.0V"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == Refvsel::Refvsel1
    }
    #[doc = "10b = 2.5V"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        *self == Refvsel::Refvsel2
    }
    #[doc = "11b = Reserved"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == Refvsel::Refvsel3
    }
}
#[doc = "Field `REFVSEL` writer - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
pub type RefvselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refvsel, crate::Safe>;
impl<'a, REG> RefvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "00b = 1.5V"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel0)
    }
    #[doc = "01b = 2.0V"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel1)
    }
    #[doc = "10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel2)
    }
    #[doc = "11b = Reserved"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel3)
    }
}
#[doc = "Field `REFGEN` reader - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
pub type RefgenR = crate::BitReader;
#[doc = "Field `REFGEN` writer - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
pub type RefgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBGEN` reader - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
pub type RefbgenR = crate::BitReader;
#[doc = "Field `REFBGEN` writer - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
pub type RefbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFGENACT` reader - REF Reference generator active"]
pub type RefgenactR = crate::BitReader;
#[doc = "Field `REFGENACT` writer - REF Reference generator active"]
pub type RefgenactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBGACT` reader - REF Reference bandgap active"]
pub type RefbgactR = crate::BitReader;
#[doc = "Field `REFBGACT` writer - REF Reference bandgap active"]
pub type RefbgactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGMODE` reader - REF Bandgap mode"]
pub type BgmodeR = crate::BitReader;
#[doc = "Field `BGMODE` writer - REF Bandgap mode"]
pub type BgmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFGENRDY` reader - REF Reference generator ready"]
pub type RefgenrdyR = crate::BitReader;
#[doc = "Field `REFGENRDY` writer - REF Reference generator ready"]
pub type RefgenrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBGRDY` reader - REF Reference bandgap ready"]
pub type RefbgrdyR = crate::BitReader;
#[doc = "Field `REFBGRDY` writer - REF Reference bandgap ready"]
pub type RefbgrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    pub fn intrefen(&self) -> IntrefenR {
        IntrefenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    pub fn extrefen(&self) -> ExtrefenR {
        ExtrefenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsensoren(&self) -> TsensorenR {
        TsensorenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel(&self) -> RefvselR {
        RefvselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refgen(&self) -> RefgenR {
        RefgenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refbgen(&self) -> RefbgenR {
        RefbgenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> RefgenactR {
        RefgenactR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> RefbgactR {
        RefbgactR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BgmodeR {
        BgmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    pub fn refgenrdy(&self) -> RefgenrdyR {
        RefgenrdyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    pub fn refbgrdy(&self) -> RefbgrdyR {
        RefbgrdyR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    pub fn intrefen(&mut self) -> IntrefenW<'_, Pmmctl2Spec> {
        IntrefenW::new(self, 0)
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    pub fn extrefen(&mut self) -> ExtrefenW<'_, Pmmctl2Spec> {
        ExtrefenW::new(self, 1)
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsensoren(&mut self) -> TsensorenW<'_, Pmmctl2Spec> {
        TsensorenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel(&mut self) -> RefvselW<'_, Pmmctl2Spec> {
        RefvselW::new(self, 4)
    }
    #[doc = "Bit 6 - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refgen(&mut self) -> RefgenW<'_, Pmmctl2Spec> {
        RefgenW::new(self, 6)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refbgen(&mut self) -> RefbgenW<'_, Pmmctl2Spec> {
        RefbgenW::new(self, 7)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&mut self) -> RefgenactW<'_, Pmmctl2Spec> {
        RefgenactW::new(self, 8)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&mut self) -> RefbgactW<'_, Pmmctl2Spec> {
        RefbgactW::new(self, 9)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BgmodeW<'_, Pmmctl2Spec> {
        BgmodeW::new(self, 11)
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    pub fn refgenrdy(&mut self) -> RefgenrdyW<'_, Pmmctl2Spec> {
        RefgenrdyW::new(self, 12)
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    pub fn refbgrdy(&mut self) -> RefbgrdyW<'_, Pmmctl2Spec> {
        RefbgrdyW::new(self, 13)
    }
}
#[doc = "PMM Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmmctl2Spec;
impl crate::RegisterSpec for Pmmctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl2::R`](R) reader structure"]
impl crate::Readable for Pmmctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pmmctl2::W`](W) writer structure"]
impl crate::Writable for Pmmctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMCTL2 to value 0"]
impl crate::Resettable for Pmmctl2Spec {}
