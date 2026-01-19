#[doc = "Register `P1SELC` reader"]
pub type R = crate::R<P1selcSpec>;
#[doc = "Register `P1SELC` writer"]
pub type W = crate::W<P1selcSpec>;
#[doc = "Field `P1SELC` reader - P1SELC"]
pub type P1selcR = crate::FieldReader;
#[doc = "Field `P1SELC` writer - P1SELC"]
pub type P1selcW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - P1SELC"]
    #[inline(always)]
    pub fn p1selc(&self) -> P1selcR {
        P1selcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1SELC"]
    #[inline(always)]
    pub fn p1selc(&mut self) -> P1selcW<'_, P1selcSpec> {
        P1selcW::new(self, 0)
    }
}
#[doc = "Port 1 Complement Select register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1selcSpec;
impl crate::RegisterSpec for P1selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1selc::R`](R) reader structure"]
impl crate::Readable for P1selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p1selc::W`](W) writer structure"]
impl crate::Writable for P1selcSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1SELC to value 0"]
impl crate::Resettable for P1selcSpec {}
