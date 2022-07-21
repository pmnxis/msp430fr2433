#[doc = "Register `UCB0IE` reader"]
pub struct R(crate::R<UCB0IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0IE` writer"]
pub struct W(crate::W<UCB0IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0IE_SPEC>;
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
impl From<crate::W<UCB0IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0IE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ie](index.html) module"]
pub struct UCB0IE_SPEC;
impl crate::RegisterSpec for UCB0IE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0ie::R](R) reader structure"]
impl crate::Readable for UCB0IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ie::W](W) writer structure"]
impl crate::Writable for UCB0IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0IE to value 0"]
impl crate::Resettable for UCB0IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
