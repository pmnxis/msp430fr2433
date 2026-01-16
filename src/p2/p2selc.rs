#[doc = "Register `P2SELC` reader"]
pub type R = crate::R<P2selcSpec>;
#[doc = "Register `P2SELC` writer"]
pub type W = crate::W<P2selcSpec>;
#[doc = "Field `P2SELC` reader - P2SELC"]
pub type P2selcR = crate::FieldReader;
#[doc = "Field `P2SELC` writer - P2SELC"]
pub type P2selcW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P2SELC"]
    #[inline(always)]
    pub fn p2selc(&self) -> P2selcR {
        P2selcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2SELC"]
    #[inline(always)]
    pub fn p2selc(&mut self) -> P2selcW<'_, P2selcSpec> {
        P2selcW::new(self, 0)
    }
}
#[doc = "Port 2 Complement Select register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2selcSpec;
impl crate::RegisterSpec for P2selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2selc::R`](R) reader structure"]
impl crate::Readable for P2selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p2selc::W`](W) writer structure"]
impl crate::Writable for P2selcSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P2SELC to value 0"]
impl crate::Resettable for P2selcSpec {}
