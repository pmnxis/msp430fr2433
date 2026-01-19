#[doc = "Register `P2SEL1` reader"]
pub type R = crate::R<P2sel1Spec>;
#[doc = "Register `P2SEL1` writer"]
pub type W = crate::W<P2sel1Spec>;
#[doc = "Field `P2SEL1` reader - P2SEL1"]
pub type P2sel1R = crate::FieldReader;
#[doc = "Field `P2SEL1` writer - P2SEL1"]
pub type P2sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P2SEL1"]
    #[inline(always)]
    pub fn p2sel1(&self) -> P2sel1R {
        P2sel1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2SEL1"]
    #[inline(always)]
    pub fn p2sel1(&mut self) -> P2sel1W<'_, P2sel1Spec> {
        P2sel1W::new(self, 0)
    }
}
#[doc = "Port 2 Selection register bit 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p2sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2sel1Spec;
impl crate::RegisterSpec for P2sel1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2sel1::R`](R) reader structure"]
impl crate::Readable for P2sel1Spec {}
#[doc = "`write(|w| ..)` method takes [`p2sel1::W`](W) writer structure"]
impl crate::Writable for P2sel1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2SEL1 to value 0"]
impl crate::Resettable for P2sel1Spec {}
