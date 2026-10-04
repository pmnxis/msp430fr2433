#[doc = "Register `P3SEL0` reader"]
pub type R = crate::R<P3sel0Spec>;
#[doc = "Register `P3SEL0` writer"]
pub type W = crate::W<P3sel0Spec>;
#[doc = "Field `P3SEL0` reader - P3SEL0_0"]
pub type P3sel0R = crate::FieldReader;
#[doc = "Field `P3SEL0` writer - P3SEL0_0"]
pub type P3sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - P3SEL0_0"]
    #[inline(always)]
    pub fn p3sel0(&self) -> P3sel0R {
        P3sel0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P3SEL0_0"]
    #[inline(always)]
    pub fn p3sel0(&mut self) -> P3sel0W<'_, P3sel0Spec> {
        P3sel0W::new(self, 0)
    }
}
#[doc = "Port 3 Selection0\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3sel0Spec;
impl crate::RegisterSpec for P3sel0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3sel0::R`](R) reader structure"]
impl crate::Readable for P3sel0Spec {}
#[doc = "`write(|w| ..)` method takes [`p3sel0::W`](W) writer structure"]
impl crate::Writable for P3sel0Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3SEL0 to value 0"]
impl crate::Resettable for P3sel0Spec {}
