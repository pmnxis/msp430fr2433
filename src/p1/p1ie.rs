#[doc = "Register `P1IE` reader"]
pub type R = crate::R<P1ieSpec>;
#[doc = "Register `P1IE` writer"]
pub type W = crate::W<P1ieSpec>;
#[doc = "Field `P1IE` reader - P1IE"]
pub type P1ieR = crate::FieldReader;
#[doc = "Field `P1IE` writer - P1IE"]
pub type P1ieW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P1IE"]
    #[inline(always)]
    pub fn p1ie(&self) -> P1ieR {
        P1ieR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1IE"]
    #[inline(always)]
    pub fn p1ie(&mut self) -> P1ieW<'_, P1ieSpec> {
        P1ieW::new(self, 0)
    }
}
#[doc = "Port 1 Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1ieSpec;
impl crate::RegisterSpec for P1ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ie::R`](R) reader structure"]
impl crate::Readable for P1ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ie::W`](W) writer structure"]
impl crate::Writable for P1ieSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1IE to value 0"]
impl crate::Resettable for P1ieSpec {}
