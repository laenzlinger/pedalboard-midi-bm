use midi_types::{Channel, Control, MidiMessage, Program, Value7};

const RC500_CHANNEL: Channel = Channel::new(0);
const MAX_VALUE: Value7 = midi_types::Value7::new(127);
const MIN_VALUE: Value7 = midi_types::Value7::new(1);

pub enum RC500 {
    Memory(u8),
    MemUp(),
    MemDown(),
}

impl RC500 {
    pub fn midi_message(&self) -> MidiMessage {
        match *self {
            RC500::Memory(nr) => MidiMessage::ProgramChange(RC500_CHANNEL, Program::new(nr - 1)),
            RC500::MemUp() => MidiMessage::ControlChange(RC500_CHANNEL, Control::new(1), MAX_VALUE),
            RC500::MemDown() => {
                MidiMessage::ControlChange(RC500_CHANNEL, Control::new(1), MIN_VALUE)
            }
        }
    }
}
