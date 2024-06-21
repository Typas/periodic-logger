use tokio::time;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct RoutineBuilder {
    start_time: time::Instant,
    cancel_token: CancellationToken,
}

impl RoutineBuilder {
    pub fn new(st: time::Instant, token: CancellationToken) -> Self {
        Self {
            start_time: st,
            cancel_token: token,
        }
    }

    pub(crate) async fn info(self, mut interval: time::Interval) {
        loop {
            if self.cancel_token.is_cancelled() {
                break;
            }
            let t = interval.tick().await;
            log::info!(
                "information at {:.1} sec",
                (t - self.start_time).as_secs_f32()
            );
        }
    }

    pub(crate) async fn warn(self, mut interval: time::Interval) {
        loop {
            if self.cancel_token.is_cancelled() {
                break;
            }
            let t = interval.tick().await;
            log::warn!("warning at {:.1} sec", (t - self.start_time).as_secs_f32());
        }
    }

    pub(crate) async fn debug(self, mut interval: time::Interval) {
        loop {
            if self.cancel_token.is_cancelled() {
                break;
            }
            let t = interval.tick().await;
            log::debug!(
                "debug message at {:.1} sec",
                (t - self.start_time).as_secs_f32()
            );
        }
    }
}
