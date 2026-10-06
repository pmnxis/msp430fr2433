#[doc = "Register `P3REN` reader"]
pub type R = crate::R<P3renSpec>;
#[doc = "Register `P3REN` writer"]
pub type W = crate::W<P3renSpec>;
#[doc = "Field `P3REN` reader - P3REN0"]
pub type P3renR = crate::FieldReader;
#[doc = "Field `P3REN` writer - P3REN0"]
pub type P3renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - P3REN0"]
    #[inline(always)]
    pub fn p3ren(&self) -> P3renR {
        P3renR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P3REN0"]
    #[inline(always)]
    pub fn p3ren(&mut self) -> P3renW<'_, P3renSpec> {
        P3renW::new(self, 0)
    }
}
#[doc = "Port 3 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3renSpec;
impl crate::RegisterSpec for P3renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3ren::R`](R) reader structure"]
impl crate::Readable for P3renSpec {}
#[doc = "`write(|w| ..)` method takes [`p3ren::W`](W) writer structure"]
impl crate::Writable for P3renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3REN to value 0"]
impl crate::Resettable for P3renSpec {}
