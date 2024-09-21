/* Fundamental Commands */

// Set entire display ON.
// A4 -> Resume to RAM content display, output follows RAM content
// A5 -> Entire display on, output ignores stored RAM
pub const SET_ENTIRE_DISPLAY_ON: u8 = 0xA4;
pub const SET_ENTIRE_DISPLAY_ON_IGNORE_RAM: u8 = 0xA5;

// Set normal display mode (0 in RAM = off pixel, 1 in RAM = on pixel)
pub const SET_NORMAL_DISPLAY: u8 = 0xA6;

// Set inverse display mode (1 in RAM = off pixel, 0 in RAM = on pixel)
pub const SET_INVERSE_DISPLAY: u8 = 0xA7;

// Set display off (sleep mode)
pub const SET_DISPLAY_OFF: u8 = 0xAE;

// Set display on in normal mode
pub const SET_DISPLAY_ON: u8 = 0xAF;

/* Hardware Configuration Commands (Panel resolution and layout related) */

// Set multiplex ratio to N+1 MUX - This value represents how many vertical rows are active at a time
// Datasheet specifies 15<=N<=63, but I'v tested successfully 1<=N<=63
// This command must be followed by a multiplex ratio value (N).
// Min multiplex ratio, according to datasheet, 15 (+1=16)
// Max multiplex ratio, 63 (+1=64)
pub const SET_MULTIPLEX_RATIO: u8 = 0xA8;
pub const MULTIPLEX_RATIO_MIN: u8 = 0x0F;
pub const MULTIPLEX_RATIO_MAX: u8 = 0x3F;

// Set display offset - Adjust vertical position, shifting display content up or down certain number of rows
// This command should be followed by the offset value 0-63.
// Min display offset, 0 rows
// Max display offset, 63 rows
pub const SET_DISPLAY_OFFSET: u8 = 0xD3;
pub const DISPLAY_OFFSET_MIN: u8 = 0x00;
pub const DISPLAY_OFFSET_MAX: u8 = 0x3F;

// Set display RAM display start line register from 0-63 (0x40 - 0x7F)
// Sets the starting line for the display data.
// This command is used to determine which line of the display will be the first to show the data sent to the display.
pub const DISPLAY_START_LINE_0: u8 = 0x40;

// Set charge pump
// Controls the internal charge pump circuitry, responsible for generating the necessary voltage levels for the display.
// The Charge Pump must be enabled by the following commands in order:
// [ 8Dh Charge Pump Setting , 14h Enable Charge Pump, AFh Display ON ]
pub const SET_CHARGE_PUMP: u8 = 0x8D;
pub const CHARGE_PUMP_ENABLE: u8 = 0x14;
pub const CHARGE_PUMP_DISABLE: u8 = 0x10;

// Set segment re-map.
// Column address 0 mapped to SEG0 / Column address 127 mapped to SEG0
pub const SET_SEGMENT_REMAP_0_TO_SEG0: u8 = 0xA0;
pub const SET_SEGMENT_REMAP_127_TO_SEG0: u8 = 0xA1;

// Set COM output scan direction.
// Normal mode (Scan from COM0 to COM[N–1]) / Remapped mode (Scan from COM[N-1] to COM0)
// where N is the multiplex ratio
pub const SET_COM_OUTPUT_SCAN_DIRECTION_NORMAL: u8 = 0xC0;
pub const SET_COM_OUTPUT_SCAN_DIRECTION_REMAPED: u8 = 0xC8;

// Set COM pins hardware configuration.
// This command must be followed by its value:
// Register: [0,0,A5,A4,0,0,1,0]
// A4=0 Sequential COM pin configuration | A4=1 Alternative COM pin configuration
// A5=0 Disable COM Left/Right remap | A5=1 Enable COM Left/Right remap
pub const SET_COM_PINS_HW_CONFIG: u8 = 0xDA;
pub const COM_PINS_HW_CONFIG_SEQ_DISABLE_REMAP: u8 = 0b00000010;
pub const COM_PINS_HW_CONFIG_ALT_DISABLE_REMAP: u8 = 0b00010010;
pub const COM_PINS_HW_CONFIG_SEQ_ENABLE_REMAP: u8 = 0b00100010;
pub const COM_PINS_HW_CONFIG_ALT_ENABLE_REMAP: u8 = 0b00110010;

// Set Contrast Control
// This command must be followed by the contrast value, contrast increses as value does, 0-255
pub const SET_CONTRAST: u8 = 0x81;
pub const CONTRAST_MAX: u8 = 0xFF;
pub const CONTRAST_MID: u8 = 0x7F;
pub const CONTRAST_MIN: u8 = 0x01;

/* Timing and Driving Scheme Settings Commands */

// Set Display Clock Divide Ratio/Oscillator Frequency
// This command must be followed by its value, given by:
// [ 0 | A7 | A6 | A5 | A4 | A3 | A2 | A1 | A0 ]
// A[3:0] Define the divide ratio (D) of the display clocks (DCLK): Divide ratio= A[3:0] + 1, RESET is 0000b (divide ratio = 1)
// A[7:4] : Set the Oscillator Frequency, FOSC. Oscillator Frequency increases with the value of A[7:4] and vice versa.
// RESET is 1000b
// Range:0000b~1111b
// Frequency increases as setting value increases
//
// The display clock (DCLK) for the Display Timing Generator is derived from CLK. The division factor “D”
// can be programmed from 1 to 16 by command D5h --> DCLK = FOSC / D
// FOSC is the oscillator frequency. It can be changed by command D5h A[7:4].
// The higher the register setting results in higher frequency.
// min: 333khz, typ: 370khz, max: 407khz
pub const SET_DISPLAY_CLOCK_DIVIDE_RATIO_AND_OSCILLATOR_FREQUENCY: u8 = 0xD5;
pub const DISPLAY_CLOCK_DIVIDE_RATIO_AND_OSCILLATOR_FREQUENCY_NORMAL: u8 = 0b10000000; // Divide ratio 1, min oscillator frequency
pub const DISPLAY_CLOCK_DIVIDE_RATIO_AND_OSCILLATOR_FREQUENCY_MAX: u8 = 0b11110000; // Divide ratio 1, max oscillator frequency

/* Addressing Setting Commands */

// Set memory addressing mode
// Supports horizontal, vertical and page adressing modes
pub const SET_MEMORY_ADDRESSING_MODE: u8 = 0x20;
pub enum MemoryAddressingMode {
    HORIZONTAL = 0b00000000,
    VERTICAL = 0b00000001,
    PAGE = 0b00000010,
}

// Page addressing mode setup sequence:
// -> Set the page start address of the target display location by command B0h to B7h
// -> Set the lower start column address of pointer by command 00h~0Fh
// -> Set the upper start column address of pointer by command 10h~1Fh
//
// This value is meant to be masked with the lower 3 bits
pub const SET_PAGE_START_ADDRESS: u8 = 0b10110000;
