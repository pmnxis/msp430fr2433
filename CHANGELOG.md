# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [v0.3.0]
- (Breaking) Peripherals, registers, fields and enum variants named as in the other MSP430FR2xx PACs:
  - Peripherals: `FRAM` is `FRCTL`, `CRC16` is `CRC`, `WATCHDOG_TIMER` is `WDT_A`, `REAL_TIME_CLOCK` is `RTC`, `BACKUP_MEMORY` is `BAKMEM` (SLAU445I Table 7-1), `MPY_16` and `MPY_32` are one `MPY32` (SLAU445I Table 16-7), `TIMER_0_A3` to `TIMER_3_A2` are `TA0` to `TA3`, and `USCI_A0_UART_MODE`, `USCI_A1_UART_MODE` and `USCI_B0_I2C_MODE` are `E_USCI_A0`, `E_USCI_A1` and `E_USCI_B0`. The SPI mode peripherals are merged into these, as registers with an `_spi` suffix (e.g. `uca0ctlw0_spi`).
  - eUSCI: the byte registers `UCA0CTL0`, `UCA0CTL1`, `UCA0BR0` and `UCA0BR1` are removed, as `UCA0CTLW0` and `UCA0BRW` cover them, `UCAxIRCTL` replaces `UCAxIRTCTL` and `UCAxIRRCTL`, and `UCB0STAT_I2C`, `UCB0IE_I2C` and `UCB0IFG_I2C` are `UCB0STATW`, `UCB0IE` and `UCB0IFG`. Multi-bit fields are single fields (`UCDELIM`, `ADDRX`, `ADDMASK`, `I2CSA`), and `UCADDR` is `UCADDR_UCIDLE`. Every field has the enum it has on the other devices, and the fields `UCIV`, `UCTBCNT`, `UCB0RXBUF.UCRXBUF`, `UCB0TXBUF.UCTXBUF` and the UART start-bit and transmit-complete interrupt bits are added.
  - `CSCTL0.DCO` and `CSCTL0.MOD` are single fields instead of `DCO0` to `DCO8` and `MOD0` to `MOD4`.
  - Enum variants: `WDTCTL.WDTIS` (`_2g` to `_64`), `WDTCTL.WDTSSEL`, `FRCTL0.NWAITS` (`Wait0` to `Wait7`), `CSCTL3.FLLREFDIV` (`_1` to `_512`), the Timer_A `ID`, `MC`, `TASSEL`, `TAIDEX`, `CCIS` and `CM`, `ADCCTL1.ADCCONSEQ`, `ADCCTL2.ADCPDIV`, `MPY32CTL0.MPYM`, `CSCTL7.FLLUNLOCK` (`Locked`, `TooSlow`, `TooFast`, `OutOfRange`) and `CSCTL7.FLLUNLOCKHIS` (`Locked`, `TooSlow`, `TooFast`, `TooSlowAndFast`).
- Add the `Tlv` peripheral with the device descriptors (SLASE59F Table 6-22).
- Add `pmmctl0_h` and `frctl0_h`, the upper bytes of PMMCTL0 and FRCTL0, whose `lock()` locks the PMM and FRAM controller registers again.
- Add `SFRRPCR.SYSFLTE`, and enums for the `SFRRPCR` fields, `SFRIFG1.OFIFG`, `ADCCTL2.ADCDF` and `SYSCFG1.IRMSEL`.
- Add enums for `CSCTL7.FLLWARNEN`, `GCCTL0.FRPWR`, the `SYSCTL` and `SYSBSLC` fields, and the LPM3.5 switch, `PM5CTL0.LPM5SM` (`Automatic`, `Manual`) and `PM5CTL0.LPM5SW` (`Disconnected`, `Connected`).
- (Breaking) Names as in the user's guide, SLAU445I: `MPY32CTL0.OP1_32` and `OP2_32` are `MPYOP1_32` and `MPYOP2_32` (SLAU445I Table 16-9), and the P3 registers have one field each, `P3IN` and so on, like P1 and P2, instead of a field per bit (SLAU445I Table 8-9 to Table 8-14).
- (Breaking) Remove the `PA` peripheral (TI's `PORT_1_2`), which repeated the P1 and P2 registers, and `GCCTL0.ACCTEIE`, which neither the user's guide (SLAU445I Table 6-3) nor the data sheet has.
- Add the CRC data and result fields, and `crcdi_l` and `crcdirb_l` for byte writes.
- `CSCTL0.DCO`, `CSCTL0.MOD`, `CSCTL1.DCOFTRIM`, `CSCTL2.FLLN` and the CRC data fields can be written with the safe `set()`.
- Add the `defmt` feature, which implements `defmt::Format` for the enumerated values and `Interrupt` (svd2rust `--impl-defmt defmt`, also in `regenerate.sh`).
- Add the `SYSRSTIV`, `SYSSNIV`, `SYSUNIV`, `ADCIV` and `TAxIV` fields, which the registers lacked, with their values named after their sources: `SYSRSTIV` is `Brownout`, `ResetPin`, `SoftwareBor`, `Lpmx5WakeUp`, `SecurityViolation`, `Svsh`, `SoftwarePor`, `WatchdogTimeout`, `WatchdogPassword`, `FramPassword`, `FramBitError`, `PeripheralAreaFetch`, `PmmPassword` and `FllUnlock`, `SYSSNIV` is `SvsLowPowerResetEntry`, `FramUncorrectableBitError`, `VacantMemoryAccess`, `JtagMailboxIn`, `JtagMailboxOut` and `FramCorrectableBitError`, and `SYSUNIV` is `NmiPin` and `OscillatorFault`, as in the data sheet (SLASE59F Table 6-9), without a variant for 00h and the reserved values; `ADCIV` is `None`, `Overflow`, `TimeOverflow`, `AboveWindow`, `BelowWindow`, `InsideWindow` and `ResultReady` (SLAU445I Table 21-15), and `TAxIV` `None`, `Taccr1` to `Taccr6` and `Taifg`, as on the other devices (SLAU445I Table 13-8).
- (Breaking) Enum variants named after their meaning: the ADC's `ADCSHT` is `Cycles4` to `Cycles1024`, `ADCDIV` `_1` to `_8`, `ADCSSEL` `Modclk`, `Aclk` and `Smclk`, `ADCRES` `Bits8` and `Bits10`, `ADCSR` `Max200ksps` and `Max50ksps`, `ADCSHS` `Software`, `Rtc` and `Timer` (SLASE59F Table 6-16) and `ADCSREF` `AvccAvss` to `VerefPlusVerefMinus` (SLAU445I Table 21-3 to Table 21-8); `CSCTL1.DCORSEL` is `Range1mhz` to `Range16mhz` (SLAU445I Table 3-5), `PMMCTL2.REFVSEL` `V1_5` (SLAU445I Table 2-4), and the `P1IV` and `P2IV` values `None` and `Ifg0` to `Ifg7`. Values a field has twice, and reserved ones, have no variant.

## [v0.2.0]
- Regenerated with svd2rust 0.37.1: 
  - (Breaking) Peripheral names have changed from `SCREAMING_SNAKE` to `snake_case`, and type names have changed to `PascalCase`, dropping any underscores.
  - (Breaking) All registers are now accessed through methods. Previously registers were sometimes fields and sometimes methods (based on whether the hardware address was aliased by multiple registers).
- (Breaking) Rename some registers to match user guide. Ex: GPIO registers: `PORT_1_2` --> `PA`. `PORT_3` --> `P3`.
- (Breaking) Replace some generic field enums with user-friendly versions (e.g. `Flld` variants names are now the clock division ratio instead of just `Flld0`, `Flld1`, etc.).
- Add several missing registers
- Added individual `P1` and `P2` GPIO peripherals. Previously they could only be accessed all together via `PA`.

## [v0.1.0] - 2025-10-30
- Initial release
