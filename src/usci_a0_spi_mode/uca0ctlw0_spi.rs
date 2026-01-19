#[doc = "Register `UCA0CTLW0_SPI` reader"]
pub type R = crate::R<Uca0ctlw0SpiSpec>;
#[doc = "Register `UCA0CTLW0_SPI` writer"]
pub type W = crate::W<Uca0ctlw0SpiSpec>;
#[doc = "Field `UCSWRST` reader - Software reset enable"]
pub type UcswrstR = crate::BitReader;
#[doc = "Field `UCSWRST` writer - Software reset enable"]
pub type UcswrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTEM` reader - STE mode select in master mode."]
pub type UcstemR = crate::BitReader;
#[doc = "Field `UCSTEM` writer - STE mode select in master mode."]
pub type UcstemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSSEL` reader - eUSCI_A clock source select"]
pub type UcsselR = crate::FieldReader;
#[doc = "Field `UCSSEL` writer - eUSCI_A clock source select"]
pub type UcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UCSYNC` reader - Synchronous mode enable"]
pub type UcsyncR = crate::BitReader;
#[doc = "Field `UCSYNC` writer - Synchronous mode enable"]
pub type UcsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMODE` reader - eUSCI_A mode"]
pub type UcmodeR = crate::FieldReader;
#[doc = "Field `UCMODE` writer - eUSCI_A mode"]
pub type UcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UCMST` reader - Master mode select"]
pub type UcmstR = crate::BitReader;
#[doc = "Field `UCMST` writer - Master mode select"]
pub type UcmstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UC7BIT` reader - Character length"]
pub type Uc7bitR = crate::BitReader;
#[doc = "Field `UC7BIT` writer - Character length"]
pub type Uc7bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMSB` reader - MSB first select"]
pub type UcmsbR = crate::BitReader;
#[doc = "Field `UCMSB` writer - MSB first select"]
pub type UcmsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCKPL` reader - Clock polarity select"]
pub type UcckplR = crate::BitReader;
#[doc = "Field `UCCKPL` writer - Clock polarity select"]
pub type UcckplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCKPH` reader - Clock phase select"]
pub type UcckphR = crate::BitReader;
#[doc = "Field `UCCKPH` writer - Clock phase select"]
pub type UcckphW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UcswrstR {
        UcswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STE mode select in master mode."]
    #[inline(always)]
    pub fn ucstem(&self) -> UcstemR {
        UcstemR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 6:7 - eUSCI_A clock source select"]
    #[inline(always)]
    pub fn ucssel(&self) -> UcsselR {
        UcsselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&self) -> UcsyncR {
        UcsyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - eUSCI_A mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UcmodeR {
        UcmodeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UcmstR {
        UcmstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&self) -> Uc7bitR {
        Uc7bitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UcmsbR {
        UcmsbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clock polarity select"]
    #[inline(always)]
    pub fn ucckpl(&self) -> UcckplR {
        UcckplR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock phase select"]
    #[inline(always)]
    pub fn ucckph(&self) -> UcckphR {
        UcckphR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UcswrstW<'_, Uca0ctlw0SpiSpec> {
        UcswrstW::new(self, 0)
    }
    #[doc = "Bit 1 - STE mode select in master mode."]
    #[inline(always)]
    pub fn ucstem(&mut self) -> UcstemW<'_, Uca0ctlw0SpiSpec> {
        UcstemW::new(self, 1)
    }
    #[doc = "Bits 6:7 - eUSCI_A clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UcsselW<'_, Uca0ctlw0SpiSpec> {
        UcsselW::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UcsyncW<'_, Uca0ctlw0SpiSpec> {
        UcsyncW::new(self, 8)
    }
    #[doc = "Bits 9:10 - eUSCI_A mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UcmodeW<'_, Uca0ctlw0SpiSpec> {
        UcmodeW::new(self, 9)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UcmstW<'_, Uca0ctlw0SpiSpec> {
        UcmstW::new(self, 11)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> Uc7bitW<'_, Uca0ctlw0SpiSpec> {
        Uc7bitW::new(self, 12)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UcmsbW<'_, Uca0ctlw0SpiSpec> {
        UcmsbW::new(self, 13)
    }
    #[doc = "Bit 14 - Clock polarity select"]
    #[inline(always)]
    pub fn ucckpl(&mut self) -> UcckplW<'_, Uca0ctlw0SpiSpec> {
        UcckplW::new(self, 14)
    }
    #[doc = "Bit 15 - Clock phase select"]
    #[inline(always)]
    pub fn ucckph(&mut self) -> UcckphW<'_, Uca0ctlw0SpiSpec> {
        UcckphW::new(self, 15)
    }
}
#[doc = "eUSCI_A0 Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctlw0_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctlw0_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0ctlw0SpiSpec;
impl crate::RegisterSpec for Uca0ctlw0SpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0ctlw0_spi::R`](R) reader structure"]
impl crate::Readable for Uca0ctlw0SpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0ctlw0_spi::W`](W) writer structure"]
impl crate::Writable for Uca0ctlw0SpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0CTLW0_SPI to value 0"]
impl crate::Resettable for Uca0ctlw0SpiSpec {}
