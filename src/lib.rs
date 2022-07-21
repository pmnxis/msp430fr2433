#![feature(abi_msp430_interrupt)]
#![doc = "Peripheral access API for MSP430FR2433 microcontrollers (generated using svd2rust v0.24.1 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.24.1/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[cfg(feature = "rt")]
pub use msp430_rt::interrupt;
#[doc = "Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn USCI_B0();
    fn USCI_A1();
    fn USCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn UNMI();
    fn SYSNMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 59] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORT2 },
    Vector { _handler: PORT1 },
    Vector { _handler: ADC },
    Vector { _handler: USCI_B0 },
    Vector { _handler: USCI_A1 },
    Vector { _handler: USCI_A0 },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector {
        _handler: TIMER3_A1,
    },
    Vector {
        _handler: TIMER3_A0,
    },
    Vector {
        _handler: TIMER2_A1,
    },
    Vector {
        _handler: TIMER2_A0,
    },
    Vector {
        _handler: TIMER1_A1,
    },
    Vector {
        _handler: TIMER1_A0,
    },
    Vector {
        _handler: TIMER0_A1,
    },
    Vector {
        _handler: TIMER0_A0,
    },
    Vector { _handler: UNMI },
    Vector { _handler: SYSNMI },
];
#[doc = r"Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "41 - 0xFFDA Port 2"]
    PORT2 = 41,
    #[doc = "42 - 0xFFDC Port 1"]
    PORT1 = 42,
    #[doc = "43 - 0xFFDE ADC"]
    ADC = 43,
    #[doc = "44 - 0xFFE0 USCI B0 Receive/Transmit"]
    USCI_B0 = 44,
    #[doc = "45 - 0xFFE2 USCI A1 Receive/Transmit"]
    USCI_A1 = 45,
    #[doc = "46 - 0xFFE4 USCI A0 Receive/Transmit"]
    USCI_A0 = 46,
    #[doc = "47 - 0xFFE6 Watchdog Timer"]
    WDT = 47,
    #[doc = "48 - 0xFFE8 RTC"]
    RTC = 48,
    #[doc = "49 - 0xFFEA Timer3_A2 CC1, TA"]
    TIMER3_A1 = 49,
    #[doc = "50 - 0xFFEC Timer3_A2 CC0"]
    TIMER3_A0 = 50,
    #[doc = "51 - 0xFFEE Timer2_A2 CC1, TA"]
    TIMER2_A1 = 51,
    #[doc = "52 - 0xFFF0 Timer2_A2 CC0"]
    TIMER2_A0 = 52,
    #[doc = "53 - 0xFFF2 Timer1_A3 CC1-2, TA"]
    TIMER1_A1 = 53,
    #[doc = "54 - 0xFFF4 Timer1_A3 CC0"]
    TIMER1_A0 = 54,
    #[doc = "55 - 0xFFF6 Timer0_A3 CC1-2, TA"]
    TIMER0_A1 = 55,
    #[doc = "56 - 0xFFE8 Timer0_A3 CC0"]
    TIMER0_A0 = 56,
    #[doc = "57 - 0xFFFA User Non-maskable"]
    UNMI = 57,
    #[doc = "58 - 0xFFFC System Non-maskable"]
    SYSNMI = 58,
}
#[doc = "Port 1/2"]
pub struct PORT_1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_1_2 {}
impl PORT_1_2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const port_1_2::RegisterBlock = 0x0200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_1_2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PORT_1_2 {
    type Target = port_1_2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PORT_1_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORT_1_2").finish()
    }
}
#[doc = "Port 1/2"]
pub mod port_1_2;
#[doc = "Port 3"]
pub struct PORT_3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_3 {}
impl PORT_3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const port_3::RegisterBlock = 0x0220 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PORT_3 {
    type Target = port_3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PORT_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORT_3").finish()
    }
}
#[doc = "Port 3"]
pub mod port_3;
#[doc = "USCI_A0 UART Mode"]
pub struct USCI_A0_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_UART_MODE {}
impl USCI_A0_UART_MODE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usci_a0_uart_mode::RegisterBlock = 0x0500 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_uart_mode::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USCI_A0_UART_MODE {
    type Target = usci_a0_uart_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USCI_A0_UART_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USCI_A0_UART_MODE").finish()
    }
}
#[doc = "USCI_A0 UART Mode"]
pub mod usci_a0_uart_mode;
#[doc = "USCI_A0 SPI Mode"]
pub struct USCI_A0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_SPI_MODE {}
impl USCI_A0_SPI_MODE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usci_a0_spi_mode::RegisterBlock = 0x0500 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_spi_mode::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USCI_A0_SPI_MODE {
    type Target = usci_a0_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USCI_A0_SPI_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USCI_A0_SPI_MODE").finish()
    }
}
#[doc = "USCI_A0 SPI Mode"]
pub mod usci_a0_spi_mode;
#[doc = "USCI_A1 UART Mode"]
pub struct USCI_A1_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A1_UART_MODE {}
impl USCI_A1_UART_MODE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usci_a1_uart_mode::RegisterBlock = 0x0520 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a1_uart_mode::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USCI_A1_UART_MODE {
    type Target = usci_a1_uart_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USCI_A1_UART_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USCI_A1_UART_MODE").finish()
    }
}
#[doc = "USCI_A1 UART Mode"]
pub mod usci_a1_uart_mode;
#[doc = "USCI_A1 SPI Mode"]
pub struct USCI_A1_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A1_SPI_MODE {}
impl USCI_A1_SPI_MODE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usci_a1_spi_mode::RegisterBlock = 0x0520 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a1_spi_mode::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USCI_A1_SPI_MODE {
    type Target = usci_a1_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USCI_A1_SPI_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USCI_A1_SPI_MODE").finish()
    }
}
#[doc = "USCI_A1 SPI Mode"]
pub mod usci_a1_spi_mode;
#[doc = "USCI_B0 I2C Mode"]
pub struct USCI_B0_I2C_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_I2C_MODE {}
impl USCI_B0_I2C_MODE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usci_b0_i2c_mode::RegisterBlock = 0x0540 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_i2c_mode::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USCI_B0_I2C_MODE {
    type Target = usci_b0_i2c_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USCI_B0_I2C_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USCI_B0_I2C_MODE").finish()
    }
}
#[doc = "USCI_B0 I2C Mode"]
pub mod usci_b0_i2c_mode;
#[doc = "USCI_B0 SPI Mode"]
pub struct USCI_B0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_SPI_MODE {}
impl USCI_B0_SPI_MODE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usci_b0_spi_mode::RegisterBlock = 0x0540 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_spi_mode::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USCI_B0_SPI_MODE {
    type Target = usci_b0_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USCI_B0_SPI_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USCI_B0_SPI_MODE").finish()
    }
}
#[doc = "USCI_B0 SPI Mode"]
pub mod usci_b0_spi_mode;
#[doc = "SFR Special Function Registers"]
pub struct SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SFR {}
impl SFR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sfr::RegisterBlock = 0x0100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sfr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SFR {
    type Target = sfr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SFR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFR").finish()
    }
}
#[doc = "SFR Special Function Registers"]
pub mod sfr;
#[doc = "PMM Power Management System"]
pub struct PMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMM {}
impl PMM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pmm::RegisterBlock = 0x0120 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PMM {
    type Target = pmm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PMM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMM").finish()
    }
}
#[doc = "PMM Power Management System"]
pub mod pmm;
#[doc = "SYS System Module"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sys::RegisterBlock = 0x0140 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS").finish()
    }
}
#[doc = "SYS System Module"]
pub mod sys;
#[doc = "CS Clock System"]
pub struct CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CS {}
impl CS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cs::RegisterBlock = 0x0180 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cs::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CS {
    type Target = cs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS").finish()
    }
}
#[doc = "CS Clock System"]
pub mod cs;
#[doc = "FRAM"]
pub struct FRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FRAM {}
impl FRAM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const fram::RegisterBlock = 0x01a0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fram::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FRAM {
    type Target = fram::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FRAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAM").finish()
    }
}
#[doc = "FRAM"]
pub mod fram;
#[doc = "CRC16"]
pub struct CRC16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC16 {}
impl CRC16 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crc16::RegisterBlock = 0x01c0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc16::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRC16 {
    type Target = crc16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRC16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC16").finish()
    }
}
#[doc = "CRC16"]
pub mod crc16;
#[doc = "Watchdog Timer"]
pub struct WATCHDOG_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG_TIMER {}
impl WATCHDOG_TIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const watchdog_timer::RegisterBlock = 0x01cc as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog_timer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WATCHDOG_TIMER {
    type Target = watchdog_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WATCHDOG_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WATCHDOG_TIMER").finish()
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog_timer;
#[doc = "Real-Time Clock"]
pub struct REAL_TIME_CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REAL_TIME_CLOCK {}
impl REAL_TIME_CLOCK {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const real_time_clock::RegisterBlock = 0x0300 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const real_time_clock::RegisterBlock {
        Self::PTR
    }
}
impl Deref for REAL_TIME_CLOCK {
    type Target = real_time_clock::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for REAL_TIME_CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REAL_TIME_CLOCK").finish()
    }
}
#[doc = "Real-Time Clock"]
pub mod real_time_clock;
#[doc = "Timer0_A3"]
pub struct TIMER_0_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_0_A3 {}
impl TIMER_0_A3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_0_a3::RegisterBlock = 0x0380 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_0_a3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER_0_A3 {
    type Target = timer_0_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER_0_A3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_0_A3").finish()
    }
}
#[doc = "Timer0_A3"]
pub mod timer_0_a3;
#[doc = "Timer1_A3"]
pub struct TIMER_1_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_1_A3 {}
impl TIMER_1_A3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_1_a3::RegisterBlock = 0x03c0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_1_a3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER_1_A3 {
    type Target = timer_1_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER_1_A3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_1_A3").finish()
    }
}
#[doc = "Timer1_A3"]
pub mod timer_1_a3;
#[doc = "Timer2_A2"]
pub struct TIMER_2_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_2_A2 {}
impl TIMER_2_A2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_2_a2::RegisterBlock = 0x0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_2_a2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER_2_A2 {
    type Target = timer_2_a2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER_2_A2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_2_A2").finish()
    }
}
#[doc = "Timer2_A2"]
pub mod timer_2_a2;
#[doc = "Timer3_A2"]
pub struct TIMER_3_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_3_A2 {}
impl TIMER_3_A2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_3_a2::RegisterBlock = 0x0440 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_3_a2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER_3_A2 {
    type Target = timer_3_a2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER_3_A2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_3_A2").finish()
    }
}
#[doc = "Timer3_A2"]
pub mod timer_3_a2;
#[doc = "MPY 16 Multiplier 16 Bit Mode"]
pub struct MPY_16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY_16 {}
impl MPY_16 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const mpy_16::RegisterBlock = 0x04c0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpy_16::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MPY_16 {
    type Target = mpy_16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MPY_16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPY_16").finish()
    }
}
#[doc = "MPY 16 Multiplier 16 Bit Mode"]
pub mod mpy_16;
#[doc = "MPY 32 Multiplier 32 Bit Mode"]
pub struct MPY_32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY_32 {}
impl MPY_32 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const mpy_32::RegisterBlock = 0x04d0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpy_32::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MPY_32 {
    type Target = mpy_32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MPY_32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPY_32").finish()
    }
}
#[doc = "MPY 32 Multiplier 32 Bit Mode"]
pub mod mpy_32;
#[doc = "Backup Memory"]
pub struct BACKUP_MEMORY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BACKUP_MEMORY {}
impl BACKUP_MEMORY {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const backup_memory::RegisterBlock = 0x0660 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const backup_memory::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BACKUP_MEMORY {
    type Target = backup_memory::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BACKUP_MEMORY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_MEMORY").finish()
    }
}
#[doc = "Backup Memory"]
pub mod backup_memory;
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x0700 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "ADC"]
pub mod adc;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PORT_1_2"]
    pub PORT_1_2: PORT_1_2,
    #[doc = "PORT_3"]
    pub PORT_3: PORT_3,
    #[doc = "USCI_A0_UART_MODE"]
    pub USCI_A0_UART_MODE: USCI_A0_UART_MODE,
    #[doc = "USCI_A0_SPI_MODE"]
    pub USCI_A0_SPI_MODE: USCI_A0_SPI_MODE,
    #[doc = "USCI_A1_UART_MODE"]
    pub USCI_A1_UART_MODE: USCI_A1_UART_MODE,
    #[doc = "USCI_A1_SPI_MODE"]
    pub USCI_A1_SPI_MODE: USCI_A1_SPI_MODE,
    #[doc = "USCI_B0_I2C_MODE"]
    pub USCI_B0_I2C_MODE: USCI_B0_I2C_MODE,
    #[doc = "USCI_B0_SPI_MODE"]
    pub USCI_B0_SPI_MODE: USCI_B0_SPI_MODE,
    #[doc = "SFR"]
    pub SFR: SFR,
    #[doc = "PMM"]
    pub PMM: PMM,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "CS"]
    pub CS: CS,
    #[doc = "FRAM"]
    pub FRAM: FRAM,
    #[doc = "CRC16"]
    pub CRC16: CRC16,
    #[doc = "WATCHDOG_TIMER"]
    pub WATCHDOG_TIMER: WATCHDOG_TIMER,
    #[doc = "REAL_TIME_CLOCK"]
    pub REAL_TIME_CLOCK: REAL_TIME_CLOCK,
    #[doc = "TIMER_0_A3"]
    pub TIMER_0_A3: TIMER_0_A3,
    #[doc = "TIMER_1_A3"]
    pub TIMER_1_A3: TIMER_1_A3,
    #[doc = "TIMER_2_A2"]
    pub TIMER_2_A2: TIMER_2_A2,
    #[doc = "TIMER_3_A2"]
    pub TIMER_3_A2: TIMER_3_A2,
    #[doc = "MPY_16"]
    pub MPY_16: MPY_16,
    #[doc = "MPY_32"]
    pub MPY_32: MPY_32,
    #[doc = "BACKUP_MEMORY"]
    pub BACKUP_MEMORY: BACKUP_MEMORY,
    #[doc = "ADC"]
    pub ADC: ADC,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        msp430::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PORT_1_2: PORT_1_2 {
                _marker: PhantomData,
            },
            PORT_3: PORT_3 {
                _marker: PhantomData,
            },
            USCI_A0_UART_MODE: USCI_A0_UART_MODE {
                _marker: PhantomData,
            },
            USCI_A0_SPI_MODE: USCI_A0_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_A1_UART_MODE: USCI_A1_UART_MODE {
                _marker: PhantomData,
            },
            USCI_A1_SPI_MODE: USCI_A1_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_B0_I2C_MODE: USCI_B0_I2C_MODE {
                _marker: PhantomData,
            },
            USCI_B0_SPI_MODE: USCI_B0_SPI_MODE {
                _marker: PhantomData,
            },
            SFR: SFR {
                _marker: PhantomData,
            },
            PMM: PMM {
                _marker: PhantomData,
            },
            SYS: SYS {
                _marker: PhantomData,
            },
            CS: CS {
                _marker: PhantomData,
            },
            FRAM: FRAM {
                _marker: PhantomData,
            },
            CRC16: CRC16 {
                _marker: PhantomData,
            },
            WATCHDOG_TIMER: WATCHDOG_TIMER {
                _marker: PhantomData,
            },
            REAL_TIME_CLOCK: REAL_TIME_CLOCK {
                _marker: PhantomData,
            },
            TIMER_0_A3: TIMER_0_A3 {
                _marker: PhantomData,
            },
            TIMER_1_A3: TIMER_1_A3 {
                _marker: PhantomData,
            },
            TIMER_2_A2: TIMER_2_A2 {
                _marker: PhantomData,
            },
            TIMER_3_A2: TIMER_3_A2 {
                _marker: PhantomData,
            },
            MPY_16: MPY_16 {
                _marker: PhantomData,
            },
            MPY_32: MPY_32 {
                _marker: PhantomData,
            },
            BACKUP_MEMORY: BACKUP_MEMORY {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
        }
    }
}