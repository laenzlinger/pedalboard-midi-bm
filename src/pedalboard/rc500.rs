use heapless::Vec;
use midi_types::{Channel, Control, MidiMessage, Program, Value7};

const RC500_CHANNEL: Channel = Channel::new(0);
const MAX_VALUE: Value7 = midi_types::Value7::new(127);
const MIN_VALUE: Value7 = midi_types::Value7::new(1);

const MAX_CAPACITY: usize = 8;

pub enum RC500 {
    Memory(u8),
    MemUp(),
    MemDown(),
    ClearCurrent(),
    ToggleRhythm(),
    RhythmVariation(),
    LoopEffect(),
}

impl RC500 {
    pub fn midi_messages(&self) -> Vec<MidiMessage, MAX_CAPACITY> {
        match *self {
            RC500::Memory(nr) => {
                let mut messages: Vec<MidiMessage, MAX_CAPACITY> = Vec::new();
                messages
                    .push(MidiMessage::ProgramChange(
                        RC500_CHANNEL,
                        Program::new(nr - 1),
                    ))
                    .unwrap();
                messages
            }
            RC500::MemUp() => toggle(Control::new(1)),
            RC500::MemDown() => toggle(Control::new(2)),
            RC500::ClearCurrent() => toggle(Control::new(3)),
            RC500::ToggleRhythm() => toggle(Control::new(4)),
            RC500::RhythmVariation() => toggle(Control::new(5)),
            RC500::LoopEffect() => toggle(Control::new(5)),
        }
    }
}

fn toggle(c: Control) -> Vec<MidiMessage, MAX_CAPACITY> {
    let mut messages: Vec<MidiMessage, MAX_CAPACITY> = Vec::new();
    messages
        .push(MidiMessage::ControlChange(RC500_CHANNEL, c, MAX_VALUE))
        .unwrap();
    messages
        .push(MidiMessage::ControlChange(RC500_CHANNEL, c, MIN_VALUE))
        .unwrap();
    messages
}
