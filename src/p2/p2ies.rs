#[doc = "Register `P2IES` reader"]
pub type R = crate::R<P2iesSpec>;
#[doc = "Register `P2IES` writer"]
pub type W = crate::W<P2iesSpec>;
#[doc = "Field `P2IES` reader - P2IES"]
pub type P2iesR = crate::FieldReader;
#[doc = "Field `P2IES` writer - P2IES"]
pub type P2iesW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P2IES"]
    #[inline(always)]
    pub fn p2ies(&self) -> P2iesR {
        P2iesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2IES"]
    #[inline(always)]
    pub fn p2ies(&mut self) -> P2iesW<'_, P2iesSpec> {
        P2iesW::new(self, 0)
    }
}
#[doc = "Port 2 Interrupt Edge Select register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2iesSpec;
impl crate::RegisterSpec for P2iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ies::R`](R) reader structure"]
impl crate::Readable for P2iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ies::W`](W) writer structure"]
impl crate::Writable for P2iesSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2IES to value 0"]
impl crate::Resettable for P2iesSpec {}
