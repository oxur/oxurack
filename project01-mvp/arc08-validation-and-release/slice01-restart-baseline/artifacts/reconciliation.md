# Inherited-row reconciliation

This register accounts for the complete 24-row carryover denominator required
by R-02: twelve rows from six historical slices and twelve rows from six
historical arc ledgers. The source commit for current comparison is
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`; the planning predecessor named by
the slice is `f0cf3a5172e02c717c7f924c84156f5fe0405733`.

`artifacts/verify-planning.py` passed and reported six historical slices, six
started arcs, 24 distinct carryover rows and preserved criteria. The imported
source ledgers and candidate closing reports remain unchanged. Every row below
is still open in its source ledger and is received by restart row R-02. The
historical candidate reports propose deferral for these rows, but no candidate
disposition is a CRC acceptance.

## Exact row accounting

| # | Source ledger | ID | Kind / owner | Original criterion | Receiving row | Current reason and re-entry |
| ---: | --- | --- | --- | --- | --- | --- |
| 1 | `arc03-events-and-literate-patching/slice01-notifications/ledger.md` | S-01 | historical slice / `arc03-events-and-literate-patching` | Historical specified behavior is reconciled with the current implementation | R-02 | Candidate deferred because current behavior was not independently reproduced; CRC reviews under R-02, then CDC assigns criterion-specific successor work.
| 2 | `arc03-events-and-literate-patching/slice01-notifications/ledger.md` | S-02 | historical slice / `arc03-events-and-literate-patching` | Remaining differences and deferrals have explicit arc ownership | R-02 | Candidate deferred because original-scope reconciliation is incomplete; CRC checks the retained backlog, then CDC assigns unresolved capability work.
| 3 | `arc03-events-and-literate-patching/slice02-literate-patching/ledger.md` | S-01 | historical slice / `arc03-events-and-literate-patching` | Historical specified behavior is reconciled with the current implementation | R-02 | Candidate deferred because current behavior was not independently reproduced; CRC reviews under R-02, then CDC assigns criterion-specific successor work.
| 4 | `arc03-events-and-literate-patching/slice02-literate-patching/ledger.md` | S-02 | historical slice / `arc03-events-and-literate-patching` | Remaining differences and deferrals have explicit arc ownership | R-02 | Candidate deferred because original-scope reconciliation is incomplete; CRC checks the retained backlog, then CDC assigns unresolved capability work.
| 5 | `arc04-rackgraph/slice01-foundation/ledger.md` | S-01 | historical slice / `arc04-rackgraph` | Historical specified behavior is reconciled with the current implementation | R-02 | Candidate deferred because current behavior was not independently reproduced; CRC reviews under R-02, then CDC assigns criterion-specific successor work.
| 6 | `arc04-rackgraph/slice01-foundation/ledger.md` | S-02 | historical slice / `arc04-rackgraph` | Remaining differences and deferrals have explicit arc ownership | R-02 | Candidate deferred because original-scope reconciliation is incomplete; CRC checks the retained backlog, then CDC assigns unresolved capability work.
| 7 | `arc04-rackgraph/slice02-navigation/ledger.md` | S-01 | historical slice / `arc04-rackgraph` | Historical specified behavior is reconciled with the current implementation | R-02 | Candidate deferred because current behavior was not independently reproduced; CRC reviews under R-02, then CDC assigns criterion-specific successor work.
| 8 | `arc04-rackgraph/slice02-navigation/ledger.md` | S-02 | historical slice / `arc04-rackgraph` | Remaining differences and deferrals have explicit arc ownership | R-02 | Candidate deferred because original-scope reconciliation is incomplete; CRC checks the retained backlog, then CDC assigns unresolved capability work.
| 9 | `arc04-rackgraph/slice03-diagnostics/ledger.md` | S-01 | historical slice / `arc04-rackgraph` | Historical specified behavior is reconciled with the current implementation | R-02 | Candidate deferred because current behavior was not independently reproduced; CRC reviews under R-02, then CDC assigns criterion-specific successor work.
| 10 | `arc04-rackgraph/slice03-diagnostics/ledger.md` | S-02 | historical slice / `arc04-rackgraph` | Remaining differences and deferrals have explicit arc ownership | R-02 | Candidate deferred because original-scope reconciliation is incomplete; CRC checks the retained backlog, then CDC assigns unresolved capability work.
| 11 | `arc06-module-catalog/slice01-turing-machine/ledger.md` | S-01 | historical slice / `arc06-module-catalog` | Historical specified behavior is reconciled with the current implementation | R-02 | Candidate deferred because current behavior was not independently reproduced; CRC reviews under R-02, then CDC assigns criterion-specific successor work.
| 12 | `arc06-module-catalog/slice01-turing-machine/ledger.md` | S-02 | historical slice / `arc06-module-catalog` | Remaining differences and deferrals have explicit arc ownership | R-02 | Candidate deferred because original-scope reconciliation is incomplete; CRC checks the retained backlog, then CDC assigns unresolved capability work.
| 13 | `arc01-realtime-midi/ledger.md` | A-01-01 | historical arc / `arc01-realtime-midi` | RT clock and I/O primitives compose with the ECS bridge | R-02 | Arc candidate deferred; current RT/bridge behavior and named master/slave loopback are not independently accepted. CRC assembles the child review, then CDC assigns a successor arc or slice.
| 14 | `arc01-realtime-midi/ledger.md` | A-01-02 | historical arc / `arc01-realtime-midi` | Child work and deferred findings are accounted for at arc composition | R-02 | Arc candidate deferred; children remain unaccepted. CRC performs child carryover review, then CDC assigns the successor before feature execution.
| 15 | `arc02-core-and-persistence/ledger.md` | A-02-01 | historical arc / `arc02-core-and-persistence` | Tick cycle, module registration and deterministic patch persistence compose | R-02 | Arc candidate deferred; fresh-world seeded round-trip behavior is not independently accepted. CRC assembles the child review, then CDC assigns a successor arc or slice.
| 16 | `arc02-core-and-persistence/ledger.md` | A-02-02 | historical arc / `arc02-core-and-persistence` | Child work and deferred findings are accounted for at arc composition | R-02 | Arc candidate deferred; children remain unaccepted. CRC performs child carryover review, then CDC assigns the successor before feature execution.
| 17 | `arc03-events-and-literate-patching/ledger.md` | A-03-01 | historical arc / `arc03-events-and-literate-patching` | Commands and patch loads produce consistent notifications and persisted labels | R-02 | Arc candidate deferred; current command/notification behavior is not independently accepted. CRC assembles the child review, then CDC assigns a successor arc or slice.
| 18 | `arc03-events-and-literate-patching/ledger.md` | A-03-02 | historical arc / `arc03-events-and-literate-patching` | Child work and deferred findings are accounted for at arc composition | R-02 | Arc candidate deferred; children remain unaccepted. CRC performs child carryover review, then CDC assigns the successor before feature execution.
| 19 | `arc04-rackgraph/ledger.md` | A-04-01 | historical arc / `arc04-rackgraph` | Graph is fresh across topology/parameter changes and supports v0 queries | R-02 | Arc candidate deferred; current query/freshness evidence covers only exercised paths. CRC assembles the child review, then CDC assigns a successor arc or slice.
| 20 | `arc04-rackgraph/ledger.md` | A-04-02 | historical arc / `arc04-rackgraph` | Child work and deferred findings are accounted for at arc composition | R-02 | Arc candidate deferred; children remain unaccepted. CRC performs child carryover review, then CDC assigns the successor before feature execution.
| 21 | `arc05-public-api/ledger.md` | A-05-01 | historical arc / `arc05-public-api` | Public runtime patching, lifecycle and parameter surfaces operate coherently | R-02 | Arc candidate deferred; runtime command and lifecycle gaps remain in the pinned source. CRC assembles the child review, then CDC assigns a successor arc or slice.
| 22 | `arc05-public-api/ledger.md` | A-05-02 | historical arc / `arc05-public-api` | Child work and deferred findings are accounted for at arc composition | R-02 | Arc candidate deferred; children remain unaccepted. CRC performs child carryover review, then CDC assigns the successor before feature execution.
| 23 | `arc06-module-catalog/ledger.md` | A-06-01 | historical arc / `arc06-module-catalog` | Baseline musical modules work standalone and in a seeded rack | R-02 | Arc candidate deferred; standalone TM evidence does not prove a seeded rack or musical module integration. CRC assembles the child review, then CDC assigns a successor arc or slice.
| 24 | `arc06-module-catalog/ledger.md` | A-06-02 | historical arc / `arc06-module-catalog` | Child work and deferred findings are accounted for at arc composition | R-02 | Arc candidate deferred; children remain unaccepted. CRC performs child carryover review, then CDC assigns the successor before feature execution.

## Source and candidate-report cross-check

The six historical slice ledgers each retain two rows with the same two exact
criteria above. The six arc ledgers each retain the exact capability row and
the exact child/deferred composition row above. Candidate reports exist for
`arc03` slice01 and slice02, `arc04` slice01, slice02 and slice03, and `arc06`
slice01. Each says proposed only, keeps both rows open, proposes both rows as
deferred, and requires CRC review before close. The candidate reports are:

* `arc03-events-and-literate-patching/slice01-notifications/closing-report.md`
* `arc03-events-and-literate-patching/slice02-literate-patching/closing-report.md`
* `arc04-rackgraph/slice01-foundation/closing-report.md`
* `arc04-rackgraph/slice02-navigation/closing-report.md`
* `arc04-rackgraph/slice03-diagnostics/closing-report.md`
* `arc06-module-catalog/slice01-turing-machine/closing-report.md`

No row was dropped, renamed, merged, or upgraded. The source ledgers and
candidate reports remain historical/current proposals; this reconciliation is
attested evidence for CRC and does not create a verifier record.
