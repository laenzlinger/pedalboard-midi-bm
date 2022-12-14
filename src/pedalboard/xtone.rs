use crate::pedalboard::plethora::Plethora;
use crate::pedalboard::rc500::RC500;
use heapless::Vec;
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

pub const NONE: Vec<MidiMessage, 8> = Vec::new();

pub fn resolve_xtone(control: Control) -> Vec<MidiMessage, 8> {
    match control {
        XTONE_GREEN_A => Plethora::BoardDown.midi_messages(),
        XTONE_GREEN_B => NONE,
        XTONE_GREEN_C => Plethora::Board(29).midi_messages(),
        XTONE_GREEN_D => Plethora::BoardUp.midi_messages(),
        XTONE_GREEN_E => NONE,
        XTONE_GREEN_F => Plethora::Board(30).midi_messages(),

        XTONE_BLUE_A => RC500::MemUp().midi_messages(),
        XTONE_BLUE_B => RC500::MemDown().midi_messages(),
        XTONE_BLUE_C => RC500::ClearCurrent().midi_messages(),
        XTONE_BLUE_D => RC500::ToggleRhythm().midi_messages(),
        XTONE_BLUE_E => RC500::RhythmVariation().midi_messages(),
        XTONE_BLUE_F => RC500::LoopEffect().midi_messages(),

        XTONE_RED_A => NONE,
        XTONE_RED_B => NONE,
        XTONE_RED_C => NONE,
        XTONE_RED_D => NONE,
        XTONE_RED_E => NONE,
        XTONE_RED_F => NONE,

        _ => NONE,
    }
}
