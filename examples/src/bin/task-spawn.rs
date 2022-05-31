#![no_main]
#![no_std]


use nrf51_hal as hal;
use defmt_rtt as _; // global logger
use panic_probe as _;


unsafe fn make_static<T>(t: &T) -> &'static T {
    core::mem::transmute(t)
}

#[cortex_m_rt::entry]
fn main() -> ! {
    
    let p = hal::pac::Peripherals::take().unwrap();
    // we need to start lfclk in order RTC1 to run
    let _clk = hal::Clocks::new(p.CLOCK).enable_ext_hfosc().set_lfclk_src_external().start_lfclk();

    embassy_nrf51_time_driver_rtc1::init(p.RTC1);

    defmt::println!("NRF51 embassy example, no nightly features");

    static EXECUTOR_MAIN: embassy::util::Forever<embassy::executor::Executor> = embassy::util::Forever::new();

    let task_store_cpu_intensive = embassy::executor::raw::TaskStorage::new();
    let task_store_ping = embassy::executor::raw::TaskStorage::new();
     // Safety: these variables do live forever if main never returns.
    let task_store_cpu_intensive = unsafe { make_static(&task_store_cpu_intensive) };    
    let task_store_ping = unsafe { make_static(&task_store_ping) };   
   
    // Main executor: runs in thread mode, using WFE/SEV
    let executor = EXECUTOR_MAIN.put(embassy::executor::Executor::new());
    executor.run(|spawner| {
        defmt::unwrap!(spawner.spawn(task_store_cpu_intensive.spawn(|| run_cpu_intensive())));
        defmt::unwrap!(spawner.spawn(task_store_ping.spawn(|| run_ping())));
    });
}

async fn run_cpu_intensive() {
    loop {
        let start = embassy::time::Instant::now();
        defmt::println!("[run_cpu_intensive] Starting long computation");

        // Spin-wait to simulate a long CPU computation
        cortex_m::asm::delay(16_000_000); // ~2 seconds

        let end = embassy::time::Instant::now();
        let ms = end.duration_since(start).as_ticks() / 33;
        defmt::println!("[run_cpu_intensive] Long computation done in {} ms", ms);

        embassy::time::Timer::after(embassy::time::Duration::from_millis(5000)).await;
    }
}

async fn run_ping() {
    loop {
        defmt::println!("[run_ping] Ping");
        embassy::time::Timer::after(embassy::time::Duration::from_millis(1000)).await;
    }
}