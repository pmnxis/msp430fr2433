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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mpym {
    #[doc = "0: Multiplier mode: MPY"]
    Mpym0 = 0,
    #[doc = "1: Multiplier mode: MPYS"]
    Mpym1 = 1,
    #[doc = "2: Multiplier mode: MAC"]
    Mpym2 = 2,
    #[doc = "3: Multiplier mode: MACS"]
    Mpym3 = 3,
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
            0 => Mpym::Mpym0,
            1 => Mpym::Mpym1,
            2 => Mpym::Mpym2,
            3 => Mpym::Mpym3,
            _ => unreachable!(),
        }
    }
    #[doc = "Multiplier mode: MPY"]
    #[inline(always)]
    pub fn is_mpym_0(&self) -> bool {
        *self == Mpym::Mpym0
    }
    #[doc = "Multiplier mode: MPYS"]
    #[inline(always)]
    pub fn is_mpym_1(&self) -> bool {
        *self == Mpym::Mpym1
    }
    #[doc = "Multiplier mode: MAC"]
    #[inline(always)]
    pub fn is_mpym_2(&self) -> bool {
        *self == Mpym::Mpym2
    }
    #[doc = "Multiplier mode: MACS"]
    #[inline(always)]
    pub fn is_mpym_3(&self) -> bool {
        *self == Mpym::Mpym3
    }
}
#[doc = "Field `MPYM` writer - Multiplier mode Bit:0"]
pub type MpymW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mpym, crate::Safe>;
impl<'a, REG> MpymW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Multiplier mode: MPY"]
    #[inline(always)]
    pub fn mpym_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mpym0)
    }
    #[doc = "Multiplier mode: MPYS"]
    #[inline(always)]
    pub fn mpym_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mpym1)
    }
    #[doc = "Multiplier mode: MAC"]
    #[inline(always)]
    pub fn mpym_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mpym2)
    }
    #[doc = "Multiplier mode: MACS"]
    #[inline(always)]
    pub fn mpym_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mpym3)
    }
}
#[doc = "Field `OP1_32` reader - Bit-width of operand 1 0:16Bit / 1:32Bit"]
pub type Op1_32R = crate::BitReader;
#[doc = "Field `OP1_32` writer - Bit-width of operand 1 0:16Bit / 1:32Bit"]
pub type Op1_32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_32` reader - Bit-width of operand 2 0:16Bit / 1:32Bit"]
pub type Op2_32R = crate::BitReader;
#[doc = "Field `OP2_32` writer - Bit-width of operand 2 0:16Bit / 1:32Bit"]
pub type Op2_32W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn op1_32(&self) -> Op1_32R {
        Op1_32R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op2_32(&self) -> Op2_32R {
        Op2_32R::new(((self.bits >> 7) & 1) != 0)
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
    pub fn op1_32(&mut self) -> Op1_32W<'_, Mpy32ctl0Spec> {
        Op1_32W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op2_32(&mut self) -> Op2_32W<'_, Mpy32ctl0Spec> {
        Op2_32W::new(self, 7)
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
