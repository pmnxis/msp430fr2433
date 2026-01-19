#[doc = "Register `UCA1IRRCTL` reader"]
pub type R = crate::R<Uca1irrctlSpec>;
#[doc = "Register `UCA1IRRCTL` writer"]
pub type W = crate::W<Uca1irrctlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A1 IrDA Receive Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1irrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1irrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1irrctlSpec;
impl crate::RegisterSpec for Uca1irrctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1irrctl::R`](R) reader structure"]
impl crate::Readable for Uca1irrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1irrctl::W`](W) writer structure"]
impl crate::Writable for Uca1irrctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1IRRCTL to value 0"]
impl crate::Resettable for Uca1irrctlSpec {}
