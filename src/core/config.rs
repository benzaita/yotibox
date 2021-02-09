pub trait Config {
    fn rfid_spi_dev(&self) -> Option<&str>;
    fn rfid_poll_ms(&self) -> Option<u64>;
}
