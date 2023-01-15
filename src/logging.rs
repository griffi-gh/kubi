//! Custom env_logger options and styling

use env_logger::{fmt::Color, Builder, Env};
use log::Level;
use std::io::Write;

pub fn init() {
    let mut env = Env::default();
    if cfg!(debug_assertions) {
        env = env.filter_or("RUST_LOG", "info");
    }
    Builder::from_env(env)
        .format(|buf, record| {
            let mut level_style = buf.style();
            level_style.set_color(match record.level() {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                _ => Color::Blue
            }).set_bold(true);

            let mut location_style = buf.style();
            location_style.set_bold(true);
            location_style.set_dimmed(true);

            let mut location_line_style = buf.style();
            location_line_style.set_dimmed(true);
            
            writeln!(
                buf,
                "{} {:<50}\t{}{}{}",
                level_style.value(match record.level() {
                    Level::Error => "[e]",
                    Level::Warn =>  "[w]",
                    Level::Info =>  "[i]",
                    Level::Debug => "[d]",
                    Level::Trace => "[t]",
                }),
                format!("{}", record.args()),
                location_style.value(record.target()),
                location_line_style.value(" :"),
                location_line_style.value(record.line().unwrap_or(0))
            )
        })
        .init();
}
