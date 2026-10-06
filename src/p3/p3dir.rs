#[doc = "Register `P3DIR` reader"]
pub type R = crate::R<P3dirSpec>;
#[doc = "Register `P3DIR` writer"]
pub type W = crate::W<P3dirSpec>;
#[doc = "Field `P3DIR` reader - P3DIR0"]
pub type P3dirR = crate::FieldReader;
#[doc = "Field `P3DIR` writer - P3DIR0"]
pub type P3dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - P3DIR0"]
    #[inline(always)]
    pub fn p3dir(&self) -> P3dirR {
        P3dirR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P3DIR0"]
    #[inline(always)]
    pub fn p3dir(&mut self) -> P3dirW<'_, P3dirSpec> {
        P3dirW::new(self, 0)
    }
}
#[doc = "Port 3 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p3dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3dirSpec;
impl crate::RegisterSpec for P3dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3dir::R`](R) reader structure"]
impl crate::Readable for P3dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p3dir::W`](W) writer structure"]
impl crate::Writable for P3dirSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P3DIR to value 0"]
impl crate::Resettable for P3dirSpec {}
