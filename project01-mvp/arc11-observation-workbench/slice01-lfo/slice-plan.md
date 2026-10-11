# Arc11 Slice01 — LFO that modulates the playable rack

Version 1.1 — 2026-10-11. Status: open; four L rows done, L-02 open.
Current assignment: `arc11-observation-workbench/slice01-lfo/cc-prompt-iteration01.md`.
CDC authors; CC implements; CRC independently reviews; Duncan relays.
Source predecessor `65522f116a12a9ac96d35cfe25ed9c184f5bddcb` in the existing
`codex/oxurack-module-behavior-installation` worktree. Arc10 is CDC accepted.

## Outcome and scope

Add `oxurack-mod-lfo`: a small standalone sample oscillator with opt-in rack
adapter. Install it in the existing example and add a RON fixture that modulates
TM write probability. Duncan can audition it with the same endpoint flags.
Five catalog waveforms and four catalog ports are in scope. Scope capture is
next, not a prerequisite to LFO delivery. No live persistence, tempo sync ratio,
random waveform, DSP oversampling, UI framework or RT redesign in this slice.

## Binding oscillator contract

One call advances one 1 ms sample. State is instance-owned: validated config,
f64 phase accumulator in [0,1), prior sync level. No wall clock or RNG. Default
waveform sine, frequency 1 Hz, phase offset 0 cycles, pulse width 0.5.

Config parameters: `waveform` is exactly String("sine"|"tri"|"saw_up"|
"saw_down"|"square"); `frequency_hz`, `phase_offset`, `pulse_width` are Float.
Frequency must be finite in [0,100]; offset finite in [0,1); width finite in
(0,1). Reject wrong variants/unknown keys/out-of-range values before spawn or
setter mutation. Getters return actual config. All four registered setters
preserve phase and sync history; changes affect the next sample. Equal config
is a no-op. Width is valid even when another waveform is selected.

For current phase `p = (phase + phase_offset) mod 1`, bipolar output is:

| Waveform | Bipolar value | Values at p = 0, .25, .5, .75 |
| --- | --- | --- |
| sine | sin(2*pi*p) | 0, 1, 0, -1 (within tolerance) |
| tri | 1 - 4*abs(p-.5) | -1, 0, 1, 0 |
| saw_up | 2*p - 1 | -1, -.5, 0, .5 |
| saw_down | 1 - 2*p | 1, .5, 0, -.5 |
| square | 1 if p < pulse_width else -1 | 1, 1, -1, -1 at default width |

Unipolar output is `(bipolar+1)/2`. Bound rounding noise to [-1,1]/[0,1]
and emit f32 values. Evaluate output **before** incrementing phase; initial
sample evaluates phase zero plus offset. Then advance by effective_hz/1000
and wrap. Using f64 phase avoids unnecessary f32 accumulation drift; sine
results are tolerance-based across platforms, repeatable at the pinned binary.
No lookup table, unsafe code or benchmark prerequisite.

`freq_in` is normalized additive CV: finite values clamp to [0,1], nonfinite
values map to neutral 0. Effective rate = min(100, frequency_hz + 100*CV).
This documented signal-sanitization policy prevents NaN poisoning; configuration
errors remain fallible. Unconnected input is neutral. At zero effective rate
phase holds. `sync_in` rising edge resets phase to zero before output evaluation;
held high does not continually reset. Offset still applies after a reset.
Stop/resume preserves all oscillator/control state; fresh construction restarts.

## Binding rack integration and timing

Kind `lfo`; outputs `out` Float and `bipolar_out` Bipolar; inputs `freq_in` Float
and `sync_in` Gate. Use MergePolicy::Reject for all four ports (one control source
per input for now). Produce evaluates/publishes using controls retained from the
previous Consume; Consume latches current inputs for the next Produce. Initial
retained controls are 0/false. An edge consumed at sample n resets the output
at n+1. New CV affects phase increment at n+1 and thus the subsequent phase.
This avoids observing partially propagated inputs or depending on module order.
Both systems use the supplied schedule and TickPhase; core owns running gates.

Publish both outputs each running sample through CurrentValue. No additional
one-sample delay between LFO Produce and TM Consume: TM sees that sample's LFO
value. Existing TM step publication/MIDI latency remains unchanged. Reinstallation
adds no duplicate systems or state reset. Adapter uses CoreError variants for
invalid parameters; missing/wrong typed internal port state is an invariant bug,
not a new recoverable fault framework. The public engine's CV policy is above.

## Demonstration and delivery

Keep the original `turing-machine-midi.ron` unchanged. Add
`turing-machine-lfo.ron`, cloned from it with TM base write_probability 0.5,
an LFO at 0.5 Hz sine/offset0/width.5 and cable
`lfo_main.bipolar_out -> tm_main.write_modulation`, transform
`Some(Affine(factor: 0.5, offset: 0.0))`. Other notes/clock/seed settings remain.
Install LfoModule in `prepare_rack`; existing offline/live CLI and cleanup work
unchanged. Document a 10000-sample trace/live command in README. The production
runner must really load/use the new module; a standalone-only demo is insufficient.

New source/tests/fixture live in the implementation worktree. Planning return is
this slice's `closing-report.md` and `ledger.md`; concise intake/readback may be
in the closing report. Supporting artifacts, if necessary, belong in `artifacts/`
here; none are required for routine passing commands. Do not create a validator.

## Acceptance and author readiness

The [five ledger rows](ledger.md) and [prompt's concrete tests](cc-prompt.md#tests-and-checks)
are the exit contract. CRC checks source and independently reproduces affected
behavior/native checks. Known `block0.1.6` advisory and three RT ignores are
baseline, not newly passing evidence. Musical audition is available, not a new
blocking endpoint gate for this slice. Project/Arc11 remain open.

Author readiness (CDC, 2026-10-11): source-grounded in Clock adapter, TM Consume,
core ports/parameters and live runner at65522f1; design resolves waveform/rate/
phase/CV/sync and edits; applied Rust sections and executable examples are in
prompt; tests distinguish phase latency, reset-on-level, missing modulation and
setter reset errors. Reading is bounded, fixtures use real registry/runner, and
code examples are explicitly uncompiled guidance. No unresolved binding choice.

CRC review — 2026-10-11: [crc-verification.md](crc-verification.md) independently
reproduces native gates, 70-row LFO trace and original trace compatibility at
`a965386`. L-01/L-03/L-04/L-05 are done; L-02 is open. Scope capture remains held.
Iteration01 restores the existing private input-invariant contract and supplies
the already requested Update/FixedUpdate test. Only rack.rs and tests/lfo.rs
are authorized source changes. No design/scope expansion or additional gate.

Author readiness (CRC, 2026-10-11): all six checks pass for iteration01; actual
module corruption reproduced in both core schedules; typed initialization and
CV policy inspected; AP-24/AP-25/AP-33 applied; literal outputs, negative controls,
two-file scope and complete bounded reading/return contract supplied in the
[new prompt](cc-prompt-iteration01.md). One refinement pass issued, not yet executed.

## Assignment history

| Date | Prompt | Predecessor | Disposition |
| --- | --- | --- | --- |
| 2026-10-11 | arc11-observation-workbench/slice01-lfo/cc-prompt.md | directive09; accepted Arc10 at65522f1 | Returned at a965386/353766a; CRC accepted four rows; L-02 requires correction |
| 2026-10-11 | arc11-observation-workbench/slice01-lfo/cc-prompt-iteration01.md | cc-prompt.md | Active: repair wrong-type invariant fallback (F-01) and complete supplied Update proof (F-02); return to CRC |

## Version history

- v1.1 — 2026-10-11: CRC reviewed a965386, accepted four rows, kept L-02 open
  and issued one focused corrective iteration; original assignment preserved.

- v1.0 — 2026-10-11: CDC opened the first workbench slice after Arc10 acceptance;
  complete LFO behavior plus actual TM modulation example, with focused proof.
