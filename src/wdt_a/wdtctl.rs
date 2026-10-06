#[doc = "Register `WDTCTL` reader"]
pub type R = crate::R<WdtctlSpec>;
#[doc = "Register `WDTCTL` writer"]
pub type W = crate::W<WdtctlSpec>;
#[doc = "WDT - Timer Interval Select 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtis {
    #[doc = "0: Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    _2g = 0,
    #[doc = "1: Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    _128m = 1,
    #[doc = "2: Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    _8192k = 2,
    #[doc = "3: Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    _512k = 3,
    #[doc = "4: Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    _32k = 4,
    #[doc = "5: Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    _8192 = 5,
    #[doc = "6: Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    _512 = 6,
    #[doc = "7: Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    _64 = 7,
}
impl From<Wdtis> for u8 {
    #[inline(always)]
    fn from(variant: Wdtis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtis {
    type Ux = u8;
}
impl crate::IsEnum for Wdtis {}
#[doc = "Field `WDTIS` reader - WDT - Timer Interval Select 0"]
pub type WdtisR = crate::FieldReader<Wdtis>;
impl WdtisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtis {
        match self.bits {
            0 => Wdtis::_2g,
            1 => Wdtis::_128m,
            2 => Wdtis::_8192k,
            3 => Wdtis::_512k,
            4 => Wdtis::_32k,
            5 => Wdtis::_8192,
            6 => Wdtis::_512,
            7 => Wdtis::_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_2g(&self) -> bool {
        *self == Wdtis::_2g
    }
    #[doc = "Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_128m(&self) -> bool {
        *self == Wdtis::_128m
    }
    #[doc = "Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_8192k(&self) -> bool {
        *self == Wdtis::_8192k
    }
    #[doc = "Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == Wdtis::_512k
    }
    #[doc = "Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Wdtis::_32k
    }
    #[doc = "Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_8192(&self) -> bool {
        *self == Wdtis::_8192
    }
    #[doc = "Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Wdtis::_512
    }
    #[doc = "Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Wdtis::_64
    }
}
#[doc = "Field `WDTIS` writer - WDT - Timer Interval Select 0"]
pub type WdtisW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wdtis, crate::Safe>;
impl<'a, REG> WdtisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _2g(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_2g)
    }
    #[doc = "Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _128m(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_128m)
    }
    #[doc = "Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _8192k(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_8192k)
    }
    #[doc = "Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _512k(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_512k)
    }
    #[doc = "Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_32k)
    }
    #[doc = "Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn _8192(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_8192)
    }
    #[doc = "Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_512)
    }
    #[doc = "Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_64)
    }
}
#[doc = "Field `WDTCNTCL` reader - WDT - Timer Clear"]
pub type WdtcntclR = crate::BitReader;
#[doc = "Field `WDTCNTCL` writer - WDT - Timer Clear"]
pub type WdtcntclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTTMSEL` reader - WDT - Timer Mode Select"]
pub type WdttmselR = crate::BitReader;
#[doc = "Field `WDTTMSEL` writer - WDT - Timer Mode Select"]
pub type WdttmselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "WDT - Timer Clock Source Select 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtssel {
    #[doc = "0: WDT - Timer Clock Source Select: SMCLK"]
    Smclk = 0,
    #[doc = "1: WDT - Timer Clock Source Select: ACLK"]
    Aclk = 1,
    #[doc = "2: WDT - Timer Clock Source Select: VLOCLK"]
    Vloclk = 2,
    #[doc = "3: WDT - Timer Clock Source Select: Reserved"]
    Bclk = 3,
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
            0 => Wdtssel::Smclk,
            1 => Wdtssel::Aclk,
            2 => Wdtssel::Vloclk,
            3 => Wdtssel::Bclk,
            _ => unreachable!(),
        }
    }
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Wdtssel::Smclk
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Wdtssel::Aclk
    }
    #[doc = "WDT - Timer Clock Source Select: VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Wdtssel::Vloclk
    }
    #[doc = "WDT - Timer Clock Source Select: Reserved"]
    #[inline(always)]
    pub fn is_bclk(&self) -> bool {
        *self == Wdtssel::Bclk
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
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Smclk)
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Aclk)
    }
    #[doc = "WDT - Timer Clock Source Select: VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Vloclk)
    }
    #[doc = "WDT - Timer Clock Source Select: Reserved"]
    #[inline(always)]
    pub fn bclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Bclk)
    }
}
#[doc = "WDT - Timer hold\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdthold {
    #[doc = "0: Watchdog timer is not stopped"]
    Unhold = 0,
    #[doc = "1: Watchdog timer is stopped"]
    Hold = 1,
}
impl From<Wdthold> for bool {
    #[inline(always)]
    fn from(variant: Wdthold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTHOLD` reader - WDT - Timer hold"]
pub type WdtholdR = crate::BitReader<Wdthold>;
impl WdtholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdthold {
        match self.bits {
            false => Wdthold::Unhold,
            true => Wdthold::Hold,
        }
    }
    #[doc = "Watchdog timer is not stopped"]
    #[inline(always)]
    pub fn is_unhold(&self) -> bool {
        *self == Wdthold::Unhold
    }
    #[doc = "Watchdog timer is stopped"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == Wdthold::Hold
    }
}
#[doc = "Field `WDTHOLD` writer - WDT - Timer hold"]
pub type WdtholdW<'a, REG> = crate::BitWriter<'a, REG, Wdthold>;
impl<'a, REG> WdtholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog timer is not stopped"]
    #[inline(always)]
    pub fn unhold(self) -> &'a mut crate::W<REG> {
        self.variant(Wdthold::Unhold)
    }
    #[doc = "Watchdog timer is stopped"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(Wdthold::Hold)
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtpwr {
    #[doc = "105: Value always read from the Watchdog Password field"]
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
    #[doc = "Value always read from the Watchdog Password field"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Wdtpwr::Password
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WdtpwwWO {
    #[doc = "90: Value which must be written to the Watchdog Password field"]
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
    #[doc = "Value which must be written to the Watchdog Password field"]
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
