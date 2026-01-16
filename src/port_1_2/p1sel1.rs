#[doc = "Register `P1SEL1` reader"]
pub type R = crate::R<P1sel1Spec>;
#[doc = "Register `P1SEL1` writer"]
pub type W = crate::W<P1sel1Spec>;
#[doc = "Field `P1SEL1_0` reader - P1SEL1_0"]
pub type P1sel1_0R = crate::BitReader;
#[doc = "Field `P1SEL1_0` writer - P1SEL1_0"]
pub type P1sel1_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL1_1` reader - P1SEL1_1"]
pub type P1sel1_1R = crate::BitReader;
#[doc = "Field `P1SEL1_1` writer - P1SEL1_1"]
pub type P1sel1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL1_2` reader - P1SEL1_2"]
pub type P1sel1_2R = crate::BitReader;
#[doc = "Field `P1SEL1_2` writer - P1SEL1_2"]
pub type P1sel1_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL1_3` reader - P1SEL1_3"]
pub type P1sel1_3R = crate::BitReader;
#[doc = "Field `P1SEL1_3` writer - P1SEL1_3"]
pub type P1sel1_3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL1_4` reader - P1SEL1_4"]
pub type P1sel1_4R = crate::BitReader;
#[doc = "Field `P1SEL1_4` writer - P1SEL1_4"]
pub type P1sel1_4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL1_5` reader - P1SEL1_5"]
pub type P1sel1_5R = crate::BitReader;
#[doc = "Field `P1SEL1_5` writer - P1SEL1_5"]
pub type P1sel1_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL1_6` reader - P1SEL1_6"]
pub type P1sel1_6R = crate::BitReader;
#[doc = "Field `P1SEL1_6` writer - P1SEL1_6"]
pub type P1sel1_6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL1_7` reader - P1SEL1_7"]
pub type P1sel1_7R = crate::BitReader;
#[doc = "Field `P1SEL1_7` writer - P1SEL1_7"]
pub type P1sel1_7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1SEL1_0"]
    #[inline(always)]
    pub fn p1sel1_0(&self) -> P1sel1_0R {
        P1sel1_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1SEL1_1"]
    #[inline(always)]
    pub fn p1sel1_1(&self) -> P1sel1_1R {
        P1sel1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1SEL1_2"]
    #[inline(always)]
    pub fn p1sel1_2(&self) -> P1sel1_2R {
        P1sel1_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1SEL1_3"]
    #[inline(always)]
    pub fn p1sel1_3(&self) -> P1sel1_3R {
        P1sel1_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1SEL1_4"]
    #[inline(always)]
    pub fn p1sel1_4(&self) -> P1sel1_4R {
        P1sel1_4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1SEL1_5"]
    #[inline(always)]
    pub fn p1sel1_5(&self) -> P1sel1_5R {
        P1sel1_5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1SEL1_6"]
    #[inline(always)]
    pub fn p1sel1_6(&self) -> P1sel1_6R {
        P1sel1_6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1SEL1_7"]
    #[inline(always)]
    pub fn p1sel1_7(&self) -> P1sel1_7R {
        P1sel1_7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1SEL1_0"]
    #[inline(always)]
    pub fn p1sel1_0(&mut self) -> P1sel1_0W<'_, P1sel1Spec> {
        P1sel1_0W::new(self, 0)
    }
    #[doc = "Bit 1 - P1SEL1_1"]
    #[inline(always)]
    pub fn p1sel1_1(&mut self) -> P1sel1_1W<'_, P1sel1Spec> {
        P1sel1_1W::new(self, 1)
    }
    #[doc = "Bit 2 - P1SEL1_2"]
    #[inline(always)]
    pub fn p1sel1_2(&mut self) -> P1sel1_2W<'_, P1sel1Spec> {
        P1sel1_2W::new(self, 2)
    }
    #[doc = "Bit 3 - P1SEL1_3"]
    #[inline(always)]
    pub fn p1sel1_3(&mut self) -> P1sel1_3W<'_, P1sel1Spec> {
        P1sel1_3W::new(self, 3)
    }
    #[doc = "Bit 4 - P1SEL1_4"]
    #[inline(always)]
    pub fn p1sel1_4(&mut self) -> P1sel1_4W<'_, P1sel1Spec> {
        P1sel1_4W::new(self, 4)
    }
    #[doc = "Bit 5 - P1SEL1_5"]
    #[inline(always)]
    pub fn p1sel1_5(&mut self) -> P1sel1_5W<'_, P1sel1Spec> {
        P1sel1_5W::new(self, 5)
    }
    #[doc = "Bit 6 - P1SEL1_6"]
    #[inline(always)]
    pub fn p1sel1_6(&mut self) -> P1sel1_6W<'_, P1sel1Spec> {
        P1sel1_6W::new(self, 6)
    }
    #[doc = "Bit 7 - P1SEL1_7"]
    #[inline(always)]
    pub fn p1sel1_7(&mut self) -> P1sel1_7W<'_, P1sel1Spec> {
        P1sel1_7W::new(self, 7)
    }
}
#[doc = "Port 1 Selection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p1sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1sel1Spec;
impl crate::RegisterSpec for P1sel1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1sel1::R`](R) reader structure"]
impl crate::Readable for P1sel1Spec {}
#[doc = "`write(|w| ..)` method takes [`p1sel1::W`](W) writer structure"]
impl crate::Writable for P1sel1Spec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1SEL1 to value 0"]
impl crate::Resettable for P1sel1Spec {}
