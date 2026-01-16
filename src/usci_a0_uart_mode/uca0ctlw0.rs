#[doc = "Register `UCA0CTLW0` reader"]
pub type R = crate::R<Uca0ctlw0Spec>;
#[doc = "Register `UCA0CTLW0` writer"]
pub type W = crate::W<Uca0ctlw0Spec>;
#[doc = "Field `UCSWRST` reader - Software reset enable"]
pub type UcswrstR = crate::BitReader;
#[doc = "Field `UCSWRST` writer - Software reset enable"]
pub type UcswrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXBRK` reader - Transmit break"]
pub type UctxbrkR = crate::BitReader;
#[doc = "Field `UCTXBRK` writer - Transmit break"]
pub type UctxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXADDR` reader - Transmit address"]
pub type UctxaddrR = crate::BitReader;
#[doc = "Field `UCTXADDR` writer - Transmit address"]
pub type UctxaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCDORM` reader - Dormant"]
pub type UcdormR = crate::BitReader;
#[doc = "Field `UCDORM` writer - Dormant"]
pub type UcdormW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRKIE` reader - Receive break character interrupt enable"]
pub type UcbrkieR = crate::BitReader;
#[doc = "Field `UCBRKIE` writer - Receive break character interrupt enable"]
pub type UcbrkieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXEIE` reader - Receive erroneous-character interrupt enable"]
pub type UcrxeieR = crate::BitReader;
#[doc = "Field `UCRXEIE` writer - Receive erroneous-character interrupt enable"]
pub type UcrxeieW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `UCSPB` reader - Stop bit select"]
pub type UcspbR = crate::BitReader;
#[doc = "Field `UCSPB` writer - Stop bit select"]
pub type UcspbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UC7BIT` reader - Character length"]
pub type Uc7bitR = crate::BitReader;
#[doc = "Field `UC7BIT` writer - Character length"]
pub type Uc7bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMSB` reader - MSB first select"]
pub type UcmsbR = crate::BitReader;
#[doc = "Field `UCMSB` writer - MSB first select"]
pub type UcmsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPAR` reader - Parity select"]
pub type UcparR = crate::BitReader;
#[doc = "Field `UCPAR` writer - Parity select"]
pub type UcparW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPEN` reader - Parity enable"]
pub type UcpenR = crate::BitReader;
#[doc = "Field `UCPEN` writer - Parity enable"]
pub type UcpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UcswrstR {
        UcswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uctxbrk(&self) -> UctxbrkR {
        UctxbrkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit address"]
    #[inline(always)]
    pub fn uctxaddr(&self) -> UctxaddrR {
        UctxaddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dormant"]
    #[inline(always)]
    pub fn ucdorm(&self) -> UcdormR {
        UcdormR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive break character interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&self) -> UcbrkieR {
        UcbrkieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive erroneous-character interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&self) -> UcrxeieR {
        UcrxeieR::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 11 - Stop bit select"]
    #[inline(always)]
    pub fn ucspb(&self) -> UcspbR {
        UcspbR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - Parity select"]
    #[inline(always)]
    pub fn ucpar(&self) -> UcparR {
        UcparR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Parity enable"]
    #[inline(always)]
    pub fn ucpen(&self) -> UcpenR {
        UcpenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UcswrstW<'_, Uca0ctlw0Spec> {
        UcswrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uctxbrk(&mut self) -> UctxbrkW<'_, Uca0ctlw0Spec> {
        UctxbrkW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit address"]
    #[inline(always)]
    pub fn uctxaddr(&mut self) -> UctxaddrW<'_, Uca0ctlw0Spec> {
        UctxaddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Dormant"]
    #[inline(always)]
    pub fn ucdorm(&mut self) -> UcdormW<'_, Uca0ctlw0Spec> {
        UcdormW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive break character interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&mut self) -> UcbrkieW<'_, Uca0ctlw0Spec> {
        UcbrkieW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive erroneous-character interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&mut self) -> UcrxeieW<'_, Uca0ctlw0Spec> {
        UcrxeieW::new(self, 5)
    }
    #[doc = "Bits 6:7 - eUSCI_A clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UcsselW<'_, Uca0ctlw0Spec> {
        UcsselW::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UcsyncW<'_, Uca0ctlw0Spec> {
        UcsyncW::new(self, 8)
    }
    #[doc = "Bits 9:10 - eUSCI_A mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UcmodeW<'_, Uca0ctlw0Spec> {
        UcmodeW::new(self, 9)
    }
    #[doc = "Bit 11 - Stop bit select"]
    #[inline(always)]
    pub fn ucspb(&mut self) -> UcspbW<'_, Uca0ctlw0Spec> {
        UcspbW::new(self, 11)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> Uc7bitW<'_, Uca0ctlw0Spec> {
        Uc7bitW::new(self, 12)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UcmsbW<'_, Uca0ctlw0Spec> {
        UcmsbW::new(self, 13)
    }
    #[doc = "Bit 14 - Parity select"]
    #[inline(always)]
    pub fn ucpar(&mut self) -> UcparW<'_, Uca0ctlw0Spec> {
        UcparW::new(self, 14)
    }
    #[doc = "Bit 15 - Parity enable"]
    #[inline(always)]
    pub fn ucpen(&mut self) -> UcpenW<'_, Uca0ctlw0Spec> {
        UcpenW::new(self, 15)
    }
}
#[doc = "eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctlw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctlw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0ctlw0Spec;
impl crate::RegisterSpec for Uca0ctlw0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0ctlw0::R`](R) reader structure"]
impl crate::Readable for Uca0ctlw0Spec {}
#[doc = "`write(|w| ..)` method takes [`uca0ctlw0::W`](W) writer structure"]
impl crate::Writable for Uca0ctlw0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0CTLW0 to value 0"]
impl crate::Resettable for Uca0ctlw0Spec {}
