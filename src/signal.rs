use tokio_util::sync::CancellationToken;
// https://stackoverflow.com/questions/77585473/rust-tokio-how-to-handle-more-signals-than-just-sigint-i-e-sigquit
/// Waits for a signal that requests a graceful shutdown, like SIGTERM or SIGINT.
#[cfg(unix)]
async fn wait_for_signal_impl(token: CancellationToken) {
    use tokio::signal::unix::{signal, SignalKind};

    // Infos here:
    // https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
    let mut signal_terminate = signal(SignalKind::terminate()).unwrap();
    let mut signal_interrupt = signal(SignalKind::interrupt()).unwrap();

    tokio::select! {
        _ = signal_terminate.recv() => {
            log::debug!("Received SIGTERM.");
            token.cancel()
        },
        _ = signal_interrupt.recv() => {
            log::debug!("Received SIGINT.");
            token.cancel()
        },
    };
}

/// Waits for a signal that requests a graceful shutdown, Ctrl-C (SIGINT).
#[cfg(windows)]
async fn wait_for_signal_impl(token: CancellationToken) {
    use tokio::signal::windows;

    // Infos here:
    // https://learn.microsoft.com/en-us/windows/console/handlerroutine
    let mut signal_c = windows::ctrl_c().unwrap();
    let mut signal_break = windows::ctrl_break().unwrap();
    let mut signal_close = windows::ctrl_close().unwrap();
    let mut signal_shutdown = windows::ctrl_shutdown().unwrap();

    tokio::select! {
        _ = signal_c.recv() => {
            log::debug!("Received CTRL_C.");
            token.cancel()
        },
        _ = signal_break.recv() => {
            log::debug!("Received CTRL_BREAK.");
            token.cancel()
        },
        _ = signal_close.recv() => {
            log::debug!("Received CTRL_CLOSE.");
            token.cancel()
        } ,
        _ = signal_shutdown.recv() => {
            log::debug!("Received CTRL_SHUTDOWN.");
            token.cancel()
        },
    };
}

/// Registers signal handlers and waits for a signal that
/// indicates a shutdown request.
pub(crate) async fn wait_for_signal(token: CancellationToken) {
    wait_for_signal_impl(token).await
}
