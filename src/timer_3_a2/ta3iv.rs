#[doc = "Register `TA3IV` reader"]
pub type R = crate::R<Ta3ivSpec>;
#[doc = "Register `TA3IV` writer"]
pub type W = crate::W<Ta3ivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer3_A2 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta3ivSpec;
impl crate::RegisterSpec for Ta3ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta3iv::R`](R) reader structure"]
impl crate::Readable for Ta3ivSpec {}
#[doc = "`write(|w| ..)` method takes [`ta3iv::W`](W) writer structure"]
impl crate::Writable for Ta3ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TA3IV to value 0"]
impl crate::Resettable for Ta3ivSpec {}
