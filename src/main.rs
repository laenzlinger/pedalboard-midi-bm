//! Midi message converter
//!
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_midi::{Channel, Control, MidiIn, MidiMessage, MidiOut, Program};
use fugit::HertzU32;
use nb::block;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let uart_pins = (
        // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
        pins.gpio0.into_mode::<bsp::hal::gpio::FunctionUart>(),
        // UART RX (characters received by RP2040) on pin 2 (GPIO1)
        pins.gpio1.into_mode::<bsp::hal::gpio::FunctionUart>(),
    );

    // set the MIDI baud rate
    let mut conf = bsp::hal::uart::common_configs::_9600_8_N_1;
    conf.baudrate = HertzU32::from_raw(31250);
    let uart = bsp::hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(conf, clocks.peripheral_clock.freq())
        .unwrap();

    // Configure Midi
    let (rx, tx) = uart.split();

    let mut midi_in = MidiIn::new(rx);
    let mut midi_out = MidiOut::new(tx);

    loop {
        if let Ok(event) = block!(midi_in.read()) {
            info!("received {}", event);
            match resolve(event) {
                Some(out) => {
                    info!("send {}", out);
                    midi_out.write(&out).ok()
                }
                None => None,
            };
        }
    }
}

const PLETHORA_CHANNEL: Channel = Channel::new(1);

const HIGH_VALUE: midi_types::Value7 = midi_types::Value7::new(127);

enum Plethora {
    Board(u8),
    BoardUp,
    BoardDown,
}
impl Plethora {
    fn midi_message(&self) -> MidiMessage {
        match *self {
            Plethora::BoardUp => {
                MidiMessage::ControlChange(PLETHORA_CHANNEL, Control::new(95), HIGH_VALUE)
            }
            Plethora::BoardDown => {
                MidiMessage::ControlChange(PLETHORA_CHANNEL, Control::new(94), HIGH_VALUE)
            }
            Plethora::Board(nr) => MidiMessage::ProgramChange(PLETHORA_CHANNEL, Program::new(nr)),
        }
    }
}

fn resolve(event: MidiMessage) -> Option<MidiMessage> {
    match event {
        embedded_midi::MidiMessage::ControlChange(channel, control, _value) => {
            if XTONE_CHANNEL == channel {
                return resolve_xtone(control);
            } else {
                return None;
            }
        }
        _ => {
            return None;
        }
    }
}

const XTONE_CHANNEL: Channel = Channel::new(0);

const XTONE_GREEN_A: Control = Control::new(10);
const XTONE_GREEN_B: Control = Control::new(22);
const XTONE_GREEN_C: Control = Control::new(11);
const XTONE_GREEN_D: Control = Control::new(24);
const XTONE_GREEN_E: Control = Control::new(25);
const XTONE_GREEN_F: Control = Control::new(26);

fn resolve_xtone(control: Control) -> Option<MidiMessage> {
    match control {
        XTONE_GREEN_A => Some(Plethora::BoardDown.midi_message()),
        XTONE_GREEN_B => None,
        XTONE_GREEN_C => Some(Plethora::Board(20).midi_message()),
        XTONE_GREEN_D => Some(Plethora::BoardUp.midi_message()),
        XTONE_GREEN_E => None,
        XTONE_GREEN_F => Some(Plethora::Board(21).midi_message()),
        _ => None,
    }
}
