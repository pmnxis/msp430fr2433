#[doc = "Register `P2REN` reader"]
pub type R = crate::R<P2renSpec>;
#[doc = "Register `P2REN` writer"]
pub type W = crate::W<P2renSpec>;
#[doc = "Field `P2REN` reader - P2REN"]
pub type P2renR = crate::FieldReader;
#[doc = "Field `P2REN` writer - P2REN"]
pub type P2renW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P2REN"]
    #[inline(always)]
    pub fn p2ren(&self) -> P2renR {
        P2renR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2REN"]
    #[inline(always)]
    pub fn p2ren(&mut self) -> P2renW<'_, P2renSpec> {
        P2renW::new(self, 0)
    }
}
#[doc = "Port 2 Resistor Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2renSpec;
impl crate::RegisterSpec for P2renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ren::R`](R) reader structure"]
impl crate::Readable for P2renSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ren::W`](W) writer structure"]
impl crate::Writable for P2renSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2REN to value 0"]
impl crate::Resettable for P2renSpec {}
