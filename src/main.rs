use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_t_or_exit, App, Arg,
};

use crate::config::TomatoConfig;
use crate::session::Session;

mod config;
mod session;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("work_time")
                .long("work-time")
                .short("w")
                .default_value("25")
                .help("Duration of work time (minutes)"),
        )
        .arg(
            Arg::with_name("short_break_time")
                .long("short-break-time")
                .short("s")
                .default_value("5")
                .help("Duration of short break time (minutes)"),
        )
        .arg(
            Arg::with_name("long_break_time")
                .long("long-break-time")
                .short("l")
                .default_value("15")
                .help("Duration of long break time (minutes)"),
        )
        .get_matches();

    let work_time = value_t_or_exit!(matches.value_of("work_time"), u64);
    let short_break_time = value_t_or_exit!(matches.value_of("short_break_time"), u64);
    let long_break_time = value_t_or_exit!(matches.value_of("long_break_time"), u64);

    let config = TomatoConfig::new(work_time, short_break_time, long_break_time);
    let mut session = Session::new(config);
    session.run();
}
