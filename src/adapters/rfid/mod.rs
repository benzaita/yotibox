use log::{debug, trace};
use spidev;
use std::io;

mod rfid_rs;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    RC522Error(rfid_rs::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<rfid_rs::Error> for Error {
    fn from(error: rfid_rs::Error) -> Self {
        Error::RC522Error(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait RfidController {
    fn init(&mut self) -> Result<()>;
    fn cleanup(&mut self) -> Result<()>;
    fn poll_for_tag(&mut self) -> Result<Option<String>>;
}

pub struct RC522RfidController {
    mfrc522: rfid_rs::MFRC522,
}

impl RC522RfidController {
    pub fn new(dev: &str) -> RC522RfidController {
        let spi_dev = create_spi(dev).unwrap();

        RC522RfidController {
            mfrc522: rfid_rs::MFRC522 { spi: spi_dev },
        }
    }
}

impl RfidController for RC522RfidController {
    fn init(&mut self) -> Result<()> {
        debug!("Initializing RC522");
        self.mfrc522.init()?;
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn poll_for_tag(&mut self) -> Result<Option<String>> {
        trace!("new_card_present()");
        let new_card = self.mfrc522.new_card_present();

        if let Err(error) = new_card {
            match error {
                rfid_rs::Error::Timeout => {
                    debug!("No card present");
                    return Ok(None);
                }
                _ => {
                    trace!("new_card_present() == Err({:?})", error);
                    return Err(Error::from(error));
                }
            }
        }

        trace!("read_card_serial()");
        let uid = self.mfrc522.read_card_serial()?;
        trace!("read_card_serial() == {:?}", uid);

        let mut block = 4;
        let len = 18;
        let key: rfid_rs::MifareKey = [0xffu8; 6];
        let command = rfid_rs::picc::Command::MfAuthKeyA;

        trace!(
            "authenticate(command={}, block={}, key={}, uid={})",
            "(command)",
            block,
            "(key)",
            "(uid)"
        );
        self.mfrc522.authenticate(command, block, key, &uid)?;
        trace!("authenticate(...) == Ok");

        trace!("mifare_read(block={}, len={})", block, len);
        let response = self.mfrc522.mifare_read(block, len)?;
        trace!("mifare_read(...) == {:?}", response.data);

        block = 1;

        trace!(
            "authenticate(command={}, block={}, key={}, uid={})",
            "(command)",
            block,
            "(key)",
            "(uid)"
        );
        self.mfrc522.authenticate(command, block, key, &uid)?;
        trace!("authenticate(...) == Ok");

        trace!("mifare_read(block={}, len={})", block, len);
        let response = self.mfrc522.mifare_read(block, len)?;
        trace!("mifare_read(...) == {:?}", response.data);

        self.mfrc522.halt_a()?;
        self.mfrc522.stop_crypto1()?;

        trace!("All seems ok. Panicing");
        unimplemented!();
    }
}

fn create_spi(dev: &str) -> Result<spidev::Spidev> {
    let mut spi = spidev::Spidev::open(dev)?;
    let options = spidev::SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(20_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}
