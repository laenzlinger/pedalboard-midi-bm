use crate::pedalboard::plethora::Plethora;
use crate::pedalboard::rc500::{Direction, RC500Event, RC500};
use heapless::Vec;
use midi_types::{Channel, Control, MidiMessage, Value7};
use smart_leds::RGB8;

pub const CHANNEL: Channel = Channel::new(0);

const XTONE_GREEN_A: Control = Control::new(10);
const XTONE_GREEN_B: Control = Control::new(22);
const XTONE_GREEN_C: Control = Control::new(11);
const XTONE_GREEN_D: Control = Control::new(24);
const XTONE_GREEN_E: Control = Control::new(25);
const XTONE_GREEN_F: Control = Control::new(26);
const XTONE_GREEN_EXP: Control = Control::new(7);

const XTONE_BLUE_A: Control = Control::new(40);
const XTONE_BLUE_B: Control = Control::new(42);
const XTONE_BLUE_C: Control = Control::new(43);
const XTONE_BLUE_D: Control = Control::new(64);
const XTONE_BLUE_E: Control = Control::new(15);
const XTONE_BLUE_F: Control = Control::new(41);
const XTONE_BLUE_EXP: Control = Control::new(4);

const XTONE_RED_A: Control = Control::new(70);
const XTONE_RED_B: Control = Control::new(71);
const XTONE_RED_C: Control = Control::new(72);
const XTONE_RED_D: Control = Control::new(73);
const XTONE_RED_E: Control = Control::new(74);
const XTONE_RED_F: Control = Control::new(75);
const XTONE_RED_EXP: Control = Control::new(1);

const NONE: Vec<MidiMessage, 8> = Vec::new();

pub struct EventResult {
    pub messages: Vec<MidiMessage, 8>,
    pub pixel: Option<RGB8>,
}

impl EventResult {
    pub fn none() -> EventResult {
        EventResult {
            messages: NONE,
            pixel: None,
        }
    }
}

pub fn handle(rc500: &mut RC500, control: Control, value: Value7) -> EventResult {
    let messages = match control {
        XTONE_GREEN_A => Plethora::BoardDown.midi_messages(),
        XTONE_GREEN_B => Plethora::Board(1).midi_messages(),
        XTONE_GREEN_C => Plethora::Board(2).midi_messages(),
        XTONE_GREEN_D => Plethora::BoardUp.midi_messages(),
        XTONE_GREEN_E => Plethora::Board(3).midi_messages(),
        XTONE_GREEN_F => Plethora::Board(4).midi_messages(),
        XTONE_GREEN_EXP => NONE,

        XTONE_BLUE_A => rc500.midi_messages(RC500Event::ToggleRhythm()),
        XTONE_BLUE_B => rc500.midi_messages(RC500Event::RhythmVariation()),
        XTONE_BLUE_C => NONE,
        XTONE_BLUE_D => rc500.midi_messages(RC500Event::Mem(Direction::Up)),
        XTONE_BLUE_E => rc500.midi_messages(RC500Event::Mem(Direction::Down)),
        XTONE_BLUE_F => rc500.midi_messages(RC500Event::ClearCurrent()),
        XTONE_BLUE_EXP => rc500.midi_messages(RC500Event::CurrentChannelLevel(value)),

        XTONE_RED_A => rc500.midi_messages(RC500Event::RhythmPattern(Direction::Down)),
        XTONE_RED_B => rc500.midi_messages(RC500Event::DrumKit(Direction::Down)),
        XTONE_RED_C => NONE,
        XTONE_RED_D => rc500.midi_messages(RC500Event::RhythmPattern(Direction::Up)),
        XTONE_RED_E => rc500.midi_messages(RC500Event::DrumKit(Direction::Up)),
        XTONE_RED_F => NONE,
        XTONE_RED_EXP => NONE,

        _ => NONE,
    };
    let pixel = match control {
        XTONE_RED_A | XTONE_RED_B | XTONE_RED_C | XTONE_RED_D | XTONE_RED_E | XTONE_RED_F
        | XTONE_RED_EXP => Some(smart_leds::RGB8::new(255, 8, 8)),
        XTONE_BLUE_A | XTONE_BLUE_B | XTONE_BLUE_C | XTONE_BLUE_D | XTONE_BLUE_E | XTONE_BLUE_F
        | XTONE_BLUE_EXP => Some(smart_leds::RGB8::new(8, 8, 255)),
        XTONE_GREEN_A | XTONE_GREEN_B | XTONE_GREEN_C | XTONE_GREEN_D | XTONE_GREEN_E
        | XTONE_GREEN_F | XTONE_GREEN_EXP => Some(smart_leds::RGB8::new(8, 255, 8)),
        _ => None,
    };
    EventResult { messages, pixel }
}
