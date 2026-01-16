#[doc = "Register `P1IFG` reader"]
pub type R = crate::R<P1ifgSpec>;
#[doc = "Register `P1IFG` writer"]
pub type W = crate::W<P1ifgSpec>;
#[doc = "Field `P1IFG` reader - P1IFG"]
pub type P1ifgR = crate::FieldReader;
#[doc = "Field `P1IFG` writer - P1IFG"]
pub type P1ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P1IFG"]
    #[inline(always)]
    pub fn p1ifg(&self) -> P1ifgR {
        P1ifgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1IFG"]
    #[inline(always)]
    pub fn p1ifg(&mut self) -> P1ifgW<'_, P1ifgSpec> {
        P1ifgW::new(self, 0)
    }
}
#[doc = "Port 1 Interrupt Flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1ifgSpec;
impl crate::RegisterSpec for P1ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ifg::R`](R) reader structure"]
impl crate::Readable for P1ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ifg::W`](W) writer structure"]
impl crate::Writable for P1ifgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1IFG to value 0"]
impl crate::Resettable for P1ifgSpec {}
