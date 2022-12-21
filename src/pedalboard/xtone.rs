use crate::pedalboard::plethora::Plethora;
use crate::pedalboard::rc500::{RC500Event, RC500};
use heapless::Vec;
use midi_types::{Channel, Control, MidiMessage, Value7};

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

pub const NONE: Vec<MidiMessage, 8> = Vec::new();

pub fn handle(rc500: &mut RC500, control: Control, value: Value7) -> Vec<MidiMessage, 8> {
    match control {
        XTONE_GREEN_A => Plethora::Board(29).midi_messages(),
        XTONE_GREEN_B => Plethora::Board(30).midi_messages(),
        XTONE_GREEN_C => NONE,
        XTONE_GREEN_D => Plethora::BoardUp.midi_messages(),
        XTONE_GREEN_E => Plethora::BoardDown.midi_messages(),
        XTONE_GREEN_F => NONE,
        XTONE_GREEN_EXP => Plethora::HotKnob(1, value).midi_messages(),

        XTONE_BLUE_A => rc500.midi_messages(RC500Event::ToggleRhythm()),
        XTONE_BLUE_B => rc500.midi_messages(RC500Event::RhythmVariation()),
        XTONE_BLUE_C => rc500.midi_messages(RC500Event::LoopEffect()),
        XTONE_BLUE_D => rc500.midi_messages(RC500Event::MemUp()),
        XTONE_BLUE_E => rc500.midi_messages(RC500Event::MemDown()),
        XTONE_BLUE_F => rc500.midi_messages(RC500Event::ClearCurrent()),
        XTONE_BLUE_EXP => Plethora::HotKnob(1, value).midi_messages(),

        XTONE_RED_A => rc500.midi_messages(RC500Event::RhythmPatternUp()),
        XTONE_RED_B => rc500.midi_messages(RC500Event::RhythmPatternDown()),
        XTONE_RED_C => NONE,
        XTONE_RED_D => NONE,
        XTONE_RED_E => NONE,
        XTONE_RED_F => NONE,
        XTONE_RED_EXP => Plethora::HotKnob(1, value).midi_messages(),

        _ => NONE,
    }
}
