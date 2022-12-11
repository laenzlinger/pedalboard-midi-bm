mod plethora;
mod rc500;
mod xtone;

use midi_types::MidiMessage;

pub fn resolve(event: MidiMessage) -> Option<MidiMessage> {
    match event {
        midi_types::MidiMessage::ControlChange(channel, control, _value) => {
            if xtone::CHANNEL == channel {
                xtone::resolve_xtone(control)
            } else {
                None
            }
        }
        _ => None,
    }
}
