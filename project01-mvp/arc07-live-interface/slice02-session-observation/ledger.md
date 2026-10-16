# Session observation — acceptance ledger

Opened2026-10-15 before implementation. All five rows open; CC reports proposed
results and CRC independently verifies. Authority: slice-plan.md, cc-prompt.md and
CDC directive14 D1–D3. Three contributors / Expedited Mode; initial correction0/5.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SO-01 | Actual producer chronology reaches an owned public journal without breaking typed messages or graph consumers | Real21-record interleaved load vector; direct/command delta/source order, no duplicates; independent typed readers/default graph and core without staging | correctness-grade | Directive14 D1/common-order residual | open | — | Sorted parameter notifications; Rack Tags before Notes; no snapshot reconstruction |
| SO-02 | Lifetime transitions, failed candidates and cable identity are truthful across replacement/removal | Same-name replace/restart/unload twice; actual retirement/sample/origin and failed-private-candidate preservation; incident removal and surviving parallel ID | correctness-grade | Directive14 D1/D2; PatchUnloaded residual | open | — | Failed candidate consumes no global position; no old Entity payload |
| SO-03 | Independent cursors and bounded retention expose exact gaps and checked exhaustion | Now/Oldest/clone/foreign cursor, page split,1025 changes and1101-emission candidate; typed exhaustion without mutation rollback | correctness-grade | Directive14 D1 retention/readers | open | — | Default1024 record bound, not arbitrary string byte budget |
| SO-04 | Usable session commands/watch preserve one MIDI owner, Scope boundaries and command/sample fairness | Real owner/fake sender cleanup, explicit fallback/unloaded recovery,32-record watch bound, actual default/no-graph stdin-open session and native gates | correctness-grade | Directive14 D3; accepted01 integration | open | — | Cleanup before new Scope; no endpoint audition; no completion/editor dependency |
| SO-05 | Current metadata/tag queries and watch/save compose with unchanged music and legacy persistence | Exact Any/All across rack/modules/parallel cables, malformed-input preservation; live direct control and nonempty fresh replay; pinned musical hashes | correctness-grade | Directive14 D3; metadata/query residual | open | — | Existing full-list notes only; note IDs/history05, completion06 |

## Execution notes

CC records actual pre-edit required-reading extents/source state and contract
readback here, then concise commands/results including failures/retries and each
row's proposed evidence. The closing report links this record and adds self-review,
artifact inventory and bubble-up. No prefilled completion/CRC acceptance.

## Historical scope reconciliation

This initial open set implements directive14's approved runtime/current-metadata
scope only. Persistent note identity/authorship/revisions remain05 with actual
RON compatibility probes before its prompt. Terminal completion remains06 with
actual PTY/stdout-stderr/shutdown probe before its prompt. Hierarchical tag and
formatter/config policy remains03; external transport04; richer port/graph/typed
catalog and exact checkpoints Arc13; physical timing/coverage/soak/release Arc08.
Both Arc07 composition rows and P-01–P-06 remain open. P-07 retains acceptance.

### CC pre-edit intake — 2026-10-15

Source checkout rechecked at `e582aed6e87c234799921cc66a5dd55214f76afd`, branch `codex/oxurack-module-behavior-installation`, clean before edits. Planning checkout is `fd3129fb22e014fd4759ae52f2dea5d72dbfeb3a`, branch `planning`, clean before edits. The assigned prompt, slice plan, ledger, directive14, current project direction/roadmap and Arc07 current executable plan were opened. The prompt is 573 lines; its required reading manifest and producer/facade/session recipes govern this work. Planning `AGENTS.md` is in the planning worktree root and directs this exact prompt/source split; project status confirms CRC issued this prompt after acknowledging directive14.

Source reconnaissance at the pinned tree confirmed the named integration points: core `apply_patch_to_world` emits typed messages directly and initializes modules/cables/rack metadata in patch order; `set_module_parameter` and `set_label_value` emit after successful setters; topology connect/disconnect/removal has a prepared incident-cable path; facade `build_base_app` owns app setup; lifecycle replacement builds a private candidate then commits it; snapshot reads the live ECS; the existing rack-live owner loop and Scope cleanup are in the prompt's allowlist. The prompt's own observed-order corrections are binding: sort only parameter notification iteration, and rack labels are DisplayName, Description, Tags, Notes. Stable cable identity is logical and must be staged independently of ECS entity identity.

Contract readback before implementation: candidate observation packets remain private and are discarded on failure; the old rack, its cursor stream and MIDI state remain authoritative until candidate commit. Staging/journal overflow or checked counter exhaustion makes observation unavailable but must not fail an otherwise successful mutation or falsify loaded membership/origin. Cursors are independent pull positions; reads are bounded, return exact half-open gaps, and advance only after success. Lifecycle change batches use one lifetime boundary and source order, never publish candidate records before commit. For replacement/restart, the accepted MIDI cleanup is delivered against the old owner before Scope is configured for the new owner; a Scope allocation/selector failure after commit is reported as MIDI-only capture fallback and cannot roll back the successful load. Slice05 note history and Slice06 terminal completion remain out of scope.

Initial implementation gate commands and row evidence are appended below; all SO-01–SO-05 remain open for CRC. The contract readback above was recorded before production edits. At closeout, CC found that complete manifest-wide read extents were not all independently recorded in this ledger. Treat intake evidence as partial and include this process deviation in CRC review; no planning acceptance is implied.

### CC execution pass — 2026-10-15

Source remained at pinned base `e582aed6e87c234799921cc66a5dd55214f76afd`
until the authorized scoped commit. Planning remained at
`fd3129fb22e014fd4759ae52f2dea5d72dbfeb3a` until its separate record commit.
The source and planning worktrees were clean before implementation. The named
manifest/prompt and contract readback above informed the work; targeted Rust
guidance sections API-02/03, TD-02/03, EH-01, CA-01/02/07 and CLI-27 were applied.
However, this ledger does not claim complete independent re-reading of every
full/section entry in the prompt manifest. CRC should decide whether this needs
a corrective intake pass.

#### Proposed evidence by row — all statuses remain open

- **SO-01:** The real session fixture emits the exact 21-record two-module,
  one-cable load batch. Tests assert patch/module order, sorted parameter
  notifications, Rack DisplayName/Description/Tags/Notes order, a split page
  with the true final marker, envelope sequence/batch/lifetime/sample, and
  PatchLoaded counts. Direct/command parameter and label/cable/BPM/reset paths
  assert one record per accepted call and provenance. Existing core/graph tests
  pass. CRC should independently inspect the typed-reader compatibility and
  standalone-core-without-staging cases against the exact row oracle.
- **SO-02:** Lifecycle integration covers restart origin retention, unload and
  repeated empty unload, explicit retirement reasons, failed file loading, and
  a candidate that fails after an earlier module was privately spawned. The
  nine-cable LFO fixture confirms cable IDs 2 and 10 remain distinct when the
  latter shifts to occurrence zero. A fake MIDI sender test covers same-session
  cleanup across restart and unload. CRC should still inspect dirty-sample
  retirement envelopes and multi-incident cable-removal order against the full
  row oracle.
- **SO-03:** Integration tests cover independent/cloned cursors, wrong-facade
  immobility, a 7+14 page split, exact `[22,23)` eviction after 1025 writes,
  and the 1101-emission candidate gap `[1,78)`. Counter tests cover sequence,
  batch, lifetime, cable ID and staging-position exhaustion without wrapping.
  CRC should inspect empty Oldest/Now behavior and mutation continuity after
  public journal exhaustion.
- **SO-04:** The live owner still uses one synchronous rack/session path.
  Tests cover a 32-record watch limit with an advancing diagnostic writer and
  one sample per due owner turn, failed-load preservation, and the same fake
  sender across restart/unload after an active voice. Timed default/no-graph
  subprocess smokes exited zero, kept MIDI rows on stdout and diagnostics/watch
  on stderr, and produced nonempty traces (20 rows in each final run). No MIDI
  endpoint was opened. CRC should inspect cleanup-on-remove, watch-writer
  failure plus shutdown cleanup, and EOF/Ctrl-C with retained watch backlog.
- **SO-05:** Exact tag matching/deduplication, malformed tag rejection,
  rack/module/cable label staging, current cable listing and the ordinary
  save/replay regression suites pass. The runbook documents a playable
  two-fixture session. A new controlled 200-sample edit/watch/query/save versus
  unchanged control and fresh 2000-sample replay was not added in this pass;
  CRC should treat SO-05 as open pending that exact current-integration proof.

#### Commands, results and preserved attempts

- `cargo test --locked -p oxurack-core --all-features`: passed (451 unit, 5
  integration, 9 seeded-spawn and 2 doctests).
- `cargo test --locked -p oxurack-graph`: passed (68 unit and 1 integration).
- `cargo test --locked -p oxurack --all-targets`: passed after final edits;
  includes the observation integration suite and rack-live owner tests.
- `cargo test --locked -p oxurack --all-targets --no-default-features`: passed
  after final edits; the existing `cables.rs` unused-mut warning remains visible.
- `cargo build --locked -p oxurack --examples` and the same command with
  `--no-default-features`: passed.
- `make lint`: passed. Strict
  `cargo clippy --locked -p oxurack-core -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings`:
  passed. Strict RUSTDOCFLAGS cargo doc command from the prompt: passed.
- `git diff --check`: passed before record/commit preparation.
- Restricted `make test`: reproduced exactly 146 passed, 4 failed, 3 ignored;
  the four failures are the pre-existing CoreMIDI initialization failures in
  `oxurack-rt` (both port listings and both nonexistent-port checks). The
  unchanged, prompt-authorized host retry passed (“All tests passed”). Final
  temporary logs were kept under `/tmp/oxurack-slice02-make-test-*-final2.log`.
- Default and no-graph rack-live subprocesses used timed stdin, retained stdout
  and stderr separately, and terminated successfully. A first short no-graph
  attempt paused before any sample and produced zero stdout lines; it is
  preserved as a failed attempt, then rerun with enough play time and produced
  20 MIDI rows. The default captured run produced 4 MIDI rows. These were
  software traces only, not hardware or listening evidence.
- Strict Clippy first failed on counter-test initialization and CLI parser
  complexity; those were corrected and strict Clippy passed. An initial
  parallel-cable test fixture lacked the intended LFO instance; it was changed
  to the existing LFO/Turing fixture and passed. No failed attempt was removed
  from the record.

#### Scope and self-review

Implemented source changes stay within the 25 allowed paths: two observation
modules, query helper, producer/facade/CLI integrations, one fixture, one
integration test, README and runbook, plus the allowed ron dev-dependency lock
edge. No graph, Scope, playback, workbench, note schema, terminal editor,
endpoint, release process or future-slice source was changed. No hardware
endpoint was opened. CRC should use the evidence and the explicit per-row gaps
above; there is no CC acceptance or closure claim.

Artifact inventory: source fixture and tests at their named paths; user runbook
`docs/user-testing-rack-session.md`; this ledger and the closing report. No
durable scratch logs were added; transient run logs remain in `/tmp`.
