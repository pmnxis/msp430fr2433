#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uca1ctlw0_spi: Uca1ctlw0Spi,
    _reserved1: [u8; 0x04],
    uca1brw_spi: Uca1brwSpi,
    _reserved2: [u8; 0x02],
    uca1statw_spi: Uca1statwSpi,
    uca1rxbuf_spi: Uca1rxbufSpi,
    uca1txbuf_spi: Uca1txbufSpi,
    _reserved5: [u8; 0x0a],
    uca1ie_spi: Uca1ieSpi,
    uca1ifg_spi: Uca1ifgSpi,
    _reserved7: [u8; 0x01],
    uca1iv_spi: Uca1ivSpi,
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_A1 Control Word Register 0"]
    #[inline(always)]
    pub const fn uca1ctlw0_spi(&self) -> &Uca1ctlw0Spi {
        &self.uca1ctlw0_spi
    }
    #[doc = "0x06 - eUSCI_A1 Bit Rate Control Register"]
    #[inline(always)]
    pub const fn uca1brw_spi(&self) -> &Uca1brwSpi {
        &self.uca1brw_spi
    }
    #[doc = "0x0a - USCI A1 Status Register"]
    #[inline(always)]
    pub const fn uca1statw_spi(&self) -> &Uca1statwSpi {
        &self.uca1statw_spi
    }
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    #[inline(always)]
    pub const fn uca1rxbuf_spi(&self) -> &Uca1rxbufSpi {
        &self.uca1rxbuf_spi
    }
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca1txbuf_spi(&self) -> &Uca1txbufSpi {
        &self.uca1txbuf_spi
    }
    #[doc = "0x1a - USCI A1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca1ie_spi(&self) -> &Uca1ieSpi {
        &self.uca1ie_spi
    }
    #[doc = "0x1c - USCI A1 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn uca1ifg_spi(&self) -> &Uca1ifgSpi {
        &self.uca1ifg_spi
    }
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca1iv_spi(&self) -> &Uca1ivSpi {
        &self.uca1iv_spi
    }
}
#[doc = "UCA1CTLW0_SPI (rw) register accessor: eUSCI_A1 Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctlw0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctlw0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctlw0_spi`] module"]
#[doc(alias = "UCA1CTLW0_SPI")]
pub type Uca1ctlw0Spi = crate::Reg<uca1ctlw0_spi::Uca1ctlw0SpiSpec>;
#[doc = "eUSCI_A1 Control Word Register 0"]
pub mod uca1ctlw0_spi;
#[doc = "UCA1BRW_SPI (rw) register accessor: eUSCI_A1 Bit Rate Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1brw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1brw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1brw_spi`] module"]
#[doc(alias = "UCA1BRW_SPI")]
pub type Uca1brwSpi = crate::Reg<uca1brw_spi::Uca1brwSpiSpec>;
#[doc = "eUSCI_A1 Bit Rate Control Register"]
pub mod uca1brw_spi;
#[doc = "UCA1STATW_SPI (rw) register accessor: USCI A1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1statw_spi`] module"]
#[doc(alias = "UCA1STATW_SPI")]
pub type Uca1statwSpi = crate::Reg<uca1statw_spi::Uca1statwSpiSpec>;
#[doc = "USCI A1 Status Register"]
pub mod uca1statw_spi;
#[doc = "UCA1IE_SPI (rw) register accessor: USCI A1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ie_spi`] module"]
#[doc(alias = "UCA1IE_SPI")]
pub type Uca1ieSpi = crate::Reg<uca1ie_spi::Uca1ieSpiSpec>;
#[doc = "USCI A1 Interrupt Enable Register"]
pub mod uca1ie_spi;
#[doc = "UCA1IFG_SPI (rw) register accessor: USCI A1 Interrupt Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ifg_spi`] module"]
#[doc(alias = "UCA1IFG_SPI")]
pub type Uca1ifgSpi = crate::Reg<uca1ifg_spi::Uca1ifgSpiSpec>;
#[doc = "USCI A1 Interrupt Flags Register"]
pub mod uca1ifg_spi;
#[doc = "UCA1RXBUF_SPI (rw) register accessor: USCI A1 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1rxbuf_spi`] module"]
#[doc(alias = "UCA1RXBUF_SPI")]
pub type Uca1rxbufSpi = crate::Reg<uca1rxbuf_spi::Uca1rxbufSpiSpec>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf_spi;
#[doc = "UCA1TXBUF_SPI (rw) register accessor: USCI A1 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1txbuf_spi`] module"]
#[doc(alias = "UCA1TXBUF_SPI")]
pub type Uca1txbufSpi = crate::Reg<uca1txbuf_spi::Uca1txbufSpiSpec>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf_spi;
#[doc = "UCA1IV_SPI (rw) register accessor: USCI A1 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1iv_spi`] module"]
#[doc(alias = "UCA1IV_SPI")]
pub type Uca1ivSpi = crate::Reg<uca1iv_spi::Uca1ivSpiSpec>;
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv_spi;
