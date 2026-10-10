# oxurack-mod-midiout

An installed, sample-driven MIDI output sink for oxurack. The module converts
the Turing Machine's published scalar ports into ordered `MidiWire` events; it
does not open a MIDI endpoint. Install `MidiOutModule`, load a patch with the
`midi_out` kind, then drain its bounded event buffer before advancing another
output-producing sample.

The sink has seven inputs (`step`, `note_valid`, `velocity_valid`, `gate`,
`note`, `velocity`, and `noise_cc`) and static `channel`, `noise_cc`, `note_cc`,
and `velocity_cc` parameters. Each enabled CC route is configured with a
controller number from 0 through 119; `-1` disables it.
