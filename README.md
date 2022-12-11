# Pedalboard Midi Controller

This project implements a MIDI controller for my pedalboard:

The midi devices are daisy chained with MIDI cables in the following order

```
XSONIC XTONE => MIDI Controller => Plethora X3 => RC500
```

Since the XSONIC XTONE is very invflexible this MIDI Controller takes the Events of the XTONE inputs and coverts them into
useful MIDI control messages for the rest of the MIDI controllable pedals on my board.

## Hardware

The target hardware is a Adafruit [MIDI Feather Wing](https://www.adafruit.com/product/4740) on top of an Adafruit
[Feather RP2040](https://www.adafruit.com/product/4884)


## Development
This project was generated with the [RP2040 Project Teamplate](https://github.com/rp-rs/rp2040-project-template)
