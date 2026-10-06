#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ta3ctl: Ta3ctl,
    ta3cctl0: Ta3cctl0,
    ta3cctl1: Ta3cctl1,
    _reserved3: [u8; 0x0a],
    ta3r: Ta3r,
    ta3ccr0: Ta3ccr0,
    ta3ccr1: Ta3ccr1,
    _reserved6: [u8; 0x0a],
    ta3ex0: Ta3ex0,
    _reserved7: [u8; 0x0c],
    ta3iv: Ta3iv,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer3_A2 Control"]
    #[inline(always)]
    pub const fn ta3ctl(&self) -> &Ta3ctl {
        &self.ta3ctl
    }
    #[doc = "0x02 - Timer3_A2 Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn ta3cctl0(&self) -> &Ta3cctl0 {
        &self.ta3cctl0
    }
    #[doc = "0x04 - Timer3_A2 Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn ta3cctl1(&self) -> &Ta3cctl1 {
        &self.ta3cctl1
    }
    #[doc = "0x10 - Timer3_A2"]
    #[inline(always)]
    pub const fn ta3r(&self) -> &Ta3r {
        &self.ta3r
    }
    #[doc = "0x12 - Timer3_A2 Capture/Compare 0"]
    #[inline(always)]
    pub const fn ta3ccr0(&self) -> &Ta3ccr0 {
        &self.ta3ccr0
    }
    #[doc = "0x14 - Timer3_A2 Capture/Compare 1"]
    #[inline(always)]
    pub const fn ta3ccr1(&self) -> &Ta3ccr1 {
        &self.ta3ccr1
    }
    #[doc = "0x20 - Timer3_A2 Expansion Register 0"]
    #[inline(always)]
    pub const fn ta3ex0(&self) -> &Ta3ex0 {
        &self.ta3ex0
    }
    #[doc = "0x2e - Timer3_A2 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn ta3iv(&self) -> &Ta3iv {
        &self.ta3iv
    }
}
#[doc = "TA3CTL (rw) register accessor: Timer3_A2 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3ctl`] module"]
#[doc(alias = "TA3CTL")]
pub type Ta3ctl = crate::Reg<ta3ctl::Ta3ctlSpec>;
#[doc = "Timer3_A2 Control"]
pub mod ta3ctl;
#[doc = "TA3CCTL0 (rw) register accessor: Timer3_A2 Capture/Compare Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3cctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3cctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3cctl0`] module"]
#[doc(alias = "TA3CCTL0")]
pub type Ta3cctl0 = crate::Reg<ta3cctl0::Ta3cctl0Spec>;
#[doc = "Timer3_A2 Capture/Compare Control 0"]
pub mod ta3cctl0;
#[doc = "TA3CCTL1 (rw) register accessor: Timer3_A2 Capture/Compare Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3cctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3cctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3cctl1`] module"]
#[doc(alias = "TA3CCTL1")]
pub type Ta3cctl1 = crate::Reg<ta3cctl1::Ta3cctl1Spec>;
#[doc = "Timer3_A2 Capture/Compare Control 1"]
pub mod ta3cctl1;
#[doc = "TA3R (rw) register accessor: Timer3_A2\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3r`] module"]
#[doc(alias = "TA3R")]
pub type Ta3r = crate::Reg<ta3r::Ta3rSpec>;
#[doc = "Timer3_A2"]
pub mod ta3r;
#[doc = "TA3CCR0 (rw) register accessor: Timer3_A2 Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3ccr0`] module"]
#[doc(alias = "TA3CCR0")]
pub type Ta3ccr0 = crate::Reg<ta3ccr0::Ta3ccr0Spec>;
#[doc = "Timer3_A2 Capture/Compare 0"]
pub mod ta3ccr0;
#[doc = "TA3CCR1 (rw) register accessor: Timer3_A2 Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3ccr1`] module"]
#[doc(alias = "TA3CCR1")]
pub type Ta3ccr1 = crate::Reg<ta3ccr1::Ta3ccr1Spec>;
#[doc = "Timer3_A2 Capture/Compare 1"]
pub mod ta3ccr1;
#[doc = "TA3EX0 (rw) register accessor: Timer3_A2 Expansion Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3ex0`] module"]
#[doc(alias = "TA3EX0")]
pub type Ta3ex0 = crate::Reg<ta3ex0::Ta3ex0Spec>;
#[doc = "Timer3_A2 Expansion Register 0"]
pub mod ta3ex0;
#[doc = "TA3IV (rw) register accessor: Timer3_A2 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ta3iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta3iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3iv`] module"]
#[doc(alias = "TA3IV")]
pub type Ta3iv = crate::Reg<ta3iv::Ta3ivSpec>;
#[doc = "Timer3_A2 Interrupt Vector Word"]
pub mod ta3iv;
