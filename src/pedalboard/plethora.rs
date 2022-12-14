use midi_types::{Channel, Control, MidiMessage, Program, Value7};

const PLETHORA_CHANNEL: Channel = Channel::new(1);

const MAX_VALUE: Value7 = midi_types::Value7::new(127);

pub enum Plethora {
    Board(u8),
    BoardUp,
    BoardDown,
}
impl Plethora {
    pub fn midi_message(&self) -> MidiMessage {
        match *self {
            Plethora::BoardUp => {
                MidiMessage::ControlChange(PLETHORA_CHANNEL, Control::new(95), MAX_VALUE)
            }
            Plethora::BoardDown => {
                MidiMessage::ControlChange(PLETHORA_CHANNEL, Control::new(94), MAX_VALUE)
            }
            Plethora::Board(nr) => {
                MidiMessage::ProgramChange(PLETHORA_CHANNEL, Program::new(nr - 1))
            }
        }
    }
}
