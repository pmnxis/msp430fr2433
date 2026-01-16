#[doc = "Register `UCA1IFG` reader"]
pub type R = crate::R<Uca1ifgSpec>;
#[doc = "Register `UCA1IFG` writer"]
pub type W = crate::W<Uca1ifgSpec>;
#[doc = "Field `UCRXIFG` reader - Receive interrupt flag. Is set when RXBUF has received a complete character"]
pub type UcrxifgR = crate::BitReader;
#[doc = "Field `UCRXIFG` writer - Receive interrupt flag. Is set when RXBUF has received a complete character"]
pub type UcrxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIFG` reader - Transmit interrupt flag. Is set when TXBUF is empty."]
pub type UctxifgR = crate::BitReader;
#[doc = "Field `UCTXIFG` writer - Transmit interrupt flag. Is set when TXBUF is empty."]
pub type UctxifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive interrupt flag. Is set when RXBUF has received a complete character"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UcrxifgR {
        UcrxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt flag. Is set when TXBUF is empty."]
    #[inline(always)]
    pub fn uctxifg(&self) -> UctxifgR {
        UctxifgR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt flag. Is set when RXBUF has received a complete character"]
    #[inline(always)]
    pub fn ucrxifg(&mut self) -> UcrxifgW<'_, Uca1ifgSpec> {
        UcrxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit interrupt flag. Is set when TXBUF is empty."]
    #[inline(always)]
    pub fn uctxifg(&mut self) -> UctxifgW<'_, Uca1ifgSpec> {
        UctxifgW::new(self, 1)
    }
}
#[doc = "eUSCI_A1 Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1ifgSpec;
impl crate::RegisterSpec for Uca1ifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca1ifg::R`](R) reader structure"]
impl crate::Readable for Uca1ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1ifg::W`](W) writer structure"]
impl crate::Writable for Uca1ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1IFG to value 0"]
impl crate::Resettable for Uca1ifgSpec {}
