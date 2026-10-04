#[doc = "Register `CSCTL0` reader"]
pub type R = crate::R<Csctl0Spec>;
#[doc = "Register `CSCTL0` writer"]
pub type W = crate::W<Csctl0Spec>;
#[doc = "Field `DCO` reader - DCO TAP Bit : 0"]
pub type DcoR = crate::FieldReader<u16>;
#[doc = "Field `DCO` writer - DCO TAP Bit : 0"]
pub type DcoW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16, crate::Safe>;
#[doc = "Field `MOD` reader - Modulation Bit Counter Bit : 0"]
pub type ModR = crate::FieldReader;
#[doc = "Field `MOD` writer - Modulation Bit Counter Bit : 0"]
pub type ModW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:8 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco(&self) -> DcoR {
        DcoR::new(self.bits & 0x01ff)
    }
    #[doc = "Bits 9:13 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco(&mut self) -> DcoW<'_, Csctl0Spec> {
        DcoW::new(self, 0)
    }
    #[doc = "Bits 9:13 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod_(&mut self) -> ModW<'_, Csctl0Spec> {
        ModW::new(self, 9)
    }
}
#[doc = "CS Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl0Spec;
impl crate::RegisterSpec for Csctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl0::R`](R) reader structure"]
impl crate::Readable for Csctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl0::W`](W) writer structure"]
impl crate::Writable for Csctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL0 to value 0"]
impl crate::Resettable for Csctl0Spec {}
