mod plethora;
pub mod rc500;
mod xtone;

use heapless::Vec;
use midi_types::MidiMessage;

pub fn handle(event: MidiMessage, rc500: &mut rc500::RC500) -> Vec<MidiMessage, 8> {
    match event {
        midi_types::MidiMessage::ControlChange(channel, control, value) => {
            if xtone::CHANNEL == channel {
                xtone::handle(rc500, control, value)
            } else {
                xtone::NONE
            }
        }
        _ => xtone::NONE,
    }
}
