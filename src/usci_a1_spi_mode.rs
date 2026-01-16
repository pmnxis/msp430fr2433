#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uca1ctl1_spi: Uca1ctl1Spi,
    uca1ctl0_spi: Uca1ctl0Spi,
    _reserved2: [u8; 0x04],
    uca1br0_spi: Uca1br0Spi,
    uca1br1_spi: Uca1br1Spi,
    _reserved4: [u8; 0x02],
    uca1statw_spi: Uca1statwSpi,
    _reserved5: [u8; 0x01],
    uca1rxbuf_spi: Uca1rxbufSpi,
    uca1txbuf_spi: Uca1txbufSpi,
    _reserved7: [u8; 0x0a],
    uca1ie_spi: Uca1ieSpi,
    _reserved8: [u8; 0x01],
    uca1ifg_spi: Uca1ifgSpi,
    _reserved9: [u8; 0x01],
    uca1iv_spi: Uca1ivSpi,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    #[inline(always)]
    pub const fn uca1ctl1_spi(&self) -> &Uca1ctl1Spi {
        &self.uca1ctl1_spi
    }
    #[doc = "0x01 - USCI A1 Control Register 0"]
    #[inline(always)]
    pub const fn uca1ctl0_spi(&self) -> &Uca1ctl0Spi {
        &self.uca1ctl0_spi
    }
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca1br0_spi(&self) -> &Uca1br0Spi {
        &self.uca1br0_spi
    }
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca1br1_spi(&self) -> &Uca1br1Spi {
        &self.uca1br1_spi
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
#[doc = "UCA1CTL1_SPI (rw) register accessor: USCI A1 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctl1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctl1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl1_spi`] module"]
#[doc(alias = "UCA1CTL1_SPI")]
pub type Uca1ctl1Spi = crate::Reg<uca1ctl1_spi::Uca1ctl1SpiSpec>;
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1_spi;
#[doc = "UCA1CTL0_SPI (rw) register accessor: USCI A1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctl0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctl0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl0_spi`] module"]
#[doc(alias = "UCA1CTL0_SPI")]
pub type Uca1ctl0Spi = crate::Reg<uca1ctl0_spi::Uca1ctl0SpiSpec>;
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0_spi;
#[doc = "UCA1BR0_SPI (rw) register accessor: USCI A1 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1br0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1br0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br0_spi`] module"]
#[doc(alias = "UCA1BR0_SPI")]
pub type Uca1br0Spi = crate::Reg<uca1br0_spi::Uca1br0SpiSpec>;
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0_spi;
#[doc = "UCA1BR1_SPI (rw) register accessor: USCI A1 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1br1_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1br1_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br1_spi`] module"]
#[doc(alias = "UCA1BR1_SPI")]
pub type Uca1br1Spi = crate::Reg<uca1br1_spi::Uca1br1SpiSpec>;
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1_spi;
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
