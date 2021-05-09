use crate::{
    error::initialization::Result,
    traits::logger::{Constructor, Logger},
};
use slog::{
    debug, error, info, o as ctx, trace, warn, Drain, Never, SendSyncRefUnwindSafeDrain, SendSyncUnwindSafeDrain,
};

#[derive(Clone, Debug)]
pub struct Log(slog::Logger);

impl<OutputDevice: 'static> Constructor<OutputDevice> for Log
where
    OutputDevice: Drain + Drain<Ok = ()> + Drain<Err = Never> + SendSyncUnwindSafeDrain + SendSyncRefUnwindSafeDrain,
{
    fn new(output_device: OutputDevice) -> Result<Self> {
        Ok(Self(slog::Logger::root(
            output_device,
            ctx!("version" => option_env!("CARGO_PKG_VERSION")
                .map_or_else(|| "<not available>", |ver| ver)),
        )))
    }
}

impl Logger for Log {
    fn trc<Msg: Into<String>>(&self, msg: Msg) {
        trace!(self.0, "{}", msg.into());
    }

    fn dbg<Msg: Into<String>>(&self, msg: Msg) {
        debug!(self.0, "{}", msg.into());
    }

    fn nfo<Msg: Into<String>>(&self, msg: Msg) {
        info!(self.0, "{}", msg.into());
    }

    fn wrn<Msg: Into<String>>(&self, msg: Msg) {
        warn!(self.0, "{}", msg.into());
    }

    fn err<Msg: Into<String>>(&self, msg: Msg) {
        error!(self.0, "{}", msg.into());
    }
}
