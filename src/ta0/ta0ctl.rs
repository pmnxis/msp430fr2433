#[doc = "Register `TA0CTL` reader"]
pub type R = crate::R<Ta0ctlSpec>;
#[doc = "Register `TA0CTL` writer"]
pub type W = crate::W<Ta0ctlSpec>;
#[doc = "Field `TAIFG` reader - Timer A counter interrupt flag"]
pub type TaifgR = crate::BitReader;
#[doc = "Field `TAIFG` writer - Timer A counter interrupt flag"]
pub type TaifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAIE` reader - Timer A counter interrupt enable"]
pub type TaieR = crate::BitReader;
#[doc = "Field `TAIE` writer - Timer A counter interrupt enable"]
pub type TaieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACLR` reader - Timer A counter clear"]
pub type TaclrR = crate::BitReader;
#[doc = "Field `TACLR` writer - Timer A counter clear"]
pub type TaclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer A mode control 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mc {
    #[doc = "0: Stop mode: Timer is halted"]
    Stop = 0,
    #[doc = "1: Up mode: Timer counts up to TAxCCR0"]
    Up = 1,
    #[doc = "2: Continuous mode: Timer counts up to 0FFFFh"]
    Continuous = 2,
    #[doc = "3: Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    Updown = 3,
}
impl From<Mc> for u8 {
    #[inline(always)]
    fn from(variant: Mc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mc {
    type Ux = u8;
}
impl crate::IsEnum for Mc {}
#[doc = "Field `MC` reader - Timer A mode control 1"]
pub type McR = crate::FieldReader<Mc>;
impl McR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mc {
        match self.bits {
            0 => Mc::Stop,
            1 => Mc::Up,
            2 => Mc::Continuous,
            3 => Mc::Updown,
            _ => unreachable!(),
        }
    }
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Mc::Stop
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Mc::Up
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Mc::Continuous
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == Mc::Updown
    }
}
#[doc = "Field `MC` writer - Timer A mode control 1"]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mc, crate::Safe>;
impl<'a, REG> McW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Stop)
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Up)
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Continuous)
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Updown)
    }
}
#[doc = "Timer A clock input divider 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Id {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
}
impl From<Id> for u8 {
    #[inline(always)]
    fn from(variant: Id) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Id {
    type Ux = u8;
}
impl crate::IsEnum for Id {}
#[doc = "Field `ID` reader - Timer A clock input divider 1"]
pub type IdR = crate::FieldReader<Id>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id {
        match self.bits {
            0 => Id::_1,
            1 => Id::_2,
            2 => Id::_4,
            3 => Id::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Id::_1
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Id::_2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Id::_4
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Id::_8
    }
}
#[doc = "Field `ID` writer - Timer A clock input divider 1"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Id, crate::Safe>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_8)
    }
}
#[doc = "Timer A clock source select 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tassel {
    #[doc = "0: TAxCLK"]
    Taclk = 0,
    #[doc = "1: ACLK"]
    Aclk = 1,
    #[doc = "2: SMCLK"]
    Smclk = 2,
    #[doc = "3: INCLK"]
    Inclk = 3,
}
impl From<Tassel> for u8 {
    #[inline(always)]
    fn from(variant: Tassel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tassel {
    type Ux = u8;
}
impl crate::IsEnum for Tassel {}
#[doc = "Field `TASSEL` reader - Timer A clock source select 1"]
pub type TasselR = crate::FieldReader<Tassel>;
impl TasselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tassel {
        match self.bits {
            0 => Tassel::Taclk,
            1 => Tassel::Aclk,
            2 => Tassel::Smclk,
            3 => Tassel::Inclk,
            _ => unreachable!(),
        }
    }
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn is_taclk(&self) -> bool {
        *self == Tassel::Taclk
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Tassel::Aclk
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Tassel::Smclk
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn is_inclk(&self) -> bool {
        *self == Tassel::Inclk
    }
}
#[doc = "Field `TASSEL` writer - Timer A clock source select 1"]
pub type TasselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tassel, crate::Safe>;
impl<'a, REG> TasselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn taclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Taclk)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Aclk)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Smclk)
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn inclk(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Inclk)
    }
}
impl R {
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TaifgR {
        TaifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TaieR {
        TaieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TaclrR {
        TaclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline(always)]
    pub fn tassel(&self) -> TasselR {
        TasselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline(always)]
    pub fn taifg(&mut self) -> TaifgW<'_, Ta0ctlSpec> {
        TaifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TaieW<'_, Ta0ctlSpec> {
        TaieW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline(always)]
    pub fn taclr(&mut self) -> TaclrW<'_, Ta0ctlSpec> {
        TaclrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline(always)]
    pub fn mc(&mut self) -> McW<'_, Ta0ctlSpec> {
        McW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, Ta0ctlSpec> {
        IdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline(always)]
    pub fn tassel(&mut self) -> TasselW<'_, Ta0ctlSpec> {
        TasselW::new(self, 8)
    }
}
#[doc = "Timer0_A3 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ta0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta0ctlSpec;
impl crate::RegisterSpec for Ta0ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta0ctl::R`](R) reader structure"]
impl crate::Readable for Ta0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ta0ctl::W`](W) writer structure"]
impl crate::Writable for Ta0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA0CTL to value 0"]
impl crate::Resettable for Ta0ctlSpec {}
