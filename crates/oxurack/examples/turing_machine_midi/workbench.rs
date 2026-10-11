use std::io::{self, Write};
use std::path::Path;

use oxurack::{MidiDelivery, SampleTime, ScopeCapture, ScopeRecord, Value};

/// Write an immutable view of the retained Scope window.
pub(super) fn write_view(
    writer: &mut impl Write,
    patch: &Path,
    time: SampleTime,
    capture: &ScopeCapture,
    rows: usize,
) -> io::Result<()> {
    let mut signal_count = 0usize;
    let mut generated_count = 0usize;
    let mut sent_count = 0usize;
    let mut sample_span: Option<(u64, u64)> = None;
    let mut latest = vec![None; capture.selectors().len()];
    let mut retained = 0usize;

    for record in capture.records() {
        retained += 1;
        match *record {
            ScopeRecord::Signal {
                sample,
                selection,
                value,
            } => {
                let slot = latest.get_mut(selection).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Scope signal selector index is invalid",
                    )
                })?;
                signal_count += 1;
                *slot = Some((sample, value));
                extend_span(&mut sample_span, sample);
            }
            ScopeRecord::Midi { event, delivery } => {
                match delivery {
                    MidiDelivery::Generated => generated_count += 1,
                    MidiDelivery::Sent => sent_count += 1,
                    _ => return Err(unsupported("MIDI delivery")),
                }
                extend_span(&mut sample_span, event.sample);
            }
            _ => return Err(unsupported("Scope record")),
        }
    }

    writeln!(writer, "Scope workbench")?;
    writeln!(writer, "patch={patch:?}")?;
    writeln!(
        writer,
        "completed_samples={} capacity={} retained={} overwritten={}",
        time.completed_samples(),
        capture.capacity(),
        retained,
        capture.overwritten()
    )?;
    match sample_span {
        Some((first, last)) => writeln!(writer, "retained_sample_span={first}..={last}")?,
        None => writeln!(writer, "retained_sample_span=none")?,
    }
    writeln!(
        writer,
        "retained_counts signal={signal_count} generated={generated_count} sent={sent_count}"
    )?;
    writeln!(writer, "latest signals (selector order; retained window):")?;
    if capture.selectors().is_empty() {
        writeln!(writer, "(none selected)")?;
    } else {
        for (selector, current) in capture.selectors().iter().zip(latest) {
            match current {
                Some((sample, value)) => {
                    let (kind, scalar) = display_value(value)?;
                    writeln!(
                        writer,
                        "module={:?} port={:?} sample={sample} kind={kind} value={scalar}",
                        selector.module, selector.port
                    )?;
                }
                None => writeln!(
                    writer,
                    "module={:?} port={:?} sample=none value=unobserved",
                    selector.module, selector.port
                )?,
            }
        }
    }

    writeln!(
        writer,
        "recent signals (last {rows} retained; insertion order):"
    )?;
    let signal_skip = signal_count.saturating_sub(rows);
    let mut signals_written = 0usize;
    for record in capture
        .records()
        .filter(|record| matches!(record, ScopeRecord::Signal { .. }))
        .skip(signal_skip)
    {
        let ScopeRecord::Signal {
            sample,
            selection,
            value,
        } = *record
        else {
            unreachable!()
        };
        let selector = capture.selectors().get(selection).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Scope signal selector index is invalid",
            )
        })?;
        let (kind, scalar) = display_value(value)?;
        writeln!(
            writer,
            "{sample} module={:?} port={:?} kind={kind} value={scalar}",
            selector.module, selector.port
        )?;
        signals_written += 1;
    }
    if signals_written == 0 {
        writeln!(writer, "(none retained)")?;
    }

    writeln!(
        writer,
        "recent MIDI (last {rows} retained; insertion order):"
    )?;
    let midi_skip = generated_count + sent_count;
    let midi_skip = midi_skip.saturating_sub(rows);
    let mut midi_written = 0usize;
    for record in capture
        .records()
        .filter(|record| matches!(record, ScopeRecord::Midi { .. }))
        .skip(midi_skip)
    {
        let ScopeRecord::Midi { event, delivery } = *record else {
            unreachable!()
        };
        let label = match delivery {
            MidiDelivery::Generated => "generated",
            MidiDelivery::Sent => "sent",
            _ => return Err(unsupported("MIDI delivery")),
        };
        let [status, data1, data2] = event.message.to_bytes();
        writeln!(
            writer,
            "{} {label} {status:02X} {data1:02X} {data2:02X}",
            event.sample
        )?;
        midi_written += 1;
    }
    if midi_written == 0 {
        writeln!(writer, "(none retained)")?;
    }
    Ok(())
}

fn extend_span(span: &mut Option<(u64, u64)>, sample: u64) {
    match span {
        Some((first, last)) => {
            *first = (*first).min(sample);
            *last = (*last).max(sample);
        }
        None => *span = Some((sample, sample)),
    }
}

fn display_value(value: Value) -> io::Result<(&'static str, String)> {
    match value {
        Value::Float(value) => Ok(("float", value.to_string())),
        Value::Bipolar(value) => Ok(("bipolar", value.to_string())),
        Value::Gate(value) => Ok(("gate", value.to_string())),
        Value::Raw(value) => Ok(("raw", value.to_string())),
        Value::Midi(_) => Err(unsupported("MIDI value in signal record")),
        _ => Err(unsupported("signal value")),
    }
}

fn unsupported(kind: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("unsupported {kind} variant in Scope workbench"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    use oxurack::ParameterValue;
    use oxurack::{MidiOutModule, MidiOutputEvent, Oxurack, ScopeSelector, SimulationDriver};
    use oxurack_core::{deserialize_patch, save_patch_to_file};
    use oxurack_mod_clock::ClockModule;
    use oxurack_mod_lfo::LfoModule;
    use oxurack_mod_turingmachine::TuringMachineModule;

    static NEXT: AtomicU64 = AtomicU64::new(0);

    fn patch_path() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron")
    }

    fn temp_path(suffix: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "oxurack-workbench-{}-{}-{suffix}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ))
    }

    fn lfo_rack(name: Option<&str>) -> (Oxurack, std::path::PathBuf) {
        let mut patch = deserialize_patch(&std::fs::read_to_string(patch_path()).unwrap()).unwrap();
        patch.modules.retain(|module| module.kind == "lfo");
        patch.cables.clear();
        patch.modules[0]
            .parameters
            .insert("waveform".into(), ParameterValue::String("saw_up".into()));
        patch.modules[0]
            .parameters
            .insert("frequency_hz".into(), ParameterValue::Float(62.5));
        patch.modules[0]
            .parameters
            .insert("phase_offset".into(), ParameterValue::Float(0.0));
        if let Some(name) = name {
            patch.modules[0].instance_name = name.to_owned();
        }
        let path = temp_path(".ron");
        save_patch_to_file(&patch, &path).unwrap();
        let mut rack = Oxurack::new();
        rack.try_install::<LfoModule>().unwrap();
        rack.load_patch(&path).unwrap();
        rack.set_driver_mode(SimulationDriver::Manual);
        (rack, path)
    }

    fn event_template() -> MidiOutputEvent {
        let mut patch = deserialize_patch(&std::fs::read_to_string(patch_path()).unwrap()).unwrap();
        patch.modules.retain(|module| module.kind != "lfo");
        patch.cables.retain(|cable| {
            !cable.source.0.starts_with("lfo_main") && !cable.target.0.starts_with("lfo_main")
        });
        for module in &mut patch.modules {
            if module.kind == "clock" {
                module
                    .parameters
                    .insert("bpm".into(), ParameterValue::Float(1000.0));
                module
                    .parameters
                    .insert("ppqn".into(), ParameterValue::Int(30));
            }
        }
        let path = temp_path("-midi.ron");
        save_patch_to_file(&patch, &path).unwrap();
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<MidiOutModule>().unwrap();
        rack.load_patch(&path).unwrap();
        rack.set_driver_mode(SimulationDriver::Manual).start();
        let event = (0..64)
            .find_map(|_| {
                rack.advance_samples(1).unwrap();
                rack.drain_midi().next()
            })
            .expect("seeded MIDI fixture produces an event within 64 samples");
        rack.stop();
        let _ = std::fs::remove_file(path);
        event
    }

    fn render(rack: &Oxurack, path: &Path, rows: usize) -> String {
        let mut output = Vec::new();
        write_view(
            &mut output,
            path,
            rack.sample_time(),
            rack.scope().unwrap(),
            rows,
        )
        .unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn loaded_saw_up_view_reports_latest_and_independent_tails_in_selector_order() {
        let (mut rack, path) = lfo_rack(None);
        let selectors = [ScopeSelector::new("lfo_main", "bipolar_out")];
        rack.configure_scope(&selectors, 20).unwrap().start();
        for _ in 0..4 {
            rack.advance_samples(1).unwrap();
            let events: Vec<_> = rack.drain_midi().collect();
            for event in events {
                rack.record_scope_midi(event, MidiDelivery::Generated);
            }
        }
        rack.stop();
        let midi = event_template();
        let mut generated = midi;
        generated.sample = 1;
        generated.message.status = 0x90;
        generated.message.data1 = 0x3c;
        generated.message.data2 = 0x5a;
        let sent = generated;
        rack.record_scope_midi(generated, MidiDelivery::Generated);
        rack.record_scope_midi(sent, MidiDelivery::Sent);
        let before_records: Vec<_> = rack.scope().unwrap().records().copied().collect();
        let before_time = rack.sample_time();
        let output = render(&rack, Path::new("fixture.ron"), 2);
        assert!(output.contains("patch=\"fixture.ron\"\ncompleted_samples=4 capacity=20 retained=6 overwritten=0\nretained_sample_span=0..=3\nretained_counts signal=4 generated=1 sent=1\n"));
        assert!(output.contains(
            "module=\"lfo_main\" port=\"bipolar_out\" sample=3 kind=bipolar value=-0.625\n"
        ));
        assert!(output.contains("2 module=\"lfo_main\" port=\"bipolar_out\" kind=bipolar value=-0.75\n3 module=\"lfo_main\" port=\"bipolar_out\" kind=bipolar value=-0.625\n"));
        assert!(output.contains("1 generated 90 3C 5A\n1 sent 90 3C 5A\n"));
        assert_eq!(
            rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
            before_records
        );
        assert_eq!(rack.sample_time(), before_time);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn bounded_signal_tail_empty_capture_and_escaped_labels_are_honest() {
        let (mut rack, path) = lfo_rack(None);
        rack.configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 3)
            .unwrap()
            .start();
        for _ in 0..5 {
            rack.advance_samples(1).unwrap();
            let _: Vec<_> = rack.drain_midi().collect();
        }
        rack.stop();
        let output = render(&rack, &path, 1);
        assert!(output.contains("completed_samples=5 capacity=3 retained=3 overwritten=2\nretained_sample_span=2..=4\nretained_counts signal=3 generated=0 sent=0\n"));
        assert!(
            output.contains("4 module=\"lfo_main\" port=\"bipolar_out\" kind=bipolar value=-0.5\n")
        );
        rack.clear_scope();
        let empty = render(&rack, &path, 2);
        assert!(
            empty.contains(
                "retained_sample_span=none\nretained_counts signal=0 generated=0 sent=0\n"
            )
        );
        assert!(
            empty.contains(
                "module=\"lfo_main\" port=\"bipolar_out\" sample=none value=unobserved\n"
            )
        );
        assert_eq!(empty.matches("(none retained)").count(), 2);
        let _ = std::fs::remove_file(path);

        let (mut escaped, path) = lfo_rack(Some("lfo\nmain"));
        escaped
            .configure_scope(&[ScopeSelector::new("lfo\nmain", "bipolar_out")], 2)
            .unwrap();
        let escaped_output = render(&escaped, Path::new("path\nname.ron"), 1);
        assert!(escaped_output.contains("patch=\"path\\nname.ron\""));
        assert!(escaped_output.contains("module=\"lfo\\nmain\""));
        assert_eq!(
            escaped_output.lines().count(),
            escaped_output.matches('\n').count()
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn midi_only_capture_has_no_selected_signals_and_keeps_delivery_labels() {
        let (mut rack, path) = lfo_rack(None);
        rack.configure_scope(&[], 8).unwrap();
        let midi = event_template();
        rack.record_scope_midi(midi, MidiDelivery::Generated);
        rack.record_scope_midi(midi, MidiDelivery::Sent);
        let output = render(&rack, &path, 12);
        assert!(
            output.contains("latest signals (selector order; retained window):\n(none selected)\n")
        );
        assert!(
            output
                .contains("recent signals (last 12 retained; insertion order):\n(none retained)\n")
        );
        assert!(output.contains("generated "));
        assert!(output.contains(" sent "));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn selectors_keep_caller_order_and_scalar_kinds_are_displayed_without_sanitizing() {
        let (mut rack, path) = lfo_rack(None);
        let selectors = [
            ScopeSelector::new("lfo_main", "out"),
            ScopeSelector::new("lfo_main", "bipolar_out"),
        ];
        rack.configure_scope(&selectors, 4).unwrap().start();
        rack.advance_samples(1).unwrap();
        let _: Vec<_> = rack.drain_midi().collect();
        rack.stop();
        let output = render(&rack, &path, 1);
        let first = output.find("port=\"out\"").unwrap();
        let second = output.find("port=\"bipolar_out\"").unwrap();
        assert!(
            first < second,
            "latest rows follow the caller's selector order"
        );
        assert_eq!(
            display_value(Value::Float(f32::INFINITY)).unwrap(),
            ("float", "inf".to_owned())
        );
        assert_eq!(
            display_value(Value::Gate(true)).unwrap(),
            ("gate", "true".to_owned())
        );
        assert_eq!(
            display_value(Value::Raw(17)).unwrap(),
            ("raw", "17".to_owned())
        );
        assert_eq!(
            display_value(Value::Bipolar(f32::NAN)).unwrap(),
            ("bipolar", "NaN".to_owned())
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn clear_then_resume_keeps_absolute_sample_and_phase_while_fresh_rack_starts_at_zero() {
        let selector = [ScopeSelector::new("lfo_main", "bipolar_out")];
        let (mut rack, path) = lfo_rack(None);
        rack.configure_scope(&selector, 8).unwrap().start();
        for _ in 0..4 {
            rack.advance_samples(1).unwrap();
            let _: Vec<_> = rack.drain_midi().collect();
        }
        rack.stop();
        rack.clear_scope().start();
        rack.advance_samples(1).unwrap();
        let _: Vec<_> = rack.drain_midi().collect();
        rack.stop();
        let continued = render(&rack, &path, 2);
        assert!(continued.contains("completed_samples=5"));
        assert!(
            continued
                .contains("4 module=\"lfo_main\" port=\"bipolar_out\" kind=bipolar value=-0.5")
        );

        let (mut fresh, fresh_path) = lfo_rack(None);
        fresh.configure_scope(&selector, 8).unwrap();
        assert!(render(&fresh, &fresh_path, 2).contains("completed_samples=0"));
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(fresh_path);
    }
}
