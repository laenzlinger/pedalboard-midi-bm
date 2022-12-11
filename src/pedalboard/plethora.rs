use midi_types::{Channel, Control, MidiMessage, Program};

const PLETHORA_CHANNEL: Channel = Channel::new(1);

const HIGH_VALUE: midi_types::Value7 = midi_types::Value7::new(100);

pub enum Plethora {
    Board(u8),
    BoardUp,
    BoardDown,
}
impl Plethora {
    pub fn midi_message(&self) -> MidiMessage {
        match *self {
            Plethora::BoardUp => {
                MidiMessage::ControlChange(PLETHORA_CHANNEL, Control::new(95), HIGH_VALUE)
            }
            Plethora::BoardDown => {
                MidiMessage::ControlChange(PLETHORA_CHANNEL, Control::new(94), HIGH_VALUE)
            }
            Plethora::Board(nr) => {
                MidiMessage::ProgramChange(PLETHORA_CHANNEL, Program::new(nr - 1))
            }
        }
    }
}
