#[doc = "Register `UCA1IE` reader"]
pub type R = crate::R<Uca1ieSpec>;
#[doc = "Register `UCA1IE` writer"]
pub type W = crate::W<Uca1ieSpec>;
#[doc = "Field `UCRXIE` reader - "]
pub type UcrxieR = crate::BitReader;
#[doc = "Field `UCRXIE` writer - "]
pub type UcrxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE` reader - "]
pub type UctxieR = crate::BitReader;
#[doc = "Field `UCTXIE` writer - "]
pub type UctxieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UcrxieR {
        UcrxieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uctxie(&self) -> UctxieR {
        UctxieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ucrxie(&mut self) -> UcrxieW<'_, Uca1ieSpec> {
        UcrxieW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uctxie(&mut self) -> UctxieW<'_, Uca1ieSpec> {
        UctxieW::new(self, 1)
    }
}
#[doc = "eUSCI_A1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1ieSpec;
impl crate::RegisterSpec for Uca1ieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca1ie::R`](R) reader structure"]
impl crate::Readable for Uca1ieSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1ie::W`](W) writer structure"]
impl crate::Writable for Uca1ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1IE to value 0"]
impl crate::Resettable for Uca1ieSpec {}
