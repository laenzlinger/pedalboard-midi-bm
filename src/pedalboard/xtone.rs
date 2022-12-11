use crate::pedalboard::plethora::Plethora;
use crate::pedalboard::rc500::RC500;
use midi_types::{Channel, Control, MidiMessage};

pub const CHANNEL: Channel = Channel::new(0);

const XTONE_GREEN_A: Control = Control::new(10);
const XTONE_GREEN_B: Control = Control::new(22);
const XTONE_GREEN_C: Control = Control::new(11);
const XTONE_GREEN_D: Control = Control::new(24);
const XTONE_GREEN_E: Control = Control::new(25);
const XTONE_GREEN_F: Control = Control::new(26);

const XTONE_BLUE_A: Control = Control::new(40);
const XTONE_BLUE_B: Control = Control::new(42);
const XTONE_BLUE_C: Control = Control::new(43);
const XTONE_BLUE_D: Control = Control::new(64);
const XTONE_BLUE_E: Control = Control::new(15);
const XTONE_BLUE_F: Control = Control::new(41);

const XTONE_RED_A: Control = Control::new(70);
const XTONE_RED_B: Control = Control::new(71);
const XTONE_RED_C: Control = Control::new(72);
const XTONE_RED_D: Control = Control::new(73);
const XTONE_RED_E: Control = Control::new(74);
const XTONE_RED_F: Control = Control::new(75);

pub fn resolve_xtone(control: Control) -> Option<MidiMessage> {
    match control {
        XTONE_GREEN_A => Some(Plethora::BoardDown.midi_message()),
        XTONE_GREEN_B => Some(RC500::Memory(20).midi_message()),
        XTONE_GREEN_C => Some(Plethora::Board(29).midi_message()),
        XTONE_GREEN_D => Some(Plethora::BoardUp.midi_message()),
        XTONE_GREEN_E => Some(RC500::Memory(21).midi_message()),
        XTONE_GREEN_F => Some(Plethora::Board(30).midi_message()),

        XTONE_BLUE_A => None,
        XTONE_BLUE_B => None,
        XTONE_BLUE_C => None,
        XTONE_BLUE_D => None,
        XTONE_BLUE_E => None,
        XTONE_BLUE_F => None,

        XTONE_RED_A => None,
        XTONE_RED_B => None,
        XTONE_RED_C => None,
        XTONE_RED_D => None,
        XTONE_RED_E => None,
        XTONE_RED_F => None,

        _ => None,
    }
}
