#[doc = "Register `P1REN` reader"]
pub type R = crate::R<P1renSpec>;
#[doc = "Register `P1REN` writer"]
pub type W = crate::W<P1renSpec>;
#[doc = "Field `P1REN` reader - P1REN"]
pub type P1renR = crate::FieldReader;
#[doc = "Field `P1REN` writer - P1REN"]
pub type P1renW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P1REN"]
    #[inline(always)]
    pub fn p1ren(&self) -> P1renR {
        P1renR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1REN"]
    #[inline(always)]
    pub fn p1ren(&mut self) -> P1renW<'_, P1renSpec> {
        P1renW::new(self, 0)
    }
}
#[doc = "Port 1 Resistor Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1renSpec;
impl crate::RegisterSpec for P1renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ren::R`](R) reader structure"]
impl crate::Readable for P1renSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ren::W`](W) writer structure"]
impl crate::Writable for P1renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1REN to value 0"]
impl crate::Resettable for P1renSpec {}
