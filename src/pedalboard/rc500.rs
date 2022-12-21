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

const MAX_CAPACITY: usize = 8;

struct Kit {
    current: u8,
}

impl Kit {
    fn new() -> Self {
        Self { current: 0 }
    }
}

struct Pattern {
    current: usize,
}

impl Pattern {
    fn new() -> Self {
        Self { current: 0 }
    }
    fn up(&mut self) -> Vec<MidiMessage, MAX_CAPACITY> {
        if (self.current) < PATTERNS.len() - 1 {
            self.current += 1;
        } else {
            self.current = 0
        }
        self.current()
    }
    fn down(&mut self) -> Vec<MidiMessage, MAX_CAPACITY> {
        if (self.current) > 0 {
            self.current -= 1;
        } else {
            self.current = PATTERNS.len() - 1
        }
        self.current()
    }
    fn current(&self) -> Vec<MidiMessage, MAX_CAPACITY> {
        let mut messages: Vec<MidiMessage, MAX_CAPACITY> = Vec::new();
        messages
            .push(MidiMessage::ControlChange(
                RC500_CHANNEL,
                Control::new(7),
                Value7::new(PATTERNS[self.current]),
            ))
            .unwrap();
        messages
    }
}

pub struct RC500 {
    kit: Kit,
    pattern: Pattern,
}

pub enum RC500Event {
    #[allow(dead_code)]
    Memory(u8),
    MemUp(),
    MemDown(),
    ClearCurrent(),
    ToggleRhythm(),
    LoopEffect(),
    RhythmVariation(),
    RhythmPatternUp(),
    RhythmPatternDown(),
}

impl RC500 {
    pub fn new() -> Self {
        Self {
            kit: Kit::new(),
            pattern: Pattern::new(),
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
            RC500Event::MemUp() => toggle(Control::new(1)),
            RC500Event::MemDown() => toggle(Control::new(2)),
            RC500Event::ClearCurrent() => toggle(Control::new(3)),
            RC500Event::ToggleRhythm() => toggle(Control::new(4)),
            RC500Event::RhythmVariation() => toggle(Control::new(5)),
            RC500Event::LoopEffect() => toggle(Control::new(6)),
            RC500Event::RhythmPatternUp() => self.pattern.up(),
            RC500Event::RhythmPatternDown() => self.pattern.down(),
        }
    }
}

fn toggle(c: Control) -> Vec<MidiMessage, MAX_CAPACITY> {
    let mut messages: Vec<MidiMessage, MAX_CAPACITY> = Vec::new();
    messages
        .push(MidiMessage::ControlChange(RC500_CHANNEL, c, MAX_VALUE))
        .unwrap();
    messages
        .push(MidiMessage::ControlChange(RC500_CHANNEL, c, MIN_VALUE))
        .unwrap();
    messages
}
