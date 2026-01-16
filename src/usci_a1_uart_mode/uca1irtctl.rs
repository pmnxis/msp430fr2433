#[doc = "Register `UCA1IRTCTL` reader"]
pub type R = crate::R<Uca1irtctlSpec>;
#[doc = "Register `UCA1IRTCTL` writer"]
pub type W = crate::W<Uca1irtctlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A1 IrDA Transmit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1irtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1irtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1irtctlSpec;
impl crate::RegisterSpec for Uca1irtctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1irtctl::R`](R) reader structure"]
impl crate::Readable for Uca1irtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1irtctl::W`](W) writer structure"]
impl crate::Writable for Uca1irtctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1IRTCTL to value 0"]
impl crate::Resettable for Uca1irtctlSpec {}
