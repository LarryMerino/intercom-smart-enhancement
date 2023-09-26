use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn run1() {
    loop {
        log::trace!("Hello world from embassy using esp-hal-async!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[embassy_executor::task]
pub async fn run2() {
    loop {
        log::warn!("Bing!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}
