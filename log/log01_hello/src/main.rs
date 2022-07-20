use log::Level::Debug;
use log::{trace, info, warn, log_enabled, debug};

fn main() {
    env_logger::init();
    trace!("commencing yak shaving");
    info!("info");
    warn!("warn");
    if log_enabled!(target: "Global", Debug) {
        debug!(target: "Global", "expensive debug");
    }
}
