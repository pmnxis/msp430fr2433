#[doc = "Register `CSCTL5` reader"]
pub type R = crate::R<Csctl5Spec>;
#[doc = "Register `CSCTL5` writer"]
pub type W = crate::W<Csctl5Spec>;
#[doc = "MCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divm {
    #[doc = "0: MCLK = Clock source /1"]
    _1 = 0,
    #[doc = "1: MCLK = Clock source /2"]
    _2 = 1,
    #[doc = "2: MCLK = Clock source /4"]
    _4 = 2,
    #[doc = "3: MCLK = Clock source /8"]
    _8 = 3,
    #[doc = "4: MCLK = Clock source /16"]
    _16 = 4,
    #[doc = "5: MCLK = Clock source /32"]
    _32 = 5,
    #[doc = "6: MCLK = Clock source /64"]
    _64 = 6,
    #[doc = "7: MCLK = Clock source /128"]
    _128 = 7,
}
impl From<Divm> for u8 {
    #[inline(always)]
    fn from(variant: Divm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divm {
    type Ux = u8;
}
impl crate::IsEnum for Divm {}
#[doc = "Field `DIVM` reader - MCLK Divider Bit: 0"]
pub type DivmR = crate::FieldReader<Divm>;
impl DivmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divm {
        match self.bits {
            0 => Divm::_1,
            1 => Divm::_2,
            2 => Divm::_4,
            3 => Divm::_8,
            4 => Divm::_16,
            5 => Divm::_32,
            6 => Divm::_64,
            7 => Divm::_128,
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK = Clock source /1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Divm::_1
    }
    #[doc = "MCLK = Clock source /2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Divm::_2
    }
    #[doc = "MCLK = Clock source /4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Divm::_4
    }
    #[doc = "MCLK = Clock source /8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Divm::_8
    }
    #[doc = "MCLK = Clock source /16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Divm::_16
    }
    #[doc = "MCLK = Clock source /32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Divm::_32
    }
    #[doc = "MCLK = Clock source /64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Divm::_64
    }
    #[doc = "MCLK = Clock source /128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Divm::_128
    }
}
#[doc = "Field `DIVM` writer - MCLK Divider Bit: 0"]
pub type DivmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Divm, crate::Safe>;
impl<'a, REG> DivmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK = Clock source /1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_1)
    }
    #[doc = "MCLK = Clock source /2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_2)
    }
    #[doc = "MCLK = Clock source /4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_4)
    }
    #[doc = "MCLK = Clock source /8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_8)
    }
    #[doc = "MCLK = Clock source /16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_16)
    }
    #[doc = "MCLK = Clock source /32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_32)
    }
    #[doc = "MCLK = Clock source /64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_64)
    }
    #[doc = "MCLK = Clock source /128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::_128)
    }
}
#[doc = "SMCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divs {
    #[doc = "0: SMCLK = MCLK /1"]
    _1 = 0,
    #[doc = "1: SMCLK = MCLK /2"]
    _2 = 1,
    #[doc = "2: SMCLK = MCLK /4"]
    _4 = 2,
    #[doc = "3: SMCLK = MCLK /8"]
    _8 = 3,
}
impl From<Divs> for u8 {
    #[inline(always)]
    fn from(variant: Divs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divs {
    type Ux = u8;
}
impl crate::IsEnum for Divs {}
#[doc = "Field `DIVS` reader - SMCLK Divider Bit: 0"]
pub type DivsR = crate::FieldReader<Divs>;
impl DivsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divs {
        match self.bits {
            0 => Divs::_1,
            1 => Divs::_2,
            2 => Divs::_4,
            3 => Divs::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "SMCLK = MCLK /1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Divs::_1
    }
    #[doc = "SMCLK = MCLK /2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Divs::_2
    }
    #[doc = "SMCLK = MCLK /4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Divs::_4
    }
    #[doc = "SMCLK = MCLK /8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Divs::_8
    }
}
#[doc = "Field `DIVS` writer - SMCLK Divider Bit: 0"]
pub type DivsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Divs, crate::Safe>;
impl<'a, REG> DivsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SMCLK = MCLK /1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_1)
    }
    #[doc = "SMCLK = MCLK /2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_2)
    }
    #[doc = "SMCLK = MCLK /4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_4)
    }
    #[doc = "SMCLK = MCLK /8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::_8)
    }
}
#[doc = "Field `SMCLKOFF` reader - SMCLK off"]
pub type SmclkoffR = crate::BitReader;
#[doc = "Field `SMCLKOFF` writer - SMCLK off"]
pub type SmclkoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLOAUTOOFF` reader - VLO automatic off enable"]
pub type VloautooffR = crate::BitReader;
#[doc = "Field `VLOAUTOOFF` writer - VLO automatic off enable"]
pub type VloautooffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divm(&self) -> DivmR {
        DivmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divs(&self) -> DivsR {
        DivsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SmclkoffR {
        SmclkoffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - VLO automatic off enable"]
    #[inline(always)]
    pub fn vloautooff(&self) -> VloautooffR {
        VloautooffR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divm(&mut self) -> DivmW<'_, Csctl5Spec> {
        DivmW::new(self, 0)
    }
    #[doc = "Bits 4:5 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divs(&mut self) -> DivsW<'_, Csctl5Spec> {
        DivsW::new(self, 4)
    }
    #[doc = "Bit 8 - SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SmclkoffW<'_, Csctl5Spec> {
        SmclkoffW::new(self, 8)
    }
    #[doc = "Bit 12 - VLO automatic off enable"]
    #[inline(always)]
    pub fn vloautooff(&mut self) -> VloautooffW<'_, Csctl5Spec> {
        VloautooffW::new(self, 12)
    }
}
#[doc = "CS Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl5Spec;
impl crate::RegisterSpec for Csctl5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl5::R`](R) reader structure"]
impl crate::Readable for Csctl5Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl5::W`](W) writer structure"]
impl crate::Writable for Csctl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL5 to value 0"]
impl crate::Resettable for Csctl5Spec {}
