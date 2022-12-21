mod plethora;
mod rc500;
mod xtone;

use heapless::Vec;
use midi_types::MidiMessage;

pub fn resolve(event: MidiMessage) -> Vec<MidiMessage, 8> {
    match event {
        midi_types::MidiMessage::ControlChange(channel, control, value) => {
            if xtone::CHANNEL == channel {
                xtone::resolve_xtone(control, value)
            } else {
                xtone::NONE
            }
        }
        _ => xtone::NONE,
    }
}
