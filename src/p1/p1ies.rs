#[doc = "Register `P1IES` reader"]
pub type R = crate::R<P1iesSpec>;
#[doc = "Register `P1IES` writer"]
pub type W = crate::W<P1iesSpec>;
#[doc = "Field `P1IES` reader - P1IES"]
pub type P1iesR = crate::FieldReader;
#[doc = "Field `P1IES` writer - P1IES"]
pub type P1iesW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P1IES"]
    #[inline(always)]
    pub fn p1ies(&self) -> P1iesR {
        P1iesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1IES"]
    #[inline(always)]
    pub fn p1ies(&mut self) -> P1iesW<'_, P1iesSpec> {
        P1iesW::new(self, 0)
    }
}
#[doc = "Port 1 Interrupt Edge Select register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1iesSpec;
impl crate::RegisterSpec for P1iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ies::R`](R) reader structure"]
impl crate::Readable for P1iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ies::W`](W) writer structure"]
impl crate::Writable for P1iesSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1IES to value 0"]
impl crate::Resettable for P1iesSpec {}
