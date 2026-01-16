#[doc = "Register `UCA1IV_SPI` reader"]
pub type R = crate::R<Uca1ivSpiSpec>;
#[doc = "Register `UCA1IV_SPI` writer"]
pub type W = crate::W<Uca1ivSpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A1 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1iv_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1iv_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1ivSpiSpec;
impl crate::RegisterSpec for Uca1ivSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca1iv_spi::R`](R) reader structure"]
impl crate::Readable for Uca1ivSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1iv_spi::W`](W) writer structure"]
impl crate::Writable for Uca1ivSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1IV_SPI to value 0"]
impl crate::Resettable for Uca1ivSpiSpec {}
