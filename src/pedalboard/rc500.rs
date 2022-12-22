use heapless::Vec;
use midi_types::{Channel, Control, MidiMessage, Program, Value7};

const RC500_CHANNEL: Channel = Channel::new(0);
const MAX_VALUE: Value7 = midi_types::Value7::new(127);
const MIN_VALUE: Value7 = midi_types::Value7::new(1);

const PATTERNS: [u8; 58] = [
    0, 2, 4, 6, 8, 11, 13, 15, 17, 19, 22, 24, 26, 28, 31, 33, 35, 37, 39, 42, 44, 46, 48, 51, 53,
    55, 57, 60, 62, 64, 66, 68, 71, 73, 75, 77, 80, 82, 84, 86, 89, 91, 93, 95, 97, 100, 102, 104,
    106, 109, 111, 113, 115, 117, 120, 122, 124, 126,
];

const DRUMKITS: [u8; 16] = [
    0, 8, 17, 26, 35, 43, 51, 59, 68, 76, 85, 94, 102, 110, 118, 126,
];

const MAX_CAPACITY: usize = 8;

pub enum Direction {
    Up,
    Down,
}

struct BidirectionalIterator {
    current: usize,
    control: Control,
}

impl BidirectionalIterator {
    fn new(control: Control) -> Self {
        Self {
            current: 0,
            control,
        }
    }

    fn up(&mut self, values: &[u8]) -> Vec<MidiMessage, MAX_CAPACITY> {
        if (self.current) < values.len() - 1 {
            self.current += 1;
        } else {
            self.current = 0
        }
        self.current(values)
    }
    fn down(&mut self, values: &[u8]) -> Vec<MidiMessage, MAX_CAPACITY> {
        if (self.current) > 0 {
            self.current -= 1;
        } else {
            self.current = values.len() - 1
        }
        self.current(values)
    }
    fn current(&self, values: &[u8]) -> Vec<MidiMessage, MAX_CAPACITY> {
        let mut messages: Vec<MidiMessage, MAX_CAPACITY> = Vec::new();
        match values.get(self.current) {
            Some(value) => {
                messages
                    .push(MidiMessage::ControlChange(
                        RC500_CHANNEL,
                        self.control,
                        Value7::new(*value),
                    ))
                    .unwrap();
                messages
            }
            None => messages,
        }
    }
}

pub struct RC500 {
    drumkits: BidirectionalIterator,
    patterns: BidirectionalIterator,
}

pub enum RC500Event {
    #[allow(dead_code)]
    Memory(u8),
    Mem(Direction),
    ClearCurrent(),
    ToggleRhythm(),
    LoopEffect(),
    RhythmVariation(),
    RhythmPatternUp(),
    RhythmPatternDown(),
    DrumkitsUp(),
    DrumkitsDown(),
}

impl RC500 {
    pub fn new() -> Self {
        Self {
            drumkits: BidirectionalIterator::new(Control::new(8)),
            patterns: BidirectionalIterator::new(Control::new(7)),
        }
    }
    pub fn midi_messages(&mut self, event: RC500Event) -> Vec<MidiMessage, MAX_CAPACITY> {
        match event {
            RC500Event::Memory(nr) => {
                let mut messages: Vec<MidiMessage, MAX_CAPACITY> = Vec::new();
                messages
                    .push(MidiMessage::ProgramChange(
                        RC500_CHANNEL,
                        Program::new(nr - 1),
                    ))
                    .unwrap();
                messages
            }
            RC500Event::Mem(dir) => match dir {
                Direction::Up => toggle(1),
                Direction::Down => toggle(2),
            },
            RC500Event::ClearCurrent() => toggle(3),
            RC500Event::ToggleRhythm() => toggle(4),
            RC500Event::RhythmVariation() => toggle(5),
            RC500Event::LoopEffect() => toggle(6),
            RC500Event::RhythmPatternUp() => self.patterns.up(&PATTERNS),
            RC500Event::RhythmPatternDown() => self.patterns.down(&PATTERNS),
            RC500Event::DrumkitsUp() => self.drumkits.up(&DRUMKITS),
            RC500Event::DrumkitsDown() => self.drumkits.down(&DRUMKITS),
        }
    }
}

fn toggle(control: u8) -> Vec<MidiMessage, MAX_CAPACITY> {
    let c = Control::new(control);
    let mut messages: Vec<MidiMessage, MAX_CAPACITY> = Vec::new();
    messages
        .push(MidiMessage::ControlChange(RC500_CHANNEL, c, MAX_VALUE))
        .unwrap();
    messages
        .push(MidiMessage::ControlChange(RC500_CHANNEL, c, MIN_VALUE))
        .unwrap();
    messages
}
