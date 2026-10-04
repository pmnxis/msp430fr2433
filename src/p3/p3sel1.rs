#[doc = "Register `P3SEL1` reader"]
pub type R = crate::R<P3sel1Spec>;
#[doc = "Register `P3SEL1` writer"]
pub type W = crate::W<P3sel1Spec>;
#[doc = "Field `P3SEL1` reader - P3SEL1_0"]
pub type P3sel1R = crate::FieldReader;
#[doc = "Field `P3SEL1` writer - P3SEL1_0"]
pub type P3sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - P3SEL1_0"]
    #[inline(always)]
    pub fn p3sel1(&self) -> P3sel1R {
        P3sel1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P3SEL1_0"]
    #[inline(always)]
    pub fn p3sel1(&mut self) -> P3sel1W<'_, P3sel1Spec> {
        P3sel1W::new(self, 0)
    }
}
#[doc = "Port 3 Selection1\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3sel1Spec;
impl crate::RegisterSpec for P3sel1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3sel1::R`](R) reader structure"]
impl crate::Readable for P3sel1Spec {}
#[doc = "`write(|w| ..)` method takes [`p3sel1::W`](W) writer structure"]
impl crate::Writable for P3sel1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3SEL1 to value 0"]
impl crate::Resettable for P3sel1Spec {}
