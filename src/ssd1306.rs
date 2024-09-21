use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::{
    MemoryAddressingMode, CHARGE_PUMP_ENABLE, COM_PINS_HW_CONFIG_ALT_DISABLE_REMAP,
    DISPLAY_CLOCK_DIVIDE_RATIO_AND_OSCILLATOR_FREQUENCY_NORMAL, DISPLAY_OFFSET_MIN,
    DISPLAY_START_LINE_0, MULTIPLEX_RATIO_MAX, SET_CHARGE_PUMP,
    SET_COM_OUTPUT_SCAN_DIRECTION_NORMAL, SET_COM_PINS_HW_CONFIG, SET_CONTRAST,
    SET_DISPLAY_CLOCK_DIVIDE_RATIO_AND_OSCILLATOR_FREQUENCY, SET_DISPLAY_OFF, SET_DISPLAY_OFFSET,
    SET_DISPLAY_ON, SET_ENTIRE_DISPLAY_ON, SET_MEMORY_ADDRESSING_MODE, SET_MULTIPLEX_RATIO,
    SET_NORMAL_DISPLAY, SET_SEGMENT_REMAP_0_TO_SEG0,
};
use crate::{CONTRAST_MID, SET_ENTIRE_DISPLAY_ON_IGNORE_RAM};

const MAX_BUFFER_SIZE: usize = 128;
const DATASHEET_INITIALIZATION_COMMANDS: [u8; 19] = [
    SET_DISPLAY_OFF,
    SET_MULTIPLEX_RATIO,
    MULTIPLEX_RATIO_MAX,
    SET_DISPLAY_OFFSET,
    DISPLAY_OFFSET_MIN,
    DISPLAY_START_LINE_0,
    SET_SEGMENT_REMAP_0_TO_SEG0,
    SET_COM_OUTPUT_SCAN_DIRECTION_NORMAL,
    SET_COM_PINS_HW_CONFIG,
    COM_PINS_HW_CONFIG_ALT_DISABLE_REMAP,
    SET_CONTRAST,
    CONTRAST_MID,
    SET_ENTIRE_DISPLAY_ON,
    SET_NORMAL_DISPLAY,
    SET_DISPLAY_CLOCK_DIVIDE_RATIO_AND_OSCILLATOR_FREQUENCY,
    DISPLAY_CLOCK_DIVIDE_RATIO_AND_OSCILLATOR_FREQUENCY_NORMAL,
    SET_CHARGE_PUMP,
    CHARGE_PUMP_ENABLE,
    SET_DISPLAY_ON,
];

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
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C, address: Address) -> Self {
        SSD1306 { i2c, address }
    }

    pub fn init(&mut self) -> Result<(), SSD1306Error<E>> {
        self.send_commands(&DATASHEET_INITIALIZATION_COMMANDS)
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

    pub fn set_memory_addressing_page_mode(
        &mut self,
        start_page: u8,
        start_column: u8,
    ) -> Result<(), SSD1306Error<E>> {
        assert!(start_page < 8, "Page number must be between 0 and 7");
        assert!(
            start_column < 128,
            "Column number must be between 0 and 127"
        );

        const SET_PAGE_START_ADDRESS_BASE: u8 = 0b1011_0000;
        let start_page_address = SET_PAGE_START_ADDRESS_BASE | start_page;

        let column_lower_nibble = start_column & 0x0F;
        let column_higher_nibble = (start_column >> 4) & 0x0F;

        self.send_commands(&[
            SET_MEMORY_ADDRESSING_MODE,
            MemoryAddressingMode::PAGE as u8,
            start_page_address,
            column_lower_nibble,
            column_higher_nibble,
        ])
    }

    pub fn set_page(&mut self, page: u8) -> Result<(), SSD1306Error<E>> {
        assert!(page < 8, "Page number must be between 0 and 7");

        const SET_PAGE_START_ADDRESS_BASE: u8 = 0b1011_0000;
        let page_address = SET_PAGE_START_ADDRESS_BASE | page;

        self.send_commands(&[page_address])
    }

    pub fn set_column(&mut self, column: u8) -> Result<(), SSD1306Error<E>> {
        assert!(column < 128, "Column number must be between 0 and 127");

        let column_lower_nibble = column & 0x0F;
        let column_higher_nibble = (column >> 4) & 0x0F;

        const SET_LOWER_COLUMN_COMMAND: u8 = 0x00;
        const SET_HIGHER_COLUMN_COMMAND: u8 = 0x10;

        self.send_commands(&[
            SET_LOWER_COLUMN_COMMAND | column_lower_nibble,
            SET_HIGHER_COLUMN_COMMAND | column_higher_nibble,
        ])
    }

    pub fn clear(&mut self) -> Result<(), SSD1306Error<E>> {
        let buffer = [0u8; MAX_BUFFER_SIZE];

        for page in 0..8 {
            self.set_page(page)?;
            self.set_column(0)?;
            self.send_data(&buffer)?;
        }

        self.set_page(0)?;
        self.set_column(0)?;

        Ok(())
    }

    pub fn display_on(&mut self) -> Result<(), SSD1306Error<E>> {
        self.send_commands(&[SET_ENTIRE_DISPLAY_ON_IGNORE_RAM])
    }

    pub fn display_on_ignore_ram(&mut self) -> Result<(), SSD1306Error<E>> {
        self.send_commands(&[SET_ENTIRE_DISPLAY_ON])
    }
}

/* Init sequence from datasheet
Set MUX Ratio -> A8h, 3Fh
Set Display Offset -> D3h, 00h
Set Display Start Line -> 40h
Set Segment re-map -> A0h/A1h
Set COM Output Scan Direction -> C0h/C8h
Set COM Pins hardware configuration -> DAh, 02
Set Contrast Control -> 81h, 7Fh
Disable Entire Display On -> A4h
Set Normal Display -> A6h
Set Osc Frequency -> D5h, 80h
Enable charge pump regulator -> 8Dh, 14h
Display On -> AFh
*/
