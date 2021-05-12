use log::Level;
use colored::*;
use chrono::Local;
use std::io::prelude::*;

pub fn init_log() {
    #[cfg(debug_assertions)]
        std::env::set_var("RUST_LOG", "debug");
    #[cfg(debug_assertions)]
        let level = log::LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
        let level = log::LevelFilter::Info;
    #[cfg(not(debug_assertions))]
        std::env::set_var("RUST_LOG", "info");
    let logger = env_logger::builder()
        .format(|buf, record| {
            let now = Local::now();
            writeln!(
                buf,
                "[{}] [{}] {}: {}",
                now.format("%a %d %b %Y %H:%M:%S").to_string(),
                match record.level() {
                    Level::Error => "  ERROR  ".red().bold(),
                    Level::Warn => " WARNING ".yellow().bold(),
                    Level::Info => "  INFO   ".green().bold(),
                    Level::Debug => "  DEBUG  ".blue().bold(),
                    Level::Trace => "  TRACE  ".white().bold(),
                },
                record.target(),
                record.args()
            )
        }).build();

    async_log::Logger::wrap(logger, || 12).start(level).unwrap();
}