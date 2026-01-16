#[doc = "Register `WDTCTL` reader"]
pub type R = crate::R<WdtctlSpec>;
#[doc = "Register `WDTCTL` writer"]
pub type W = crate::W<WdtctlSpec>;
#[doc = "Field `WDTIS` reader - WDT - Timer Interval Select 0"]
pub type WdtisR = crate::FieldReader;
#[doc = "Field `WDTIS` writer - WDT - Timer Interval Select 0"]
pub type WdtisW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDTCNTCL` reader - WDT - Timer Clear"]
pub type WdtcntclR = crate::BitReader;
#[doc = "Field `WDTCNTCL` writer - WDT - Timer Clear"]
pub type WdtcntclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTTMSEL` reader - WDT - Timer Mode Select"]
pub type WdttmselR = crate::BitReader;
#[doc = "Field `WDTTMSEL` writer - WDT - Timer Mode Select"]
pub type WdttmselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "WDT - Timer Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtssel {
    #[doc = "0: WDT - Timer Clock Source Select: SMCLK"]
    Wdtssel0 = 0,
    #[doc = "1: WDT - Timer Clock Source Select: ACLK"]
    Wdtssel1 = 1,
    #[doc = "2: WDT - Timer Clock Source Select: VLO_CLK"]
    Wdtssel2 = 2,
    #[doc = "3: WDT - Timer Clock Source Select: reserved"]
    Wdtssel3 = 3,
}
impl From<Wdtssel> for u8 {
    #[inline(always)]
    fn from(variant: Wdtssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtssel {
    type Ux = u8;
}
impl crate::IsEnum for Wdtssel {}
#[doc = "Field `WDTSSEL` reader - WDT - Timer Clock Source Select 0"]
pub type WdtsselR = crate::FieldReader<Wdtssel>;
impl WdtsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtssel {
        match self.bits {
            0 => Wdtssel::Wdtssel0,
            1 => Wdtssel::Wdtssel1,
            2 => Wdtssel::Wdtssel2,
            3 => Wdtssel::Wdtssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn is_wdtssel_0(&self) -> bool {
        *self == Wdtssel::Wdtssel0
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn is_wdtssel_1(&self) -> bool {
        *self == Wdtssel::Wdtssel1
    }
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    #[inline(always)]
    pub fn is_wdtssel_2(&self) -> bool {
        *self == Wdtssel::Wdtssel2
    }
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    #[inline(always)]
    pub fn is_wdtssel_3(&self) -> bool {
        *self == Wdtssel::Wdtssel3
    }
}
#[doc = "Field `WDTSSEL` writer - WDT - Timer Clock Source Select 0"]
pub type WdtsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wdtssel, crate::Safe>;
impl<'a, REG> WdtsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn wdtssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Wdtssel0)
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn wdtssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Wdtssel1)
    }
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    #[inline(always)]
    pub fn wdtssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Wdtssel2)
    }
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    #[inline(always)]
    pub fn wdtssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Wdtssel3)
    }
}
#[doc = "Field `WDTHOLD` reader - WDT - Timer hold"]
pub type WdtholdR = crate::BitReader;
#[doc = "Field `WDTHOLD` writer - WDT - Timer hold"]
pub type WdtholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtpwr {
    #[doc = "105: Value always read from the Watchdog Password register"]
    Password = 105,
}
impl From<Wdtpwr> for u8 {
    #[inline(always)]
    fn from(variant: Wdtpwr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtpwr {
    type Ux = u8;
}
impl crate::IsEnum for Wdtpwr {}
#[doc = "Field `WDTPW` reader - Watchdog Timer Password"]
pub type WdtpwR = crate::FieldReader<Wdtpwr>;
impl WdtpwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wdtpwr> {
        match self.bits {
            105 => Some(Wdtpwr::Password),
            _ => None,
        }
    }
    #[doc = "Value always read from the Watchdog Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Wdtpwr::Password
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WdtpwwWO {
    #[doc = "90: Value which must be written to the Watchdog Password register"]
    Password = 90,
}
impl From<WdtpwwWO> for u8 {
    #[inline(always)]
    fn from(variant: WdtpwwWO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WdtpwwWO {
    type Ux = u8;
}
impl crate::IsEnum for WdtpwwWO {}
#[doc = "Field `WDTPW` writer - Watchdog Timer Password"]
pub type WdtpwW<'a, REG> = crate::FieldWriter<'a, REG, 8, WdtpwwWO>;
impl<'a, REG> WdtpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the Watchdog Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(WdtpwwWO::Password)
    }
}
impl R {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WdtisR {
        WdtisR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WdtcntclR {
        WdtcntclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WdttmselR {
        WdttmselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WdtsselR {
        WdtsselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WdtholdR {
        WdtholdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WdtpwR {
        WdtpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WdtisW<'_, WdtctlSpec> {
        WdtisW::new(self, 0)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WdtcntclW<'_, WdtctlSpec> {
        WdtcntclW::new(self, 3)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WdttmselW<'_, WdtctlSpec> {
        WdttmselW::new(self, 4)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WdtsselW<'_, WdtctlSpec> {
        WdtsselW::new(self, 5)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WdtholdW<'_, WdtctlSpec> {
        WdtholdW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WdtpwW<'_, WdtctlSpec> {
        WdtpwW::new(self, 8)
    }
}
#[doc = "Watchdog Timer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtctlSpec;
impl crate::RegisterSpec for WdtctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdtctl::R`](R) reader structure"]
impl crate::Readable for WdtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtctl::W`](W) writer structure"]
impl crate::Writable for WdtctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WdtctlSpec {}
