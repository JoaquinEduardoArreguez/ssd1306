use embedded_hal::blocking::i2c::Write;

const MAX_BUFFER_SIZE: usize = 128;

#[derive(Debug, Clone, Copy)]
pub enum Address {
    DEFAULT = 0x3C,
    ALTERNATIVE = 0x3D,
}

#[derive(Debug, Clone, Copy)]
enum ControlByte {
    COMMAND = 0x00,
    DATA = 0x40,
}

#[derive(Debug)]
pub enum SSD1306Error<E> {
    BufferTooSmall,
    I2CError(E),
}

pub struct SSD1306<I2C> {
    i2c: I2C,
    address: Address,
}

impl<I2C, E> SSD1306<I2C>
where
    I2C: Write<Error = E>,
{
    pub fn new(i2c: I2C, address: Address) -> Self {
        SSD1306 { i2c, address }
    }

    pub fn send_commands(&mut self, commands: &[u8]) -> Result<(), SSD1306Error<E>> {
        if commands.len() > MAX_BUFFER_SIZE {
            return Err(SSD1306Error::BufferTooSmall);
        }

        let mut buffer = [0u8; MAX_BUFFER_SIZE + 1];
        buffer[0] = ControlByte::COMMAND as u8;

        let mut index = 1;
        for &command in commands {
            buffer[index] = command;
            index += 1;
        }

        self.i2c
            .write(self.address as u8, &buffer[..index])
            .map_err(SSD1306Error::I2CError)
    }

    pub fn send_data(&mut self, data: &[u8]) -> Result<(), SSD1306Error<E>> {
        if data.len() > MAX_BUFFER_SIZE {
            return Err(SSD1306Error::BufferTooSmall);
        }

        let mut buffer = [0u8; MAX_BUFFER_SIZE + 1];
        buffer[0] = ControlByte::DATA as u8;

        let mut index = 1;
        for &data_byte in data {
            buffer[index] = data_byte;
            index += 1;
        }

        self.i2c
            .write(self.address as u8, &buffer[..index])
            .map_err(SSD1306Error::I2CError)
    }
}
