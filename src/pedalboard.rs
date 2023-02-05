mod plethora;
pub mod rc500;
mod xtone;

use midi_types::MidiMessage;

pub fn handle(event: MidiMessage, rc500: &mut rc500::RC500) -> xtone::EventResult {
    match event {
        midi_types::MidiMessage::ControlChange(channel, control, value) => {
            if xtone::CHANNEL == channel {
                xtone::handle(rc500, control, value)
            } else {
                xtone::EventResult::none()
            }
        }
        _ => xtone::EventResult::none(),
    }
}
