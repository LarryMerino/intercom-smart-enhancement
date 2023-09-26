use core::fmt::Write;

use esp_println::println;
use hal::systimer::SystemTimer;
use log::{Level, Metadata, Record};
use owo_colors::{OwoColorize, AnsiColors};
use heapless::*;

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {

            let (marker, color) = match record.level() {
                Level::Error => ("E", AnsiColors::Red),
                Level::Warn => ("W", AnsiColors::Yellow),
                Level::Info => ("I", AnsiColors::Green),
                Level::Debug => ("D", AnsiColors::Blue),
                Level::Trace => ("T", AnsiColors::Magenta),
            };

            let mut target: String<24> = String::new();
            write!(target, "{:<24}", record.metadata().target().split("::").last().unwrap()).ok();

            let mut ts: String<12> = String::new();
            write!(ts, "{:<12}", SystemTimer::now()).ok();

            println!(
                "{} ({}) - [{}]: {}",
                marker.color(color),
                ts.color(color),
                target.color(color),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}