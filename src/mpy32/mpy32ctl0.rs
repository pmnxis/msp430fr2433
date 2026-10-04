#[doc = "Register `MPY32CTL0` reader"]
pub type R = crate::R<Mpy32ctl0Spec>;
#[doc = "Register `MPY32CTL0` writer"]
pub type W = crate::W<Mpy32ctl0Spec>;
#[doc = "Field `MPYC` reader - Carry of the multiplier"]
pub type MpycR = crate::BitReader;
#[doc = "Field `MPYC` writer - Carry of the multiplier"]
pub type MpycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPYFRAC` reader - Fractional mode"]
pub type MpyfracR = crate::BitReader;
#[doc = "Field `MPYFRAC` writer - Fractional mode"]
pub type MpyfracW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPYSAT` reader - Saturation mode"]
pub type MpysatR = crate::BitReader;
#[doc = "Field `MPYSAT` writer - Saturation mode"]
pub type MpysatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Multiplier mode Bit:0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mpym {
    #[doc = "0: MPY Multiply"]
    Mpy = 0,
    #[doc = "1: MPYS Signed multiply"]
    Mpys = 1,
    #[doc = "2: MAC Multiply accumulate"]
    Mac = 2,
    #[doc = "3: MACS Signed multiply accumulate"]
    Macs = 3,
}
impl From<Mpym> for u8 {
    #[inline(always)]
    fn from(variant: Mpym) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mpym {
    type Ux = u8;
}
impl crate::IsEnum for Mpym {}
#[doc = "Field `MPYM` reader - Multiplier mode Bit:0"]
pub type MpymR = crate::FieldReader<Mpym>;
impl MpymR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpym {
        match self.bits {
            0 => Mpym::Mpy,
            1 => Mpym::Mpys,
            2 => Mpym::Mac,
            3 => Mpym::Macs,
            _ => unreachable!(),
        }
    }
    #[doc = "MPY Multiply"]
    #[inline(always)]
    pub fn is_mpy(&self) -> bool {
        *self == Mpym::Mpy
    }
    #[doc = "MPYS Signed multiply"]
    #[inline(always)]
    pub fn is_mpys(&self) -> bool {
        *self == Mpym::Mpys
    }
    #[doc = "MAC Multiply accumulate"]
    #[inline(always)]
    pub fn is_mac(&self) -> bool {
        *self == Mpym::Mac
    }
    #[doc = "MACS Signed multiply accumulate"]
    #[inline(always)]
    pub fn is_macs(&self) -> bool {
        *self == Mpym::Macs
    }
}
#[doc = "Field `MPYM` writer - Multiplier mode Bit:0"]
pub type MpymW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mpym, crate::Safe>;
impl<'a, REG> MpymW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MPY Multiply"]
    #[inline(always)]
    pub fn mpy(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mpy)
    }
    #[doc = "MPYS Signed multiply"]
    #[inline(always)]
    pub fn mpys(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mpys)
    }
    #[doc = "MAC Multiply accumulate"]
    #[inline(always)]
    pub fn mac(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mac)
    }
    #[doc = "MACS Signed multiply accumulate"]
    #[inline(always)]
    pub fn macs(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Macs)
    }
}
#[doc = "Field `MPYOP1_32` reader - Bit-width of operand 1 0:16Bit / 1:32Bit"]
pub type Mpyop1_32R = crate::BitReader;
#[doc = "Field `MPYOP1_32` writer - Bit-width of operand 1 0:16Bit / 1:32Bit"]
pub type Mpyop1_32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPYOP2_32` reader - Bit-width of operand 2 0:16Bit / 1:32Bit"]
pub type Mpyop2_32R = crate::BitReader;
#[doc = "Field `MPYOP2_32` writer - Bit-width of operand 2 0:16Bit / 1:32Bit"]
pub type Mpyop2_32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPYDLYWRTEN` reader - Delayed write enable"]
pub type MpydlywrtenR = crate::BitReader;
#[doc = "Field `MPYDLYWRTEN` writer - Delayed write enable"]
pub type MpydlywrtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPYDLY32` reader - Delayed write mode"]
pub type Mpydly32R = crate::BitReader;
#[doc = "Field `MPYDLY32` writer - Delayed write mode"]
pub type Mpydly32W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&self) -> MpycR {
        MpycR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline(always)]
    pub fn mpyfrac(&self) -> MpyfracR {
        MpyfracR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&self) -> MpysatR {
        MpysatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline(always)]
    pub fn mpym(&self) -> MpymR {
        MpymR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn mpyop1_32(&self) -> Mpyop1_32R {
        Mpyop1_32R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn mpyop2_32(&self) -> Mpyop2_32R {
        Mpyop2_32R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline(always)]
    pub fn mpydlywrten(&self) -> MpydlywrtenR {
        MpydlywrtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline(always)]
    pub fn mpydly32(&self) -> Mpydly32R {
        Mpydly32R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&mut self) -> MpycW<'_, Mpy32ctl0Spec> {
        MpycW::new(self, 0)
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline(always)]
    pub fn mpyfrac(&mut self) -> MpyfracW<'_, Mpy32ctl0Spec> {
        MpyfracW::new(self, 2)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&mut self) -> MpysatW<'_, Mpy32ctl0Spec> {
        MpysatW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline(always)]
    pub fn mpym(&mut self) -> MpymW<'_, Mpy32ctl0Spec> {
        MpymW::new(self, 4)
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn mpyop1_32(&mut self) -> Mpyop1_32W<'_, Mpy32ctl0Spec> {
        Mpyop1_32W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn mpyop2_32(&mut self) -> Mpyop2_32W<'_, Mpy32ctl0Spec> {
        Mpyop2_32W::new(self, 7)
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline(always)]
    pub fn mpydlywrten(&mut self) -> MpydlywrtenW<'_, Mpy32ctl0Spec> {
        MpydlywrtenW::new(self, 8)
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline(always)]
    pub fn mpydly32(&mut self) -> Mpydly32W<'_, Mpy32ctl0Spec> {
        Mpydly32W::new(self, 9)
    }
}
#[doc = "MPY32 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy32ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy32ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpy32ctl0Spec;
impl crate::RegisterSpec for Mpy32ctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpy32ctl0::R`](R) reader structure"]
impl crate::Readable for Mpy32ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mpy32ctl0::W`](W) writer structure"]
impl crate::Writable for Mpy32ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPY32CTL0 to value 0"]
impl crate::Resettable for Mpy32ctl0Spec {}
