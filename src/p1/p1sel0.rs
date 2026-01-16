#[doc = "Register `P1SEL0` reader"]
pub type R = crate::R<P1sel0Spec>;
#[doc = "Register `P1SEL0` writer"]
pub type W = crate::W<P1sel0Spec>;
#[doc = "Field `P1SEL0` reader - P1SEL0"]
pub type P1sel0R = crate::FieldReader;
#[doc = "Field `P1SEL0` writer - P1SEL0"]
pub type P1sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P1SEL0"]
    #[inline(always)]
    pub fn p1sel0(&self) -> P1sel0R {
        P1sel0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1SEL0"]
    #[inline(always)]
    pub fn p1sel0(&mut self) -> P1sel0W<'_, P1sel0Spec> {
        P1sel0W::new(self, 0)
    }
}
#[doc = "Port 1 Selection register bit 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p1sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1sel0Spec;
impl crate::RegisterSpec for P1sel0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1sel0::R`](R) reader structure"]
impl crate::Readable for P1sel0Spec {}
#[doc = "`write(|w| ..)` method takes [`p1sel0::W`](W) writer structure"]
impl crate::Writable for P1sel0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1SEL0 to value 0"]
impl crate::Resettable for P1sel0Spec {}
