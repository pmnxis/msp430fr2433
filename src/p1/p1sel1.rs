#[doc = "Register `P1SEL1` reader"]
pub type R = crate::R<P1sel1Spec>;
#[doc = "Register `P1SEL1` writer"]
pub type W = crate::W<P1sel1Spec>;
#[doc = "Field `P1SEL1` reader - P1SEL1"]
pub type P1sel1R = crate::FieldReader;
#[doc = "Field `P1SEL1` writer - P1SEL1"]
pub type P1sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P1SEL1"]
    #[inline(always)]
    pub fn p1sel1(&self) -> P1sel1R {
        P1sel1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1SEL1"]
    #[inline(always)]
    pub fn p1sel1(&mut self) -> P1sel1W<'_, P1sel1Spec> {
        P1sel1W::new(self, 0)
    }
}
#[doc = "Port 1 Selection register bit 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p1sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1sel1Spec;
impl crate::RegisterSpec for P1sel1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1sel1::R`](R) reader structure"]
impl crate::Readable for P1sel1Spec {}
#[doc = "`write(|w| ..)` method takes [`p1sel1::W`](W) writer structure"]
impl crate::Writable for P1sel1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1SEL1 to value 0"]
impl crate::Resettable for P1sel1Spec {}
