#[doc = "Register `P2DIR` reader"]
pub type R = crate::R<P2dirSpec>;
#[doc = "Register `P2DIR` writer"]
pub type W = crate::W<P2dirSpec>;
#[doc = "Field `P2DIR` reader - P2DIR"]
pub type P2dirR = crate::FieldReader;
#[doc = "Field `P2DIR` writer - P2DIR"]
pub type P2dirW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P2DIR"]
    #[inline(always)]
    pub fn p2dir(&self) -> P2dirR {
        P2dirR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2DIR"]
    #[inline(always)]
    pub fn p2dir(&mut self) -> P2dirW<'_, P2dirSpec> {
        P2dirW::new(self, 0)
    }
}
#[doc = "Port 2 Direction register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2dirSpec;
impl crate::RegisterSpec for P2dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2dir::R`](R) reader structure"]
impl crate::Readable for P2dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p2dir::W`](W) writer structure"]
impl crate::Writable for P2dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2DIR to value 0"]
impl crate::Resettable for P2dirSpec {}
