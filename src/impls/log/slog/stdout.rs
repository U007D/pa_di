use slog::{Drain, Never, SendSyncRefUnwindSafeDrain, SendSyncUnwindSafeDrain};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};

pub fn stdout(
) -> impl Drain + Drain<Ok = ()> + Drain<Err = Never> + SendSyncUnwindSafeDrain + SendSyncRefUnwindSafeDrain {
    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();
    Async::new(drain).build().fuse()
}
