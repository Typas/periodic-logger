pub mod routine;
pub mod signal;

use std::{io::Write, time::Duration};
use tokio::time;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}: {}[1]",
                level_display(record.level()),
                record.target()
            )?;
            writeln!(buf, "      {}", record.args())
        })
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .init();

    let start_time = time::Instant::now();
    let cancel_token = CancellationToken::new();
    let tracker = TaskTracker::new();
    let routine = routine::RoutineBuilder::new(start_time.clone(), cancel_token.clone());

    let r_info = routine.clone().info(time::interval(Duration::from_secs(3)));
    let r_warn = routine.clone().warn(time::interval(Duration::from_secs(5)));
    let r_debug = routine.clone().debug(time::interval(Duration::from_secs(2)));

    tracker.spawn(r_info);
    tracker.spawn(r_warn);
    tracker.spawn(r_debug);
    tracker.spawn(signal::wait_for_signal(cancel_token.clone()));

    tracker.close();
    tracker.wait().await;

    log::info!("successfully end with message");
}

fn level_display(lv: log::Level) -> &'static str {
    match lv {
        log::Level::Trace => "trac",
        log::Level::Debug => "debg",
        log::Level::Info => "info",
        log::Level::Warn => "warn",
        log::Level::Error => "eror",
    }
}
