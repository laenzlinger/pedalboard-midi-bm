use heapless::Vec;
use midi_types::{Channel, Control, MidiMessage, Program, Value7};
const PLETHORA_CHANNEL: Channel = Channel::new(1);

const MAX_VALUE: Value7 = midi_types::Value7::new(127);

#[allow(dead_code)]
pub enum Plethora {
    Board(u8),
    BoardUp,
    BoardDown,
    HotKnob(u8, Value7),
}
impl Plethora {
    pub fn midi_messages(&self) -> Vec<MidiMessage, 8> {
        let mut messages: Vec<MidiMessage, 8> = Vec::new();
        let m = match *self {
            Plethora::BoardUp => {
                MidiMessage::ControlChange(PLETHORA_CHANNEL, Control::new(95), MAX_VALUE)
            }
            Plethora::BoardDown => {
                MidiMessage::ControlChange(PLETHORA_CHANNEL, Control::new(94), MAX_VALUE)
            }
            Plethora::Board(nr) => {
                MidiMessage::ProgramChange(PLETHORA_CHANNEL, Program::new(nr - 1))
            }
            Plethora::HotKnob(nr, value) => {
                MidiMessage::ControlChange(PLETHORA_CHANNEL, Control::new(106 + nr), value)
            }
        };
        messages.push(m).unwrap();
        messages
    }
}
