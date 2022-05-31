#![no_std]


pub use nrf51_pac as pac;
mod time_driver_rtc1;

pub mod irqs {

    use crate::pac::Interrupt as InterruptEnum;
    use embassy_macros::interrupt_declare;

    /*interrupt_declare!(POWER_CLOCK);
    interrupt_declare!(RADIO);
    interrupt_declare!(UART0);
    interrupt_declare!(SPI0_TWI0);
    interrupt_declare!(SPI1_TWI1);
    interrupt_declare!(GPIOTE);
    interrupt_declare!(ADC);
    interrupt_declare!(TIMER0);
    interrupt_declare!(TIMER1);
    interrupt_declare!(TIMER2);
    interrupt_declare!(RTC0);
    interrupt_declare!(TEMP);
    interrupt_declare!(RNG);
    interrupt_declare!(ECB);
    interrupt_declare!(CCM_AAR);
    interrupt_declare!(WDT);*/
    interrupt_declare!(RTC1);
    /*interrupt_declare!(QDEC);
    interrupt_declare!(LPCOMP);
    interrupt_declare!(SWI0);
    interrupt_declare!(SWI1);
    interrupt_declare!(SWI2);
    interrupt_declare!(SWI3);
    interrupt_declare!(SWI4);
    interrupt_declare!(SWI5);*/
}

pub mod interrupt {
    //pub use crate::chip::irqs::*;
    pub use crate::irqs::*;
    pub use cortex_m::interrupt::{CriticalSection, Mutex};
    pub use embassy::interrupt::{declare, take, Interrupt};
    pub use embassy_hal_common::interrupt::Priority3 as Priority;
}
pub use embassy_macros::interrupt;

pub fn init(_rtc_peripheral: nrf51_pac::RTC1){
    let p = unsafe  { pac::Peripherals::steal() }; //we just need to check that lfclk is running
    assert!(p.CLOCK.lfclkstat.read().state().is_running(), 
        "LFCLK must be running for embassy timer to work! Example:
        let p = hal::pac::Peripherals::take().unwrap();
        let _clk = hal::Clocks::new(p.CLOCK).enable_ext_hfosc().set_lfclk_src_external().start_lfclk(););
    ");

    time_driver_rtc1::init(crate::interrupt::Priority::P0);
}