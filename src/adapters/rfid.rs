use std::io::Result;
use log::trace;

pub trait RfidController {
    fn poll_for_tag(&self) -> Result<Option<String>>;
}

pub struct RC522RfidController<'a> {
    pub dev: &'a str,
}

impl RfidController for RC522RfidController<'_> {
    fn poll_for_tag(&self) -> Result<Option<String>> {
        trace!("polled rc522 for RFID tag, found none");
        Ok(None)
    }
}