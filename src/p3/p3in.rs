#[doc = "Register `P3IN` reader"]
pub type R = crate::R<P3inSpec>;
#[doc = "Register `P3IN` writer"]
pub type W = crate::W<P3inSpec>;
#[doc = "Field `P3IN` reader - P3IN0"]
pub type P3inR = crate::FieldReader;
#[doc = "Field `P3IN` writer - P3IN0"]
pub type P3inW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - P3IN0"]
    #[inline(always)]
    pub fn p3in(&self) -> P3inR {
        P3inR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P3IN0"]
    #[inline(always)]
    pub fn p3in(&mut self) -> P3inW<'_, P3inSpec> {
        P3inW::new(self, 0)
    }
}
#[doc = "Port 3 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p3in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3inSpec;
impl crate::RegisterSpec for P3inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3in::R`](R) reader structure"]
impl crate::Readable for P3inSpec {}
#[doc = "`write(|w| ..)` method takes [`p3in::W`](W) writer structure"]
impl crate::Writable for P3inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3IN to value 0"]
impl crate::Resettable for P3inSpec {}
