// Base taken from https://github.com/bschwind/key-ripper/blob/main/firmware/src/main.rs

#![no_main]
#![no_std]



use core::convert::Infallible;
use cortex_m::delay::Delay;
use defmt::{error, info};
use defmt_rtt as _;
use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    timer::CountDown,
};
use embedded_time::duration::Extensions;
// use panic_reset as _;
use panic_probe as _;
use rp2040_hal::{pac, usb::UsbBus, Clock, Watchdog};
use usb_device::{bus::UsbBusAllocator, device::UsbDeviceBuilder, prelude::UsbVidPid, UsbError};
use usbd_hid::{
    descriptor::KeyboardReport,
    hid_class::{
        HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig,
    },
};

// lcd traits
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use st7735_lcd;
use st7735_lcd::Orientation;
use embedded_time::rate::Hertz;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

mod hid_descriptor;
mod key_codes;
mod key_mapping;

const NUM_COLS: usize = 14;
const NUM_ROWS: usize = 6;

const EXTERNAL_CRYSTAL_FREQUENCY_HZ: u32 = 12_000_000;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Start of main()");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = rp2040_hal::clocks::init_clocks_and_plls(
        EXTERNAL_CRYSTAL_FREQUENCY_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Setup USB
    let force_vbus_detect_bit = true;
    let usb_bus = UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        force_vbus_detect_bit,
        &mut pac.RESETS,
    );

    let bus_allocator = UsbBusAllocator::new(usb_bus);

    // Note - Going lower than this requires switch debouncing.
    let poll_ms = 8;
    let mut hid_endpoint = HIDClass::new_with_settings(
        &bus_allocator,
        hid_descriptor::KEYBOARD_REPORT_DESCRIPTOR,
        poll_ms,
        HidClassSettings {
            subclass: HidSubClass::NoSubClass,
            protocol: HidProtocol::Keyboard,
            config: ProtocolModeConfig::ForceReport,
            // locale: HidCountryCode::NotSupported,
            locale: HidCountryCode::US,
        },
    );

    info!("USB initialized");

    // https://github.com/obdev/v-usb/blob/7a28fdc685952412dad2b8842429127bc1cf9fa7/usbdrv/USB-IDs-for-free.txt#L128
    let mut keyboard_usb_device = UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27db))
        .manufacturer("bschwind")
        .product("key ripper")
        .build();

    // Get the GPIO peripherals.
    let sio = rp2040_hal::Sio::new(pac.SIO);

    let pins =
        rp2040_hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    // Set up keyboard matrix pins.
    let rows: &[&dyn InputPin<Error = Infallible>] = &[
        &pins.gpio5.into_pull_down_input(),
        &pins.gpio6.into_pull_down_input(),
        &pins.gpio7.into_pull_down_input(),
        &pins.gpio8.into_pull_down_input(),
    ];

    let cols: &mut [&mut dyn OutputPin<Error = Infallible>] = &mut [
        &mut pins.gpio27.into_push_pull_output(),
        &mut pins.gpio26.into_push_pull_output(),
        &mut pins.gpio22.into_push_pull_output(),
        &mut pins.gpio21.into_push_pull_output(),
        &mut pins.gpio20.into_push_pull_output(),
        &mut pins.gpio4.into_push_pull_output(),
        &mut pins.gpio3.into_push_pull_output(),
        &mut pins.gpio2.into_push_pull_output(),
        &mut pins.gpio1.into_push_pull_output(),
        &mut pins.gpio0.into_push_pull_output(),
    ];

    // Timer-based resources.
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().0);

    let timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut scan_countdown = timer.count_down();

    // Start on a 500ms countdown so the USB endpoint writes don't block.
    scan_countdown.start(500.milliseconds());

    //// Start the TFT screen

    // These are implicitly used by the spi driver if they are in the correct mode
    let _spi_sclk = pins.gpio18.into_mode::<rp2040_hal::gpio::FunctionSpi>();
    let _spi_mosi = pins.gpio19.into_mode::<rp2040_hal::gpio::FunctionSpi>();
    //let _spi_miso = pins.gpio4.into_mode::<rp2040_hal::gpio::FunctionSpi>();
    let spi = rp2040_hal::Spi::<_, _, 8>::new(pac.SPI0);

    let mut lcd_led = pins.gpio15.into_push_pull_output();
    let dc = pins.gpio16.into_push_pull_output();
    let rst = pins.gpio14.into_push_pull_output();

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        Hertz::new(16_000_000u32),
        &embedded_hal::spi::MODE_0,
    );


    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 128, 160);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::PortraitSwapped).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();
    disp.set_offset(0, 25);

    let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86);

    let image: Image<_> = Image::new(&image_raw, Point::new(24, 28));

    image.draw(&mut disp).unwrap();
    
    // Wait until the background and image have been rendered otherwise
    // the screen will show random pixels for a brief moment
    lcd_led.set_high().unwrap();

    info!("Start main loop");

    // Main keyboard polling loop.
    loop {
        keyboard_usb_device.poll(&mut [&mut hid_endpoint]);

        if scan_countdown.wait().is_ok() {
            // Scan the keys and send a report.
            let matrix = scan_keys(rows, cols, &mut delay);
            let report = report_from_matrix(&matrix);

            match hid_endpoint.push_input(&report) {
                Ok(_) => {
                    scan_countdown.start(8.milliseconds());
                },
                Err(err) => match err {
                    UsbError::WouldBlock => info!("UsbError::WouldBlock"),
                    UsbError::ParseError => error!("UsbError::ParseError"),
                    UsbError::BufferOverflow => error!("UsbError::BufferOverflow"),
                    UsbError::EndpointOverflow => error!("UsbError::EndpointOverflow"),
                    UsbError::EndpointMemoryOverflow => error!("UsbError::EndpointMemoryOverflow"),
                    UsbError::InvalidEndpoint => error!("UsbError::InvalidEndpoint"),
                    UsbError::Unsupported => error!("UsbError::Unsupported"),
                    UsbError::InvalidState => error!("UsbError::InvalidState"),
                },
            }
        }

        hid_endpoint.pull_raw_output(&mut [0; 64]).ok();
    }
}

fn scan_keys(
    rows: &[&dyn InputPin<Error = Infallible>],
    columns: &mut [&mut dyn embedded_hal::digital::v2::OutputPin<Error = Infallible>],
    delay: &mut Delay,
) -> [[bool; NUM_ROWS]; NUM_COLS] {
    let mut matrix = [[false; NUM_ROWS]; NUM_COLS];

    for (gpio_col, matrix_col) in columns.iter_mut().zip(matrix.iter_mut()) {
        gpio_col.set_high().unwrap();
        delay.delay_us(10);

        for (gpio_row, matrix_row) in rows.iter().zip(matrix_col.iter_mut()) {
            *matrix_row = gpio_row.is_high().unwrap();
        }

        gpio_col.set_low().unwrap();
        delay.delay_us(10);
    }

    matrix
}

fn report_from_matrix(matrix: &[[bool; NUM_ROWS]; NUM_COLS]) -> KeyboardReport {
    let mut keycodes = [0u8; 6];
    let mut keycode_index = 0;
    let mut modifier = 0;

    let mut push_keycode = |key| {
        if keycode_index < keycodes.len() {
            keycodes[keycode_index] = key;
            keycode_index += 1;
        }
    };

    // let layer_mapping = if matrix[0][5] {
    //     key_mapping::FN_LAYER_MAPPING
    // } else {
    //     key_mapping::NORMAL_LAYER_MAPPING
    // };
    layer_mapping = key_mapping::NORMAL_LAYER_MAPPING;

    for (matrix_column, mapping_column) in matrix.iter().zip(layer_mapping) {
        for (key_pressed, mapping_row) in matrix_column.iter().zip(mapping_column) {
            if *key_pressed {
                if let Some(bitmask) = mapping_row.modifier_bitmask() {
                    modifier |= bitmask;
                } else {
                    push_keycode(mapping_row as u8);
                }
            }
        }
    }

    KeyboardReport { modifier, reserved: 0, leds: 0, keycodes }
}