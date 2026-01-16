#[doc = "Register `RTCCTL` reader"]
pub type R = crate::R<RtcctlSpec>;
#[doc = "Register `RTCCTL` writer"]
pub type W = crate::W<RtcctlSpec>;
#[doc = "Field `RTCIF` reader - Low-Power-Counter Interrupt Flag"]
pub type RtcifR = crate::BitReader;
#[doc = "Field `RTCIF` writer - Low-Power-Counter Interrupt Flag"]
pub type RtcifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCIE` reader - Low-Power-Counter Interrupt Enable"]
pub type RtcieR = crate::BitReader;
#[doc = "Field `RTCIE` writer - Low-Power-Counter Interrupt Enable"]
pub type RtcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSR` reader - Low-Power-Counter Software Reset"]
pub type RtcsrR = crate::BitReader;
#[doc = "Field `RTCSR` writer - Low-Power-Counter Software Reset"]
pub type RtcsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low-Power-Counter Clock Pre-divider Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcps {
    #[doc = "0: RTC Predivider /1"]
    _1 = 0,
    #[doc = "1: RTC Predivider /10"]
    _10 = 1,
    #[doc = "2: RTC Predivider /100"]
    _100 = 2,
    #[doc = "3: RTC Predivider /1000"]
    _1000 = 3,
    #[doc = "4: RTC Predivider /16"]
    _16 = 4,
    #[doc = "5: RTC Predivider /64"]
    _64 = 5,
    #[doc = "6: RTC Predivider /256"]
    _256 = 6,
    #[doc = "7: RTC Predivider /1024"]
    _1024 = 7,
}
impl From<Rtcps> for u8 {
    #[inline(always)]
    fn from(variant: Rtcps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcps {
    type Ux = u8;
}
impl crate::IsEnum for Rtcps {}
#[doc = "Field `RTCPS` reader - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
pub type RtcpsR = crate::FieldReader<Rtcps>;
impl RtcpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcps {
        match self.bits {
            0 => Rtcps::_1,
            1 => Rtcps::_10,
            2 => Rtcps::_100,
            3 => Rtcps::_1000,
            4 => Rtcps::_16,
            5 => Rtcps::_64,
            6 => Rtcps::_256,
            7 => Rtcps::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "RTC Predivider /1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcps::_1
    }
    #[doc = "RTC Predivider /10"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rtcps::_10
    }
    #[doc = "RTC Predivider /100"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rtcps::_100
    }
    #[doc = "RTC Predivider /1000"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Rtcps::_1000
    }
    #[doc = "RTC Predivider /16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Rtcps::_16
    }
    #[doc = "RTC Predivider /64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Rtcps::_64
    }
    #[doc = "RTC Predivider /256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Rtcps::_256
    }
    #[doc = "RTC Predivider /1024"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Rtcps::_1024
    }
}
#[doc = "Field `RTCPS` writer - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
pub type RtcpsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rtcps, crate::Safe>;
impl<'a, REG> RtcpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC Predivider /1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_1)
    }
    #[doc = "RTC Predivider /10"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_10)
    }
    #[doc = "RTC Predivider /100"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_100)
    }
    #[doc = "RTC Predivider /1000"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_1000)
    }
    #[doc = "RTC Predivider /16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_16)
    }
    #[doc = "RTC Predivider /64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_64)
    }
    #[doc = "RTC Predivider /256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_256)
    }
    #[doc = "RTC Predivider /1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_1024)
    }
}
#[doc = "Low-Power-Counter Clock Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcss {
    #[doc = "0: Low-Power-Counter Clock Source Select: Disabled"]
    Disabled = 0,
    #[doc = "1: Low-Power-Counter Clock Source Select: SMCLK"]
    Smclk = 1,
    #[doc = "2: Low-Power-Counter Clock Source Select: XT1CLK"]
    Xt1clk = 2,
    #[doc = "3: Low-Power-Counter Clock Source Select: VLOCLK"]
    Vloclk = 3,
}
impl From<Rtcss> for u8 {
    #[inline(always)]
    fn from(variant: Rtcss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcss {
    type Ux = u8;
}
impl crate::IsEnum for Rtcss {}
#[doc = "Field `RTCSS` reader - Low-Power-Counter Clock Source Select Bit: 0"]
pub type RtcssR = crate::FieldReader<Rtcss>;
impl RtcssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcss {
        match self.bits {
            0 => Rtcss::Disabled,
            1 => Rtcss::Smclk,
            2 => Rtcss::Xt1clk,
            3 => Rtcss::Vloclk,
            _ => unreachable!(),
        }
    }
    #[doc = "Low-Power-Counter Clock Source Select: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rtcss::Disabled
    }
    #[doc = "Low-Power-Counter Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Rtcss::Smclk
    }
    #[doc = "Low-Power-Counter Clock Source Select: XT1CLK"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == Rtcss::Xt1clk
    }
    #[doc = "Low-Power-Counter Clock Source Select: VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Rtcss::Vloclk
    }
}
#[doc = "Field `RTCSS` writer - Low-Power-Counter Clock Source Select Bit: 0"]
pub type RtcssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtcss, crate::Safe>;
impl<'a, REG> RtcssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low-Power-Counter Clock Source Select: Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Disabled)
    }
    #[doc = "Low-Power-Counter Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Smclk)
    }
    #[doc = "Low-Power-Counter Clock Source Select: XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Xt1clk)
    }
    #[doc = "Low-Power-Counter Clock Source Select: VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Vloclk)
    }
}
impl R {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    pub fn rtcif(&self) -> RtcifR {
        RtcifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    pub fn rtcie(&self) -> RtcieR {
        RtcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    pub fn rtcsr(&self) -> RtcsrR {
        RtcsrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    pub fn rtcps(&self) -> RtcpsR {
        RtcpsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn rtcss(&self) -> RtcssR {
        RtcssR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    pub fn rtcif(&mut self) -> RtcifW<'_, RtcctlSpec> {
        RtcifW::new(self, 0)
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    pub fn rtcie(&mut self) -> RtcieW<'_, RtcctlSpec> {
        RtcieW::new(self, 1)
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    pub fn rtcsr(&mut self) -> RtcsrW<'_, RtcctlSpec> {
        RtcsrW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RtcpsW<'_, RtcctlSpec> {
        RtcpsW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn rtcss(&mut self) -> RtcssW<'_, RtcctlSpec> {
        RtcssW::new(self, 12)
    }
}
#[doc = "RTC control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcctlSpec;
impl crate::RegisterSpec for RtcctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl::R`](R) reader structure"]
impl crate::Readable for RtcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcctl::W`](W) writer structure"]
impl crate::Writable for RtcctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCTL to value 0"]
impl crate::Resettable for RtcctlSpec {}
