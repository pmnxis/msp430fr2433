#[doc = "Register `UCA1IE_SPI` reader"]
pub type R = crate::R<Uca1ieSpiSpec>;
#[doc = "Register `UCA1IE_SPI` writer"]
pub type W = crate::W<Uca1ieSpiSpec>;
#[doc = "Field `UCRXIE` reader - USCI Receive Interrupt Enable"]
pub type UcrxieR = crate::BitReader;
#[doc = "Field `UCRXIE` writer - USCI Receive Interrupt Enable"]
pub type UcrxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE` reader - USCI Transmit Interrupt Enable"]
pub type UctxieR = crate::BitReader;
#[doc = "Field `UCTXIE` writer - USCI Transmit Interrupt Enable"]
pub type UctxieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UcrxieR {
        UcrxieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UctxieR {
        UctxieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ucrxie(&mut self) -> UcrxieW<'_, Uca1ieSpiSpec> {
        UcrxieW::new(self, 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn uctxie(&mut self) -> UctxieW<'_, Uca1ieSpiSpec> {
        UctxieW::new(self, 1)
    }
}
#[doc = "USCI A1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ie_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ie_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1ieSpiSpec;
impl crate::RegisterSpec for Uca1ieSpiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1ie_spi::R`](R) reader structure"]
impl crate::Readable for Uca1ieSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1ie_spi::W`](W) writer structure"]
impl crate::Writable for Uca1ieSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1IE_SPI to value 0"]
impl crate::Resettable for Uca1ieSpiSpec {}
