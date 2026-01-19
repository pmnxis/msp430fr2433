#[doc = "Register `CSCTL4` reader"]
pub type R = crate::R<Csctl4Spec>;
#[doc = "Register `CSCTL4` writer"]
pub type W = crate::W<Csctl4Spec>;
#[doc = "MCLK and SMCLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selms {
    #[doc = "0: DCO CLKDIV"]
    Dcoclkdiv = 0,
    #[doc = "1: REFOCLK"]
    Refoclk = 1,
    #[doc = "2: XT1CLK"]
    Xt1clk = 2,
    #[doc = "3: VLOCLK"]
    Vloclk = 3,
}
impl From<Selms> for u8 {
    #[inline(always)]
    fn from(variant: Selms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selms {
    type Ux = u8;
}
impl crate::IsEnum for Selms {}
#[doc = "Field `SELMS` reader - MCLK and SMCLK Source Select Bit: 0"]
pub type SelmsR = crate::FieldReader<Selms>;
impl SelmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selms> {
        match self.bits {
            0 => Some(Selms::Dcoclkdiv),
            1 => Some(Selms::Refoclk),
            2 => Some(Selms::Xt1clk),
            3 => Some(Selms::Vloclk),
            _ => None,
        }
    }
    #[doc = "DCO CLKDIV"]
    #[inline(always)]
    pub fn is_dcoclkdiv(&self) -> bool {
        *self == Selms::Dcoclkdiv
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == Selms::Refoclk
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == Selms::Xt1clk
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Selms::Vloclk
    }
}
#[doc = "Field `SELMS` writer - MCLK and SMCLK Source Select Bit: 0"]
pub type SelmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selms>;
impl<'a, REG> SelmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCO CLKDIV"]
    #[inline(always)]
    pub fn dcoclkdiv(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Dcoclkdiv)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Refoclk)
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Xt1clk)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Vloclk)
    }
}
#[doc = "ACLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sela {
    #[doc = "0: Source ACLK from XT1CLK with divider (no more than 40kHz)"]
    Xt1clk = 0,
    #[doc = "1: Source ACLK from the internal 32kHz clock source"]
    Refoclk = 1,
}
impl From<Sela> for bool {
    #[inline(always)]
    fn from(variant: Sela) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELA` reader - ACLK Source Select Bit: 0"]
pub type SelaR = crate::BitReader<Sela>;
impl SelaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sela {
        match self.bits {
            false => Sela::Xt1clk,
            true => Sela::Refoclk,
        }
    }
    #[doc = "Source ACLK from XT1CLK with divider (no more than 40kHz)"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == Sela::Xt1clk
    }
    #[doc = "Source ACLK from the internal 32kHz clock source"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == Sela::Refoclk
    }
}
#[doc = "Field `SELA` writer - ACLK Source Select Bit: 0"]
pub type SelaW<'a, REG> = crate::BitWriter<'a, REG, Sela>;
impl<'a, REG> SelaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Source ACLK from XT1CLK with divider (no more than 40kHz)"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Xt1clk)
    }
    #[doc = "Source ACLK from the internal 32kHz clock source"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Refoclk)
    }
}
impl R {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selms(&self) -> SelmsR {
        SelmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&self) -> SelaR {
        SelaR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selms(&mut self) -> SelmsW<'_, Csctl4Spec> {
        SelmsW::new(self, 0)
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&mut self) -> SelaW<'_, Csctl4Spec> {
        SelaW::new(self, 8)
    }
}
#[doc = "CS Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl4Spec;
impl crate::RegisterSpec for Csctl4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl4::R`](R) reader structure"]
impl crate::Readable for Csctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl4::W`](W) writer structure"]
impl crate::Writable for Csctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL4 to value 0"]
impl crate::Resettable for Csctl4Spec {}
