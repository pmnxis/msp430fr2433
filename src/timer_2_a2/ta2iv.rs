#[doc = "Register `TA2IV` reader"]
pub type R = crate::R<Ta2ivSpec>;
#[doc = "Register `TA2IV` writer"]
pub type W = crate::W<Ta2ivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer2_A2 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta2ivSpec;
impl crate::RegisterSpec for Ta2ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta2iv::R`](R) reader structure"]
impl crate::Readable for Ta2ivSpec {}
#[doc = "`write(|w| ..)` method takes [`ta2iv::W`](W) writer structure"]
impl crate::Writable for Ta2ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA2IV to value 0"]
impl crate::Resettable for Ta2ivSpec {}
