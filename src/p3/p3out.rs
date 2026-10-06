#[doc = "Register `P3OUT` reader"]
pub type R = crate::R<P3outSpec>;
#[doc = "Register `P3OUT` writer"]
pub type W = crate::W<P3outSpec>;
#[doc = "Field `P3OUT` reader - P3OUT0"]
pub type P3outR = crate::FieldReader;
#[doc = "Field `P3OUT` writer - P3OUT0"]
pub type P3outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - P3OUT0"]
    #[inline(always)]
    pub fn p3out(&self) -> P3outR {
        P3outR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P3OUT0"]
    #[inline(always)]
    pub fn p3out(&mut self) -> P3outW<'_, P3outSpec> {
        P3outW::new(self, 0)
    }
}
#[doc = "Port 3 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p3out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3outSpec;
impl crate::RegisterSpec for P3outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3out::R`](R) reader structure"]
impl crate::Readable for P3outSpec {}
#[doc = "`write(|w| ..)` method takes [`p3out::W`](W) writer structure"]
impl crate::Writable for P3outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3OUT to value 0"]
impl crate::Resettable for P3outSpec {}
