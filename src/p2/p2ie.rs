#[doc = "Register `P2IE` reader"]
pub type R = crate::R<P2ieSpec>;
#[doc = "Register `P2IE` writer"]
pub type W = crate::W<P2ieSpec>;
#[doc = "Field `P2IE` reader - P2IE"]
pub type P2ieR = crate::FieldReader;
#[doc = "Field `P2IE` writer - P2IE"]
pub type P2ieW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P2IE"]
    #[inline(always)]
    pub fn p2ie(&self) -> P2ieR {
        P2ieR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2IE"]
    #[inline(always)]
    pub fn p2ie(&mut self) -> P2ieW<'_, P2ieSpec> {
        P2ieW::new(self, 0)
    }
}
#[doc = "Port 2 Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2ieSpec;
impl crate::RegisterSpec for P2ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ie::R`](R) reader structure"]
impl crate::Readable for P2ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ie::W`](W) writer structure"]
impl crate::Writable for P2ieSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2IE to value 0"]
impl crate::Resettable for P2ieSpec {}
