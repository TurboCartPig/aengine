use std::ops::Deref;

use slog;
use slog::Drain;
use slog_async;
use slog_term;

pub struct Logger {
    log: slog::Logger,
}

impl Logger {
    pub fn new() -> Self {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        let log = slog::Logger::root(drain, o!());

        Self {
            log,
        }
    }
}

impl Deref for Logger {
    type Target = slog::Logger;

    fn deref(&self) -> &Self::Target {
        &self.log
    }
}