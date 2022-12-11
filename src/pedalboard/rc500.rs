use midi_types::{Channel, MidiMessage, Program};

const RC500_CHANNEL: Channel = Channel::new(0);

pub enum RC500 {
    Memory(u8),
}

impl RC500 {
    pub fn midi_message(&self) -> MidiMessage {
        match *self {
            RC500::Memory(nr) => MidiMessage::ProgramChange(RC500_CHANNEL, Program::new(nr)),
        }
    }
}
