#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Executor;
use esp_backtrace as _;
use hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, systimer::SystemTimer, Rtc};
use log::{LevelFilter, debug};
use logging::SimpleLogger;
use static_cell::make_static;

use crate::services::uart_service::*;

pub mod logging;
pub mod services;

static LOGGER: SimpleLogger = SimpleLogger;

#[entry]
fn main() -> ! {
    ::log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();
    
    log::info!("Init");
    log::info!("SystemTimer::now(): {}", SystemTimer::now());
    
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    embassy::init(
        &clocks,
        hal::timer::TimerGroup::new(
            peripherals.TIMG0,
            &clocks,
            &mut system.peripheral_clock_control,
        )
        .timer0,
    );

    let executor = make_static!(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(run1()).ok();
        spawner.spawn(run2()).ok();
    });
}
