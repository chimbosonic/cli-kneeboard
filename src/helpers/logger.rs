use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn setup_logger(level: LevelFilter) {
    _ = Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.level(),
                record.args()
            )
        })
        .filter(None, level)
        .try_init();
}
