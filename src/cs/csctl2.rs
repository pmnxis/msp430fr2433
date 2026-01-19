#[doc = "Register `CSCTL2` reader"]
pub type R = crate::R<Csctl2Spec>;
#[doc = "Register `CSCTL2` writer"]
pub type W = crate::W<Csctl2Spec>;
#[doc = "Field `FLLN` reader - FLL Multipier Bit : 0"]
pub type FllnR = crate::FieldReader<u16>;
#[doc = "Field `FLLN` writer - FLL Multipier Bit : 0"]
pub type FllnW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Loop Divider Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flld {
    #[doc = "0: Multiply selected loop frequency by 1"]
    _1 = 0,
    #[doc = "1: Multiply selected loop frequency by 2"]
    _2 = 1,
    #[doc = "2: Multiply selected loop frequency by 4"]
    _4 = 2,
    #[doc = "3: Multiply selected loop frequency by 8"]
    _8 = 3,
    #[doc = "4: Multiply selected loop frequency by 16"]
    _16 = 4,
    #[doc = "5: Multiply selected loop frequency by 32"]
    _32 = 5,
}
impl From<Flld> for u8 {
    #[inline(always)]
    fn from(variant: Flld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flld {
    type Ux = u8;
}
impl crate::IsEnum for Flld {}
#[doc = "Field `FLLD` reader - Loop Divider Bit : 0"]
pub type FlldR = crate::FieldReader<Flld>;
impl FlldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flld> {
        match self.bits {
            0 => Some(Flld::_1),
            1 => Some(Flld::_2),
            2 => Some(Flld::_4),
            3 => Some(Flld::_8),
            4 => Some(Flld::_16),
            5 => Some(Flld::_32),
            _ => None,
        }
    }
    #[doc = "Multiply selected loop frequency by 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flld::_1
    }
    #[doc = "Multiply selected loop frequency by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Flld::_2
    }
    #[doc = "Multiply selected loop frequency by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Flld::_4
    }
    #[doc = "Multiply selected loop frequency by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Flld::_8
    }
    #[doc = "Multiply selected loop frequency by 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Flld::_16
    }
    #[doc = "Multiply selected loop frequency by 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Flld::_32
    }
}
#[doc = "Field `FLLD` writer - Loop Divider Bit : 0"]
pub type FlldW<'a, REG> = crate::FieldWriter<'a, REG, 3, Flld>;
impl<'a, REG> FlldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Multiply selected loop frequency by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_1)
    }
    #[doc = "Multiply selected loop frequency by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_2)
    }
    #[doc = "Multiply selected loop frequency by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_4)
    }
    #[doc = "Multiply selected loop frequency by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_8)
    }
    #[doc = "Multiply selected loop frequency by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_16)
    }
    #[doc = "Multiply selected loop frequency by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_32)
    }
}
impl R {
    #[doc = "Bits 0:9 - FLL Multipier Bit : 0"]
    #[inline(always)]
    pub fn flln(&self) -> FllnR {
        FllnR::new(self.bits & 0x03ff)
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    pub fn flld(&self) -> FlldR {
        FlldR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - FLL Multipier Bit : 0"]
    #[inline(always)]
    pub fn flln(&mut self) -> FllnW<'_, Csctl2Spec> {
        FllnW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    pub fn flld(&mut self) -> FlldW<'_, Csctl2Spec> {
        FlldW::new(self, 12)
    }
}
#[doc = "CS Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl2Spec;
impl crate::RegisterSpec for Csctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl2::R`](R) reader structure"]
impl crate::Readable for Csctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl2::W`](W) writer structure"]
impl crate::Writable for Csctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL2 to value 0"]
impl crate::Resettable for Csctl2Spec {}
