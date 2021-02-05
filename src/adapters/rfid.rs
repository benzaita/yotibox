use log::{trace};
use std::io::Result;
use std::io::{Error, ErrorKind};

const MODE_IDLE: u8 = 0x00;
const MODE_TRANSREC: u8 = 0x0C;
const MODE_RESET: u8 = 0x0F;
const MODE_AUTH: u8 = 0x0E;

const REG_TX_CONTROL: u8 = 0x14;

const LENGTH: u8 = 16; // length of what?

const RPI_PIN_RST: u8 = 22;
const RPI_PIN_IRQ: u8 = 18;

pub trait RfidController {
    fn init(&mut self) -> Result<()>;
    fn cleanup(&mut self) -> Result<()>;
    fn poll_for_tag(&mut self) -> Result<Option<String>>;
}

pub struct RC522RfidController<'a> {
    dev: &'a str,
    authed: bool,
    pin_ce: u8,
    antenna_gain: u8,
}

impl RC522RfidController<'_> {
    pub fn new(dev: &str) -> RC522RfidController {
        RC522RfidController {
            dev,
            authed: false,
            pin_ce: 0,
            antenna_gain: 0x04,
        }
    }

    fn gpio_output(&self, pin: u8, enable: bool) {
        // is the first pin 0 or 1?
        trace!("Setting GPIO pin {} to {}", pin, if enable {1} else {0});
        use rppal::gpio::Gpio;

        let gpio = Gpio::new().unwrap();
        let mut pin = gpio.get(23).unwrap().into_output();
        
        pin.set_high();
        
        unimplemented!();
    }

    fn set_bitmask(&self, address: u8, mask: u8) {
        let current = self.dev_read(address);
        self.dev_write(address, current | mask);
    }

    fn clear_bitmask(&self, address: u8, mask: u8) {
        let current = self.dev_read(address);
        self.dev_write(address, current & (!mask));
    }

    fn set_antenna(&self, state: bool) {
        if state {
            let current = self.dev_read(REG_TX_CONTROL);
            if !(current & 0x03) != 0 {
                self.set_bitmask(REG_TX_CONTROL, 0x03);
            }
        } else {
            self.clear_bitmask(REG_TX_CONTROL, 0x03);
        }
    }

    fn reset(&mut self) {
        self.authed = false;
        self.dev_write(0x01, MODE_RESET);
    }

    fn init2(&mut self) {
        self.reset();
        self.dev_write(0x2A, 0x8D);
        self.dev_write(0x2B, 0x3E);
        self.dev_write(0x2D, 30);
        self.dev_write(0x2C, 0);
        self.dev_write(0x15, 0x40);
        self.dev_write(0x11, 0x3D);
        self.dev_write(0x26, self.antenna_gain << 4);
        self.set_antenna(true);
    }

    fn dev_write(&self, address: u8, value: u8) {
        let data = vec![(address << 1) & 0x7E, value];

        if self.pin_ce != 0 {
            self.gpio_output(self.pin_ce, false);
        }
        // todo self.spi.xfer2(data);
        if self.pin_ce != 0 {
            self.gpio_output(self.pin_ce, true);
        }
    }

    fn dev_read(&self, address: u8) -> u8 {
        let data = vec![((address << 1) & 0x7E) | 0x80, 0];

        if self.pin_ce != 0 {
            self.gpio_output(self.pin_ce, false);
        }
        // todo let value = self.spi.xfer2(data);
        if self.pin_ce != 0 {
            self.gpio_output(self.pin_ce, true);
        }

        // todo value[1]
        0
    }

    fn card_write(&self, command: u8, data: Vec<u8>) -> Result<(Vec<u8>, u32)> {
        let mut back_data = Vec::<u8>::new();
        let mut back_length: u32 = 0;
        let mut error: Option<Error> = None;
        let mut irq: u8 = 0x00;
        let mut irq_wait: u8 = 0x00;
        let mut last_bits: u8 = !0;
        let mut n = 0;

        match command {
            MODE_AUTH => {
                irq = 0x12;
                irq_wait = 0x10;
            }
            MODE_TRANSREC => {
                irq = 0x77;
                irq_wait = 0x30;
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid command {:x}", command),
                ))
            }
        }

        self.dev_write(0x02, irq | 0x80);
        self.clear_bitmask(0x04, 0x80);
        self.set_bitmask(0x0A, 0x80);
        self.dev_write(0x01, MODE_IDLE);

        for value in data {
            self.dev_write(0x09, value);
        }

        self.dev_write(0x01, command);

        if command == MODE_TRANSREC {
            self.set_bitmask(0x0D, 0x80)
        }

        let mut i = 2000;
        trace!("waiting for value in address 0x04");
        loop {
            n = self.dev_read(0x04);
            i -= 1;
            if !((i != 0) && !(n & 0x01) != 0 && !(n & irq_wait) != 0) {
                break;
            }
        }

        self.clear_bitmask(0x0D, 0x80);

        if i != 0 {
            if (self.dev_read(0x06) & 0x1B) == 0x00 {
                error = None;

                if n & irq & 0x01 != 0 {
                    error = Some(Error::new(ErrorKind::Other, "E1"));
                }

                if command == MODE_TRANSREC {
                    n = self.dev_read(0x0A);
                    last_bits = self.dev_read(0x0C) & 0x07;
                    if last_bits != 0 {
                        back_length = ((n as u32) - 1) * 8 + (last_bits as u32);
                    } else {
                        back_length = (n as u32) * 8;
                    }

                    if n == 0 {
                        n = 1;
                    }

                    if n > LENGTH {
                        n = LENGTH;
                    }

                    for _i in 0..n {
                        back_data.push(self.dev_read(0x09));
                    }
                }
            } else {
                error = Some(Error::new(ErrorKind::Other, "E1"));
            }
        }

        match error {
            None => Ok((back_data, back_length)),
            Some(err) => Err(err),
        }
    }

    fn request(&self) -> Option<u32> {
        // """
        // Requests for tag.
        // Returns (False, None) if no tag is present, otherwise returns (True, tag type)
        // """
        let req_mode = 0x26;
        let error = true;
        let back_bits = 0;

        self.dev_write(0x0D, 0x07);
        let res = self.card_write(MODE_TRANSREC, vec![req_mode]);
        match res {
            Err(_) => None,
            Ok((back_data, back_bits)) => match back_bits {
                0x10 => Some(back_bits),
                _ => None,
            },
        }
    }
}

impl RfidController for RC522RfidController<'_> {
    fn init(&mut self) -> std::io::Result<()> {
        self.gpio_output(1, true);

        unimplemented!();

        // todo init spi

        // todo init gpio

        // enable IRQ on detect
        self.init2();
        // todo self.irq.clear();
        self.dev_write(0x04, 0x00);
        self.dev_write(0x02, 0xA0);

        Ok(())
    }

    fn cleanup(&mut self) -> std::io::Result<()> {
        self.init2();
        if self.authed {
            // todo self.stop_crypto();
        }
        // todo GPIO.cleanup();
        Ok(())
    }

    fn poll_for_tag(&mut self) -> Result<Option<String>> {
        self.init2();
        self.dev_write(0x04, 0x00);
        self.dev_write(0x02, 0xA0);

        self.dev_write(0x09, 0x26);
        self.dev_write(0x01, 0x0C);
        self.dev_write(0x0D, 0x87);
        // todo self.irq.wait(0.1);

        self.request();
        // todo self.anticoll();

        unimplemented!();
    }
}
