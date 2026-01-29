#[doc = "Register `SYSCFG0` reader"]
pub type R = crate::R<Syscfg0Spec>;
#[doc = "Register `SYSCFG0` writer"]
pub type W = crate::W<Syscfg0Spec>;
#[doc = "Field `PFWP` reader - Program FRAM Write Protection"]
pub type PfwpR = crate::BitReader;
#[doc = "Field `PFWP` writer - Program FRAM Write Protection"]
pub type PfwpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFWP` reader - Data FRAM Write Protection"]
pub type DfwpR = crate::BitReader;
#[doc = "Field `DFWP` writer - Data FRAM Write Protection"]
pub type DfwpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FRAM protection password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frwppwr {
    #[doc = "150: Value always read from the SYSCFG0 register"]
    Password = 150,
}
impl From<Frwppwr> for u8 {
    #[inline(always)]
    fn from(variant: Frwppwr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frwppwr {
    type Ux = u8;
}
impl crate::IsEnum for Frwppwr {}
#[doc = "Field `FRWPPW` reader - FRAM protection password"]
pub type FrwppwR = crate::FieldReader<Frwppwr>;
impl FrwppwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frwppwr> {
        match self.bits {
            150 => Some(Frwppwr::Password),
            _ => None,
        }
    }
    #[doc = "Value always read from the SYSCFG0 register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Frwppwr::Password
    }
}
#[doc = "FRAM protection password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FrwppwwWO {
    #[doc = "165: Value which must be written to the SYSCFG0 register"]
    Password = 165,
}
impl From<FrwppwwWO> for u8 {
    #[inline(always)]
    fn from(variant: FrwppwwWO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FrwppwwWO {
    type Ux = u8;
}
impl crate::IsEnum for FrwppwwWO {}
#[doc = "Field `FRWPPW` writer - FRAM protection password"]
pub type FrwppwW<'a, REG> = crate::FieldWriter<'a, REG, 8, FrwppwwWO>;
impl<'a, REG> FrwppwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the SYSCFG0 register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(FrwppwwWO::Password)
    }
}
impl R {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    pub fn pfwp(&self) -> PfwpR {
        PfwpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    pub fn dfwp(&self) -> DfwpR {
        DfwpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - FRAM protection password"]
    #[inline(always)]
    pub fn frwppw(&self) -> FrwppwR {
        FrwppwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    pub fn pfwp(&mut self) -> PfwpW<'_, Syscfg0Spec> {
        PfwpW::new(self, 0)
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    pub fn dfwp(&mut self) -> DfwpW<'_, Syscfg0Spec> {
        DfwpW::new(self, 1)
    }
    #[doc = "Bits 8:15 - FRAM protection password"]
    #[inline(always)]
    pub fn frwppw(&mut self) -> FrwppwW<'_, Syscfg0Spec> {
        FrwppwW::new(self, 8)
    }
}
#[doc = "System Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg0Spec;
impl crate::RegisterSpec for Syscfg0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg0::R`](R) reader structure"]
impl crate::Readable for Syscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg0::W`](W) writer structure"]
impl crate::Writable for Syscfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG0 to value 0"]
impl crate::Resettable for Syscfg0Spec {}
