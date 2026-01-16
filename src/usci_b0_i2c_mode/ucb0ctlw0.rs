#[doc = "Register `UCB0CTLW0` reader"]
pub type R = crate::R<Ucb0ctlw0Spec>;
#[doc = "Register `UCB0CTLW0` writer"]
pub type W = crate::W<Ucb0ctlw0Spec>;
#[doc = "Field `UCSWRST` reader - Software reset enable"]
pub type UcswrstR = crate::BitReader;
#[doc = "Field `UCSWRST` writer - Software reset enable"]
pub type UcswrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXSTT` reader - Transmit START condition in master mode"]
pub type UctxsttR = crate::BitReader;
#[doc = "Field `UCTXSTT` writer - Transmit START condition in master mode"]
pub type UctxsttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXSTP` reader - Transmit STOP condition in master mode"]
pub type UctxstpR = crate::BitReader;
#[doc = "Field `UCTXSTP` writer - Transmit STOP condition in master mode"]
pub type UctxstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXNACK` reader - Transmit a NACK"]
pub type UctxnackR = crate::BitReader;
#[doc = "Field `UCTXNACK` writer - Transmit a NACK"]
pub type UctxnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTR` reader - Transmitter / receiver select"]
pub type UctrR = crate::BitReader;
#[doc = "Field `UCTR` writer - Transmitter / receiver select"]
pub type UctrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXACK` reader - Transmit ACK condition in slave mode"]
pub type UctxackR = crate::BitReader;
#[doc = "Field `UCTXACK` writer - Transmit ACK condition in slave mode"]
pub type UctxackW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `UCMM` reader - Multi-master environment select"]
pub type UcmmR = crate::BitReader;
#[doc = "Field `UCMM` writer - Multi-master environment select"]
pub type UcmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSLA10` reader - Slave addressing mode select"]
pub type Ucsla10R = crate::BitReader;
#[doc = "Field `UCSLA10` writer - Slave addressing mode select"]
pub type Ucsla10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCA10` reader - Own addressing mode select"]
pub type Uca10R = crate::BitReader;
#[doc = "Field `UCA10` writer - Own addressing mode select"]
pub type Uca10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UcswrstR {
        UcswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&self) -> UctxsttR {
        UctxsttR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&self) -> UctxstpR {
        UctxstpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&self) -> UctxnackR {
        UctxnackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter / receiver select"]
    #[inline(always)]
    pub fn uctr(&self) -> UctrR {
        UctrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&self) -> UctxackR {
        UctxackR::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&self) -> UcmmR {
        UcmmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&self) -> Ucsla10R {
        Ucsla10R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&self) -> Uca10R {
        Uca10R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UcswrstW<'_, Ucb0ctlw0Spec> {
        UcswrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&mut self) -> UctxsttW<'_, Ucb0ctlw0Spec> {
        UctxsttW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&mut self) -> UctxstpW<'_, Ucb0ctlw0Spec> {
        UctxstpW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&mut self) -> UctxnackW<'_, Ucb0ctlw0Spec> {
        UctxnackW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmitter / receiver select"]
    #[inline(always)]
    pub fn uctr(&mut self) -> UctrW<'_, Ucb0ctlw0Spec> {
        UctrW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&mut self) -> UctxackW<'_, Ucb0ctlw0Spec> {
        UctxackW::new(self, 5)
    }
    #[doc = "Bits 6:7 - eUSCI_A clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UcsselW<'_, Ucb0ctlw0Spec> {
        UcsselW::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UcsyncW<'_, Ucb0ctlw0Spec> {
        UcsyncW::new(self, 8)
    }
    #[doc = "Bits 9:10 - eUSCI_A mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UcmodeW<'_, Ucb0ctlw0Spec> {
        UcmodeW::new(self, 9)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UcmstW<'_, Ucb0ctlw0Spec> {
        UcmstW::new(self, 11)
    }
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&mut self) -> UcmmW<'_, Ucb0ctlw0Spec> {
        UcmmW::new(self, 13)
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&mut self) -> Ucsla10W<'_, Ucb0ctlw0Spec> {
        Ucsla10W::new(self, 14)
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&mut self) -> Uca10W<'_, Ucb0ctlw0Spec> {
        Uca10W::new(self, 15)
    }
}
#[doc = "eUSCI_B0 Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctlw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctlw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ctlw0Spec;
impl crate::RegisterSpec for Ucb0ctlw0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ctlw0::R`](R) reader structure"]
impl crate::Readable for Ucb0ctlw0Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctlw0::W`](W) writer structure"]
impl crate::Writable for Ucb0ctlw0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0CTLW0 to value 0"]
impl crate::Resettable for Ucb0ctlw0Spec {}
