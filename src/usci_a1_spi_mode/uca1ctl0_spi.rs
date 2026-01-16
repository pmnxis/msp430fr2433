#[doc = "Register `UCA1CTL0_SPI` reader"]
pub type R = crate::R<Uca1ctl0SpiSpec>;
#[doc = "Register `UCA1CTL0_SPI` writer"]
pub type W = crate::W<Uca1ctl0SpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USCI A1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctl0_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctl0_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1ctl0SpiSpec;
impl crate::RegisterSpec for Uca1ctl0SpiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1ctl0_spi::R`](R) reader structure"]
impl crate::Readable for Uca1ctl0SpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1ctl0_spi::W`](W) writer structure"]
impl crate::Writable for Uca1ctl0SpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1CTL0_SPI to value 0"]
impl crate::Resettable for Uca1ctl0SpiSpec {}
