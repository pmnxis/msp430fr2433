#[doc = "Register `UCB0I2COA1` reader"]
pub type R = crate::R<Ucb0i2coa1Spec>;
#[doc = "Register `UCB0I2COA1` writer"]
pub type W = crate::W<Ucb0i2coa1Spec>;
#[doc = "Field `I2COA1` reader - I2C Own Address Bit 0"]
pub type I2coa1R = crate::FieldReader<u16>;
#[doc = "Field `I2COA1` writer - I2C Own Address Bit 0"]
pub type I2coa1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `UCOAEN` reader - I2C Own Address enable"]
pub type UcoaenR = crate::BitReader;
#[doc = "Field `UCOAEN` writer - I2C Own Address enable"]
pub type UcoaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - I2C Own Address Bit 0"]
    #[inline(always)]
    pub fn i2coa1(&self) -> I2coa1R {
        I2coa1R::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - I2C Own Address enable"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UcoaenR {
        UcoaenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C Own Address Bit 0"]
    #[inline(always)]
    pub fn i2coa1(&mut self) -> I2coa1W<'_, Ucb0i2coa1Spec> {
        I2coa1W::new(self, 0)
    }
    #[doc = "Bit 10 - I2C Own Address enable"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UcoaenW<'_, Ucb0i2coa1Spec> {
        UcoaenW::new(self, 10)
    }
}
#[doc = "USCI B0 I2C Own Address 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0i2coa1Spec;
impl crate::RegisterSpec for Ucb0i2coa1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0i2coa1::R`](R) reader structure"]
impl crate::Readable for Ucb0i2coa1Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2coa1::W`](W) writer structure"]
impl crate::Writable for Ucb0i2coa1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0I2COA1 to value 0"]
impl crate::Resettable for Ucb0i2coa1Spec {}
