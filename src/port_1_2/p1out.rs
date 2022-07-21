#[doc = "Register `P1OUT` reader"]
pub struct R(crate::R<P1OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1OUT` writer"]
pub struct W(crate::W<P1OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<P1OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1OUT0` reader - P1OUT0"]
pub type P1OUT0_R = crate::BitReader<bool>;
#[doc = "Field `P1OUT0` writer - P1OUT0"]
pub type P1OUT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, P1OUT_SPEC, bool, O>;
#[doc = "Field `P1OUT1` reader - P1OUT1"]
pub type P1OUT1_R = crate::BitReader<bool>;
#[doc = "Field `P1OUT1` writer - P1OUT1"]
pub type P1OUT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, P1OUT_SPEC, bool, O>;
#[doc = "Field `P1OUT2` reader - P1OUT2"]
pub type P1OUT2_R = crate::BitReader<bool>;
#[doc = "Field `P1OUT2` writer - P1OUT2"]
pub type P1OUT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, P1OUT_SPEC, bool, O>;
#[doc = "Field `P1OUT3` reader - P1OUT3"]
pub type P1OUT3_R = crate::BitReader<bool>;
#[doc = "Field `P1OUT3` writer - P1OUT3"]
pub type P1OUT3_W<'a, const O: u8> = crate::BitWriter<'a, u8, P1OUT_SPEC, bool, O>;
#[doc = "Field `P1OUT4` reader - P1OUT4"]
pub type P1OUT4_R = crate::BitReader<bool>;
#[doc = "Field `P1OUT4` writer - P1OUT4"]
pub type P1OUT4_W<'a, const O: u8> = crate::BitWriter<'a, u8, P1OUT_SPEC, bool, O>;
#[doc = "Field `P1OUT5` reader - P1OUT5"]
pub type P1OUT5_R = crate::BitReader<bool>;
#[doc = "Field `P1OUT5` writer - P1OUT5"]
pub type P1OUT5_W<'a, const O: u8> = crate::BitWriter<'a, u8, P1OUT_SPEC, bool, O>;
#[doc = "Field `P1OUT6` reader - P1OUT6"]
pub type P1OUT6_R = crate::BitReader<bool>;
#[doc = "Field `P1OUT6` writer - P1OUT6"]
pub type P1OUT6_W<'a, const O: u8> = crate::BitWriter<'a, u8, P1OUT_SPEC, bool, O>;
#[doc = "Field `P1OUT7` reader - P1OUT7"]
pub type P1OUT7_R = crate::BitReader<bool>;
#[doc = "Field `P1OUT7` writer - P1OUT7"]
pub type P1OUT7_W<'a, const O: u8> = crate::BitWriter<'a, u8, P1OUT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - P1OUT0"]
    #[inline(always)]
    pub fn p1out0(&self) -> P1OUT0_R {
        P1OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1OUT1"]
    #[inline(always)]
    pub fn p1out1(&self) -> P1OUT1_R {
        P1OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1OUT2"]
    #[inline(always)]
    pub fn p1out2(&self) -> P1OUT2_R {
        P1OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1OUT3"]
    #[inline(always)]
    pub fn p1out3(&self) -> P1OUT3_R {
        P1OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1OUT4"]
    #[inline(always)]
    pub fn p1out4(&self) -> P1OUT4_R {
        P1OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1OUT5"]
    #[inline(always)]
    pub fn p1out5(&self) -> P1OUT5_R {
        P1OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1OUT6"]
    #[inline(always)]
    pub fn p1out6(&self) -> P1OUT6_R {
        P1OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1OUT7"]
    #[inline(always)]
    pub fn p1out7(&self) -> P1OUT7_R {
        P1OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1OUT0"]
    #[inline(always)]
    pub fn p1out0(&mut self) -> P1OUT0_W<0> {
        P1OUT0_W::new(self)
    }
    #[doc = "Bit 1 - P1OUT1"]
    #[inline(always)]
    pub fn p1out1(&mut self) -> P1OUT1_W<1> {
        P1OUT1_W::new(self)
    }
    #[doc = "Bit 2 - P1OUT2"]
    #[inline(always)]
    pub fn p1out2(&mut self) -> P1OUT2_W<2> {
        P1OUT2_W::new(self)
    }
    #[doc = "Bit 3 - P1OUT3"]
    #[inline(always)]
    pub fn p1out3(&mut self) -> P1OUT3_W<3> {
        P1OUT3_W::new(self)
    }
    #[doc = "Bit 4 - P1OUT4"]
    #[inline(always)]
    pub fn p1out4(&mut self) -> P1OUT4_W<4> {
        P1OUT4_W::new(self)
    }
    #[doc = "Bit 5 - P1OUT5"]
    #[inline(always)]
    pub fn p1out5(&mut self) -> P1OUT5_W<5> {
        P1OUT5_W::new(self)
    }
    #[doc = "Bit 6 - P1OUT6"]
    #[inline(always)]
    pub fn p1out6(&mut self) -> P1OUT6_W<6> {
        P1OUT6_W::new(self)
    }
    #[doc = "Bit 7 - P1OUT7"]
    #[inline(always)]
    pub fn p1out7(&mut self) -> P1OUT7_W<7> {
        P1OUT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1out](index.html) module"]
pub struct P1OUT_SPEC;
impl crate::RegisterSpec for P1OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1out::R](R) reader structure"]
impl crate::Readable for P1OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1out::W](W) writer structure"]
impl crate::Writable for P1OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1OUT to value 0"]
impl crate::Resettable for P1OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
