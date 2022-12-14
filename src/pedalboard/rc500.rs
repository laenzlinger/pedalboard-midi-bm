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
                return messages;
            }
            RC500::MemUp() => toggle(Control::new(1)),
            RC500::MemDown() => toggle(Control::new(1)),
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
    return messages;
}
