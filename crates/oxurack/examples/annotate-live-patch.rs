use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use oxurack::{LabelKind, LabelTarget, LabelValue, MidiOutModule, Oxurack, SimulationDriver, Tag};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

#[derive(Debug, Parser)]
#[command(about = "Annotate a live rack and save its current metadata")]
struct Args {
    /// RON patch to load.
    #[arg(long)]
    patch: PathBuf,
    /// Destination RON patch to create or overwrite.
    #[arg(long)]
    save: PathBuf,
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut rack = Oxurack::new();
    let result = (|| {
        rack.try_install::<ClockModule>()?;
        rack.try_install::<LfoModule>()?;
        rack.try_install::<TuringMachineModule>()?;
        rack.try_install::<MidiOutModule>()?;
        rack.load_patch(&args.patch)?;
        rack.set_driver_mode(SimulationDriver::Manual).start();

        advance_and_drain(&mut rack, 200)?;
        let rack_target = LabelTarget::Rack;
        let module = LabelTarget::Module("tm_main".into());
        let cable = LabelTarget::Cable {
            source: ("clock_main".into(), "pulse".into()),
            target: ("tm_main".into(), "clock".into()),
            occurrence: 0,
        };

        rack.set_label(
            &rack_target,
            LabelKind::DisplayName,
            LabelValue::Text("Turing workbench".into()),
        )?;
        rack.set_label(
            &rack_target,
            LabelKind::Tags,
            LabelValue::Tags(
                [Tag::new("workbench")?, Tag::new("generative")?]
                    .into_iter()
                    .collect::<std::collections::BTreeSet<_>>(),
            ),
        )?;
        let mut notes = match rack.label(&rack_target, LabelKind::Notes)? {
            Some(LabelValue::Notes(notes)) => notes,
            _ => return Err("rack notes returned an unexpected payload".into()),
        };
        notes.push("Seeded LFO modulation".into());
        notes.push("Configuration replay; engines restart".into());
        rack.set_label(&rack_target, LabelKind::Notes, LabelValue::Notes(notes))?;
        rack.set_label(
            &module,
            LabelKind::DisplayName,
            LabelValue::Text("Melody generator".into()),
        )?;
        rack.set_label(
            &cable,
            LabelKind::Description,
            LabelValue::Text("Clock pulse to Turing Machine".into()),
        )?;

        rack.save_patch(&args.save)?;
        let rack_name = rack.label(&rack_target, LabelKind::DisplayName)?;
        let rack_tags = rack.label(&rack_target, LabelKind::Tags)?;
        let rack_notes = rack.label(&rack_target, LabelKind::Notes)?;
        let module_name = rack.label(&module, LabelKind::DisplayName)?;
        let cable_description = rack.label(&cable, LabelKind::Description)?;
        advance_and_drain(&mut rack, 100)?;
        let completed_samples = rack.sample_time().completed_samples();
        Ok::<_, Box<dyn std::error::Error>>((
            rack_name,
            rack_tags,
            rack_notes,
            module_name,
            cable_description,
            completed_samples,
        ))
    })();
    rack.stop();

    let (rack_name, rack_tags, rack_notes, module_name, cable_description, completed_samples) =
        result?;
    println!(
        "rack.display_name={rack_name:?}\nrack.tags={rack_tags:?}\nrack.notes={rack_notes:?}\ntm_main.display_name={module_name:?}\nclock_main.pulse -> tm_main.clock.description={cable_description:?}\nsaved={}\ncompleted_samples={completed_samples}",
        args.save.display()
    );
    Ok(())
}

fn advance_and_drain(rack: &mut Oxurack, samples: u64) -> Result<(), oxurack::CoreError> {
    for _ in 0..samples {
        rack.advance_samples(1)?;
        for _event in rack.drain_midi() {}
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("annotate-live-patch: {error}");
            ExitCode::FAILURE
        }
    }
}
