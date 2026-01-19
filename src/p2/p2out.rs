#[doc = "Register `P2OUT` reader"]
pub type R = crate::R<P2outSpec>;
#[doc = "Register `P2OUT` writer"]
pub type W = crate::W<P2outSpec>;
#[doc = "Field `P2OUT` reader - P2OUT"]
pub type P2outR = crate::FieldReader;
#[doc = "Field `P2OUT` writer - P2OUT"]
pub type P2outW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P2OUT"]
    #[inline(always)]
    pub fn p2out(&self) -> P2outR {
        P2outR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2OUT"]
    #[inline(always)]
    pub fn p2out(&mut self) -> P2outW<'_, P2outSpec> {
        P2outW::new(self, 0)
    }
}
#[doc = "Port 2 Output register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2outSpec;
impl crate::RegisterSpec for P2outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2out::R`](R) reader structure"]
impl crate::Readable for P2outSpec {}
#[doc = "`write(|w| ..)` method takes [`p2out::W`](W) writer structure"]
impl crate::Writable for P2outSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2OUT to value 0"]
impl crate::Resettable for P2outSpec {}
