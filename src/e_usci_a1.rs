#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_uca1ctlw: [u8; 0x02],
    uca1ctlw1: Uca1ctlw1,
    _reserved2: [u8; 0x02],
    _reserved_2_uca1: [u8; 0x02],
    uca1mctlw: Uca1mctlw,
    _reserved_4_uca1: [u8; 0x02],
    _reserved_5_uca1: [u8; 0x02],
    _reserved_6_uca1: [u8; 0x02],
    uca1abctl: Uca1abctl,
    uca1irctl: Uca1irctl,
    _reserved9: [u8; 0x06],
    _reserved_9_uca1: [u8; 0x02],
    _reserved_10_uca1: [u8; 0x02],
    _reserved_11_uca1: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_A1 Control Word Register 0"]
    #[inline(always)]
    pub const fn uca1ctlw0_spi(&self) -> &Uca1ctlw0Spi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub const fn uca1ctlw0(&self) -> &Uca1ctlw0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - USCI A1 Control Word Register 1"]
    #[inline(always)]
    pub const fn uca1ctlw1(&self) -> &Uca1ctlw1 {
        &self.uca1ctlw1
    }
    #[doc = "0x06 - eUSCI_A1 Bit Rate Control Register"]
    #[inline(always)]
    pub const fn uca1brw_spi(&self) -> &Uca1brwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn uca1brw(&self) -> &Uca1brw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - USCI A1 Modulation Control"]
    #[inline(always)]
    pub const fn uca1mctlw(&self) -> &Uca1mctlw {
        &self.uca1mctlw
    }
    #[doc = "0x0a - USCI A1 Status Register"]
    #[inline(always)]
    pub const fn uca1statw_spi(&self) -> &Uca1statwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0a - USCI A1 Status Register"]
    #[inline(always)]
    pub const fn uca1statw(&self) -> &Uca1statw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    #[inline(always)]
    pub const fn uca1rxbuf_spi(&self) -> &Uca1rxbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    #[inline(always)]
    pub const fn uca1rxbuf(&self) -> &Uca1rxbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca1txbuf_spi(&self) -> &Uca1txbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca1txbuf(&self) -> &Uca1txbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x10 - USCI A1 LIN Control"]
    #[inline(always)]
    pub const fn uca1abctl(&self) -> &Uca1abctl {
        &self.uca1abctl
    }
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    #[inline(always)]
    pub const fn uca1irctl(&self) -> &Uca1irctl {
        &self.uca1irctl
    }
    #[doc = "0x1a - USCI A1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca1ie_spi(&self) -> &Uca1ieSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1a - eUSCI_A1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca1ie(&self) -> &Uca1ie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1c - USCI A1 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn uca1ifg_spi(&self) -> &Uca1ifgSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - eUSCI_A1 Interrupt Flag Register"]
    #[inline(always)]
    pub const fn uca1ifg(&self) -> &Uca1ifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca1iv_spi(&self) -> &Uca1ivSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca1iv(&self) -> &Uca1iv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
}
#[doc = "UCA1CTLW0 (rw) register accessor: eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctlw0`] module"]
#[doc(alias = "UCA1CTLW0")]
pub type Uca1ctlw0 = crate::Reg<uca1ctlw0::Uca1ctlw0Spec>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca1ctlw0;
#[doc = "UCA1CTLW0_SPI (rw) register accessor: eUSCI_A1 Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctlw0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctlw0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctlw0_spi`] module"]
#[doc(alias = "UCA1CTLW0_SPI")]
pub type Uca1ctlw0Spi = crate::Reg<uca1ctlw0_spi::Uca1ctlw0SpiSpec>;
#[doc = "eUSCI_A1 Control Word Register 0"]
pub mod uca1ctlw0_spi;
#[doc = "UCA1STATW (rw) register accessor: USCI A1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1statw`] module"]
#[doc(alias = "UCA1STATW")]
pub type Uca1statw = crate::Reg<uca1statw::Uca1statwSpec>;
#[doc = "USCI A1 Status Register"]
pub mod uca1statw;
#[doc = "UCA1STATW_SPI (rw) register accessor: USCI A1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1statw_spi`] module"]
#[doc(alias = "UCA1STATW_SPI")]
pub type Uca1statwSpi = crate::Reg<uca1statw_spi::Uca1statwSpiSpec>;
#[doc = "USCI A1 Status Register"]
pub mod uca1statw_spi;
#[doc = "UCA1ABCTL (rw) register accessor: USCI A1 LIN Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1abctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1abctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1abctl`] module"]
#[doc(alias = "UCA1ABCTL")]
pub type Uca1abctl = crate::Reg<uca1abctl::Uca1abctlSpec>;
#[doc = "USCI A1 LIN Control"]
pub mod uca1abctl;
#[doc = "UCA1IRCTL (rw) register accessor: eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1irctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1irctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1irctl`] module"]
#[doc(alias = "UCA1IRCTL")]
pub type Uca1irctl = crate::Reg<uca1irctl::Uca1irctlSpec>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca1irctl;
#[doc = "UCA1IE (rw) register accessor: eUSCI_A1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ie`] module"]
#[doc(alias = "UCA1IE")]
pub type Uca1ie = crate::Reg<uca1ie::Uca1ieSpec>;
#[doc = "eUSCI_A1 Interrupt Enable Register"]
pub mod uca1ie;
#[doc = "UCA1IE_SPI (rw) register accessor: USCI A1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ie_spi`] module"]
#[doc(alias = "UCA1IE_SPI")]
pub type Uca1ieSpi = crate::Reg<uca1ie_spi::Uca1ieSpiSpec>;
#[doc = "USCI A1 Interrupt Enable Register"]
pub mod uca1ie_spi;
#[doc = "UCA1IFG (rw) register accessor: eUSCI_A1 Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ifg`] module"]
#[doc(alias = "UCA1IFG")]
pub type Uca1ifg = crate::Reg<uca1ifg::Uca1ifgSpec>;
#[doc = "eUSCI_A1 Interrupt Flag Register"]
pub mod uca1ifg;
#[doc = "UCA1IFG_SPI (rw) register accessor: USCI A1 Interrupt Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ifg_spi`] module"]
#[doc(alias = "UCA1IFG_SPI")]
pub type Uca1ifgSpi = crate::Reg<uca1ifg_spi::Uca1ifgSpiSpec>;
#[doc = "USCI A1 Interrupt Flags Register"]
pub mod uca1ifg_spi;
#[doc = "UCA1CTLW1 (rw) register accessor: USCI A1 Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctlw1`] module"]
#[doc(alias = "UCA1CTLW1")]
pub type Uca1ctlw1 = crate::Reg<uca1ctlw1::Uca1ctlw1Spec>;
#[doc = "USCI A1 Control Word Register 1"]
pub mod uca1ctlw1;
#[doc = "UCA1BRW (rw) register accessor: eUSCI_Ax Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1brw`] module"]
#[doc(alias = "UCA1BRW")]
pub type Uca1brw = crate::Reg<uca1brw::Uca1brwSpec>;
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca1brw;
#[doc = "UCA1BRW_SPI (rw) register accessor: eUSCI_A1 Bit Rate Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1brw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1brw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1brw_spi`] module"]
#[doc(alias = "UCA1BRW_SPI")]
pub type Uca1brwSpi = crate::Reg<uca1brw_spi::Uca1brwSpiSpec>;
#[doc = "eUSCI_A1 Bit Rate Control Register"]
pub mod uca1brw_spi;
#[doc = "UCA1MCTLW (rw) register accessor: USCI A1 Modulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1mctlw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1mctlw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1mctlw`] module"]
#[doc(alias = "UCA1MCTLW")]
pub type Uca1mctlw = crate::Reg<uca1mctlw::Uca1mctlwSpec>;
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctlw;
#[doc = "UCA1RXBUF (rw) register accessor: USCI A1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1rxbuf`] module"]
#[doc(alias = "UCA1RXBUF")]
pub type Uca1rxbuf = crate::Reg<uca1rxbuf::Uca1rxbufSpec>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf;
#[doc = "UCA1RXBUF_SPI (rw) register accessor: USCI A1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1rxbuf_spi`] module"]
#[doc(alias = "UCA1RXBUF_SPI")]
pub type Uca1rxbufSpi = crate::Reg<uca1rxbuf_spi::Uca1rxbufSpiSpec>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf_spi;
#[doc = "UCA1TXBUF (rw) register accessor: USCI A1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1txbuf`] module"]
#[doc(alias = "UCA1TXBUF")]
pub type Uca1txbuf = crate::Reg<uca1txbuf::Uca1txbufSpec>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf;
#[doc = "UCA1TXBUF_SPI (rw) register accessor: USCI A1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1txbuf_spi`] module"]
#[doc(alias = "UCA1TXBUF_SPI")]
pub type Uca1txbufSpi = crate::Reg<uca1txbuf_spi::Uca1txbufSpiSpec>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf_spi;
#[doc = "UCA1IV (rw) register accessor: USCI A1 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1iv`] module"]
#[doc(alias = "UCA1IV")]
pub type Uca1iv = crate::Reg<uca1iv::Uca1ivSpec>;
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv;
#[doc = "UCA1IV_SPI (rw) register accessor: USCI A1 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1iv_spi`] module"]
#[doc(alias = "UCA1IV_SPI")]
pub type Uca1ivSpi = crate::Reg<uca1iv_spi::Uca1ivSpiSpec>;
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv_spi;
