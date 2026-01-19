#[doc = "Register `P1IN` reader"]
pub type R = crate::R<P1inSpec>;
#[doc = "Register `P1IN` writer"]
pub type W = crate::W<P1inSpec>;
#[doc = "Field `P1IN` reader - P1IN"]
pub type P1inR = crate::FieldReader;
#[doc = "Field `P1IN` writer - P1IN"]
pub type P1inW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P1IN"]
    #[inline(always)]
    pub fn p1in(&self) -> P1inR {
        P1inR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1IN"]
    #[inline(always)]
    pub fn p1in(&mut self) -> P1inW<'_, P1inSpec> {
        P1inW::new(self, 0)
    }
}
#[doc = "Port 1 Input register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1inSpec;
impl crate::RegisterSpec for P1inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1in::R`](R) reader structure"]
impl crate::Readable for P1inSpec {}
#[doc = "`write(|w| ..)` method takes [`p1in::W`](W) writer structure"]
impl crate::Writable for P1inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1IN to value 0"]
impl crate::Resettable for P1inSpec {}
