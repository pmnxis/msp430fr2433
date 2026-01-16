#[doc = "Register `P2IN` reader"]
pub type R = crate::R<P2inSpec>;
#[doc = "Register `P2IN` writer"]
pub type W = crate::W<P2inSpec>;
#[doc = "Field `P2IN` reader - P2IN"]
pub type P2inR = crate::FieldReader;
#[doc = "Field `P2IN` writer - P2IN"]
pub type P2inW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P2IN"]
    #[inline(always)]
    pub fn p2in(&self) -> P2inR {
        P2inR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2IN"]
    #[inline(always)]
    pub fn p2in(&mut self) -> P2inW<'_, P2inSpec> {
        P2inW::new(self, 0)
    }
}
#[doc = "Port 2 Input register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2inSpec;
impl crate::RegisterSpec for P2inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2in::R`](R) reader structure"]
impl crate::Readable for P2inSpec {}
#[doc = "`write(|w| ..)` method takes [`p2in::W`](W) writer structure"]
impl crate::Writable for P2inSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2IN to value 0"]
impl crate::Resettable for P2inSpec {}
