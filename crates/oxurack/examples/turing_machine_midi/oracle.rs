use oxurack::ModuleSpawnContext;
use oxurack_mod_turingmachine::{Scale, StepOutputs, TuringMachine};

pub(crate) fn expected_trace(samples: u64) -> Vec<(u64, [u8; 3])> {
    let mut engine =
        TuringMachine::with_seed(ModuleSpawnContext::new(42, "tm_main").instance_seed());
    engine.set_write(0.75);
    engine.set_length(8);
    engine.set_scale(Scale::new(vec![0, 2, 4, 7, 9], "Major Pentatonic"));
    engine.set_note_range(48..=72);
    let mut pending: Option<StepOutputs> = None;
    let mut active_note = None;
    let mut expected = Vec::new();
    for sample in 0..samples {
        if sample % 2 == 1 {
            let output = pending.take().expect("Clock edge at previous even sample");
            if let Some(old) = active_note.take() {
                expected.push((sample, [0x80, old, 0]));
            }
            if let Some(note) = output.note
                && output.gate
            {
                let velocity = output.velocity.unwrap_or(64);
                if velocity > 0 {
                    expected.push((sample, [0x90, note, velocity]));
                    active_note = Some(note);
                }
            }
        }
        if sample % 2 == 0 {
            pending = Some(engine.tick());
        }
    }
    expected
}

#[allow(dead_code)] // This shared oracle module is also compiled by the baseline MIDI tests.
pub(crate) fn expected_modulated_trace(samples: u64) -> Vec<(u64, [u8; 3])> {
    let mut engine =
        TuringMachine::with_seed(ModuleSpawnContext::new(42, "tm_main").instance_seed());
    engine.set_write(0.5);
    engine.set_length(8);
    engine.set_scale(Scale::new(vec![0, 2, 4, 7, 9], "Major Pentatonic"));
    engine.set_note_range(48..=72);
    let mut pending: Option<StepOutputs> = None;
    let mut active_note = None;
    let mut expected = Vec::new();
    for sample in 0..samples {
        if sample % 2 == 1 {
            let output = pending.take().expect("Clock edge at previous even sample");
            if let Some(old) = active_note.take() {
                expected.push((sample, [0x80, old, 0]));
            }
            if let Some(note) = output.note
                && output.gate
            {
                let velocity = output.velocity.unwrap_or(64);
                if velocity > 0 {
                    expected.push((sample, [0x90, note, velocity]));
                    active_note = Some(note);
                }
            }
        }

        let target = if sample % 16 < 8 { 1.0 } else { 0.0 };
        engine.modulate_write(target - engine.write_probability());
        if sample % 2 == 0 {
            pending = Some(engine.tick());
        }
    }
    expected
}
