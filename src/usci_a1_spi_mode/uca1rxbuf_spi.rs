#[doc = "Register `UCA1RXBUF_SPI` reader"]
pub type R = crate::R<Uca1rxbufSpiSpec>;
#[doc = "Register `UCA1RXBUF_SPI` writer"]
pub type W = crate::W<Uca1rxbufSpiSpec>;
#[doc = "Field `UCRXBUF` reader - Receive data buffer"]
pub type UcrxbufR = crate::FieldReader;
#[doc = "Field `UCRXBUF` writer - Receive data buffer"]
pub type UcrxbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UcrxbufR {
        UcrxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&mut self) -> UcrxbufW<'_, Uca1rxbufSpiSpec> {
        UcrxbufW::new(self, 0)
    }
}
#[doc = "USCI A1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1rxbuf_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1rxbuf_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1rxbufSpiSpec;
impl crate::RegisterSpec for Uca1rxbufSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca1rxbuf_spi::R`](R) reader structure"]
impl crate::Readable for Uca1rxbufSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1rxbuf_spi::W`](W) writer structure"]
impl crate::Writable for Uca1rxbufSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1RXBUF_SPI to value 0"]
impl crate::Resettable for Uca1rxbufSpiSpec {}
