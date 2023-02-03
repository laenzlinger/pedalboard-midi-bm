//! Midi message converter
//!
#![no_std]
#![no_main]

pub mod pedalboard;

use adafruit_feather_rp2040::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_midi::{MidiIn, MidiOut};
use fugit::HertzU32;
use nb::block;
use panic_probe as _;
use pedalboard::rc500::RC500;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use adafruit_feather_rp2040 as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<bsp::hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<bsp::hal::usb::UsbBus>> = None;

/// The USB Serial Device Driver (shared with the interrupt).
static mut USB_SERIAL: Option<SerialPort<bsp::hal::usb::UsbBus>> = None;

// The macro for marking our interrupt functions
use bsp::hal::pac::interrupt;

use bsp::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        sio::Sio,
        uart::{DataBits, StopBits, UartConfig, UartPeripheral},
        watchdog::Watchdog,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(bsp::hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Set up the USB Communications Class Device driver
    let serial = SerialPort::new(bus_ref);
    unsafe {
        USB_SERIAL = Some(serial);
    }

    // Create a USB device with a fake VID and PID
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x2E8A, 0x0005))
        .manufacturer("laenzlinger")
        .product("pedalboard-midi")
        .serial_number("0.0.1")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(bsp::hal::pac::Interrupt::USBCTRL_IRQ);
    }

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led_pin = pins.led.into_push_pull_output(); 
    led_pin.set_high().unwrap();

    let uart_pins = (
        pins.tx.into_mode::<bsp::hal::gpio::FunctionUart>(),
        pins.rx.into_mode::<bsp::hal::gpio::FunctionUart>(),
    );

    // set the MIDI baud rate
    let conf = UartConfig::new(
        HertzU32::from_raw(31250),
        DataBits::Eight,
        None,
        StopBits::One,
    );
    let uart = UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(conf, clocks.peripheral_clock.freq())
        .unwrap();

    // Configure Midi
    let (rx, tx) = uart.split();

    let mut midi_in = MidiIn::new(rx);
    let mut midi_out = MidiOut::new(tx);

    let mut rc = RC500::new();
    loop {
        if let Ok(event) = block!(midi_in.read()) {
            info!("received {}", event);

            let messages = pedalboard::handle(event, &mut rc);
            for m in messages.into_iter() {
                info!("send {}", m);
                midi_out.write(&m).ok();
            }
        }
    }
}

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    use core::sync::atomic::{AtomicBool, Ordering};

    /// Note whether we've already printed the "hello" message.
    static SAID_HELLO: AtomicBool = AtomicBool::new(false);

    // Grab the global objects. This is OK as we only access them under interrupt.
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let serial = USB_SERIAL.as_mut().unwrap();

    // Say hello exactly once on start-up
    if !SAID_HELLO.load(Ordering::Relaxed) {
        SAID_HELLO.store(true, Ordering::Relaxed);
        let _ = serial.write(b"pedalboard-midi\r\nz: reboot");
    }

    // Poll the USB driver with all of our supported USB Classes
    if usb_dev.poll(&mut [serial]) {
        let mut buf = [0u8; 64];
        match serial.read(&mut buf) {
            Err(_e) => {
                // Do nothing
            }
            Ok(0) => {
                // Do nothing
            }
            Ok(count) => {
                // Convert to upper case
                buf.iter().take(count).for_each(|b| {
                    if b == &b'z' {
                        let _ = serial.write(b"Reboot\r\n");
                        bsp::hal::rom_data::reset_to_usb_boot(0, 0)
                    }
                });
            }
        }
    }
}
