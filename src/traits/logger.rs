pub trait Logger: Clone {
    fn trc<Msg: Into<String>>(&self, msg: Msg);
    fn dbg<Msg: Into<String>>(&self, msg: Msg);
    fn nfo<Msg: Into<String>>(&self, msg: Msg);
    fn wrn<Msg: Into<String>>(&self, msg: Msg);
    fn err<Msg: Into<String>>(&self, msg: Msg);
}

#[allow(clippy::useless_attribute, clippy::redundant_pub_crate)]
pub(crate) trait Constructor<OutputDevice>: Logger {
    fn new(output_device: OutputDevice) -> Result<Self, crate::error::initialization::Error>;
}
