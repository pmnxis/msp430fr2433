#[doc = "Register `UCB0I2COA2` reader"]
pub type R = crate::R<Ucb0i2coa2Spec>;
#[doc = "Register `UCB0I2COA2` writer"]
pub type W = crate::W<Ucb0i2coa2Spec>;
#[doc = "Field `I2COA2` reader - I2C Own Address Bit 0"]
pub type I2coa2R = crate::FieldReader<u16>;
#[doc = "Field `I2COA2` writer - I2C Own Address Bit 0"]
pub type I2coa2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `UCOAEN` reader - I2C Own Address enable"]
pub type UcoaenR = crate::BitReader;
#[doc = "Field `UCOAEN` writer - I2C Own Address enable"]
pub type UcoaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - I2C Own Address Bit 0"]
    #[inline(always)]
    pub fn i2coa2(&self) -> I2coa2R {
        I2coa2R::new(self.bits & 0x03ff)
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
    pub fn i2coa2(&mut self) -> I2coa2W<'_, Ucb0i2coa2Spec> {
        I2coa2W::new(self, 0)
    }
    #[doc = "Bit 10 - I2C Own Address enable"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UcoaenW<'_, Ucb0i2coa2Spec> {
        UcoaenW::new(self, 10)
    }
}
#[doc = "USCI B0 I2C Own Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0i2coa2Spec;
impl crate::RegisterSpec for Ucb0i2coa2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0i2coa2::R`](R) reader structure"]
impl crate::Readable for Ucb0i2coa2Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2coa2::W`](W) writer structure"]
impl crate::Writable for Ucb0i2coa2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0I2COA2 to value 0"]
impl crate::Resettable for Ucb0i2coa2Spec {}
