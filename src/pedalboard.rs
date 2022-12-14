mod plethora;
mod rc500;
mod xtone;

use heapless::Vec;
use midi_types::MidiMessage;

pub fn resolve(event: MidiMessage) -> Vec<MidiMessage, 8> {
    match event {
        midi_types::MidiMessage::ControlChange(channel, control, _value) => {
            if xtone::CHANNEL == channel {
                xtone::resolve_xtone(control)
            } else {
                xtone::NONE
            }
        }
        _ => xtone::NONE,
    }
}
