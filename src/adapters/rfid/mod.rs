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
    fn read_id_from_idle_picc(&mut self) -> Result<Option<String>>;
    fn read_if_from_halted_picc(&mut self) -> Result<Option<String>>;
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
        debug!("Running self test of RC522");
        self.mfrc522.self_test()?;

        debug!("Initializing RC522");
        self.mfrc522.init()?;
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn read_id_from_idle_picc(&mut self) -> Result<Option<String>> {
        trace!("new_card_present()");
        let new_card = self.mfrc522.new_card_present();

        if let Err(error) = new_card {
            match error {
                rfid_rs::Error::Timeout(_) => {
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

        trace!("halt_a()");
        self.mfrc522.halt_a()?;

        let uid_hex_str = format!("{:x}", uid);
        Ok(Some(uid_hex_str))
    }

    fn read_if_from_halted_picc(&mut self) -> Result<Option<String>> {
        trace!("wakeup_a()");
        let res = self.mfrc522.wakeup_a(2);
        if let Err(error) = res {
            match error {
                rfid_rs::Error::Timeout(_) => {
                    debug!("No card present");
                    return Ok(None);
                }
                _ => {
                    trace!("wakeup_a() == Err({:?})", error);
                    return Err(Error::from(error));
                }
            }
        }

        trace!("read_card_serial()");
        let uid = self.mfrc522.read_card_serial()?;
        trace!("read_card_serial() == {:?}", uid);

        trace!("halt_a()");
        self.mfrc522.halt_a()?;

        let uid_hex_str = format!("{:x}", uid);
        Ok(Some(uid_hex_str))
    }
}

impl std::fmt::LowerHex for rfid_rs::Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for b in &self.bytes {
            write!(f, "{:02x}", b)?;
        }

        Ok(())
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
