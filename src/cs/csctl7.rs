#[doc = "Register `CSCTL7` reader"]
pub type R = crate::R<Csctl7Spec>;
#[doc = "Register `CSCTL7` writer"]
pub type W = crate::W<Csctl7Spec>;
#[doc = "Field `DCOFFG` reader - DCO fault flag"]
pub type DcoffgR = crate::BitReader;
#[doc = "Field `DCOFFG` writer - DCO fault flag"]
pub type DcoffgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT1OFFG` reader - XT1 Low Frequency Oscillator Fault Flag"]
pub type Xt1offgR = crate::BitReader;
#[doc = "Field `XT1OFFG` writer - XT1 Low Frequency Oscillator Fault Flag"]
pub type Xt1offgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLULIFG` reader - FLL unlock interrupt flag"]
pub type FllulifgR = crate::BitReader;
#[doc = "Field `FLLULIFG` writer - FLL unlock interrupt flag"]
pub type FllulifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENSTFCNT1` reader - Enable start counter for XT1"]
pub type Enstfcnt1R = crate::BitReader;
#[doc = "Field `ENSTFCNT1` writer - Enable start counter for XT1"]
pub type Enstfcnt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FLL unlock condition Bit: 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fllunlock {
    #[doc = "0: FLL is locked. No unlock condition currently active."]
    Locked = 0,
    #[doc = "1: DCOCLK is currently too slow."]
    TooSlow = 1,
    #[doc = "2: DCOCLK is currently too fast."]
    TooFast = 2,
    #[doc = "3: DCOERROR. DCO out of range."]
    OutOfRange = 3,
}
impl From<Fllunlock> for u8 {
    #[inline(always)]
    fn from(variant: Fllunlock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fllunlock {
    type Ux = u8;
}
impl crate::IsEnum for Fllunlock {}
#[doc = "Field `FLLUNLOCK` reader - FLL unlock condition Bit: 0"]
pub type FllunlockR = crate::FieldReader<Fllunlock>;
impl FllunlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fllunlock {
        match self.bits {
            0 => Fllunlock::Locked,
            1 => Fllunlock::TooSlow,
            2 => Fllunlock::TooFast,
            3 => Fllunlock::OutOfRange,
            _ => unreachable!(),
        }
    }
    #[doc = "FLL is locked. No unlock condition currently active."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Fllunlock::Locked
    }
    #[doc = "DCOCLK is currently too slow."]
    #[inline(always)]
    pub fn is_too_slow(&self) -> bool {
        *self == Fllunlock::TooSlow
    }
    #[doc = "DCOCLK is currently too fast."]
    #[inline(always)]
    pub fn is_too_fast(&self) -> bool {
        *self == Fllunlock::TooFast
    }
    #[doc = "DCOERROR. DCO out of range."]
    #[inline(always)]
    pub fn is_out_of_range(&self) -> bool {
        *self == Fllunlock::OutOfRange
    }
}
#[doc = "Field `FLLUNLOCK` writer - FLL unlock condition Bit: 0"]
pub type FllunlockW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fllunlock, crate::Safe>;
impl<'a, REG> FllunlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLL is locked. No unlock condition currently active."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlock::Locked)
    }
    #[doc = "DCOCLK is currently too slow."]
    #[inline(always)]
    pub fn too_slow(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlock::TooSlow)
    }
    #[doc = "DCOCLK is currently too fast."]
    #[inline(always)]
    pub fn too_fast(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlock::TooFast)
    }
    #[doc = "DCOERROR. DCO out of range."]
    #[inline(always)]
    pub fn out_of_range(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlock::OutOfRange)
    }
}
#[doc = "Unlock history Bit: 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fllunlockhis {
    #[doc = "0: FLL is locked. No unlock situation has been detected since the last reset of these bits."]
    Locked = 0,
    #[doc = "1: DCOCLK has been too slow since the bits were cleared."]
    TooSlow = 1,
    #[doc = "2: DCOCLK has been too fast since the bits were cleared."]
    TooFast = 2,
    #[doc = "3: DCOCLK has been both too fast and too slow since the bits were cleared."]
    TooSlowAndFast = 3,
}
impl From<Fllunlockhis> for u8 {
    #[inline(always)]
    fn from(variant: Fllunlockhis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fllunlockhis {
    type Ux = u8;
}
impl crate::IsEnum for Fllunlockhis {}
#[doc = "Field `FLLUNLOCKHIS` reader - Unlock history Bit: 0"]
pub type FllunlockhisR = crate::FieldReader<Fllunlockhis>;
impl FllunlockhisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fllunlockhis {
        match self.bits {
            0 => Fllunlockhis::Locked,
            1 => Fllunlockhis::TooSlow,
            2 => Fllunlockhis::TooFast,
            3 => Fllunlockhis::TooSlowAndFast,
            _ => unreachable!(),
        }
    }
    #[doc = "FLL is locked. No unlock situation has been detected since the last reset of these bits."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Fllunlockhis::Locked
    }
    #[doc = "DCOCLK has been too slow since the bits were cleared."]
    #[inline(always)]
    pub fn is_too_slow(&self) -> bool {
        *self == Fllunlockhis::TooSlow
    }
    #[doc = "DCOCLK has been too fast since the bits were cleared."]
    #[inline(always)]
    pub fn is_too_fast(&self) -> bool {
        *self == Fllunlockhis::TooFast
    }
    #[doc = "DCOCLK has been both too fast and too slow since the bits were cleared."]
    #[inline(always)]
    pub fn is_too_slow_and_fast(&self) -> bool {
        *self == Fllunlockhis::TooSlowAndFast
    }
}
#[doc = "Field `FLLUNLOCKHIS` writer - Unlock history Bit: 0"]
pub type FllunlockhisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fllunlockhis, crate::Safe>;
impl<'a, REG> FllunlockhisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLL is locked. No unlock situation has been detected since the last reset of these bits."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlockhis::Locked)
    }
    #[doc = "DCOCLK has been too slow since the bits were cleared."]
    #[inline(always)]
    pub fn too_slow(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlockhis::TooSlow)
    }
    #[doc = "DCOCLK has been too fast since the bits were cleared."]
    #[inline(always)]
    pub fn too_fast(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlockhis::TooFast)
    }
    #[doc = "DCOCLK has been both too fast and too slow since the bits were cleared."]
    #[inline(always)]
    pub fn too_slow_and_fast(self) -> &'a mut crate::W<REG> {
        self.variant(Fllunlockhis::TooSlowAndFast)
    }
}
#[doc = "Field `FLLULPUC` reader - FLL unlock PUC enable"]
pub type FllulpucR = crate::BitReader;
#[doc = "Field `FLLULPUC` writer - FLL unlock PUC enable"]
pub type FllulpucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Warning enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fllwarnen {
    #[doc = "0: FLLUNLOCKHIS status cannot set OFIFG."]
    Disabled = 0,
    #[doc = "1: FLLUNLOCKHIS status can set OFIFG."]
    Enabled = 1,
}
impl From<Fllwarnen> for bool {
    #[inline(always)]
    fn from(variant: Fllwarnen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLLWARNEN` reader - Warning enable"]
pub type FllwarnenR = crate::BitReader<Fllwarnen>;
impl FllwarnenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fllwarnen {
        match self.bits {
            false => Fllwarnen::Disabled,
            true => Fllwarnen::Enabled,
        }
    }
    #[doc = "FLLUNLOCKHIS status cannot set OFIFG."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fllwarnen::Disabled
    }
    #[doc = "FLLUNLOCKHIS status can set OFIFG."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fllwarnen::Enabled
    }
}
#[doc = "Field `FLLWARNEN` writer - Warning enable"]
pub type FllwarnenW<'a, REG> = crate::BitWriter<'a, REG, Fllwarnen>;
impl<'a, REG> FllwarnenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLLUNLOCKHIS status cannot set OFIFG."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fllwarnen::Disabled)
    }
    #[doc = "FLLUNLOCKHIS status can set OFIFG."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fllwarnen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DCO fault flag"]
    #[inline(always)]
    pub fn dcoffg(&self) -> DcoffgR {
        DcoffgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1offg(&self) -> Xt1offgR {
        Xt1offgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag"]
    #[inline(always)]
    pub fn fllulifg(&self) -> FllulifgR {
        FllulifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> Enstfcnt1R {
        Enstfcnt1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - FLL unlock condition Bit: 0"]
    #[inline(always)]
    pub fn fllunlock(&self) -> FllunlockR {
        FllunlockR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Unlock history Bit: 0"]
    #[inline(always)]
    pub fn fllunlockhis(&self) -> FllunlockhisR {
        FllunlockhisR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable"]
    #[inline(always)]
    pub fn fllulpuc(&self) -> FllulpucR {
        FllulpucR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Warning enable"]
    #[inline(always)]
    pub fn fllwarnen(&self) -> FllwarnenR {
        FllwarnenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO fault flag"]
    #[inline(always)]
    pub fn dcoffg(&mut self) -> DcoffgW<'_, Csctl7Spec> {
        DcoffgW::new(self, 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1offg(&mut self) -> Xt1offgW<'_, Csctl7Spec> {
        Xt1offgW::new(self, 1)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag"]
    #[inline(always)]
    pub fn fllulifg(&mut self) -> FllulifgW<'_, Csctl7Spec> {
        FllulifgW::new(self, 4)
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    pub fn enstfcnt1(&mut self) -> Enstfcnt1W<'_, Csctl7Spec> {
        Enstfcnt1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - FLL unlock condition Bit: 0"]
    #[inline(always)]
    pub fn fllunlock(&mut self) -> FllunlockW<'_, Csctl7Spec> {
        FllunlockW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Unlock history Bit: 0"]
    #[inline(always)]
    pub fn fllunlockhis(&mut self) -> FllunlockhisW<'_, Csctl7Spec> {
        FllunlockhisW::new(self, 10)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable"]
    #[inline(always)]
    pub fn fllulpuc(&mut self) -> FllulpucW<'_, Csctl7Spec> {
        FllulpucW::new(self, 12)
    }
    #[doc = "Bit 13 - Warning enable"]
    #[inline(always)]
    pub fn fllwarnen(&mut self) -> FllwarnenW<'_, Csctl7Spec> {
        FllwarnenW::new(self, 13)
    }
}
#[doc = "CS Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl7Spec;
impl crate::RegisterSpec for Csctl7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl7::R`](R) reader structure"]
impl crate::Readable for Csctl7Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl7::W`](W) writer structure"]
impl crate::Writable for Csctl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL7 to value 0"]
impl crate::Resettable for Csctl7Spec {}
