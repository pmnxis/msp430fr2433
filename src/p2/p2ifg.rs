#[doc = "Register `P2IFG` reader"]
pub type R = crate::R<P2ifgSpec>;
#[doc = "Register `P2IFG` writer"]
pub type W = crate::W<P2ifgSpec>;
#[doc = "Field `P2IFG` reader - P2IFG"]
pub type P2ifgR = crate::FieldReader;
#[doc = "Field `P2IFG` writer - P2IFG"]
pub type P2ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P2IFG"]
    #[inline(always)]
    pub fn p2ifg(&self) -> P2ifgR {
        P2ifgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2IFG"]
    #[inline(always)]
    pub fn p2ifg(&mut self) -> P2ifgW<'_, P2ifgSpec> {
        P2ifgW::new(self, 0)
    }
}
#[doc = "Port 2 Interrupt Flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2ifgSpec;
impl crate::RegisterSpec for P2ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2ifg::R`](R) reader structure"]
impl crate::Readable for P2ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p2ifg::W`](W) writer structure"]
impl crate::Writable for P2ifgSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2IFG to value 0"]
impl crate::Resettable for P2ifgSpec {}
