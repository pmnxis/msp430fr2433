#[doc = "Register `PMMCTL0` reader"]
pub type R = crate::R<Pmmctl0Spec>;
#[doc = "Register `PMMCTL0` writer"]
pub type W = crate::W<Pmmctl0Spec>;
#[doc = "Field `PMMSWBOR` reader - PMM Software BOR"]
pub type PmmswborR = crate::BitReader;
#[doc = "Field `PMMSWBOR` writer - PMM Software BOR"]
pub type PmmswborW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMSWPOR` reader - PMM Software POR"]
pub type PmmswporR = crate::BitReader;
#[doc = "Field `PMMSWPOR` writer - PMM Software POR"]
pub type PmmswporW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMREGOFF` reader - PMM Turn Regulator off"]
pub type PmmregoffR = crate::BitReader;
#[doc = "Field `PMMREGOFF` writer - PMM Turn Regulator off"]
pub type PmmregoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SVS high side enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svshe {
    #[doc = "0: High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    Disabled = 0,
    #[doc = "1: SVSH is always enabled."]
    Enabled = 1,
}
impl From<Svshe> for bool {
    #[inline(always)]
    fn from(variant: Svshe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSHE` reader - SVS high side enable"]
pub type SvsheR = crate::BitReader<Svshe>;
impl SvsheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svshe {
        match self.bits {
            false => Svshe::Disabled,
            true => Svshe::Enabled,
        }
    }
    #[doc = "High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Svshe::Disabled
    }
    #[doc = "SVSH is always enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Svshe::Enabled
    }
}
#[doc = "Field `SVSHE` writer - SVS high side enable"]
pub type SvsheW<'a, REG> = crate::BitWriter<'a, REG, Svshe>;
impl<'a, REG> SvsheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Svshe::Disabled)
    }
    #[doc = "SVSH is always enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Svshe::Enabled)
    }
}
#[doc = "PMM Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmmpwr {
    #[doc = "150: Values always reads from the PMMCTL0 register"]
    Password = 150,
}
impl From<Pmmpwr> for u8 {
    #[inline(always)]
    fn from(variant: Pmmpwr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmmpwr {
    type Ux = u8;
}
impl crate::IsEnum for Pmmpwr {}
#[doc = "Field `PMMPW` reader - PMM Password"]
pub type PmmpwR = crate::FieldReader<Pmmpwr>;
impl PmmpwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmmpwr> {
        match self.bits {
            150 => Some(Pmmpwr::Password),
            _ => None,
        }
    }
    #[doc = "Values always reads from the PMMCTL0 register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Pmmpwr::Password
    }
}
#[doc = "PMM Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmmpwwWO {
    #[doc = "165: Values which must be written to the PMMCTL0 register"]
    Password = 165,
}
impl From<PmmpwwWO> for u8 {
    #[inline(always)]
    fn from(variant: PmmpwwWO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmmpwwWO {
    type Ux = u8;
}
impl crate::IsEnum for PmmpwwWO {}
#[doc = "Field `PMMPW` writer - PMM Password"]
pub type PmmpwW<'a, REG> = crate::FieldWriter<'a, REG, 8, PmmpwwWO>;
impl<'a, REG> PmmpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Values which must be written to the PMMCTL0 register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(PmmpwwWO::Password)
    }
}
impl R {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PmmswborR {
        PmmswborR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PmmswporR {
        PmmswporR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PmmregoffR {
        PmmregoffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&self) -> SvsheR {
        SvsheR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PMM Password"]
    #[inline(always)]
    pub fn pmmpw(&self) -> PmmpwR {
        PmmpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PmmswborW<'_, Pmmctl0Spec> {
        PmmswborW::new(self, 2)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PmmswporW<'_, Pmmctl0Spec> {
        PmmswporW::new(self, 3)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PmmregoffW<'_, Pmmctl0Spec> {
        PmmregoffW::new(self, 4)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&mut self) -> SvsheW<'_, Pmmctl0Spec> {
        SvsheW::new(self, 6)
    }
    #[doc = "Bits 8:15 - PMM Password"]
    #[inline(always)]
    pub fn pmmpw(&mut self) -> PmmpwW<'_, Pmmctl0Spec> {
        PmmpwW::new(self, 8)
    }
}
#[doc = "PMM Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmmctl0Spec;
impl crate::RegisterSpec for Pmmctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl0::R`](R) reader structure"]
impl crate::Readable for Pmmctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmmctl0::W`](W) writer structure"]
impl crate::Writable for Pmmctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMCTL0 to value 0"]
impl crate::Resettable for Pmmctl0Spec {}
